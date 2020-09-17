use crate::bstr::{bstr_len, bstr_ptr};
use crate::error::Result;
use crate::hook::DataHook;
use crate::htp_connection_parser::State;
use crate::htp_transaction::Protocol;
use crate::htp_util::Flags;
use crate::{
    bstr, htp_connection_parser, htp_decompressors, htp_request, htp_transaction, htp_util, Status,
};
use std::cmp::Ordering;

pub type htp_time_t = libc::timeval;

/// Sends outstanding connection data to the currently active data receiver hook.
///
/// Returns HTP_OK, or a value returned from a callback.
unsafe fn htp_connp_res_receiver_send_data(
    connp: &mut htp_connection_parser::htp_connp_t,
    is_last: bool,
) -> Result<()> {
    let mut data = htp_transaction::htp_tx_data_t::new(
        (*connp).out_tx_mut_ptr(),
        (*connp)
            .out_current_data
            .offset((*connp).out_current_receiver_offset as isize),
        ((*connp).out_current_read_offset - (*connp).out_current_receiver_offset) as usize,
        is_last,
    );
    if let Some(hook) = &(*connp).out_data_receiver_hook {
        hook.run_all(&mut data)?;
    } else {
        return Ok(());
    };
    (*connp).out_current_receiver_offset = (*connp).out_current_read_offset;
    Ok(())
}

/// Finalizes an existing data receiver hook by sending any outstanding data to it. The
/// hook is then removed so that it receives no more data.
///
/// Returns HTP_OK, or a value returned from a callback.
pub unsafe fn htp_connp_res_receiver_finalize_clear(
    connp: &mut htp_connection_parser::htp_connp_t,
) -> Result<()> {
    if (*connp).out_data_receiver_hook.is_none() {
        return Ok(());
    }
    let rc = htp_connp_res_receiver_send_data(connp, true);
    (*connp).out_data_receiver_hook = None;
    rc
}

/// Configures the data receiver hook. If there is a previous hook, it will be finalized and cleared.
///
/// Returns HTP_OK, or a value returned from a callback.
unsafe fn htp_connp_res_receiver_set(
    connp: &mut htp_connection_parser::htp_connp_t,
    data_receiver_hook: Option<DataHook>,
) -> Result<()> {
    // Ignore result.
    let _ = htp_connp_res_receiver_finalize_clear(connp);
    (*connp).out_data_receiver_hook = data_receiver_hook;
    (*connp).out_current_receiver_offset = (*connp).out_current_read_offset;
    Ok(())
}

/// Handles request parser state changes. At the moment, this function is used only
/// to configure data receivers, which are sent raw connection data.
///
/// Returns HTP_OK, or a value returned from a callback.
unsafe fn htp_res_handle_state_change(
    connp: &mut htp_connection_parser::htp_connp_t,
) -> Result<()> {
    if (*connp).out_state_previous == (*connp).out_state {
        return Ok(());
    }
    if (*connp).out_state == State::HEADERS {
        let header_fn = Some(
            (*connp.out_tx_mut_ok()?.cfg)
                .hook_response_header_data
                .clone(),
        );
        let trailer_fn = Some(
            (*connp.out_tx_mut_ok()?.cfg)
                .hook_response_trailer_data
                .clone(),
        );
        match connp.out_tx_mut_ok()?.response_progress {
            htp_transaction::htp_tx_res_progress_t::HTP_RESPONSE_HEADERS => {
                htp_connp_res_receiver_set(connp, header_fn)
            }
            htp_transaction::htp_tx_res_progress_t::HTP_RESPONSE_TRAILER => {
                htp_connp_res_receiver_set(connp, trailer_fn)
            }
            _ => Ok(()),
        }?;
    }
    // Same comment as in htp_req_handle_state_change(). Below is a copy.
    // Initially, I had the finalization of raw data sending here, but that
    // caused the last REQUEST_HEADER_DATA hook to be invoked after the
    // REQUEST_HEADERS hook -- which I thought made no sense. For that reason,
    // the finalization is now initiated from the request header processing code,
    // which is less elegant but provides a better user experience. Having some
    // (or all) hooks to be invoked on state change might work better.
    (*connp).out_state_previous = (*connp).out_state;
    Ok(())
}

/// If there is any data left in the outbound data chunk, this function will preserve
/// it for later consumption. The maximum amount accepted for buffering is controlled
/// by htp_config_t::field_limit_hard.
///
/// Returns HTP_OK, or HTP_ERROR on fatal failure.
unsafe fn htp_connp_res_buffer(connp: &mut htp_connection_parser::htp_connp_t) -> Result<()> {
    if (*connp).out_current_data.is_null() {
        return Ok(());
    }
    let data: *mut u8 = (*connp)
        .out_current_data
        .offset((*connp).out_current_consume_offset as isize);
    let len: usize =
        ((*connp).out_current_read_offset - (*connp).out_current_consume_offset) as usize;
    // Check the hard (buffering) limit.
    let mut newlen: usize = (*connp).out_buf.len().wrapping_add(len);
    // When calculating the size of the buffer, take into account the
    // space we're using for the response header buffer.
    if !(*connp).out_header.is_null() {
        newlen = newlen.wrapping_add(bstr_len((*connp).out_header))
    }
    if newlen > (*connp.out_tx_mut_ok()?.cfg).field_limit_hard {
        htp_error!(
            connp as *mut htp_connection_parser::htp_connp_t,
            htp_log_code::RESPONSE_FIELD_TOO_LONG,
            format!(
                "Response the buffer limit: size {} limit {}.",
                newlen,
                (*connp.out_tx_mut_ok()?.cfg).field_limit_hard
            )
        );
        return Err(Status::ERROR);
    }
    // Copy the data remaining in the buffer.
    (*connp).out_buf.add(std::slice::from_raw_parts(data, len));
    // Reset the consumer position.
    (*connp).out_current_consume_offset = (*connp).out_current_read_offset;
    Ok(())
}

/// Returns to the caller the memory region that should be processed next. This function
/// hides away the buffering process from the rest of the code, allowing it to work with
/// non-buffered data that's in the outbound chunk, or buffered data that's in our structures.
///
/// Returns HTP_OK
unsafe fn htp_connp_res_consolidate_data(
    connp: &mut htp_connection_parser::htp_connp_t,
    data: *mut *mut u8,
    len: *mut usize,
) -> Result<()> {
    if (*connp).out_buf.is_empty() {
        // We do not have any data buffered; point to the current data chunk.
        *data = (*connp)
            .out_current_data
            .offset((*connp).out_current_consume_offset as isize);
        *len = ((*connp).out_current_read_offset - (*connp).out_current_consume_offset) as usize
    } else {
        // We do have data in the buffer. Add data from the current
        // chunk, and point to the consolidated buffer.
        htp_connp_res_buffer(connp)?;
        *data = (*connp).out_buf.as_mut_ptr();
        *len = (*connp).out_buf.len();
    }
    Ok(())
}

/// Clears buffered outbound data and resets the consumer position to the reader position.
unsafe fn htp_connp_res_clear_buffer(connp: &mut htp_connection_parser::htp_connp_t) {
    (*connp).out_current_consume_offset = (*connp).out_current_read_offset;
    (*connp).out_buf.clear()
}

