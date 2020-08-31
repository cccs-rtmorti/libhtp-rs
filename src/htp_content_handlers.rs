use crate::error::Result;
use crate::htp_multipart::MultipartFlags;
use crate::{bstr, htp_multipart, htp_transaction, htp_urlencoded, Status};

/// This callback function feeds request body data to a Urlencoded parser
/// and, later, feeds the parsed parameters to the correct structures.
///
/// Returns HTP_OK on success, HTP_ERROR on failure.
pub fn htp_ch_urlencoded_callback_request_body_data(
    d: *mut htp_transaction::htp_tx_data_t,
) -> Result<()> {
    unsafe {
        let tx = (*d).tx().as_mut().ok_or(Status::ERROR)?;
        if !(*d).data().is_null() {
            let data = std::slice::from_raw_parts((*d).data(), (*d).len());
            // Process one chunk of data.
            htp_urlencoded::htp_urlenp_parse_partial(&mut *(*tx).request_urlenp_body, data);
        } else {
            // Finalize parsing.
            htp_urlencoded::htp_urlenp_finalize(&mut *(*tx).request_urlenp_body);
            // Add all parameters to the transaction.
            for (name, value) in (*(*tx).request_urlenp_body).params.elements.iter() {
                let param = htp_transaction::htp_param_t::new(
                    bstr::bstr_t::from((*name).as_slice()),
                    bstr::bstr_t::from((*value).as_slice()),
                    htp_transaction::htp_data_source_t::HTP_SOURCE_BODY,
                    htp_transaction::htp_parser_id_t::HTP_PARSER_URLENCODED,
                );
                if tx.req_add_param(param).is_err() {
                    return Err(Status::ERROR);
                }
            }
            // All the parameter data is now owned by the transaction, and
            // the parser table used to store it is no longer needed
            (*(*tx).request_urlenp_body).params.elements.clear();
        }
    }
    Ok(())
}

/// Determine if the request has a Urlencoded body, and, if it does, create and
/// attach an instance of the Urlencoded parser to the transaction.
///
/// Returns HTP_OK if a new parser has been setup, HTP_DECLINED if the MIME type
///         is not appropriate for this parser, and HTP_ERROR on failure.
pub fn htp_ch_urlencoded_callback_request_headers(
    tx: *mut htp_transaction::htp_tx_t,
) -> Result<()> {
    unsafe {
        // Check the request content type to see if it matches our MIME type.
        if (*tx).request_content_type.is_null()
            || !(*(*tx).request_content_type).starts_with("application/x-www-form-urlencoded")
        {
            return Err(Status::DECLINED);
        }
        // Create parser instance.
        (*tx).request_urlenp_body = htp_urlencoded::htp_urlenp_create(tx);
        if (*tx).request_urlenp_body.is_null() {
            return Err(Status::ERROR);
        }
        // Register a request body data callback.
        (*tx)
            .hook_request_body_data
            .register(htp_ch_urlencoded_callback_request_body_data);
    }
    Ok(())
}

/// Parses request query string, if present.
///
/// Returns HTP_OK if query string was parsed, HTP_DECLINED if there was no query
///         string, and HTP_ERROR on failure.
pub fn htp_ch_urlencoded_callback_request_line(tx: *mut htp_transaction::htp_tx_t) -> Result<()> {
    unsafe {
        let tx = tx.as_mut().ok_or(Status::ERROR)?;
        // Proceed only if there's something for us to parse.
        if (*(*tx).parsed_uri)
            .query
            .as_ref()
            .map(|query| query.len() == 0)
            .unwrap_or(true)
        {
            return Err(Status::DECLINED);
        }
        // We have a non-zero length query string.
        (*tx).request_urlenp_query = htp_urlencoded::htp_urlenp_create(tx);
        if (*tx).request_urlenp_query.is_null() {
            return Err(Status::ERROR);
        }
        if let Some(query) = (*(*tx).parsed_uri).query.as_ref() {
            htp_urlencoded::htp_urlenp_parse_complete(
                &mut *(*tx).request_urlenp_query,
                query.as_slice(),
            );
        }
        // Add all parameters to the transaction.
        for (name, value) in (*(*tx).request_urlenp_query).params.elements.iter() {
            let param = htp_transaction::htp_param_t::new(
                bstr::bstr_t::from(name.as_slice()),
                bstr::bstr_t::from(value.as_slice()),
                htp_transaction::htp_data_source_t::HTP_SOURCE_QUERY_STRING,
                htp_transaction::htp_parser_id_t::HTP_PARSER_URLENCODED,
            );
            if tx.req_add_param(param).is_err() {
                return Err(Status::ERROR);
            }
        }
        // All the parameter data is now owned by the transaction, and
        // the parser table used to store it is no longer needed.
        (*(*tx).request_urlenp_query).params.elements.clear();
    }
    Ok(())
}

