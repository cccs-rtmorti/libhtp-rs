use crate::connection_parser::State;
use crate::error::Result;
use crate::hook::DataHook;
use crate::transaction::Protocol;
use crate::util::Flags;
use crate::{bstr, connection_parser, decompressors, request, transaction, util, Status};
use std::cmp::Ordering;

pub type htp_time_t = libc::timeval;

impl connection_parser::ConnectionParser {
    /// Sends outstanding connection data to the currently active data receiver hook.
    ///
    /// Returns HTP_OK, or a value returned from a callback.
    fn res_receiver_send_data(&mut self, is_last: bool) -> Result<()> {
        let mut data = unsafe {
            transaction::Data::new(
                self.out_tx_mut_ptr(),
                self.out_current_data
                    .offset(self.out_current_receiver_offset as isize),
                (self.out_current_read_offset - self.out_current_receiver_offset) as usize,
                is_last,
            )
        };
        if let Some(hook) = &self.out_data_receiver_hook {
            hook.run_all(&mut data)?;
        } else {
            return Ok(());
        };
        self.out_current_receiver_offset = self.out_current_read_offset;
        Ok(())
    }

    /// Finalizes an existing data receiver hook by sending any outstanding data to it. The
    /// hook is then removed so that it receives no more data.
    ///
    /// Returns HTP_OK, or a value returned from a callback.
    pub fn res_receiver_finalize_clear(&mut self) -> Result<()> {
        if self.out_data_receiver_hook.is_none() {
            return Ok(());
        }
        let rc = self.res_receiver_send_data(true);
        self.out_data_receiver_hook = None;
        rc
    }

    /// Configures the data receiver hook. If there is a previous hook, it will be finalized and cleared.
    ///
    /// Returns HTP_OK, or a value returned from a callback.
    fn res_receiver_set(&mut self, data_receiver_hook: Option<DataHook>) -> Result<()> {
        // Ignore result.
        let _ = self.res_receiver_finalize_clear();
        self.out_data_receiver_hook = data_receiver_hook;
        self.out_current_receiver_offset = self.out_current_read_offset;
        Ok(())
    }

    /// Handles request parser state changes. At the moment, this function is used only
    /// to configure data receivers, which are sent raw connection data.
    ///
    /// Returns HTP_OK, or a value returned from a callback.
    fn res_handle_state_change(&mut self) -> Result<()> {
        if self.out_state_previous == self.out_state {
            return Ok(());
        }
        if self.out_state == State::HEADERS {
            unsafe {
                let header_fn = Some(
                    (*self.out_tx_mut_ok()?.cfg)
                        .hook_response_header_data
                        .clone(),
                );
                let trailer_fn = Some(
                    (*self.out_tx_mut_ok()?.cfg)
                        .hook_response_trailer_data
                        .clone(),
                );
                match self.out_tx_mut_ok()?.response_progress {
                    transaction::htp_tx_res_progress_t::HTP_RESPONSE_HEADERS => {
                        self.res_receiver_set(header_fn)
                    }
                    transaction::htp_tx_res_progress_t::HTP_RESPONSE_TRAILER => {
                        self.res_receiver_set(trailer_fn)
                    }
                    _ => Ok(()),
                }?;
            }
        }
        // Same comment as in htp_req_handle_state_change(). Below is a copy.
        // Initially, I had the finalization of raw data sending here, but that
        // caused the last REQUEST_HEADER_DATA hook to be invoked after the
        // REQUEST_HEADERS hook -- which I thought made no sense. For that reason,
        // the finalization is now initiated from the request header processing code,
        // which is less elegant but provides a better user experience. Having some
        // (or all) hooks to be invoked on state change might work better.
        self.out_state_previous = self.out_state;
        Ok(())
    }

    /// If there is any data left in the outbound data chunk, this function will preserve
    /// it for later consumption. The maximum amount accepted for buffering is controlled
    /// by htp_config_t::field_limit.
    ///
    /// Returns HTP_OK, or HTP_ERROR on fatal failure.
    fn res_buffer(&mut self) -> Result<()> {
        if self.out_current_data.is_null() {
            return Ok(());
        }
        unsafe {
            let data: *mut u8 = self
                .out_current_data
                .offset(self.out_current_consume_offset as isize);
            let len: usize =
                (self.out_current_read_offset - self.out_current_consume_offset) as usize;
            // Check the hard (buffering) limit.
            let mut newlen: usize = self.out_buf.len().wrapping_add(len);
            // When calculating the size of the buffer, take into account the
            // space we're using for the response header buffer.
            if let Some(out_header) = &self.out_header {
                newlen = newlen.wrapping_add(out_header.len())
            }
            if newlen > (*self.out_tx_mut_ok()?.cfg).field_limit {
                htp_error!(
                    self as *mut connection_parser::ConnectionParser,
                    htp_log_code::RESPONSE_FIELD_TOO_LONG,
                    format!(
                        "Response the buffer limit: size {} limit {}.",
                        newlen,
                        (*self.out_tx_mut_ok()?.cfg).field_limit
                    )
                );
                return Err(Status::ERROR);
            }
            // Copy the data remaining in the buffer.
            self.out_buf.add(std::slice::from_raw_parts(data, len));
        }
        // Reset the consumer position.
        self.out_current_consume_offset = self.out_current_read_offset;
        Ok(())
    }

    /// Returns to the caller the memory region that should be processed next. This function
    /// hides away the buffering process from the rest of the code, allowing it to work with
    /// non-buffered data that's in the outbound chunk, or buffered data that's in our structures.
    ///
    /// Returns HTP_OK
    fn res_consolidate_data(&mut self, data: *mut *mut u8, len: *mut usize) -> Result<()> {
        unsafe {
            if self.out_buf.is_empty() {
                // We do not have any data buffered; point to the current data chunk.
                *data = self
                    .out_current_data
                    .offset(self.out_current_consume_offset as isize);
                *len = (self.out_current_read_offset - self.out_current_consume_offset) as usize
            } else {
                // We do have data in the buffer. Add data from the current
                // chunk, and point to the consolidated buffer.
                self.res_buffer()?;
                *data = self.out_buf.as_mut_ptr();
                *len = self.out_buf.len();
            }
        }
        Ok(())
    }

    /// Clears buffered outbound data and resets the consumer position to the reader position.
    fn res_clear_buffer(&mut self) {
        self.out_current_consume_offset = self.out_current_read_offset;
        self.out_buf.clear()
    }