/// Consumes bytes until the end of the current line.
///
/// Returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
pub unsafe extern "C" fn htp_connp_RES_BODY_CHUNKED_DATA_END(
    connp: &mut htp_connection_parser::htp_connp_t,
) -> Result<()> {
    loop
    // TODO We shouldn't really see anything apart from CR and LF,
    //      so we should warn about anything else.
    {
        if (*connp).out_current_read_offset < (*connp).out_current_len {
            (*connp).out_next_byte = *(*connp)
                .out_current_data
                .offset((*connp).out_current_read_offset as isize)
                as i32;
            (*connp).out_current_read_offset += 1;
            (*connp).out_current_consume_offset += 1;
            (*connp).out_stream_offset += 1
        } else {
            return Err(Status::DATA);
        }
        connp.out_tx_mut_ok()?.response_message_len += 1;
        if (*connp).out_next_byte == '\n' as i32 {
            (*connp).out_state = State::BODY_CHUNKED_LENGTH;
            return Ok(());
        }
    }
}

/// Processes a chunk of data.
///
/// Returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
pub unsafe extern "C" fn htp_connp_RES_BODY_CHUNKED_DATA(
    connp: &mut htp_connection_parser::htp_connp_t,
) -> Result<()> {
    let mut bytes_to_consume: usize = 0;
    // Determine how many bytes we can consume.
    if (*connp).out_current_len - (*connp).out_current_read_offset >= (*connp).out_chunked_length {
        bytes_to_consume = (*connp).out_chunked_length as usize
    } else {
        bytes_to_consume = ((*connp).out_current_len - (*connp).out_current_read_offset) as usize
    }
    if bytes_to_consume == 0 {
        return Err(Status::DATA);
    }
    // Consume the data.
    (*connp).res_process_body_data_ex(
        (*connp)
            .out_current_data
            .offset((*connp).out_current_read_offset as isize) as *const core::ffi::c_void,
        bytes_to_consume,
    )?;
    // Adjust the counters.
    (*connp).out_current_read_offset =
        ((*connp).out_current_read_offset as u64).wrapping_add(bytes_to_consume as u64) as i64;
    (*connp).out_current_consume_offset =
        ((*connp).out_current_consume_offset as u64).wrapping_add(bytes_to_consume as u64) as i64;
    (*connp).out_stream_offset =
        ((*connp).out_stream_offset as u64).wrapping_add(bytes_to_consume as u64) as i64;
    (*connp).out_chunked_length =
        ((*connp).out_chunked_length as u64).wrapping_sub(bytes_to_consume as u64) as i64;
    // Have we seen the entire chunk?
    if (*connp).out_chunked_length == 0 {
        (*connp).out_state = State::BODY_CHUNKED_DATA_END;
        return Ok(());
    }
    Err(Status::DATA)
}

/// Peeks ahead into the data to try to see if it starts with a valid Chunked
/// length field.
///
/// Returns 1 if it looks valid, 0 if it looks invalid
#[inline]
unsafe fn data_probe_chunk_length(connp: &mut htp_connection_parser::htp_connp_t) -> i32 {
    if (*connp).out_current_read_offset - (*connp).out_current_consume_offset < 8 {
        // not enough data so far, consider valid still
        return 1;
    }
    let data: *mut u8 = (*connp)
        .out_current_data
        .offset((*connp).out_current_consume_offset as isize);
    let len: usize =
        ((*connp).out_current_read_offset - (*connp).out_current_consume_offset) as usize;
    let mut i: usize = 0;
    while i < len {
        let c: u8 = *data.offset(i as isize);
        if c == 0xd || c == 0xa || c == 0x20 || c == 0x9 || c == 0xb || c == 0xc {
        } else if c.is_ascii_digit()
            || c >= 'a' as u8 && c <= 'f' as u8
            || c >= 'A' as u8 && c <= 'F' as u8
        {
            // real chunklen char
            return 1;
        } else {
            // leading junk, bad
            return 0;
        }
        i = i.wrapping_add(1)
    }
    1
}

/// Extracts chunk length.
///
/// Returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
pub unsafe extern "C" fn htp_connp_RES_BODY_CHUNKED_LENGTH(
    connp: &mut htp_connection_parser::htp_connp_t,
) -> Result<()> {
    loop {
        if (*connp).out_current_read_offset < (*connp).out_current_len {
            (*connp).out_next_byte = *(*connp)
                .out_current_data
                .offset((*connp).out_current_read_offset as isize)
                as i32;
            (*connp).out_current_read_offset += 1;
            (*connp).out_stream_offset += 1
        } else {
            return Err(Status::DATA_BUFFER);
        }
        // Have we reached the end of the line? Or is this not chunked after all?
        if !((*connp).out_next_byte == '\n' as i32 || data_probe_chunk_length(connp) == 0) {
            continue;
        }
        let mut data: *mut u8 = 0 as *mut u8;
        let mut len: usize = 0;
        htp_connp_res_consolidate_data(connp, &mut data, &mut len)?;
        connp.out_tx_mut_ok()?.response_message_len =
            (connp.out_tx_mut_ok()?.response_message_len as u64).wrapping_add(len as u64) as i64;

        let buf: &mut [u8] = std::slice::from_raw_parts_mut(data, len);
        if let Ok(chunked_length) = htp_util::htp_parse_chunked_length(buf) {
            if let Some(chunked_length) = chunked_length {
                (*connp).out_chunked_length = chunked_length as i64;
            } else {
                // empty chunk length line, lets try to continue
                continue;
            }
        } else {
            (*connp).out_chunked_length = -1;
        }
        if (*connp).out_chunked_length < 0 {
            // reset out_current_read_offset so htp_connp_RES_BODY_IDENTITY_STREAM_CLOSE
            // doesn't miss the first bytes
            if len > (*connp).out_current_read_offset as usize {
                (*connp).out_current_read_offset = 0
            } else {
                (*connp).out_current_read_offset =
                    ((*connp).out_current_read_offset as u64).wrapping_sub(len as u64) as i64
            }
            (*connp).out_state = State::BODY_IDENTITY_STREAM_CLOSE;
            connp.out_tx_mut_ok()?.response_transfer_coding =
                htp_transaction::htp_transfer_coding_t::HTP_CODING_IDENTITY;
            htp_error!(
                connp as *mut htp_connection_parser::htp_connp_t,
                htp_log_code::INVALID_RESPONSE_CHUNK_LEN,
                format!(
                    "Response chunk encoding: Invalid chunk length: {}",
                    (*connp).out_chunked_length
                )
            );
            return Ok(());
        }
        htp_connp_res_clear_buffer(connp);
        // Handle chunk length
        if (*connp).out_chunked_length > 0 {
            // More data available
            (*connp).out_state = State::BODY_CHUNKED_DATA
        } else if (*connp).out_chunked_length == 0 {
            // End of data
            (*connp).out_state = State::HEADERS;
            connp.out_tx_mut_ok()?.response_progress =
                htp_transaction::htp_tx_res_progress_t::HTP_RESPONSE_TRAILER
        }
        return Ok(());
    }
}

