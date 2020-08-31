use crate::htp_multipart::MultipartFlags;
use crate::{bstr, htp_multipart, htp_transaction, htp_urlencoded, Status};

extern "C" {
    #[no_mangle]
    fn calloc(_: libc::size_t, _: libc::size_t) -> *mut core::ffi::c_void;
    #[no_mangle]
    fn free(__ptr: *mut core::ffi::c_void);
}

/// This callback function feeds request body data to a Urlencoded parser
/// and, later, feeds the parsed parameters to the correct structures.
///
/// Returns HTP_OK on success, HTP_ERROR on failure.
#[no_mangle]
pub unsafe extern "C" fn htp_ch_urlencoded_callback_request_body_data(
    d: *mut htp_transaction::htp_tx_data_t,
) -> Status {
    let tx = if let Some(tx) = (*d).tx().as_mut() {
        tx
    } else {
        return Status::ERROR;
    };
    if !(*d).data().is_null() {
        // Process one chunk of data.
        htp_urlencoded::htp_urlenp_parse_partial(
            (*tx).request_urlenp_body,
            (*d).data() as *const core::ffi::c_void,
            (*d).len(),
        );
    } else {
        // Finalize parsing.
        htp_urlencoded::htp_urlenp_finalize((*tx).request_urlenp_body);
        // Add all parameters to the transaction.
        for (name, value) in (*(*tx).request_urlenp_body).params.elements.iter() {
            let param = htp_transaction::htp_param_t::new(
                bstr::bstr_t::from((*name).as_slice()),
                bstr::bstr_t::from((*value).as_slice()),
                htp_transaction::htp_data_source_t::HTP_SOURCE_BODY,
                htp_transaction::htp_parser_id_t::HTP_PARSER_URLENCODED,
            );
            if tx.req_add_param(param).is_err() {
                return Status::ERROR;
            }
        }
        // All the parameter data is now owned by the transaction, and
        // the parser table used to store it is no longer needed
        (*(*tx).request_urlenp_body).params.elements.clear();
    }
    Status::OK
}

/// Determine if the request has a Urlencoded body, and, if it does, create and
/// attach an instance of the Urlencoded parser to the transaction.
///
/// Returns HTP_OK if a new parser has been setup, HTP_DECLINED if the MIME type
///         is not appropriate for this parser, and HTP_ERROR on failure.
#[no_mangle]
pub unsafe extern "C" fn htp_ch_urlencoded_callback_request_headers(
    tx: *mut htp_transaction::htp_tx_t,
) -> Status {
    // Check the request content type to see if it matches our MIME type.
    if (*tx).request_content_type.is_null()
        || !(*(*tx).request_content_type).starts_with("application/x-www-form-urlencoded")
    {
        return Status::DECLINED;
    }
    // Create parser instance.
    (*tx).request_urlenp_body = htp_urlencoded::htp_urlenp_create(tx);
    if (*tx).request_urlenp_body.is_null() {
        return Status::ERROR;
    }
    // Register a request body data callback.
    (*tx)
        .hook_request_body_data
        .register_extern(htp_ch_urlencoded_callback_request_body_data);
    Status::OK
}

/// Parses request query string, if present.
///
/// Returns HTP_OK if query string was parsed, HTP_DECLINED if there was no query
///         string, and HTP_ERROR on failure.
#[no_mangle]
pub unsafe extern "C" fn htp_ch_urlencoded_callback_request_line(
    tx: *mut htp_transaction::htp_tx_t,
) -> Status {
    let tx = if let Some(tx) = tx.as_mut() {
        tx
    } else {
        return Status::ERROR;
    };
    // Proceed only if there's something for us to parse.
    if (*(*tx).parsed_uri).query.is_null() || bstr::bstr_len((*(*tx).parsed_uri).query) == 0 {
        return Status::DECLINED;
    }
    // We have a non-zero length query string.
    (*tx).request_urlenp_query = htp_urlencoded::htp_urlenp_create(tx);
    if (*tx).request_urlenp_query.is_null() {
        return Status::ERROR;
    }
    if htp_urlencoded::htp_urlenp_parse_complete(
        (*tx).request_urlenp_query,
        bstr::bstr_ptr((*(*tx).parsed_uri).query) as *const core::ffi::c_void,
        bstr::bstr_len((*(*tx).parsed_uri).query),
    ) != Status::OK
    {
        htp_urlencoded::htp_urlenp_destroy((*tx).request_urlenp_query);
        return Status::ERROR;
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
            return Status::ERROR;
        }
    }
    // All the parameter data is now owned by the transaction, and
    // the parser table used to store it is no longer needed.
    (*(*tx).request_urlenp_query).params.elements.clear();
    Status::OK
}