    /// Consumes bytes until the end of the current line.
    ///
    /// Returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
    pub fn RES_BODY_CHUNKED_DATA_END(&mut self) -> Result<()> {
        loop
        // TODO We shouldn't really see anything apart from CR and LF,
        //      so we should warn about anything else.
        {
            if self.out_current_read_offset < self.out_current_len {
                self.out_next_byte = unsafe {
                    *self
                        .out_current_data
                        .offset(self.out_current_read_offset as isize) as i32
                };
                self.out_current_read_offset += 1;
                self.out_current_consume_offset += 1;
                self.out_stream_offset += 1
            } else {
                return Err(Status::DATA);
            }
            self.out_tx_mut_ok()?.response_message_len += 1;
            if self.out_next_byte == '\n' as i32 {
                self.out_state = State::BODY_CHUNKED_LENGTH;
                return Ok(());
            }
        }
    }

    /// Processes a chunk of data.
    ///
    /// Returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
    pub fn RES_BODY_CHUNKED_DATA(&mut self) -> Result<()> {
        let mut bytes_to_consume: usize = 0;
        // Determine how many bytes we can consume.
        if self.out_current_len - self.out_current_read_offset >= self.out_chunked_length {
            bytes_to_consume = self.out_chunked_length as usize
        } else {
            bytes_to_consume = (self.out_current_len - self.out_current_read_offset) as usize
        }
        if bytes_to_consume == 0 {
            return Err(Status::DATA);
        }
        // Consume the data.
        unsafe {
            self.res_process_body_data_ex(
                self.out_current_data
                    .offset(self.out_current_read_offset as isize)
                    as *const core::ffi::c_void,
                bytes_to_consume,
            )?;
        }
        // Adjust the counters.
        self.out_current_read_offset =
            (self.out_current_read_offset as u64).wrapping_add(bytes_to_consume as u64) as i64;
        self.out_current_consume_offset =
            (self.out_current_consume_offset as u64).wrapping_add(bytes_to_consume as u64) as i64;
        self.out_stream_offset =
            (self.out_stream_offset as u64).wrapping_add(bytes_to_consume as u64) as i64;
        self.out_chunked_length =
            (self.out_chunked_length as u64).wrapping_sub(bytes_to_consume as u64) as i64;
        // Have we seen the entire chunk?
        if self.out_chunked_length == 0 {
            self.out_state = State::BODY_CHUNKED_DATA_END;
            return Ok(());
        }
        Err(Status::DATA)
    }

    /// Peeks ahead into the data to try to see if it starts with a valid Chunked
    /// length field.
    ///
    /// Returns true if it looks valid, false if it looks invalid
    #[inline]
    fn data_probe_chunk_length(&self) -> bool {
        if self.out_current_read_offset - self.out_current_consume_offset < 8 {
            // not enough data so far, consider valid still
            return true;
        }
        let data: *mut u8 = unsafe {
            self.out_current_data
                .offset(self.out_current_consume_offset as isize)
        };
        let len: usize = (self.out_current_read_offset - self.out_current_consume_offset) as usize;
        let mut i: usize = 0;
        while i < len {
            let c = unsafe { *data.offset(i as isize) };
            if is_chunked_ctl_char(c as u8) {
                // ctl char, still good.
            } else if c.is_ascii_digit()
                || c >= 'a' as u8 && c <= 'f' as u8
                || c >= 'A' as u8 && c <= 'F' as u8
            {
                // real chunklen char
                return true;
            } else {
                // leading junk, bad
                return false;
            }
            i = i.wrapping_add(1)
        }
        true
    }