/// Processes an identity response body of known length.
///
/// Returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
pub unsafe extern "C" fn htp_connp_RES_BODY_IDENTITY_CL_KNOWN(
    connp: &mut htp_connection_parser::htp_connp_t,
) -> Result<()> {
    let mut bytes_to_consume: usize = 0;
    // Determine how many bytes we can consume.
    if (*connp).out_current_len - (*connp).out_current_read_offset >= (*connp).out_body_data_left {
        bytes_to_consume = (*connp).out_body_data_left as usize
    } else {
        bytes_to_consume = ((*connp).out_current_len - (*connp).out_current_read_offset) as usize
    }
    if (*connp).out_status == htp_connection_parser::htp_stream_state_t::HTP_STREAM_CLOSED {
        (*connp).out_state = State::FINALIZE;
        // Sends close signal to decompressors
        return (*connp).res_process_body_data_ex(0 as *const core::ffi::c_void, 0);
    }
    if bytes_to_consume == 0 {
        return Err(Status::DATA);
    }
    // Consume the data.
    (*connp).res_process_body_data_ex(
        (*connp)
            .out_current_data
            .offset((*connp).out_current_read_offset as isize) as *const core::ffi::c_void,
        bytes_to_consume,
    )?;
    // Adjust the counters.
    (*connp).out_current_read_offset =
        ((*connp).out_current_read_offset as u64).wrapping_add(bytes_to_consume as u64) as i64;
    (*connp).out_current_consume_offset =
        ((*connp).out_current_consume_offset as u64).wrapping_add(bytes_to_consume as u64) as i64;
    (*connp).out_stream_offset =
        ((*connp).out_stream_offset as u64).wrapping_add(bytes_to_consume as u64) as i64;
    (*connp).out_body_data_left =
        ((*connp).out_body_data_left as u64).wrapping_sub(bytes_to_consume as u64) as i64;
    // Have we seen the entire response body?
    if (*connp).out_body_data_left == 0 {
        (*connp).out_state = State::FINALIZE;
        // Tells decompressors to output partially decompressed data
        return (*connp).res_process_body_data_ex(0 as *const core::ffi::c_void, 0);
    }
    Err(Status::DATA)
}

/// Processes identity response body of unknown length. In this case, we assume the
/// response body consumes all data until the end of the stream.
///
/// Returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
pub unsafe extern "C" fn htp_connp_RES_BODY_IDENTITY_STREAM_CLOSE(
    connp: &mut htp_connection_parser::htp_connp_t,
) -> Result<()> {
    // Consume all data from the input buffer.
    let bytes_to_consume: usize =
        ((*connp).out_current_len - (*connp).out_current_read_offset) as usize;
    if bytes_to_consume != 0 {
        (*connp).res_process_body_data_ex(
            (*connp)
                .out_current_data
                .offset((*connp).out_current_read_offset as isize)
                as *const core::ffi::c_void,
            bytes_to_consume,
        )?;
        // Adjust the counters.
        (*connp).out_current_read_offset =
            ((*connp).out_current_read_offset as u64).wrapping_add(bytes_to_consume as u64) as i64;
        (*connp).out_current_consume_offset = ((*connp).out_current_consume_offset as u64)
            .wrapping_add(bytes_to_consume as u64)
            as i64;
        (*connp).out_stream_offset =
            ((*connp).out_stream_offset as u64).wrapping_add(bytes_to_consume as u64) as i64;
    }
    // Have we seen the entire response body?
    if (*connp).out_status == htp_connection_parser::htp_stream_state_t::HTP_STREAM_CLOSED {
        (*connp).out_state = State::FINALIZE;
        return Ok(());
    }
    Err(Status::DATA)
}