/// Finalize Multipart processing.
///
/// Returns HTP_OK on success, HTP_ERROR on failure.
#[no_mangle]
pub unsafe extern "C" fn htp_ch_multipart_callback_request_body_data(
    d: *mut htp_transaction::htp_tx_data_t,
) -> Status {
    let tx = if let Some(tx) = (*d).tx().as_mut() {
        tx
    } else {
        return Status::ERROR;
    };
    if !(*d).data().is_null() {
        // Process one chunk of data.
        let data = std::slice::from_raw_parts((*d).data(), (*d).len());
        htp_multipart::htp_mpartp_parse(&mut *(*tx).request_mpartp, data);
    } else {
        // Finalize parsing.
        htp_multipart::htp_mpartp_finalize(&mut *(*tx).request_mpartp);
        let body: *mut htp_multipart::htp_multipart_t =
            htp_multipart::htp_mpartp_get_multipart((*tx).request_mpartp);
        for part in &(*body).parts {
            // Use text parameters.
            if (*(*part)).type_0 == htp_multipart::htp_multipart_type_t::MULTIPART_PART_TEXT {
                let param = htp_transaction::htp_param_t::new(
                    bstr::bstr_t::from((*(*(*part)).name).as_slice()),
                    bstr::bstr_t::from((*(*(*part)).value).as_slice()),
                    htp_transaction::htp_data_source_t::HTP_SOURCE_BODY,
                    htp_transaction::htp_parser_id_t::HTP_PARSER_MULTIPART,
                );
                if tx.req_add_param(param).is_err() {
                    return Status::ERROR;
                }
            }
        }
    }
    Status::OK
}

/// Inspect request headers and register the Multipart request data hook
/// if it contains a multipart/form-data body.
///
/// Returns HTP_OK if a new parser has been setup, HTP_DECLINED if the MIME type
///         is not appropriate for this parser, and HTP_ERROR on failure.
#[no_mangle]
pub unsafe extern "C" fn htp_ch_multipart_callback_request_headers(
    tx: *mut htp_transaction::htp_tx_t,
) -> Status {
    // The field request_content_type does not contain the entire C-T
    // value and so we cannot use it to look for a boundary, but we can
    // use it for a quick check to determine if the C-T header exists.
    if (*tx).request_content_type.is_null() {
        return Status::DECLINED;
    }
    // Look for a boundary.
    let ct = if let Some((_, ct)) = (*tx).request_headers.get_nocase_nozero_mut("content-type") {
        ct
    } else {
        return Status::ERROR;
    };
    let mut flags: MultipartFlags = MultipartFlags::empty();
    if let Some(boundary) =
        htp_multipart::htp_mpartp_find_boundary(&(*(*ct).value).as_slice(), &mut flags)
    {
        // Create a Multipart parser instance.
        (*tx).request_mpartp =
            htp_multipart::htp_mpartp_create((*(*tx).connp).cfg, boundary, flags);
        if (*tx).request_mpartp.is_null() {
            return Status::ERROR;
        }
        // Register a request body data callback.
        (*tx)
            .hook_request_body_data
            .register_extern(htp_ch_multipart_callback_request_body_data);
    } else {
        // No boundary
        return Status::DECLINED;
    }
    Status::OK
}
