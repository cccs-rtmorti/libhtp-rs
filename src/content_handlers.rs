use crate::error::Result;
use crate::multipart::Flags;
use crate::{bstr, multipart, transaction, urlencoded, Status};

/// This callback function feeds request body data to a Urlencoded parser
/// and, later, feeds the parsed parameters to the correct structures.
///
/// Returns HTP_OK on success, HTP_ERROR on failure.
pub fn htp_ch_urlencoded_callback_request_body_data(d: *mut transaction::Data) -> Result<()> {
    unsafe {
        let tx = (*d).tx().as_mut().ok_or(Status::ERROR)?;
        if !(*d).data().is_null() {
            let data = std::slice::from_raw_parts((*d).data(), (*d).len());
            // Process one chunk of data.
            if let Some(urlenp) = (*tx).request_urlenp_body.as_mut() {
                urlencoded::urlenp_parse_partial(urlenp, data);
            }
        } else {
            // Finalize parsing.
            if let Some(urlenp) = (*tx).request_urlenp_body.as_mut() {
                urlencoded::urlenp_finalize(urlenp);
            }
            if let Some(urlenp) = (*tx).request_urlenp_body.clone() {
                // Add all parameters to the transaction.
                for (name, value) in urlenp.params.elements.iter() {
                    let param = transaction::Param::new(
                        bstr::Bstr::from((*name).as_slice()),
                        bstr::Bstr::from((*value).as_slice()),
                        transaction::htp_data_source_t::HTP_SOURCE_BODY,
                        transaction::htp_parser_id_t::HTP_PARSER_URLENCODED,
                    );
                    tx.req_add_param(param)?;
                }
            }
            if let Some(urlenp) = (*tx).request_urlenp_body.as_mut() {
                // All the parameter data is now owned by the transaction, and
                // the parser table used to store it is no longer needed
                urlenp.params.elements.clear();
            }
        }
    }
    Ok(())
}

/// Determine if the request has a Urlencoded body, and, if it does, create and
/// attach an instance of the Urlencoded parser to the transaction.
///
/// Returns HTP_OK if a new parser has been setup, HTP_DECLINED if the MIME type
///         is not appropriate for this parser, and HTP_ERROR on failure.
pub fn htp_ch_urlencoded_callback_request_headers(tx: *mut transaction::Transaction) -> Result<()> {
    unsafe {
        // Check the request content type to see if it matches our MIME type.
        if !(*tx)
            .request_content_type
            .as_ref()
            .ok_or(Status::DECLINED)?
            .starts_with("application/x-www-form-urlencoded")
        {
            return Err(Status::DECLINED);
        }
        // Create parser instance.
        (*tx).request_urlenp_body = Some(urlencoded::UrlEncodedParser::new(tx));
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
pub fn htp_ch_urlencoded_callback_request_line(tx: *mut transaction::Transaction) -> Result<()> {
    unsafe {
        let tx = tx.as_mut().ok_or(Status::ERROR)?;
        // Proceed only if there's something for us to parse.
        if (*tx)
            .parsed_uri
            .as_ref()
            .and_then(|parsed_uri| parsed_uri.query.as_ref())
            .map(|query| query.len() == 0)
            .unwrap_or(true)
        {
            return Err(Status::DECLINED);
        }
        // We have a non-zero length query string.
        let mut urlenp = urlencoded::UrlEncodedParser::new(tx);
        if let Some(query) = (*tx)
            .parsed_uri
            .as_ref()
            .and_then(|parsed_uri| parsed_uri.query.as_ref())
        {
            urlencoded::urlenp_parse_complete(&mut urlenp, query.as_slice());
        }

        // Add all parameters to the transaction.
        for (name, value) in urlenp.params.elements.iter() {
            let param = transaction::Param::new(
                bstr::Bstr::from(name.as_slice()),
                bstr::Bstr::from(value.as_slice()),
                transaction::htp_data_source_t::HTP_SOURCE_QUERY_STRING,
                transaction::htp_parser_id_t::HTP_PARSER_URLENCODED,
            );
            tx.req_add_param(param)?;
        }
    }
    Ok(())
}

/// Finalize Multipart processing.
///
/// Returns HTP_OK on success, HTP_ERROR on failure.
pub fn htp_ch_multipart_callback_request_body_data(d: *mut transaction::Data) -> Result<()> {
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
                let body: *mut multipart::Multipart = parser.get_multipart();
                for part in &(*body).parts {
                    // Use text parameters.
                    if (*(*part)).type_0 == multipart::htp_multipart_type_t::MULTIPART_PART_TEXT {
                        let param = transaction::Param::new(
                            bstr::Bstr::from((*(*(*part)).name).as_slice()),
                            bstr::Bstr::from((*(*(*part)).value).as_slice()),
                            transaction::htp_data_source_t::HTP_SOURCE_BODY,
                            transaction::htp_parser_id_t::HTP_PARSER_MULTIPART,
                        );
                        tx.req_add_param(param)?;
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
pub fn htp_ch_multipart_callback_request_headers(tx: *mut transaction::Transaction) -> Result<()> {
    unsafe {
        // The field request_content_type does not contain the entire C-T
        // value and so we cannot use it to look for a boundary, but we can
        // use it for a quick check to determine if the C-T header exists.
        if (*tx).request_content_type.is_none() {
            return Err(Status::DECLINED);
        }
        // Look for a boundary.
        let ct = if let Some((_, ct)) = (*tx).request_headers.get_nocase_nozero_mut("content-type")
        {
            ct
        } else {
            return Err(Status::ERROR);
        };
        let mut flags = Flags::empty();
        if let Some(boundary) = multipart::find_boundary(&(*(*ct).value).as_slice(), &mut flags) {
            // Create a Multipart parser instance.
            (*tx).request_mpartp = multipart::Parser::new((*(*tx).connp).cfg, boundary, flags);
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