/// Determines presence (and encoding) of a response body.
///
/// Returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
pub unsafe extern "C" fn htp_connp_RES_BODY_DETERMINE(
    connp: &mut htp_connection_parser::htp_connp_t,
) -> Result<()> {
    // If the request uses the CONNECT method, then not only are we
    // to assume there's no body, but we need to ignore all
    // subsequent data in the stream.
    if connp.out_tx_mut_ok()?.request_method_number == htp_request::htp_method_t::HTP_M_CONNECT {
        if connp.out_tx_mut_ok()?.response_status_number >= 200
            && connp.out_tx_mut_ok()?.response_status_number <= 299
        {
            // This is a successful CONNECT stream, which means
            // we need to switch into tunneling mode: on the
            // request side we'll now probe the tunnel data to see
            // if we need to parse or ignore it. So on the response
            // side we wrap up the tx and wait.
            (*connp).out_state = State::FINALIZE;
            // we may have response headers
            return (*connp).state_response_headers().into();
        } else if connp.out_tx_mut_ok()?.response_status_number == 407 {
            // proxy telling us to auth
            (*connp).in_status = htp_connection_parser::htp_stream_state_t::HTP_STREAM_DATA
        } else {
            // This is a failed CONNECT stream, which means that
            // we can unblock request parsing
            (*connp).in_status = htp_connection_parser::htp_stream_state_t::HTP_STREAM_DATA;
            // We are going to continue processing this transaction,
            // adding a note for ourselves to stop at the end (because
            // we don't want to see the beginning of a new transaction).
            (*connp).out_data_other_at_tx_end = 1
        }
    }
    let cl_opt = connp
        .out_tx_mut_ok()?
        .response_headers
        .get_nocase_nozero("content-length")
        .map(|(_, val)| val.clone());
    let te_opt = connp
        .out_tx_mut_ok()?
        .response_headers
        .get_nocase_nozero("transfer-encoding")
        .map(|(_, val)| val.clone());
    // Check for "101 Switching Protocol" response.
    // If it's seen, it means that traffic after empty line following headers
    // is no longer HTTP. We can treat it similarly to CONNECT.
    // Unlike CONNECT, however, upgrades from HTTP to HTTP seem
    // rather unlikely, so don't try to probe tunnel for nested HTTP,
    // and switch to tunnel mode right away.
    if connp.out_tx_mut_ok()?.response_status_number == 101 {
        if te_opt.is_none() && cl_opt.is_none() {
            (*connp).out_state = State::FINALIZE;
            (*connp).in_status = htp_connection_parser::htp_stream_state_t::HTP_STREAM_TUNNEL;
            (*connp).out_status = htp_connection_parser::htp_stream_state_t::HTP_STREAM_TUNNEL;
            // we may have response headers
            return (*connp).state_response_headers().into();
        } else {
            htp_warn!(
                connp as *mut htp_connection_parser::htp_connp_t,
                htp_log_code::SWITCHING_PROTO_WITH_CONTENT_LENGTH,
                "Switching Protocol with Content-Length"
            );
        }
    }
    // Check for an interim "100 Continue" response. Ignore it if found, and revert back to RES_LINE.
    if connp.out_tx_mut_ok()?.response_status_number == 100 && te_opt.is_none() && cl_opt.is_none()
    {
        if connp.out_tx_mut_ok()?.seen_100continue != 0 {
            htp_error!(
                connp as *mut htp_connection_parser::htp_connp_t,
                htp_log_code::CONTINUE_ALREADY_SEEN,
                "Already seen 100-Continue."
            );
            return Err(Status::ERROR);
        }
        // Ignore any response headers seen so far.
        connp.out_tx_mut_ok()?.response_headers.elements.clear();
        // Expecting to see another response line next.
        (*connp).out_state = State::LINE;
        connp.out_tx_mut_ok()?.response_progress =
            htp_transaction::htp_tx_res_progress_t::HTP_RESPONSE_LINE;
        connp.out_tx_mut_ok()?.seen_100continue += 1;
        return Ok(());
    }
    // 1. Any response message which MUST NOT include a message-body
    //  (such as the 1xx, 204, and 304 responses and any response to a HEAD
    //  request) is always terminated by the first empty line after the
    //  header fields, regardless of the entity-header fields present in the
    //  message.
    if connp.out_tx_mut_ok()?.request_method_number == htp_request::htp_method_t::HTP_M_HEAD {
        // There's no response body whatsoever
        connp.out_tx_mut_ok()?.response_transfer_coding =
            htp_transaction::htp_transfer_coding_t::HTP_CODING_NO_BODY;
        (*connp).out_state = State::FINALIZE
    } else if connp.out_tx_mut_ok()?.response_status_number >= 100
        && connp.out_tx_mut_ok()?.response_status_number <= 199
        || connp.out_tx_mut_ok()?.response_status_number == 204
        || connp.out_tx_mut_ok()?.response_status_number == 304
    {
        // There should be no response body
        // but browsers interpret content sent by the server as such
        if te_opt.is_none() && cl_opt.is_none() {
            connp.out_tx_mut_ok()?.response_transfer_coding =
                htp_transaction::htp_transfer_coding_t::HTP_CODING_NO_BODY;
            (*connp).out_state = State::FINALIZE
        } else {
            htp_warn!(
                connp as *mut htp_connection_parser::htp_connp_t,
                htp_log_code::RESPONSE_BODY_UNEXPECTED,
                "Unexpected Response body"
            );
        }
    }
    // Hack condition to check that we do not assume "no body"
    if (*connp).out_state != State::FINALIZE {
        // We have a response body
        let ct_opt = connp
            .out_tx_mut_ok()?
            .response_headers
            .get_nocase_nozero("content-type")
            .map(|(_, val)| val.clone());
        if let Some(ct) = &ct_opt {
            let mut response_content_type = bstr::bstr_t::from(ct.value.as_slice());
            response_content_type.make_ascii_lowercase();
            // Ignore parameters
            let data: *mut u8 = response_content_type.as_mut_ptr();
            let len: usize = ct.value.len();
            let mut newlen: usize = 0;
            while newlen < len {
                // TODO Some platforms may do things differently here.
                if htp_util::htp_is_space(*data.offset(newlen as isize))
                    || *data.offset(newlen as isize) as i32 == ';' as i32
                {
                    response_content_type.set_len(newlen);
                    break;
                } else {
                    newlen = newlen.wrapping_add(1)
                }
            }
            connp.out_tx_mut_ok()?.response_content_type = Some(response_content_type);
        }
        // 2. If a Transfer-Encoding header field (section 14.40) is present and
        //   indicates that the "chunked" transfer coding has been applied, then
        //   the length is defined by the chunked encoding (section 3.6).
        if let Some(te) =
            te_opt.and_then(|te| te.value.index_of_nocase_nozero("chunked").and(Some(te)))
        {
            if te.value.cmp_nocase("chunked") != Ordering::Equal {
                htp_warn!(
                    connp as *mut htp_connection_parser::htp_connp_t,
                    htp_log_code::RESPONSE_ABNORMAL_TRANSFER_ENCODING,
                    "Transfer-encoding has abnormal chunked value"
                );
            }
            // 3. If a Content-Length header field (section 14.14) is present, its
            // spec says chunked is HTTP/1.1 only, but some browsers accept it
            // with 1.0 as well
            if connp.out_tx_mut_ok()?.response_protocol_number < Protocol::V1_1 {
                htp_warn!(
                    connp as *mut htp_connection_parser::htp_connp_t,
                    htp_log_code::RESPONSE_CHUNKED_OLD_PROTO,
                    "Chunked transfer-encoding on HTTP/0.9 or HTTP/1.0"
                );
            }
            // If the T-E header is present we are going to use it.
            connp.out_tx_mut_ok()?.response_transfer_coding =
                htp_transaction::htp_transfer_coding_t::HTP_CODING_CHUNKED;
            // We are still going to check for the presence of C-L
            if cl_opt.is_some() {
                // This is a violation of the RFC
                connp.out_tx_mut_ok()?.flags |= Flags::HTP_REQUEST_SMUGGLING
            }
            (*connp).out_state = State::BODY_CHUNKED_LENGTH;
            connp.out_tx_mut_ok()?.response_progress =
                htp_transaction::htp_tx_res_progress_t::HTP_RESPONSE_BODY
        } else if let Some(cl) = cl_opt {
            //   value in bytes represents the length of the message-body.
            // We know the exact length
            connp.out_tx_mut_ok()?.response_transfer_coding =
                htp_transaction::htp_transfer_coding_t::HTP_CODING_IDENTITY;
            // Check for multiple C-L headers
            if cl.flags.contains(Flags::HTP_FIELD_REPEATED) {
                connp.out_tx_mut_ok()?.flags |= Flags::HTP_REQUEST_SMUGGLING
            }
            // Get body length
            if let Some(content_length) =
                htp_util::htp_parse_content_length((*cl.value).as_slice(), Some(&mut *connp))
            {
                connp.out_tx_mut_ok()?.response_content_length = content_length;
                (*connp).out_content_length = connp.out_tx_mut_ok()?.response_content_length;
                (*connp).out_body_data_left = (*connp).out_content_length;
                if (*connp).out_content_length != 0 {
                    (*connp).out_state = State::BODY_IDENTITY_CL_KNOWN;
                    connp.out_tx_mut_ok()?.response_progress =
                        htp_transaction::htp_tx_res_progress_t::HTP_RESPONSE_BODY
                } else {
                    (*connp).out_state = State::FINALIZE
                }
            } else {
                htp_error!(
                    connp as *mut htp_connection_parser::htp_connp_t,
                    htp_log_code::INVALID_CONTENT_LENGTH_FIELD_IN_RESPONSE,
                    format!(
                        "Invalid C-L field in response: {}",
                        connp.out_tx_mut_ok()?.response_content_length
                    )
                );
                return Err(Status::ERROR);
            }
        } else {
            // 4. If the message uses the media type "multipart/byteranges", which is
            //   self-delimiting, then that defines the length. This media type MUST
            //   NOT be used unless the sender knows that the recipient can parse it;
            //   the presence in a request of a Range header with multiple byte-range
            //   specifiers implies that the client can parse multipart/byteranges
            //   responses.
            if let Some(ct) = &ct_opt {
                // TODO Handle multipart/byteranges
                if ct.value.index_of_nocase("multipart/byteranges").is_some() {
                    htp_error!(
                        connp as *mut htp_connection_parser::htp_connp_t,
                        htp_log_code::RESPONSE_MULTIPART_BYTERANGES,
                        "C-T multipart/byteranges in responses not supported"
                    );
                    return Err(Status::ERROR);
                }
            }
            // 5. By the server closing the connection. (Closing the connection
            //   cannot be used to indicate the end of a request body, since that
            //   would leave no possibility for the server to send back a response.)
            (*connp).out_state = State::BODY_IDENTITY_STREAM_CLOSE;
            connp.out_tx_mut_ok()?.response_transfer_coding =
                htp_transaction::htp_transfer_coding_t::HTP_CODING_IDENTITY;
            connp.out_tx_mut_ok()?.response_progress =
                htp_transaction::htp_tx_res_progress_t::HTP_RESPONSE_BODY;
            (*connp).out_body_data_left = -1
        }
    }
    // NOTE We do not need to check for short-style HTTP/0.9 requests here because
    //      that is done earlier, before response line parsing begins
    (*connp).state_response_headers()
}