    /// Extracts chunk length.
    ///
    /// Returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
    pub fn RES_BODY_CHUNKED_LENGTH(&mut self) -> Result<()> {
        loop {
            if self.out_current_read_offset < self.out_current_len {
                self.out_next_byte = unsafe {
                    *self
                        .out_current_data
                        .offset(self.out_current_read_offset as isize) as i32
                };
                self.out_current_read_offset += 1;
                self.out_stream_offset += 1
            } else {
                return Err(Status::DATA_BUFFER);
            }
            // Have we reached the end of the line? Or is this not chunked after all?
            if !(self.out_next_byte == '\n' as i32
                || (!is_chunked_ctl_char(self.out_next_byte as u8)
                    && !self.data_probe_chunk_length()))
            {
                continue;
            }
            let mut data: *mut u8 = 0 as *mut u8;
            let mut len: usize = 0;
            self.res_consolidate_data(&mut data, &mut len)?;
            self.out_tx_mut_ok()?.response_message_len =
                (self.out_tx_mut_ok()?.response_message_len as u64).wrapping_add(len as u64) as i64;

            let buf: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(data, len) };
            if let Ok(chunked_length) = util::parse_chunked_length(buf) {
                if let Some(chunked_length) = chunked_length {
                    self.out_chunked_length = chunked_length as i64;
                } else {
                    // empty chunk length line, lets try to continue
                    continue;
                }
            } else {
                self.out_chunked_length = -1;
            }
            if self.out_chunked_length < 0 {
                // reset out_current_read_offset so RES_BODY_IDENTITY_STREAM_CLOSE
                // doesn't miss the first bytes
                if len > self.out_current_read_offset as usize {
                    self.out_current_read_offset = 0
                } else {
                    self.out_current_read_offset =
                        (self.out_current_read_offset as u64).wrapping_sub(len as u64) as i64
                }
                self.out_state = State::BODY_IDENTITY_STREAM_CLOSE;
                self.out_tx_mut_ok()?.response_transfer_coding =
                    transaction::htp_transfer_coding_t::HTP_CODING_IDENTITY;
                unsafe {
                    htp_error!(
                        self as *mut connection_parser::ConnectionParser,
                        htp_log_code::INVALID_RESPONSE_CHUNK_LEN,
                        format!(
                            "Response chunk encoding: Invalid chunk length: {}",
                            self.out_chunked_length
                        )
                    );
                }
                return Ok(());
            }
            self.res_clear_buffer();
            // Handle chunk length
            if self.out_chunked_length > 0 {
                // More data available
                self.out_state = State::BODY_CHUNKED_DATA
            } else if self.out_chunked_length == 0 {
                // End of data
                self.out_state = State::HEADERS;
                self.out_tx_mut_ok()?.response_progress =
                    transaction::htp_tx_res_progress_t::HTP_RESPONSE_TRAILER
            }
            return Ok(());
        }
    }

    /// Processes an identity response body of known length.
    ///
    /// Returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
    pub fn RES_BODY_IDENTITY_CL_KNOWN(&mut self) -> Result<()> {
        let mut bytes_to_consume: usize = 0;
        // Determine how many bytes we can consume.
        if self.out_current_len - self.out_current_read_offset >= self.out_body_data_left {
            bytes_to_consume = self.out_body_data_left as usize
        } else {
            bytes_to_consume = (self.out_current_len - self.out_current_read_offset) as usize
        }
        if self.out_status == connection_parser::htp_stream_state_t::HTP_STREAM_CLOSED {
            self.out_state = State::FINALIZE;
            // Sends close signal to decompressors
            return unsafe { self.res_process_body_data_ex(0 as *const core::ffi::c_void, 0) };
        }
        if bytes_to_consume == 0 {
            return Err(Status::DATA);
        }
        // Consume the data.
        unsafe {
            self.res_process_body_data_ex(
                self.out_current_data
                    .offset(self.out_current_read_offset as isize)
                    as *const core::ffi::c_void,
                bytes_to_consume,
            )?;
        }
        // Adjust the counters.
        self.out_current_read_offset =
            (self.out_current_read_offset as u64).wrapping_add(bytes_to_consume as u64) as i64;
        self.out_current_consume_offset =
            (self.out_current_consume_offset as u64).wrapping_add(bytes_to_consume as u64) as i64;
        self.out_stream_offset =
            (self.out_stream_offset as u64).wrapping_add(bytes_to_consume as u64) as i64;
        self.out_body_data_left =
            (self.out_body_data_left as u64).wrapping_sub(bytes_to_consume as u64) as i64;
        // Have we seen the entire response body?
        if self.out_body_data_left == 0 {
            self.out_state = State::FINALIZE;
            // Tells decompressors to output partially decompressed data
            return unsafe { self.res_process_body_data_ex(0 as *const core::ffi::c_void, 0) };
        }
        Err(Status::DATA)
    }

    /// Processes identity response body of unknown length. In this case, we assume the
    /// response body consumes all data until the end of the stream.
    ///
    /// Returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
    pub fn RES_BODY_IDENTITY_STREAM_CLOSE(&mut self) -> Result<()> {
        // Consume all data from the input buffer.
        let bytes_to_consume: usize =
            (self.out_current_len - self.out_current_read_offset) as usize;
        if bytes_to_consume != 0 {
            unsafe {
                self.res_process_body_data_ex(
                    self.out_current_data
                        .offset(self.out_current_read_offset as isize)
                        as *const core::ffi::c_void,
                    bytes_to_consume,
                )?;
            }
            // Adjust the counters.
            self.out_current_read_offset =
                (self.out_current_read_offset as u64).wrapping_add(bytes_to_consume as u64) as i64;
            self.out_current_consume_offset = (self.out_current_consume_offset as u64)
                .wrapping_add(bytes_to_consume as u64)
                as i64;
            self.out_stream_offset =
                (self.out_stream_offset as u64).wrapping_add(bytes_to_consume as u64) as i64;
        }
        // Have we seen the entire response body?
        if self.out_status == connection_parser::htp_stream_state_t::HTP_STREAM_CLOSED {
            self.out_state = State::FINALIZE;
            return Ok(());
        }
        Err(Status::DATA)
    }

    /// Determines presence (and encoding) of a response body.
    ///
    /// Returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
    pub fn RES_BODY_DETERMINE(&mut self) -> Result<()> {
        // If the request uses the CONNECT method, then not only are we
        // to assume there's no body, but we need to ignore all
        // subsequent data in the stream.
        if self.out_tx_mut_ok()?.request_method_number == request::htp_method_t::HTP_M_CONNECT {
            if self.out_tx_mut_ok()?.response_status_number >= 200
                && self.out_tx_mut_ok()?.response_status_number <= 299
            {
                // This is a successful CONNECT stream, which means
                // we need to switch into tunneling mode: on the
                // request side we'll now probe the tunnel data to see
                // if we need to parse or ignore it. So on the response
                // side we wrap up the tx and wait.
                self.out_state = State::FINALIZE;
                // we may have response headers
                return unsafe { self.state_response_headers().into() };
            } else if self.out_tx_mut_ok()?.response_status_number == 407 {
                // proxy telling us to auth
                if self.in_status != connection_parser::htp_stream_state_t::HTP_STREAM_ERROR {
                    self.in_status = connection_parser::htp_stream_state_t::HTP_STREAM_DATA
                }
            } else {
                // This is a failed CONNECT stream, which means that
                // we can unblock request parsing
                if self.in_status != connection_parser::htp_stream_state_t::HTP_STREAM_ERROR {
                    self.in_status = connection_parser::htp_stream_state_t::HTP_STREAM_DATA
                }
                // We are going to continue processing this transaction,
                // adding a note for ourselves to stop at the end (because
                // we don't want to see the beginning of a new transaction).
                self.out_data_other_at_tx_end = true
            }
        }
        let cl_opt = self
            .out_tx_mut_ok()?
            .response_headers
            .get_nocase_nozero("content-length")
            .map(|(_, val)| val.clone());
        let te_opt = self
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
        if self.out_tx_mut_ok()?.response_status_number == 101 {
            if te_opt.is_none() && cl_opt.is_none() {
                self.out_state = State::FINALIZE;
                if self.in_status != connection_parser::htp_stream_state_t::HTP_STREAM_ERROR {
                    self.in_status = connection_parser::htp_stream_state_t::HTP_STREAM_TUNNEL
                }
                self.out_status = connection_parser::htp_stream_state_t::HTP_STREAM_TUNNEL;
                // we may have response headers
                return unsafe { self.state_response_headers().into() };
            } else {
                unsafe {
                    htp_warn!(
                        self as *mut connection_parser::ConnectionParser,
                        htp_log_code::SWITCHING_PROTO_WITH_CONTENT_LENGTH,
                        "Switching Protocol with Content-Length"
                    );
                }
            }
        }
        // Check for an interim "100 Continue" response. Ignore it if found, and revert back to RES_LINE.
        if self.out_tx_mut_ok()?.response_status_number == 100
            && te_opt.is_none()
            && cl_opt.is_none()
        {
            if self.out_tx_mut_ok()?.seen_100continue {
                unsafe {
                    htp_error!(
                        self as *mut connection_parser::ConnectionParser,
                        htp_log_code::CONTINUE_ALREADY_SEEN,
                        "Already seen 100-Continue."
                    );
                }
                return Err(Status::ERROR);
            }
            // Ignore any response headers seen so far.
            self.out_tx_mut_ok()?.response_headers.elements.clear();
            // Expecting to see another response line next.
            self.out_state = State::LINE;
            self.out_tx_mut_ok()?.response_progress =
                transaction::htp_tx_res_progress_t::HTP_RESPONSE_LINE;
            self.out_tx_mut_ok()?.seen_100continue;
            return Ok(());
        }

        // A request can indicate it waits for headers validation
        // before sending its body cf
        // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Expect
        if self.out_tx_mut_ok()?.response_status_number >= 400
            && self.out_tx_mut_ok()?.response_status_number <= 499
            && self.in_content_length > 0
            && self.in_body_data_left == self.in_content_length
        {
            if let Some((_, expect)) = self.out_tx_mut_ok()?.request_headers.get_nocase("expect") {
                if expect.value == "100-continue" {
                    self.in_state = State::FINALIZE;
                }
            }
        }

        // 1. Any response message which MUST NOT include a message-body
        //  (such as the 1xx, 204, and 304 responses and any response to a HEAD
        //  request) is always terminated by the first empty line after the
        //  header fields, regardless of the entity-header fields present in the
        //  message.
        if self.out_tx_mut_ok()?.request_method_number == request::htp_method_t::HTP_M_HEAD {
            // There's no response body whatsoever
            self.out_tx_mut_ok()?.response_transfer_coding =
                transaction::htp_transfer_coding_t::HTP_CODING_NO_BODY;
            self.out_state = State::FINALIZE
        } else if self.out_tx_mut_ok()?.response_status_number >= 100
            && self.out_tx_mut_ok()?.response_status_number <= 199
            || self.out_tx_mut_ok()?.response_status_number == 204
            || self.out_tx_mut_ok()?.response_status_number == 304
        {
            // There should be no response body
            // but browsers interpret content sent by the server as such
            if te_opt.is_none() && cl_opt.is_none() {
                self.out_tx_mut_ok()?.response_transfer_coding =
                    transaction::htp_transfer_coding_t::HTP_CODING_NO_BODY;
                self.out_state = State::FINALIZE
            } else {
                unsafe {
                    htp_warn!(
                        self as *mut connection_parser::ConnectionParser,
                        htp_log_code::RESPONSE_BODY_UNEXPECTED,
                        "Unexpected Response body"
                    );
                }
            }
        }
        // Hack condition to check that we do not assume "no body"
        if self.out_state != State::FINALIZE {
            // We have a response body
            let ct_opt = self
                .out_tx_mut_ok()?
                .response_headers
                .get_nocase_nozero("content-type")
                .map(|(_, val)| val.clone());
            if let Some(ct) = &ct_opt {
                let mut response_content_type = bstr::Bstr::from(ct.value.as_slice());
                response_content_type.make_ascii_lowercase();
                // Ignore parameters
                let data: *mut u8 = response_content_type.as_mut_ptr();
                let len: usize = ct.value.len();
                let mut newlen: usize = 0;
                while newlen < len {
                    // TODO Some platforms may do things differently here.
                    unsafe {
                        if util::is_space(*data.offset(newlen as isize))
                            || *data.offset(newlen as isize) as i32 == ';' as i32
                        {
                            response_content_type.set_len(newlen);
                            break;
                        } else {
                            newlen = newlen.wrapping_add(1)
                        }
                    }
                }
                self.out_tx_mut_ok()?.response_content_type = Some(response_content_type);
            }
            // 2. If a Transfer-Encoding header field (section 14.40) is present and
            //   indicates that the "chunked" transfer coding has been applied, then
            //   the length is defined by the chunked encoding (section 3.6).
            if let Some(te) =
                te_opt.and_then(|te| te.value.index_of_nocase_nozero("chunked").and(Some(te)))
            {
                if te.value.cmp_nocase("chunked") != Ordering::Equal {
                    unsafe {
                        htp_warn!(
                            self as *mut connection_parser::ConnectionParser,
                            htp_log_code::RESPONSE_ABNORMAL_TRANSFER_ENCODING,
                            "Transfer-encoding has abnormal chunked value"
                        );
                    }
                }
                // 3. If a Content-Length header field (section 14.14) is present, its
                // spec says chunked is HTTP/1.1 only, but some browsers accept it
                // with 1.0 as well
                if self.out_tx_mut_ok()?.response_protocol_number < Protocol::V1_1 {
                    unsafe {
                        htp_warn!(
                            self as *mut connection_parser::ConnectionParser,
                            htp_log_code::RESPONSE_CHUNKED_OLD_PROTO,
                            "Chunked transfer-encoding on HTTP/0.9 or HTTP/1.0"
                        );
                    }
                }
                // If the T-E header is present we are going to use it.
                self.out_tx_mut_ok()?.response_transfer_coding =
                    transaction::htp_transfer_coding_t::HTP_CODING_CHUNKED;
                // We are still going to check for the presence of C-L
                if cl_opt.is_some() {
                    // This is a violation of the RFC
                    self.out_tx_mut_ok()?.flags |= Flags::HTP_REQUEST_SMUGGLING
                }
                self.out_state = State::BODY_CHUNKED_LENGTH;
                self.out_tx_mut_ok()?.response_progress =
                    transaction::htp_tx_res_progress_t::HTP_RESPONSE_BODY
            } else if let Some(cl) = cl_opt {
                //   value in bytes represents the length of the message-body.
                // We know the exact length
                self.out_tx_mut_ok()?.response_transfer_coding =
                    transaction::htp_transfer_coding_t::HTP_CODING_IDENTITY;
                // Check for multiple C-L headers
                if cl.flags.contains(Flags::HTP_FIELD_REPEATED) {
                    self.out_tx_mut_ok()?.flags |= Flags::HTP_REQUEST_SMUGGLING
                }
                // Get body length
                if let Some(content_length) =
                    util::parse_content_length((*cl.value).as_slice(), Some(&mut *self))
                {
                    self.out_tx_mut_ok()?.response_content_length = content_length;
                    self.out_content_length = self.out_tx_mut_ok()?.response_content_length;
                    self.out_body_data_left = self.out_content_length;
                    if self.out_content_length != 0 {
                        self.out_state = State::BODY_IDENTITY_CL_KNOWN;
                        self.out_tx_mut_ok()?.response_progress =
                            transaction::htp_tx_res_progress_t::HTP_RESPONSE_BODY
                    } else {
                        self.out_state = State::FINALIZE
                    }
                } else {
                    unsafe {
                        htp_error!(
                            self as *mut connection_parser::ConnectionParser,
                            htp_log_code::INVALID_CONTENT_LENGTH_FIELD_IN_RESPONSE,
                            format!(
                                "Invalid C-L field in response: {}",
                                self.out_tx_mut_ok()?.response_content_length
                            )
                        );
                    };
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
                        unsafe {
                            htp_error!(
                                self as *mut connection_parser::ConnectionParser,
                                htp_log_code::RESPONSE_MULTIPART_BYTERANGES,
                                "C-T multipart/byteranges in responses not supported"
                            );
                        }
                        return Err(Status::ERROR);
                    }
                }
                // 5. By the server closing the connection. (Closing the connection
                //   cannot be used to indicate the end of a request body, since that
                //   would leave no possibility for the server to send back a response.)
                self.out_state = State::BODY_IDENTITY_STREAM_CLOSE;
                self.out_tx_mut_ok()?.response_transfer_coding =
                    transaction::htp_transfer_coding_t::HTP_CODING_IDENTITY;
                self.out_tx_mut_ok()?.response_progress =
                    transaction::htp_tx_res_progress_t::HTP_RESPONSE_BODY;
                self.out_body_data_left = -1
            }
        }
        // NOTE We do not need to check for short-style HTTP/0.9 requests here because
        //      that is done earlier, before response line parsing begins
        unsafe { self.state_response_headers() }
    }

    /// Parses response headers.
    ///
    /// Returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
    pub fn RES_HEADERS(&mut self) -> Result<()> {
        let mut endwithcr = false;
        let mut lfcrending = false;
        loop {
            if self.out_status == connection_parser::htp_stream_state_t::HTP_STREAM_CLOSED {
                // Finalize sending raw trailer data.
                self.res_receiver_finalize_clear()?;
                // Run hook response_TRAILER.
                unsafe {
                    (*self.cfg)
                        .hook_response_trailer
                        .run_all(self.out_tx_mut_ptr())?;
                }
                self.out_state = State::FINALIZE;
                return Ok(());
            }
            if self.out_current_read_offset < self.out_current_len {
                self.out_next_byte = unsafe {
                    *self
                        .out_current_data
                        .offset(self.out_current_read_offset as isize) as i32
                };
                self.out_current_read_offset += 1;
                self.out_stream_offset += 1
            } else {
                return Err(Status::DATA_BUFFER);
            }
            // Have we reached the end of the line?
            if self.out_next_byte != '\n' as i32 && self.out_next_byte != '\r' as i32 {
                lfcrending = false
            } else {
                endwithcr = false;
                if self.out_next_byte == '\r' as i32 {
                    if self.out_current_read_offset >= self.out_current_len {
                        self.out_next_byte = -1
                    } else {
                        self.out_next_byte = unsafe {
                            *self
                                .out_current_data
                                .offset(self.out_current_read_offset as isize)
                                as i32
                        }
                    }
                    if self.out_next_byte == -1 {
                        return Err(Status::DATA_BUFFER);
                    } else {
                        if self.out_next_byte == '\n' as i32 {
                            if self.out_current_read_offset < self.out_current_len {
                                self.out_next_byte = unsafe {
                                    *self
                                        .out_current_data
                                        .offset(self.out_current_read_offset as isize)
                                        as i32
                                };
                                self.out_current_read_offset += 1;
                                self.out_stream_offset += 1
                            } else {
                                return Err(Status::DATA_BUFFER);
                            }
                            if lfcrending {
                                // Handling LFCRCRLFCRLF
                                // These 6 characters mean only 2 end of lines
                                if self.out_current_read_offset >= self.out_current_len {
                                    self.out_next_byte = -1
                                } else {
                                    self.out_next_byte = unsafe {
                                        *self
                                            .out_current_data
                                            .offset(self.out_current_read_offset as isize)
                                            as i32
                                    }
                                }
                                if self.out_next_byte == '\r' as i32 {
                                    if self.out_current_read_offset < self.out_current_len {
                                        self.out_next_byte = unsafe {
                                            *self
                                                .out_current_data
                                                .offset(self.out_current_read_offset as isize)
                                                as i32
                                        };
                                        self.out_current_read_offset += 1;
                                        self.out_stream_offset += 1
                                    } else {
                                        return Err(Status::DATA_BUFFER);
                                    }
                                    self.out_current_consume_offset += 1;
                                    if self.out_current_read_offset >= self.out_current_len {
                                        self.out_next_byte = -1
                                    } else {
                                        self.out_next_byte = unsafe {
                                            *self
                                                .out_current_data
                                                .offset(self.out_current_read_offset as isize)
                                                as i32
                                        }
                                    }
                                    if self.out_next_byte == '\n' as i32 {
                                        if self.out_current_read_offset < self.out_current_len {
                                            self.out_next_byte = unsafe {
                                                *self
                                                    .out_current_data
                                                    .offset(self.out_current_read_offset as isize)
                                                    as i32
                                            };
                                            self.out_current_read_offset += 1;
                                            self.out_stream_offset += 1
                                        } else {
                                            return Err(Status::DATA_BUFFER);
                                        }
                                        self.out_current_consume_offset += 1;
                                        unsafe {
                                            htp_warn!(
                                                self as *mut connection_parser::ConnectionParser,
                                                htp_log_code::DEFORMED_EOL,
                                                "Weird response end of lines mix"
                                            );
                                        }
                                    }
                                }
                            }
                        } else if self.out_next_byte == '\r' as i32 {
                            continue;
                        }
                        lfcrending = false;
                        endwithcr = true
                    }
                } else {
                    // connp->out_next_byte == LF
                    if self.out_current_read_offset >= self.out_current_len {
                        self.out_next_byte = -1
                    } else {
                        self.out_next_byte = unsafe {
                            *self
                                .out_current_data
                                .offset(self.out_current_read_offset as isize)
                                as i32
                        }
                    }
                    lfcrending = false;
                    if self.out_next_byte == '\r' as i32 {
                        // hanldes LF-CR sequence as end of line
                        if self.out_current_read_offset < self.out_current_len {
                            self.out_next_byte = unsafe {
                                *self
                                    .out_current_data
                                    .offset(self.out_current_read_offset as isize)
                                    as i32
                            };
                            self.out_current_read_offset += 1;
                            self.out_stream_offset += 1
                        } else {
                            return Err(Status::DATA_BUFFER);
                        }
                        lfcrending = true
                    }
                }
                let mut data: *mut u8 = 0 as *mut u8;
                let mut len: usize = 0;
                self.res_consolidate_data(&mut data, &mut len)?;
                // CRCRLF is not an empty line
                if endwithcr && len < 2 {
                    continue;
                }
                let mut next_no_lf: bool = false;
                if self.out_current_read_offset < self.out_current_len
                    && unsafe {
                        *self
                            .out_current_data
                            .offset(self.out_current_read_offset as isize)
                            as i32
                            != '\n' as i32
                    }
                {
                    next_no_lf = true
                }
                // Should we terminate headers?
                if !data.is_null()
                    && util::connp_is_line_terminator(
                        unsafe { (*self.cfg).server_personality },
                        unsafe { std::slice::from_raw_parts(data, len) },
                        next_no_lf,
                    )
                {
                    // Parse previous header, if any.
                    if let Some(out_header) = self.out_header.take() {
                        self.process_response_header(out_header.as_slice())?;
                    }
                    self.res_clear_buffer();
                    // We've seen all response headers.
                    if self.out_tx_mut_ok()?.response_progress
                        == transaction::htp_tx_res_progress_t::HTP_RESPONSE_HEADERS
                    {
                        // Response headers.
                        // The next step is to determine if this response has a body.
                        self.out_state = State::BODY_DETERMINE
                    } else {
                        // Response trailer.
                        // Finalize sending raw trailer data.
                        self.res_receiver_finalize_clear()?;
                        // Run hook response_TRAILER.
                        unsafe {
                            (*self.cfg)
                                .hook_response_trailer
                                .run_all(self.out_tx_mut_ptr())?;
                        }
                        // The next step is to finalize this response.
                        self.out_state = State::FINALIZE
                    }
                    return Ok(());
                }
                let s = unsafe { std::slice::from_raw_parts(data as *const u8, len) };
                let s = util::chomp(&s);
                len = s.len();
                // Check for header folding.
                if !util::connp_is_line_folded(s) {
                    // New header line.
                    // Parse previous header, if any.
                    if let Some(out_header) = self.out_header.take() {
                        self.process_response_header(out_header.as_slice())?;
                    }
                    if self.out_current_read_offset >= self.out_current_len {
                        self.out_next_byte = -1
                    } else {
                        self.out_next_byte = unsafe {
                            *self
                                .out_current_data
                                .offset(self.out_current_read_offset as isize)
                                as i32
                        }
                    }
                    if !util::is_folding_char(self.out_next_byte as u8) {
                        // Because we know this header is not folded, we can process the buffer straight away.
                        self.process_response_header(s)?;
                    } else {
                        // Keep the partial header data for parsing later.
                        self.out_header = Some(bstr::Bstr::from(s));
                    }
                } else if self.out_header.is_none() {
                    // Folding; check that there's a previous header line to add to.
                    // Invalid folding.
                    // Warn only once per transaction.
                    if !self
                        .out_tx_mut_ok()?
                        .flags
                        .contains(Flags::HTP_INVALID_FOLDING)
                    {
                        self.out_tx_mut_ok()?.flags |= Flags::HTP_INVALID_FOLDING;
                        unsafe {
                            htp_warn!(
                                self as *mut connection_parser::ConnectionParser,
                                htp_log_code::INVALID_RESPONSE_FIELD_FOLDING,
                                "Invalid response field folding"
                            );
                        }
                    }
                    // Keep the header data for parsing later.
                    self.out_header = Some(bstr::Bstr::from(s));
                } else {
                    let mut colon_pos: usize = 0;
                    while colon_pos < len
                        && unsafe { *data.offset(colon_pos as isize) != ':' as u8 }
                    {
                        colon_pos = colon_pos.wrapping_add(1)
                    }
                    if colon_pos < len
                        && self
                            .out_header
                            .as_ref()
                            .and_then(|hdr| hdr.index_of(":"))
                            .is_some()
                        && self.out_tx_mut_ok()?.response_protocol_number == Protocol::V1_1
                    {
                        // Warn only once per transaction.
                        if !self
                            .out_tx_mut_ok()?
                            .flags
                            .contains(Flags::HTP_INVALID_FOLDING)
                        {
                            self.out_tx_mut_ok()?.flags |= Flags::HTP_INVALID_FOLDING;
                            unsafe {
                                htp_warn!(
                                    self as *mut connection_parser::ConnectionParser,
                                    htp_log_code::INVALID_RESPONSE_FIELD_FOLDING,
                                    "Invalid response field folding"
                                );
                            }
                        }
                        if let Some(out_header) = self.out_header.take() {
                            self.process_response_header(out_header.as_slice())?;
                        }
                        self.out_header = Some(bstr::Bstr::from(&s[1..]));
                    } else if let Some(out_header) = &mut self.out_header {
                        // Add to the existing header.
                        out_header.add(s);
                    }
                }
                self.res_clear_buffer();
            }
        }
    }

    /// Parses response line.
    ///
    /// Returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
    pub fn RES_LINE(&mut self) -> Result<()> {
        loop {
            // Don't try to get more data if the stream is closed. If we do, we'll return, asking for more data.
            if self.out_status != connection_parser::htp_stream_state_t::HTP_STREAM_CLOSED {
                // Get one byte
                if self.out_current_read_offset < self.out_current_len {
                    self.out_next_byte = unsafe {
                        *self
                            .out_current_data
                            .offset(self.out_current_read_offset as isize)
                            as i32
                    };
                    self.out_current_read_offset += 1;
                    self.out_stream_offset += 1
                } else {
                    return Err(Status::DATA_BUFFER);
                }
            }
            // Have we reached the end of the line? We treat stream closure as end of line in
            // order to handle the case when the first line of the response is actually response body
            // (and we wish it processed as such).
            if self.out_next_byte == '\r' as i32 {
                if self.out_current_read_offset >= self.out_current_len {
                    self.out_next_byte = -1
                } else {
                    self.out_next_byte = unsafe {
                        *self
                            .out_current_data
                            .offset(self.out_current_read_offset as isize)
                            as i32
                    }
                }
                if self.out_next_byte == -1 {
                    return Err(Status::DATA_BUFFER);
                } else {
                    if self.out_next_byte == '\n' as i32 {
                        continue;
                    }
                    self.out_next_byte = '\n' as i32
                }
            }
            if self.out_next_byte == '\n' as i32
                || self.out_status == connection_parser::htp_stream_state_t::HTP_STREAM_CLOSED
            {
                let mut data: *mut u8 = 0 as *mut u8;
                let mut len: usize = 0;
                self.res_consolidate_data(&mut data, &mut len)?;
                // Is this a line that should be ignored?
                if !data.is_null()
                    && util::connp_is_line_ignorable(
                        unsafe { (*self.cfg).server_personality },
                        unsafe { std::slice::from_raw_parts(data, len) },
                    )
                {
                    if self.out_status == connection_parser::htp_stream_state_t::HTP_STREAM_CLOSED {
                        self.out_state = State::FINALIZE
                    }
                    // We have an empty/whitespace line, which we'll note, ignore and move on
                    self.out_tx_mut_ok()?.response_ignored_lines =
                        self.out_tx_mut_ok()?.response_ignored_lines.wrapping_add(1);
                    // TODO How many lines are we willing to accept?
                    // Start again
                    self.res_clear_buffer();
                    return Ok(());
                }
                // Deallocate previous response line allocations, which we would have on a 100 response.
                self.out_tx_mut_ok()?.response_line = None;
                self.out_tx_mut_ok()?.response_protocol = None;
                self.out_tx_mut_ok()?.response_status = None;
                self.out_tx_mut_ok()?.response_message = None;
                // Process response line.
                let s = unsafe { std::slice::from_raw_parts(data as *const u8, len) };
                let s = util::chomp(&s);
                let chomp_result = len - s.len();
                len = s.len();
                // If the response line is invalid, determine if it _looks_ like
                // a response line. If it does not look like a line, process the
                // data as a response body because that is what browsers do.
                if util::treat_response_line_as_body(s) {
                    self.out_tx_mut_ok()?.response_content_encoding_processing =
                        decompressors::htp_content_encoding_t::HTP_COMPRESSION_NONE;
                    self.out_current_consume_offset = self.out_current_read_offset;
                    unsafe {
                        self.res_process_body_data_ex(
                            data as *const core::ffi::c_void,
                            len.wrapping_add(chomp_result),
                        )?;
                    }
                    // Continue to process response body. Because we don't have
                    // any headers to parse, we assume the body continues until
                    // the end of the stream.
                    // Have we seen the entire response body?
                    if self.out_current_len <= self.out_current_read_offset {
                        self.out_tx_mut_ok()?.response_transfer_coding =
                            transaction::htp_transfer_coding_t::HTP_CODING_IDENTITY;
                        self.out_tx_mut_ok()?.response_progress =
                            transaction::htp_tx_res_progress_t::HTP_RESPONSE_BODY;
                        self.out_body_data_left = -1;
                        self.out_state = State::FINALIZE
                    }
                    return Ok(());
                }
                self.out_tx_mut_ok()?.response_line = Some(bstr::Bstr::from(s));
                self.parse_response_line()?;
                unsafe { self.state_response_line()? };
                self.res_clear_buffer();
                // Move on to the next phase.
                self.out_state = State::HEADERS;
                self.out_tx_mut_ok()?.response_progress =
                    transaction::htp_tx_res_progress_t::HTP_RESPONSE_HEADERS;
                return Ok(());
            }
        }
    }

    pub fn RES_FINALIZE(&mut self) -> Result<()> {
        if self.out_status != connection_parser::htp_stream_state_t::HTP_STREAM_CLOSED {
            if self.out_current_read_offset >= self.out_current_len {
                self.out_next_byte = -1
            } else {
                self.out_next_byte = unsafe {
                    *self
                        .out_current_data
                        .offset(self.out_current_read_offset as isize) as i32
                }
            }
            if self.out_next_byte == -1 {
                return unsafe { self.state_response_complete_ex(0).into() };
            }
            if self.out_next_byte != '\n' as i32
                || self.out_current_consume_offset >= self.out_current_read_offset
            {
                loop {
                    //;i < max_read; i++) {
                    if self.out_current_read_offset < self.out_current_len {
                        self.out_next_byte = unsafe {
                            *self
                                .out_current_data
                                .offset(self.out_current_read_offset as isize)
                                as i32
                        };
                        self.out_current_read_offset += 1;
                        self.out_stream_offset += 1
                    } else {
                        return Err(Status::DATA_BUFFER);
                    }
                    // Have we reached the end of the line? For some reason
                    // we can't test after IN_COPY_BYTE_OR_RETURN */
                    if self.out_next_byte == '\n' as i32 {
                        break;
                    }
                }
            }
        }
        let mut bytes_left: usize = 0;
        let mut data: *mut u8 = 0 as *mut u8;
        self.res_consolidate_data(&mut data, &mut bytes_left)?;
        if bytes_left == 0 {
            //closing
            return unsafe { self.state_response_complete_ex(0).into() };
        }
        if util::treat_response_line_as_body(unsafe {
            std::slice::from_raw_parts(data, bytes_left)
        }) {
            // Interpret remaining bytes as body data
            unsafe {
                htp_warn!(
                    self as *mut connection_parser::ConnectionParser,
                    htp_log_code::RESPONSE_BODY_UNEXPECTED,
                    "Unexpected response body"
                );
            }
            let rc = unsafe {
                self.res_process_body_data_ex(data as *const core::ffi::c_void, bytes_left)
            };
            self.res_clear_buffer();
            return rc;
        }
        //unread last end of line so that RES_LINE works
        if self.out_current_read_offset < bytes_left as i64 {
            self.out_current_read_offset = 0
        } else {
            self.out_current_read_offset =
                (self.out_current_read_offset as u64).wrapping_sub(bytes_left as u64) as i64
        }
        if self.out_current_read_offset < self.out_current_consume_offset {
            self.out_current_consume_offset = self.out_current_read_offset
        }
        unsafe { self.state_response_complete_ex(0).into() }
    }

    /// The response idle state will initialize response processing, as well as
    /// finalize each transactions after we are done with it.
    ///
    /// Returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
    pub fn RES_IDLE(&mut self) -> Result<()> {
        // We want to start parsing the next response (and change
        // the state from IDLE) only if there's at least one
        // byte of data available. Otherwise we could be creating
        // new structures even if there's no more data on the
        // connection.
        if self.out_current_read_offset >= self.out_current_len {
            return Err(Status::DATA);
        }
        // Parsing a new response
        // Find the next outgoing transaction
        // If there is none, we just create one so that responses without
        // request can still be processed.
        self.set_out_tx_id(self.conn.tx(self.out_next_tx_index).map(|tx| tx.index));

        if self.out_tx().is_none() {
            unsafe {
                htp_error!(
                    self as *mut connection_parser::ConnectionParser,
                    htp_log_code::UNABLE_TO_MATCH_RESPONSE_TO_REQUEST,
                    "Unable to match response to request"
                );
            }
            // finalize dangling request waiting for next request or body
            if self.in_state == State::FINALIZE {
                // Ignore result.
                let _ = unsafe { self.state_request_complete() };
            }
            let tx_id = self.create_tx()?;
            self.set_out_tx_id(Some(tx_id));
            let out_tx = self.out_tx_mut_ok()?;

            let mut uri = util::Uri::new();
            uri.set_path(b"/libhtp::request_uri_not_seen");
            out_tx.parsed_uri = Some(uri);
            out_tx.request_uri = Some(bstr::Bstr::from("/libhtp::request_uri_not_seen"));
            self.in_state = State::FINALIZE;
            // We've used one transaction
            self.out_next_tx_index = self.out_next_tx_index.wrapping_add(1)
        } else {
            // We've used one transaction
            self.out_next_tx_index = self.out_next_tx_index.wrapping_add(1);
            // TODO Detect state mismatch
            self.out_content_length = -1;
            self.out_body_data_left = -1
        }
        unsafe { self.state_response_start() }
    }

    /// Process a chunk of outbound (server or response) data.
    ///
    /// timestamp: Optional.
    ///
    /// Returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed
    pub fn res_data(
        &mut self,
        timestamp: Option<htp_time_t>,
        data: *const core::ffi::c_void,
        len: usize,
    ) -> connection_parser::htp_stream_state_t {
        // Return if the connection is in stop state
        if self.out_status == connection_parser::htp_stream_state_t::HTP_STREAM_STOP {
            unsafe {
                htp_info!(
                    self as *mut connection_parser::ConnectionParser,
                    htp_log_code::PARSER_STATE_ERROR,
                    "Outbound parser is in HTP_STREAM_STOP"
                );
            }
            return connection_parser::htp_stream_state_t::HTP_STREAM_STOP;
        }
        // Return if the connection has had a fatal error
        if self.out_status == connection_parser::htp_stream_state_t::HTP_STREAM_ERROR {
            unsafe {
                htp_error!(
                    self as *mut connection_parser::ConnectionParser,
                    htp_log_code::PARSER_STATE_ERROR,
                    "Outbound parser is in HTP_STREAM_ERROR"
                );
            }
            return connection_parser::htp_stream_state_t::HTP_STREAM_ERROR;
        }
        // Sanity check: we must have a transaction pointer if the state is not IDLE (no outbound transaction)
        if self.out_tx().is_none() && self.out_state != State::IDLE {
            self.out_status = connection_parser::htp_stream_state_t::HTP_STREAM_ERROR;
            unsafe {
                htp_error!(
                    self as *mut connection_parser::ConnectionParser,
                    htp_log_code::MISSING_OUTBOUND_TRANSACTION_DATA,
                    "Missing outbound transaction data"
                );
            }
            return connection_parser::htp_stream_state_t::HTP_STREAM_ERROR;
        }
        // If the length of the supplied data chunk is zero, proceed
        // only if the stream has been closed. We do not allow zero-sized
        // chunks in the API, but we use it internally to force the parsers
        // to finalize parsing.
        if len == 0 && self.out_status != connection_parser::htp_stream_state_t::HTP_STREAM_CLOSED {
            unsafe {
                htp_error!(
                    self as *mut connection_parser::ConnectionParser,
                    htp_log_code::ZERO_LENGTH_DATA_CHUNKS,
                    "Zero-length data chunks are not allowed"
                );
            }
            return connection_parser::htp_stream_state_t::HTP_STREAM_CLOSED;
        }
        // Remember the timestamp of the current response data chunk
        if let Some(timestamp) = timestamp {
            self.out_timestamp = timestamp;
        }
        // Store the current chunk information
        self.out_current_data = data as *mut u8;
        self.out_current_len = len as i64;
        self.out_current_read_offset = 0;
        self.out_current_consume_offset = 0;
        self.out_current_receiver_offset = 0;
        self.conn.track_outbound_data(len);
        // Return without processing any data if the stream is in tunneling
        // mode (which it would be after an initial CONNECT transaction.
        if self.out_status == connection_parser::htp_stream_state_t::HTP_STREAM_TUNNEL {
            return connection_parser::htp_stream_state_t::HTP_STREAM_TUNNEL;
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
            let mut rc;
            //handle gap
            if data.is_null() && len > 0 {
                match self.out_state {
                    State::BODY_IDENTITY_CL_KNOWN | State::BODY_IDENTITY_STREAM_CLOSE => {
                        rc = self.handle_out_state()
                    }
                    State::FINALIZE => unsafe {
                        rc = self.state_response_complete_ex(0);
                    },
                    _ => {
                        unsafe {
                            htp_error!(
                                self as *mut connection_parser::ConnectionParser,
                                htp_log_code::INVALID_GAP,
                                "Gaps are not allowed during this state"
                            );
                        }
                        return connection_parser::htp_stream_state_t::HTP_STREAM_CLOSED;
                    }
                }
            } else {
                rc = self.handle_out_state();
            }

            if rc.is_ok() {
                if self.out_status == connection_parser::htp_stream_state_t::HTP_STREAM_TUNNEL {
                    return connection_parser::htp_stream_state_t::HTP_STREAM_TUNNEL;
                }
                rc = self.res_handle_state_change();
            }
            match rc {
                // Continue looping.
                Ok(_) => {}
                // Do we need more data?
                Err(Status::DATA) | Err(Status::DATA_BUFFER) => {
                    // Ignore result.
                    let _ = self.res_receiver_send_data(false);
                    if rc == Err(Status::DATA_BUFFER) && self.res_buffer().is_err() {
                        self.out_status = connection_parser::htp_stream_state_t::HTP_STREAM_ERROR;
                        return connection_parser::htp_stream_state_t::HTP_STREAM_ERROR;
                    }
                    self.out_status = connection_parser::htp_stream_state_t::HTP_STREAM_DATA;
                    return connection_parser::htp_stream_state_t::HTP_STREAM_DATA;
                }
                // Check for stop
                Err(Status::STOP) => {
                    self.out_status = connection_parser::htp_stream_state_t::HTP_STREAM_STOP;
                    return connection_parser::htp_stream_state_t::HTP_STREAM_STOP;
                }
                // Check for suspended parsing
                Err(Status::DATA_OTHER) => {
                    // We might have actually consumed the entire data chunk?
                    if self.out_current_read_offset >= self.out_current_len {
                        self.out_status = connection_parser::htp_stream_state_t::HTP_STREAM_DATA;
                        // Do not send STREAM_DATE_DATA_OTHER if we've
                        // consumed the entire chunk
                        return connection_parser::htp_stream_state_t::HTP_STREAM_DATA;
                    } else {
                        self.out_status =
                            connection_parser::htp_stream_state_t::HTP_STREAM_DATA_OTHER;
                        // Partial chunk consumption
                        return connection_parser::htp_stream_state_t::HTP_STREAM_DATA_OTHER;
                    }
                }
                // Permanent stream error.
                Err(_) => {
                    self.out_status = connection_parser::htp_stream_state_t::HTP_STREAM_ERROR;
                    return connection_parser::htp_stream_state_t::HTP_STREAM_ERROR;
                }
            }
        }
    }
}

pub fn is_chunked_ctl_char(c: u8) -> bool {
    c == 0x0d || c == 0x0a || c == 0x20 || c == 0x09 || c == 0x0b || c == 0x0
}