/// Finalize Multipart processing.
///
/// Returns HTP_OK on success, HTP_ERROR on failure.
pub fn htp_ch_multipart_callback_request_body_data(
    d: *mut htp_transaction::htp_tx_data_t,
) -> Result<()> {
    unsafe {
        let tx = (*d).tx().as_mut().ok_or(Status::ERROR)?;
        if let Some(parser) = &mut (*tx).request_mpartp {
            if !(*d).data().is_null() {
                // Process one chunk of data.
                let data = std::slice::from_raw_parts((*d).data(), (*d).len());
                parser.parse(data);
            } else {
                // Finalize parsing.
                // Ignore result.
                let _ = parser.finalize();
                let body: *mut htp_multipart::htp_multipart_t = parser.get_multipart();
                for part in &(*body).parts {
                    // Use text parameters.
                    if (*(*part)).type_0 == htp_multipart::htp_multipart_type_t::MULTIPART_PART_TEXT
                    {
                        let param = htp_transaction::htp_param_t::new(
                            bstr::bstr_t::from((*(*(*part)).name).as_slice()),
                            bstr::bstr_t::from((*(*(*part)).value).as_slice()),
                            htp_transaction::htp_data_source_t::HTP_SOURCE_BODY,
                            htp_transaction::htp_parser_id_t::HTP_PARSER_MULTIPART,
                        );
                        if tx.req_add_param(param).is_err() {
                            return Err(Status::ERROR);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

/// Inspect request headers and register the Multipart request data hook
/// if it contains a multipart/form-data body.
///
/// Returns HTP_OK if a new parser has been setup, HTP_DECLINED if the MIME type
///         is not appropriate for this parser, and HTP_ERROR on failure.
pub fn htp_ch_multipart_callback_request_headers(tx: *mut htp_transaction::htp_tx_t) -> Result<()> {
    unsafe {
        // The field request_content_type does not contain the entire C-T
        // value and so we cannot use it to look for a boundary, but we can
        // use it for a quick check to determine if the C-T header exists.
        if (*tx).request_content_type.is_null() {
            return Err(Status::DECLINED);
        }
        // Look for a boundary.
        let ct = if let Some((_, ct)) = (*tx).request_headers.get_nocase_nozero_mut("content-type")
        {
            ct
        } else {
            return Err(Status::ERROR);
        };
        let mut flags: MultipartFlags = MultipartFlags::empty();
        if let Some(boundary) =
            htp_multipart::htp_mpartp_find_boundary(&(*(*ct).value).as_slice(), &mut flags)
        {
            // Create a Multipart parser instance.
            (*tx).request_mpartp =
                htp_multipart::htp_mpartp_t::new((*(*tx).connp).cfg, boundary, flags);
            if (*tx).request_mpartp.is_none() {
                return Err(Status::ERROR);
            }
            // Register a request body data callback.
            (*tx)
                .hook_request_body_data
                .register(htp_ch_multipart_callback_request_body_data);
            Ok(())
        } else {
            // No boundary
            Err(Status::DECLINED)
        }
    }
}