/// Parses response headers.
///
/// Returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
pub unsafe extern "C" fn htp_connp_RES_HEADERS(
    connp: &mut htp_connection_parser::htp_connp_t,
) -> Result<()> {
    let mut endwithcr: i32 = 0;
    let mut lfcrending: i32 = 0;
    loop {
        if (*connp).out_status == htp_connection_parser::htp_stream_state_t::HTP_STREAM_CLOSED {
            // Finalize sending raw trailer data.
            htp_connp_res_receiver_finalize_clear(connp)?;
            // Run hook response_TRAILER.
            (*(*connp).cfg)
                .hook_response_trailer
                .run_all((*connp).out_tx_mut_ptr())?;
            (*connp).out_state = State::FINALIZE;
            return Ok(());
        }
        if (*connp).out_current_read_offset < (*connp).out_current_len {
            (*connp).out_next_byte = *(*connp)
                .out_current_data
                .offset((*connp).out_current_read_offset as isize)
                as i32;
            (*connp).out_current_read_offset += 1;
            (*connp).out_stream_offset += 1
        } else {
            return Err(Status::DATA_BUFFER);
        }
        // Have we reached the end of the line?
        if (*connp).out_next_byte != '\n' as i32 && (*connp).out_next_byte != '\r' as i32 {
            lfcrending = 0
        } else {
            endwithcr = 0;
            if (*connp).out_next_byte == '\r' as i32 {
                if (*connp).out_current_read_offset >= (*connp).out_current_len {
                    (*connp).out_next_byte = -1
                } else {
                    (*connp).out_next_byte = *(*connp)
                        .out_current_data
                        .offset((*connp).out_current_read_offset as isize)
                        as i32
                }
                if (*connp).out_next_byte == -1 {
                    return Err(Status::DATA_BUFFER);
                } else {
                    if (*connp).out_next_byte == '\n' as i32 {
                        if (*connp).out_current_read_offset < (*connp).out_current_len {
                            (*connp).out_next_byte = *(*connp)
                                .out_current_data
                                .offset((*connp).out_current_read_offset as isize)
                                as i32;
                            (*connp).out_current_read_offset += 1;
                            (*connp).out_stream_offset += 1
                        } else {
                            return Err(Status::DATA_BUFFER);
                        }
                        if lfcrending != 0 {
                            // Handling LFCRCRLFCRLF
                            // These 6 characters mean only 2 end of lines
                            if (*connp).out_current_read_offset >= (*connp).out_current_len {
                                (*connp).out_next_byte = -1
                            } else {
                                (*connp).out_next_byte = *(*connp)
                                    .out_current_data
                                    .offset((*connp).out_current_read_offset as isize)
                                    as i32
                            }
                            if (*connp).out_next_byte == '\r' as i32 {
                                if (*connp).out_current_read_offset < (*connp).out_current_len {
                                    (*connp).out_next_byte = *(*connp)
                                        .out_current_data
                                        .offset((*connp).out_current_read_offset as isize)
                                        as i32;
                                    (*connp).out_current_read_offset += 1;
                                    (*connp).out_stream_offset += 1
                                } else {
                                    return Err(Status::DATA_BUFFER);
                                }
                                (*connp).out_current_consume_offset += 1;
                                if (*connp).out_current_read_offset >= (*connp).out_current_len {
                                    (*connp).out_next_byte = -1
                                } else {
                                    (*connp).out_next_byte = *(*connp)
                                        .out_current_data
                                        .offset((*connp).out_current_read_offset as isize)
                                        as i32
                                }
                                if (*connp).out_next_byte == '\n' as i32 {
                                    if (*connp).out_current_read_offset < (*connp).out_current_len {
                                        (*connp).out_next_byte = *(*connp)
                                            .out_current_data
                                            .offset((*connp).out_current_read_offset as isize)
                                            as i32;
                                        (*connp).out_current_read_offset += 1;
                                        (*connp).out_stream_offset += 1
                                    } else {
                                        return Err(Status::DATA_BUFFER);
                                    }
                                    (*connp).out_current_consume_offset += 1;
                                    htp_warn!(
                                        connp as *mut htp_connection_parser::htp_connp_t,
                                        htp_log_code::DEFORMED_EOL,
                                        "Weird response end of lines mix"
                                    );
                                }
                            }
                        }
                    } else if (*connp).out_next_byte == '\r' as i32 {
                        continue;
                    }
                    lfcrending = 0;
                    endwithcr = 1
                }
            } else {
                // connp->out_next_byte == LF
                if (*connp).out_current_read_offset >= (*connp).out_current_len {
                    (*connp).out_next_byte = -1
                } else {
                    (*connp).out_next_byte = *(*connp)
                        .out_current_data
                        .offset((*connp).out_current_read_offset as isize)
                        as i32
                }
                lfcrending = 0;
                if (*connp).out_next_byte == '\r' as i32 {
                    // hanldes LF-CR sequence as end of line
                    if (*connp).out_current_read_offset < (*connp).out_current_len {
                        (*connp).out_next_byte = *(*connp)
                            .out_current_data
                            .offset((*connp).out_current_read_offset as isize)
                            as i32;
                        (*connp).out_current_read_offset += 1;
                        (*connp).out_stream_offset += 1
                    } else {
                        return Err(Status::DATA_BUFFER);
                    }
                    lfcrending = 1
                }
            }
            let mut data: *mut u8 = 0 as *mut u8;
            let mut len: usize = 0;
            htp_connp_res_consolidate_data(connp, &mut data, &mut len)?;
            // CRCRLF is not an empty line
            if endwithcr != 0 && len < 2 {
                continue;
            }
            let mut next_no_lf: bool = false;
            if (*connp).out_current_read_offset < (*connp).out_current_len
                && *(*connp)
                    .out_current_data
                    .offset((*connp).out_current_read_offset as isize) as i32
                    != '\n' as i32
            {
                next_no_lf = true
            }
            // Should we terminate headers?
            if !data.is_null()
                && htp_util::htp_connp_is_line_terminator(
                    (*(*connp).cfg).server_personality,
                    std::slice::from_raw_parts(data, len),
                    next_no_lf,
                )
            {
                // Parse previous header, if any.
                if !(*connp).out_header.is_null() {
                    if (*connp)
                        .process_response_header(
                            bstr_ptr((*connp).out_header),
                            bstr_len((*connp).out_header),
                        )
                        .is_err()
                    {
                        return Err(Status::ERROR);
                    }
                    bstr::bstr_free((*connp).out_header);
                    (*connp).out_header = 0 as *mut bstr::bstr_t
                }
                htp_connp_res_clear_buffer(connp);
                // We've seen all response headers.
                if connp.out_tx_mut_ok()?.response_progress
                    == htp_transaction::htp_tx_res_progress_t::HTP_RESPONSE_HEADERS
                {
                    // Response headers.
                    // The next step is to determine if this response has a body.
                    (*connp).out_state = State::BODY_DETERMINE
                } else {
                    // Response trailer.
                    // Finalize sending raw trailer data.
                    htp_connp_res_receiver_finalize_clear(connp)?;
                    // Run hook response_TRAILER.
                    (*(*connp).cfg)
                        .hook_response_trailer
                        .run_all((*connp).out_tx_mut_ptr())?;
                    // The next step is to finalize this response.
                    (*connp).out_state = State::FINALIZE
                }
                return Ok(());
            }
            let s = std::slice::from_raw_parts(data as *const u8, len);
            let s = htp_util::htp_chomp(&s);
            len = s.len();
            // Check for header folding.
            if !data.is_null()
                && !htp_util::htp_connp_is_line_folded(std::slice::from_raw_parts(data, len))
            {
                // New header line.
                // Parse previous header, if any.
                if !(*connp).out_header.is_null() {
                    if (*connp)
                        .process_response_header(
                            bstr_ptr((*connp).out_header),
                            bstr_len((*connp).out_header),
                        )
                        .is_err()
                    {
                        return Err(Status::ERROR);
                    }
                    bstr::bstr_free((*connp).out_header);
                    (*connp).out_header = 0 as *mut bstr::bstr_t
                }
                if (*connp).out_current_read_offset >= (*connp).out_current_len {
                    (*connp).out_next_byte = -1
                } else {
                    (*connp).out_next_byte = *(*connp)
                        .out_current_data
                        .offset((*connp).out_current_read_offset as isize)
                        as i32
                }
                if !htp_util::htp_is_folding_char((*connp).out_next_byte as u8) {
                    // Because we know this header is not folded, we can process the buffer straight away.
                    if (*connp).process_response_header(data, len).is_err() {
                        return Err(Status::ERROR);
                    }
                } else {
                    // Keep the partial header data for parsing later.
                    (*connp).out_header = bstr::bstr_dup_mem(data as *const core::ffi::c_void, len);
                    if (*connp).out_header.is_null() {
                        return Err(Status::ERROR);
                    }
                }
            } else if (*connp).out_header.is_null() {
                // Folding; check that there's a previous header line to add to.
                // Invalid folding.
                // Warn only once per transaction.
                if !connp
                    .out_tx_mut_ok()?
                    .flags
                    .contains(Flags::HTP_INVALID_FOLDING)
                {
                    connp.out_tx_mut_ok()?.flags |= Flags::HTP_INVALID_FOLDING;
                    htp_warn!(
                        connp as *mut htp_connection_parser::htp_connp_t,
                        htp_log_code::INVALID_RESPONSE_FIELD_FOLDING,
                        "Invalid response field folding"
                    );
                }
                // Keep the header data for parsing later.
                (*connp).out_header = bstr::bstr_dup_mem(data as *const core::ffi::c_void, len);
                if (*connp).out_header.is_null() {
                    return Err(Status::ERROR);
                }
            } else {
                let mut colon_pos: usize = 0;
                while colon_pos < len && *data.offset(colon_pos as isize) != ':' as u8 {
                    colon_pos = colon_pos.wrapping_add(1)
                }
                if colon_pos < len
                    && bstr::bstr_chr((*connp).out_header, ':' as i32) >= 0
                    && connp.out_tx_mut_ok()?.response_protocol_number == Protocol::V1_1
                {
                    // Warn only once per transaction.
                    if !connp
                        .out_tx_mut_ok()?
                        .flags
                        .contains(Flags::HTP_INVALID_FOLDING)
                    {
                        connp.out_tx_mut_ok()?.flags |= Flags::HTP_INVALID_FOLDING;
                        htp_warn!(
                            connp as *mut htp_connection_parser::htp_connp_t,
                            htp_log_code::INVALID_RESPONSE_FIELD_FOLDING,
                            "Invalid response field folding"
                        );
                    }
                    if (*connp)
                        .process_response_header(
                            bstr_ptr((*connp).out_header),
                            bstr_len((*connp).out_header),
                        )
                        .is_err()
                    {
                        return Err(Status::ERROR);
                    }
                    bstr::bstr_free((*connp).out_header);
                    (*connp).out_header = bstr::bstr_dup_mem(
                        data.offset(1 as isize) as *const core::ffi::c_void,
                        len.wrapping_sub(1),
                    );
                    if (*connp).out_header.is_null() {
                        return Err(Status::ERROR);
                    }
                } else {
                    // Add to the existing header.
                    let new_out_header: *mut bstr::bstr_t = bstr::bstr_add_mem(
                        (*connp).out_header,
                        data as *const core::ffi::c_void,
                        len,
                    );
                    if new_out_header.is_null() {
                        return Err(Status::ERROR);
                    }
                    (*connp).out_header = new_out_header
                }
            }
            htp_connp_res_clear_buffer(connp);
        }
    }
}

/// Parses response line.
///
/// Returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
pub unsafe extern "C" fn htp_connp_RES_LINE(
    connp: &mut htp_connection_parser::htp_connp_t,
) -> Result<()> {
    loop {
        // Don't try to get more data if the stream is closed. If we do, we'll return, asking for more data.
        if (*connp).out_status != htp_connection_parser::htp_stream_state_t::HTP_STREAM_CLOSED {
            // Get one byte
            if (*connp).out_current_read_offset < (*connp).out_current_len {
                (*connp).out_next_byte = *(*connp)
                    .out_current_data
                    .offset((*connp).out_current_read_offset as isize)
                    as i32;
                (*connp).out_current_read_offset += 1;
                (*connp).out_stream_offset += 1
            } else {
                return Err(Status::DATA_BUFFER);
            }
        }
        // Have we reached the end of the line? We treat stream closure as end of line in
        // order to handle the case when the first line of the response is actually response body
        // (and we wish it processed as such).
        if (*connp).out_next_byte == '\r' as i32 {
            if (*connp).out_current_read_offset >= (*connp).out_current_len {
                (*connp).out_next_byte = -1
            } else {
                (*connp).out_next_byte = *(*connp)
                    .out_current_data
                    .offset((*connp).out_current_read_offset as isize)
                    as i32
            }
            if (*connp).out_next_byte == -1 {
                return Err(Status::DATA_BUFFER);
            } else {
                if (*connp).out_next_byte == '\n' as i32 {
                    continue;
                }
                (*connp).out_next_byte = '\n' as i32
            }
        }
        if (*connp).out_next_byte == '\n' as i32
            || (*connp).out_status == htp_connection_parser::htp_stream_state_t::HTP_STREAM_CLOSED
        {
            let mut data: *mut u8 = 0 as *mut u8;
            let mut len: usize = 0;
            htp_connp_res_consolidate_data(connp, &mut data, &mut len)?;
            // Is this a line that should be ignored?
            if !data.is_null()
                && htp_util::htp_connp_is_line_ignorable(
                    (*(*connp).cfg).server_personality,
                    std::slice::from_raw_parts(data, len),
                )
            {
                if (*connp).out_status
                    == htp_connection_parser::htp_stream_state_t::HTP_STREAM_CLOSED
                {
                    (*connp).out_state = State::FINALIZE
                }
                // We have an empty/whitespace line, which we'll note, ignore and move on
                connp.out_tx_mut_ok()?.response_ignored_lines = connp
                    .out_tx_mut_ok()?
                    .response_ignored_lines
                    .wrapping_add(1);
                // TODO How many lines are we willing to accept?
                // Start again
                htp_connp_res_clear_buffer(connp);
                return Ok(());
            }
            // Deallocate previous response line allocations, which we would have on a 100 response.
            connp.out_tx_mut_ok()?.response_line = None;
            connp.out_tx_mut_ok()?.response_protocol = None;
            connp.out_tx_mut_ok()?.response_status = None;
            connp.out_tx_mut_ok()?.response_message = None;
            // Process response line.
            let s = std::slice::from_raw_parts(data as *const u8, len);
            let s = htp_util::htp_chomp(&s);
            let chomp_result = len - s.len();
            len = s.len();
            // If the response line is invalid, determine if it _looks_ like
            // a response line. If it does not look like a line, process the
            // data as a response body because that is what browsers do.
            if htp_util::htp_treat_response_line_as_body(std::slice::from_raw_parts(data, len)) {
                connp.out_tx_mut_ok()?.response_content_encoding_processing =
                    htp_decompressors::htp_content_encoding_t::HTP_COMPRESSION_NONE;
                (*connp).out_current_consume_offset = (*connp).out_current_read_offset;
                (*connp).res_process_body_data_ex(
                    data as *const core::ffi::c_void,
                    len.wrapping_add(chomp_result),
                )?;
                // Continue to process response body. Because we don't have
                // any headers to parse, we assume the body continues until
                // the end of the stream.
                // Have we seen the entire response body?
                if (*connp).out_current_len <= (*connp).out_current_read_offset {
                    connp.out_tx_mut_ok()?.response_transfer_coding =
                        htp_transaction::htp_transfer_coding_t::HTP_CODING_IDENTITY;
                    connp.out_tx_mut_ok()?.response_progress =
                        htp_transaction::htp_tx_res_progress_t::HTP_RESPONSE_BODY;
                    (*connp).out_body_data_left = -1;
                    (*connp).out_state = State::FINALIZE
                }
                return Ok(());
            }
            connp.out_tx_mut_ok()?.response_line = Some(bstr::bstr_t::from(s));
            (*connp).parse_response_line()?;
            (*connp).state_response_line()?;
            htp_connp_res_clear_buffer(connp);
            // Move on to the next phase.
            (*connp).out_state = State::HEADERS;
            connp.out_tx_mut_ok()?.response_progress =
                htp_transaction::htp_tx_res_progress_t::HTP_RESPONSE_HEADERS;
            return Ok(());
        }
    }
}

pub unsafe extern "C" fn htp_connp_RES_FINALIZE(
    connp: &mut htp_connection_parser::htp_connp_t,
) -> Result<()> {
    if (*connp).out_status != htp_connection_parser::htp_stream_state_t::HTP_STREAM_CLOSED {
        if (*connp).out_current_read_offset >= (*connp).out_current_len {
            (*connp).out_next_byte = -1
        } else {
            (*connp).out_next_byte = *(*connp)
                .out_current_data
                .offset((*connp).out_current_read_offset as isize)
                as i32
        }
        if (*connp).out_next_byte == -1 {
            return (*connp).state_response_complete_ex(0).into();
        }
        if (*connp).out_next_byte != '\n' as i32
            || (*connp).out_current_consume_offset >= (*connp).out_current_read_offset
        {
            loop {
                //;i < max_read; i++) {
                if (*connp).out_current_read_offset < (*connp).out_current_len {
                    (*connp).out_next_byte = *(*connp)
                        .out_current_data
                        .offset((*connp).out_current_read_offset as isize)
                        as i32;
                    (*connp).out_current_read_offset += 1;
                    (*connp).out_stream_offset += 1
                } else {
                    return Err(Status::DATA_BUFFER);
                }
                // Have we reached the end of the line? For some reason
                // we can't test after IN_COPY_BYTE_OR_RETURN */
                if (*connp).out_next_byte == '\n' as i32 {
                    break;
                }
            }
        }
    }
    let mut bytes_left: usize = 0;
    let mut data: *mut u8 = 0 as *mut u8;
    htp_connp_res_consolidate_data(connp, &mut data, &mut bytes_left)?;
    if bytes_left == 0 {
        //closing
        return (*connp).state_response_complete_ex(0).into();
    }
    if htp_util::htp_treat_response_line_as_body(std::slice::from_raw_parts(data, bytes_left)) {
        // Interpret remaining bytes as body data
        htp_warn!(
            connp as *mut htp_connection_parser::htp_connp_t,
            htp_log_code::RESPONSE_BODY_UNEXPECTED,
            "Unexpected response body"
        );
        let rc = (*connp).res_process_body_data_ex(data as *const core::ffi::c_void, bytes_left);
        htp_connp_res_clear_buffer(connp);
        return rc;
    }
    //unread last end of line so that RES_LINE works
    if (*connp).out_current_read_offset < bytes_left as i64 {
        (*connp).out_current_read_offset = 0
    } else {
        (*connp).out_current_read_offset =
            ((*connp).out_current_read_offset as u64).wrapping_sub(bytes_left as u64) as i64
    }
    if (*connp).out_current_read_offset < (*connp).out_current_consume_offset {
        (*connp).out_current_consume_offset = (*connp).out_current_read_offset
    }
    (*connp).state_response_complete_ex(0).into()
}

/// The response idle state will initialize response processing, as well as
/// finalize each transactions after we are done with it.
///
/// Returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
pub unsafe extern "C" fn htp_connp_RES_IDLE(
    connp: &mut htp_connection_parser::htp_connp_t,
) -> Result<()> {
    // We want to start parsing the next response (and change
    // the state from IDLE) only if there's at least one
    // byte of data available. Otherwise we could be creating
    // new structures even if there's no more data on the
    // connection.
    if (*connp).out_current_read_offset >= (*connp).out_current_len {
        return Err(Status::DATA);
    }
    // Parsing a new response
    // Find the next outgoing transaction
    // If there is none, we just create one so that responses without
    // request can still be processed.
    (*connp).set_out_tx_id(
        (*connp)
            .conn
            .tx((*connp).out_next_tx_index)
            .map(|tx| tx.index),
    );

    if (*connp).out_tx().is_none() {
        htp_error!(
            connp as *mut htp_connection_parser::htp_connp_t,
            htp_log_code::UNABLE_TO_MATCH_RESPONSE_TO_REQUEST,
            "Unable to match response to request"
        );
        // finalize dangling request waiting for next request or body
        if (*connp).in_state == State::FINALIZE {
            // Ignore result.
            let _ = (*connp).state_request_complete();
        }
        let tx_id = (*connp).create_tx()?;
        (*connp).set_out_tx_id(Some(tx_id));
        let mut out_tx = connp.out_tx_mut_ok()?;

        out_tx.parsed_uri = htp_util::htp_uri_alloc();
        if out_tx.parsed_uri.is_null() {
            return Err(Status::ERROR);
        }
        (*out_tx.parsed_uri).path = Some(bstr::bstr_t::from("/libhtp::request_uri_not_seen"));
        out_tx.request_uri = bstr::bstr_dup_str("/libhtp::request_uri_not_seen");
        if out_tx.request_uri.is_null() {
            return Err(Status::ERROR);
        }
        (*connp).in_state = State::FINALIZE;
        // We've used one transaction
        (*connp).out_next_tx_index = (*connp).out_next_tx_index.wrapping_add(1)
    } else {
        // We've used one transaction
        (*connp).out_next_tx_index = (*connp).out_next_tx_index.wrapping_add(1);
        // TODO Detect state mismatch
        (*connp).out_content_length = -1;
        (*connp).out_body_data_left = -1
    }
    (*connp).state_response_start()
}

/// Process a chunk of outbound (server or response) data.
///
/// timestamp: Optional.
///
/// Returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed
pub unsafe fn htp_connp_res_data(
    connp: &mut htp_connection_parser::htp_connp_t,
    timestamp: Option<htp_time_t>,
    data: *const core::ffi::c_void,
    len: usize,
) -> htp_connection_parser::htp_stream_state_t {
    // Return if the connection is in stop state
    if (*connp).out_status == htp_connection_parser::htp_stream_state_t::HTP_STREAM_STOP {
        htp_info!(
            connp as *mut htp_connection_parser::htp_connp_t,
            htp_log_code::PARSER_STATE_ERROR,
            "Outbound parser is in HTP_STREAM_STOP"
        );
        return htp_connection_parser::htp_stream_state_t::HTP_STREAM_STOP;
    }
    // Return if the connection has had a fatal error
    if (*connp).out_status == htp_connection_parser::htp_stream_state_t::HTP_STREAM_ERROR {
        htp_error!(
            connp as *mut htp_connection_parser::htp_connp_t,
            htp_log_code::PARSER_STATE_ERROR,
            "Outbound parser is in HTP_STREAM_ERROR"
        );
        return htp_connection_parser::htp_stream_state_t::HTP_STREAM_ERROR;
    }
    // Sanity check: we must have a transaction pointer if the state is not IDLE (no outbound transaction)
    if (*connp).out_tx().is_none() && (*connp).out_state != State::IDLE {
        (*connp).out_status = htp_connection_parser::htp_stream_state_t::HTP_STREAM_ERROR;
        htp_error!(
            connp as *mut htp_connection_parser::htp_connp_t,
            htp_log_code::MISSING_OUTBOUND_TRANSACTION_DATA,
            "Missing outbound transaction data"
        );
        return htp_connection_parser::htp_stream_state_t::HTP_STREAM_ERROR;
    }
    // If the length of the supplied data chunk is zero, proceed
    // only if the stream has been closed. We do not allow zero-sized
    // chunks in the API, but we use it internally to force the parsers
    // to finalize parsing.
    if (data == 0 as *mut core::ffi::c_void || len == 0)
        && (*connp).out_status != htp_connection_parser::htp_stream_state_t::HTP_STREAM_CLOSED
    {
        htp_error!(
            connp as *mut htp_connection_parser::htp_connp_t,
            htp_log_code::ZERO_LENGTH_DATA_CHUNKS,
            "Zero-length data chunks are not allowed"
        );
        return htp_connection_parser::htp_stream_state_t::HTP_STREAM_CLOSED;
    }
    // Remember the timestamp of the current response data chunk
    if let Some(timestamp) = timestamp {
        (*connp).out_timestamp = timestamp;
    }
    // Store the current chunk information
    (*connp).out_current_data = data as *mut u8;
    (*connp).out_current_len = len as i64;
    (*connp).out_current_read_offset = 0;
    (*connp).out_current_consume_offset = 0;
    (*connp).out_current_receiver_offset = 0;
    (*connp).conn.track_outbound_data(len);
    // Return without processing any data if the stream is in tunneling
    // mode (which it would be after an initial CONNECT transaction.
    if (*connp).out_status == htp_connection_parser::htp_stream_state_t::HTP_STREAM_TUNNEL {
        return htp_connection_parser::htp_stream_state_t::HTP_STREAM_TUNNEL;
    }
    loop
    // Invoke a processor, in a loop, until an error
    // occurs or until we run out of data. Many processors
    // will process a request, each pointing to the next
    // processor that needs to run.
    // Return if there's been an error
    // or if we've run out of data. We are relying
    // on processors to add error messages, so we'll
    // keep quiet here.
    {
        let mut rc = (*connp).handle_out_state();
        if rc.is_ok() {
            if (*connp).out_status == htp_connection_parser::htp_stream_state_t::HTP_STREAM_TUNNEL {
                return htp_connection_parser::htp_stream_state_t::HTP_STREAM_TUNNEL;
            }
            rc = htp_res_handle_state_change(connp);
        }
        match rc {
            // Continue looping.
            Ok(_) => {}
            // Do we need more data?
            Err(Status::DATA) | Err(Status::DATA_BUFFER) => {
                // Ignore result.
                let _ = htp_connp_res_receiver_send_data(connp, false);
                if rc == Err(Status::DATA_BUFFER) && htp_connp_res_buffer(connp).is_err() {
                    (*connp).out_status =
                        htp_connection_parser::htp_stream_state_t::HTP_STREAM_ERROR;
                    return htp_connection_parser::htp_stream_state_t::HTP_STREAM_ERROR;
                }
                (*connp).out_status = htp_connection_parser::htp_stream_state_t::HTP_STREAM_DATA;
                return htp_connection_parser::htp_stream_state_t::HTP_STREAM_DATA;
            }
            // Check for stop
            Err(Status::STOP) => {
                (*connp).out_status = htp_connection_parser::htp_stream_state_t::HTP_STREAM_STOP;
                return htp_connection_parser::htp_stream_state_t::HTP_STREAM_STOP;
            }
            // Check for suspended parsing
            Err(Status::DATA_OTHER) => {
                // We might have actually consumed the entire data chunk?
                if (*connp).out_current_read_offset >= (*connp).out_current_len {
                    (*connp).out_status =
                        htp_connection_parser::htp_stream_state_t::HTP_STREAM_DATA;
                    // Do not send STREAM_DATE_DATA_OTHER if we've
                    // consumed the entire chunk
                    return htp_connection_parser::htp_stream_state_t::HTP_STREAM_DATA;
                } else {
                    (*connp).out_status =
                        htp_connection_parser::htp_stream_state_t::HTP_STREAM_DATA_OTHER;
                    // Partial chunk consumption
                    return htp_connection_parser::htp_stream_state_t::HTP_STREAM_DATA_OTHER;
                }
            }
            // Permanent stream error.
            Err(_) => {
                (*connp).out_status = htp_connection_parser::htp_stream_state_t::HTP_STREAM_ERROR;
                return htp_connection_parser::htp_stream_state_t::HTP_STREAM_ERROR;
            }
        }
    }
}
