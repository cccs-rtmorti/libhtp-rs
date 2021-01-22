use crate::{
    bstr::Bstr,
    connection_parser::{ConnectionParser, HtpStreamState, State},
    decompressors::HtpContentEncoding,
    error::Result,
    hook::DataHook,
    parsers::{parse_chunked_length, parse_content_length},
    request::HtpMethod,
    transaction::{Data, HtpProtocol, HtpResponseProgress, HtpTransferCoding},
    uri::Uri,
    util::{
        chomp, is_line_ignorable, is_space, take_till_lf, treat_response_line_as_body,
        FlagOperations, HtpFlags,
    },
    HtpStatus,
};
use chrono::{DateTime, Utc};
use nom::{bytes::streaming::take_till as streaming_take_till, error::ErrorKind};
use std::{
    cmp::Ordering,
    io::{Cursor, Seek, SeekFrom},
    mem::take,
};

impl ConnectionParser {
    /// Sends outstanding connection data to the currently active data receiver hook.
    fn res_receiver_send_data(&mut self, is_last: bool) -> Result<()> {
        let mut data = Data::new(
            self.out_tx_mut_ptr(),
            Some(
                &self.out_curr_data.get_ref()[self.out_current_receiver_offset as usize
                    ..self.out_curr_data.position() as usize],
            ),
            is_last,
        );

        if let Some(hook) = &self.out_data_receiver_hook {
            hook.run_all(self, &mut data)?;
        } else {
            return Ok(());
        };
        self.out_current_receiver_offset = self.out_curr_data.position();
        Ok(())
    }

    /// Finalizes an existing data receiver hook by sending any outstanding data to it. The
    /// hook is then removed so that it receives no more data.
    pub fn res_receiver_finalize_clear(&mut self) -> Result<()> {
        if self.out_data_receiver_hook.is_none() {
            return Ok(());
        }
        let rc = self.res_receiver_send_data(true);
        self.out_data_receiver_hook = None;
        rc
    }

    /// Configures the data receiver hook. If there is a previous hook, it will be finalized and cleared.
    fn res_receiver_set(&mut self, data_receiver_hook: Option<DataHook>) -> Result<()> {
        // Ignore result.
        let _ = self.res_receiver_finalize_clear();
        self.out_data_receiver_hook = data_receiver_hook;
        self.out_current_receiver_offset = self.out_curr_data.position();
        Ok(())
    }

    /// Handles request parser state changes. At the moment, this function is used only
    /// to configure data receivers, which are sent raw connection data.
    fn res_handle_state_change(&mut self) -> Result<()> {
        if self.out_state_previous == self.out_state {
            return Ok(());
        }
        if self.out_state == State::HEADERS {
            let header_fn = Some(self.out_tx_mut_ok()?.cfg.hook_response_header_data.clone());
            let trailer_fn = Some(self.out_tx_mut_ok()?.cfg.hook_response_trailer_data.clone());
            match self.out_tx_mut_ok()?.response_progress {
                HtpResponseProgress::HEADERS => self.res_receiver_set(header_fn),
                HtpResponseProgress::TRAILER => self.res_receiver_set(trailer_fn),
                _ => Ok(()),
            }?;
        }
        // Same comment as in req_handle_state_change(). Below is a copy.
        // Initially, I had the finalization of raw data sending here, but that
        // caused the last REQUEST_HEADER_DATA hook to be invoked after the
        // REQUEST_HEADERS hook -- which I thought made no sense. For that reason,
        // the finalization is now initiated from the request header processing code,
        // which is less elegant but provides a better user experience. Having some
        // (or all) hooks to be invoked on state change might work better.
        self.out_state_previous = self.out_state;
        Ok(())
    }

    /// The maximum amount accepted for buffering is controlled
    /// by htp_config_t::field_limit.
    fn check_out_buffer_limit(&mut self, len: usize) -> Result<()> {
        if self.out_curr_len() == 0 || len == 0 {
            return Ok(());
        }
        // Check the hard (buffering) limit.
        let mut newlen: usize = self.out_buf.len().wrapping_add(len);
        // When calculating the size of the buffer, take into account the
        // space we're using for the response header buffer.
        if let Some(out_header) = &self.out_header {
            newlen = newlen.wrapping_add(out_header.len())
        }

        let field_limit = self.out_tx_mut_ok()?.cfg.field_limit;
        if newlen > field_limit {
            htp_error!(
                self,
                HtpLogCode::RESPONSE_FIELD_TOO_LONG,
                format!(
                    "Response the buffer limit: size {} limit {}.",
                    newlen, field_limit
                )
            );
            return Err(HtpStatus::ERROR);
        }
        Ok(())
    }

    /// Consumes bytes until the end of the current line.
    ///
    /// Returns HtpStatus::OK on state change, HtpStatus::Error on error, or HtpStatus::DATA
    /// when more data is needed.
    pub fn res_body_chunked_data_end(&mut self, data: &[u8]) -> Result<()> {
        // TODO We shouldn't really see anything apart from CR and LF,
        //      so we should warn about anything else.
        match take_till_lf(data) {
            Ok((_, line)) => {
                let len = line.len() as i64;
                self.out_curr_data.seek(SeekFrom::Current(len))?;
                self.out_tx_mut_ok()?.response_message_len += len;
                self.out_state = State::BODY_CHUNKED_LENGTH;
                Ok(())
            }
            _ => {
                // Advance to end. Dont need to buffer
                self.out_curr_data.seek(SeekFrom::End(0))?;
                self.out_tx_mut_ok()?.response_message_len += data.len() as i64;
                Err(HtpStatus::DATA_BUFFER)
            }
        }
    }

    /// Processes a chunk of data.
    ///
    /// Returns HtpStatus::OK on state change, HtpStatus::Error on error, or
    /// HtpStatus::DATA when more data is needed.
    pub fn res_body_chunked_data(&mut self, data: &[u8]) -> Result<()> {
        let bytes_to_consume: usize = std::cmp::min(
            data.len(),
            self.out_chunked_length.map(|len| len).unwrap_or(0) as usize,
        );
        if bytes_to_consume == 0 {
            return Err(HtpStatus::DATA);
        }
        // Consume the data.
        self.res_process_body_data_ex(Some(&data[0..bytes_to_consume]))?;
        // Adjust the counters.
        self.out_curr_data
            .seek(SeekFrom::Current(bytes_to_consume as i64))?;
        if let Some(len) = &mut self.out_chunked_length {
            *len = len.wrapping_sub(bytes_to_consume as i32);
            // Have we seen the entire chunk?
            if *len == 0 {
                self.out_state = State::BODY_CHUNKED_DATA_END;
                return Ok(());
            }
        }

        Err(HtpStatus::DATA)
    }

    /// Extracts chunk length.
    ///
    /// Returns Ok(()) on success, Err(HTP_ERROR) on error, or Err(HTP_DATA) when more data is needed.
    pub fn res_body_chunked_length(&mut self, data: &[u8]) -> Result<()> {
        match take_till_lf(data) {
            Ok((_, line)) => {
                self.out_curr_data
                    .seek(SeekFrom::Current(line.len() as i64))?;
                if !self.out_buf.is_empty() {
                    self.check_out_buffer_limit(line.len())?;
                }
                let mut data = take(&mut self.out_buf);
                data.add(line);
                self.out_tx_mut_ok()?.response_message_len =
                    (self.out_tx_mut_ok()?.response_message_len as u64)
                        .wrapping_add(data.len() as u64) as i64;

                match parse_chunked_length(&data) {
                    Ok(len) => {
                        self.out_chunked_length = len;
                        // Handle chunk length
                        if let Some(len) = len {
                            match len.cmp(&0) {
                                Ordering::Greater => {
                                    // More data available
                                    self.out_state = State::BODY_CHUNKED_DATA
                                }
                                Ordering::Equal => {
                                    // End of data
                                    self.out_state = State::HEADERS;
                                    self.out_tx_mut_ok()?.response_progress =
                                        HtpResponseProgress::TRAILER
                                }
                                _ => return Ok(()), // empty chunk length line, lets try to continue
                            }
                        }
                    }
                    Err(_) => {
                        self.out_state = State::BODY_IDENTITY_STREAM_CLOSE;
                        self.out_tx_mut_ok()?.response_transfer_coding =
                            HtpTransferCoding::IDENTITY;
                        htp_error!(
                            self,
                            HtpLogCode::INVALID_RESPONSE_CHUNK_LEN,
                            "Response chunk encoding: Invalid chunk length"
                        );
                    }
                }

                Ok(())
            }
            _ => self.handle_out_absent_lf(data),
        }
    }

    /// Processes an identity response body of known length.
    ///
    /// Returns HtpStatus::OK on state change, HtpStatus::ERROR on error, or
    /// HtpStatus::DATA when more data is needed.
    pub fn res_body_identity_cl_known(&mut self, data: &[u8]) -> Result<()> {
        let bytes_to_consume: usize = std::cmp::min(data.len(), self.out_body_data_left as usize);
        if self.out_status == HtpStreamState::CLOSED {
            self.out_state = State::FINALIZE;
            // Sends close signal to decompressors
            return self.res_process_body_data_ex(None);
        }
        if bytes_to_consume == 0 {
            return Err(HtpStatus::DATA);
        }
        // Consume the data.
        self.res_process_body_data_ex(Some(&data[0..bytes_to_consume]))?;
        // Adjust the counters.
        self.out_curr_data
            .seek(SeekFrom::Current(bytes_to_consume as i64))?;
        self.out_body_data_left =
            (self.out_body_data_left as u64).wrapping_sub(bytes_to_consume as u64) as i64;
        // Have we seen the entire response body?
        if self.out_body_data_left == 0 {
            self.out_state = State::FINALIZE;
            // Tells decompressors to output partially decompressed data
            return self.res_process_body_data_ex(None);
        }
        Err(HtpStatus::DATA)
    }

    /// Processes identity response body of unknown length. In this case, we assume the
    /// response body consumes all data until the end of the stream.
    ///
    /// Returns HtpStatus::OK on state change, HtpStatus::ERROR on error, or HtpStatus::DATA
    /// when more data is needed.
    pub fn res_body_identity_stream_close(&mut self, data: &[u8]) -> Result<()> {
        // Consume all data from the input buffer.
        if !data.is_empty() {
            self.res_process_body_data_ex(Some(&data))?;
            // Adjust the counters.
            self.out_curr_data.seek(SeekFrom::End(0))?;
        }
        // Have we seen the entire response body?
        if self.out_status == HtpStreamState::CLOSED {
            self.out_state = State::FINALIZE;
            return Ok(());
        }
        Err(HtpStatus::DATA)
    }

    /// Determines presence (and encoding) of a response body.
    pub fn res_body_determine(&mut self) -> Result<()> {
        // If the request uses the CONNECT method, then not only are we
        // to assume there's no body, but we need to ignore all
        // subsequent data in the stream.
        if self.out_tx_mut_ok()?.request_method_number == HtpMethod::CONNECT {
            if self
                .out_tx_mut_ok()?
                .response_status_number
                .in_range(200, 299)
            {
                // This is a successful CONNECT stream, which means
                // we need to switch into tunneling mode: on the
                // request side we'll now probe the tunnel data to see
                // if we need to parse or ignore it. So on the response
                // side we wrap up the tx and wait.
                self.out_state = State::FINALIZE;
                // we may have response headers
                return self.state_response_headers();
            } else if self.out_tx_mut_ok()?.response_status_number.eq(407) {
                // proxy telling us to auth
                if self.in_status != HtpStreamState::ERROR {
                    self.in_status = HtpStreamState::DATA
                }
            } else {
                // This is a failed CONNECT stream, which means that
                // we can unblock request parsing
                if self.in_status != HtpStreamState::ERROR {
                    self.in_status = HtpStreamState::DATA
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
        if self.out_tx_mut_ok()?.response_status_number.eq(101) {
            if te_opt.is_none() && cl_opt.is_none() {
                self.out_state = State::FINALIZE;
                if self.in_status != HtpStreamState::ERROR {
                    self.in_status = HtpStreamState::TUNNEL
                }
                self.out_status = HtpStreamState::TUNNEL;
                // we may have response headers
                return self.state_response_headers();
            } else {
                htp_warn!(
                    self,
                    HtpLogCode::SWITCHING_PROTO_WITH_CONTENT_LENGTH,
                    "Switching Protocol with Content-Length"
                );
            }
        }
        // Check for an interim "100 Continue" response. Ignore it if found, and revert back to RES_LINE.
        if self.out_tx_mut_ok()?.response_status_number.eq(100)
            && te_opt.is_none()
            && cl_opt.is_none()
        {
            if self.out_tx_mut_ok()?.seen_100continue {
                htp_error!(
                    self,
                    HtpLogCode::CONTINUE_ALREADY_SEEN,
                    "Already seen 100-Continue."
                );
                return Err(HtpStatus::ERROR);
            }
            // Ignore any response headers seen so far.
            self.out_tx_mut_ok()?.response_headers.elements.clear();
            // Expecting to see another response line next.
            self.out_state = State::LINE;
            self.out_tx_mut_ok()?.response_progress = HtpResponseProgress::LINE;
            self.out_tx_mut_ok()?.seen_100continue = true;
            return Ok(());
        }

        // A request can indicate it waits for headers validation
        // before sending its body cf
        // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Expect
        if self
            .out_tx_mut_ok()?
            .response_status_number
            .in_range(400, 499)
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
        if self.out_tx_mut_ok()?.request_method_number == HtpMethod::HEAD {
            // There's no response body whatsoever
            self.out_tx_mut_ok()?.response_transfer_coding = HtpTransferCoding::NO_BODY;
            self.out_state = State::FINALIZE
        } else if self
            .out_tx_mut_ok()?
            .response_status_number
            .in_range(100, 199)
            || self.out_tx_mut_ok()?.response_status_number.eq(204)
            || self.out_tx_mut_ok()?.response_status_number.eq(304)
        {
            // There should be no response body
            // but browsers interpret content sent by the server as such
            if te_opt.is_none() && cl_opt.is_none() {
                self.out_tx_mut_ok()?.response_transfer_coding = HtpTransferCoding::NO_BODY;
                self.out_state = State::FINALIZE
            } else {
                htp_warn!(
                    self,
                    HtpLogCode::RESPONSE_BODY_UNEXPECTED,
                    "Unexpected Response body"
                );
            }
        }
        // Hack condition to check that we do not assume "no body"
        let mut multipart_byteranges = false;
        if self.out_state != State::FINALIZE {
            // We have a response body
            let response_content_type = if let Some(ct) = &self
                .out_tx_mut_ok()?
                .response_headers
                .get_nocase_nozero("content-type")
                .map(|(_, val)| val)
            {
                // TODO Some platforms may do things differently here.
                let response_content_type = if let Ok((_, ct)) =
                    streaming_take_till::<_, _, (&[u8], ErrorKind)>(|c| c == b';' || is_space(c))(
                        &ct.value,
                    ) {
                    ct
                } else {
                    &ct.value
                };

                let mut response_content_type = Bstr::from(response_content_type);
                response_content_type.make_ascii_lowercase();
                if response_content_type
                    .index_of_nocase("multipart/byteranges")
                    .is_some()
                {
                    multipart_byteranges = true;
                }
                Some(response_content_type)
            } else {
                None
            };

            if response_content_type.is_some() {
                self.out_tx_mut_ok()?.response_content_type = response_content_type;
            }
            // 2. If a Transfer-Encoding header field (section 14.40) is present and
            //   indicates that the "chunked" transfer coding has been applied, then
            //   the length is defined by the chunked encoding (section 3.6).
            if let Some(te) =
                te_opt.and_then(|te| te.value.index_of_nocase_nozero("chunked").and(Some(te)))
            {
                if te.value.cmp_nocase("chunked") != Ordering::Equal {
                    htp_warn!(
                        self,
                        HtpLogCode::RESPONSE_ABNORMAL_TRANSFER_ENCODING,
                        "Transfer-encoding has abnormal chunked value"
                    );
                }
                // 3. If a Content-Length header field (section 14.14) is present, its
                // spec says chunked is HTTP/1.1 only, but some browsers accept it
                // with 1.0 as well
                if self.out_tx_mut_ok()?.response_protocol_number < HtpProtocol::V1_1 {
                    htp_warn!(
                        self,
                        HtpLogCode::RESPONSE_CHUNKED_OLD_PROTO,
                        "Chunked transfer-encoding on HTTP/0.9 or HTTP/1.0"
                    );
                }
                // If the T-E header is present we are going to use it.
                self.out_tx_mut_ok()?.response_transfer_coding = HtpTransferCoding::CHUNKED;
                // We are still going to check for the presence of C-L
                if cl_opt.is_some() {
                    // This is a violation of the RFC
                    self.out_tx_mut_ok()?.flags.set(HtpFlags::REQUEST_SMUGGLING)
                }
                self.out_state = State::BODY_CHUNKED_LENGTH;
                self.out_tx_mut_ok()?.response_progress = HtpResponseProgress::BODY
            } else if let Some(cl) = cl_opt {
                //   value in bytes represents the length of the message-body.
                // We know the exact length
                self.out_tx_mut_ok()?.response_transfer_coding = HtpTransferCoding::IDENTITY;
                // Check for multiple C-L headers
                if cl.flags.is_set(HtpFlags::FIELD_REPEATED) {
                    self.out_tx_mut_ok()?.flags.set(HtpFlags::REQUEST_SMUGGLING)
                }
                // Get body length
                if let Some(content_length) =
                    parse_content_length((*cl.value).as_slice(), Some(self))
                {
                    self.out_tx_mut_ok()?.response_content_length = content_length;
                    self.out_content_length = self.out_tx_mut_ok()?.response_content_length;
                    self.out_body_data_left = self.out_content_length;
                    if self.out_content_length != 0 {
                        self.out_state = State::BODY_IDENTITY_CL_KNOWN;
                        self.out_tx_mut_ok()?.response_progress = HtpResponseProgress::BODY
                    } else {
                        self.out_state = State::FINALIZE
                    }
                } else {
                    let response_content_length = self.out_tx_mut_ok()?.response_content_length;
                    htp_error!(
                        self,
                        HtpLogCode::INVALID_CONTENT_LENGTH_FIELD_IN_RESPONSE,
                        format!("Invalid C-L field in response: {}", response_content_length)
                    );
                    return Err(HtpStatus::ERROR);
                }
            } else {
                // 4. If the message uses the media type "multipart/byteranges", which is
                //   self-delimiting, then that defines the length. This media type MUST
                //   NOT be used unless the sender knows that the recipient can parse it;
                //   the presence in a request of a Range header with multiple byte-range
                //   specifiers implies that the client can parse multipart/byteranges
                //   responses.
                // TODO Handle multipart/byteranges
                if multipart_byteranges {
                    htp_error!(
                        self,
                        HtpLogCode::RESPONSE_MULTIPART_BYTERANGES,
                        "C-T multipart/byteranges in responses not supported"
                    );
                    return Err(HtpStatus::ERROR);
                }
                // 5. By the server closing the connection. (Closing the connection
                //   cannot be used to indicate the end of a request body, since that
                //   would leave no possibility for the server to send back a response.)
                self.out_state = State::BODY_IDENTITY_STREAM_CLOSE;
                self.out_tx_mut_ok()?.response_transfer_coding = HtpTransferCoding::IDENTITY;
                self.out_tx_mut_ok()?.response_progress = HtpResponseProgress::BODY;
                self.out_body_data_left = -1
            }
        }
        // NOTE We do not need to check for short-style HTTP/0.9 requests here because
        //      that is done earlier, before response line parsing begins
        self.state_response_headers()
    }

    /// Parses response headers.
    ///
    /// Returns HtpStatus::OK on state change, HtpStatus::ERROR on error, or HtpStatus::DATA when more data is needed.
    pub fn res_headers(&mut self, data: &[u8]) -> Result<()> {
        if self.out_status == HtpStreamState::CLOSED {
            // Finalize sending raw trailer data.
            self.res_receiver_finalize_clear()?;
            // Run hook response_TRAILER.
            // TODO: Figure out how to do this without clone()
            let cfg = self.cfg.clone();
            let tx_ptr = self.out_tx_mut_ptr();
            cfg.hook_response_trailer
                .run_all(self, unsafe { &mut *tx_ptr })?;
            self.out_state = State::FINALIZE;
            return Ok(());
        }
        let out_header = if let Some(mut out_header) = self.out_header.take() {
            out_header.add(data);
            out_header
        } else {
            Bstr::from(data)
        };

        let (remaining, eoh) = self.process_response_headers(out_header.as_slice())?;
        //TODO: Update the response state machine so that we don't have to have this EOL check
        let eol =
            remaining.len() == out_header.len() && (remaining.eq(b"\r\n") || remaining.eq(b"\n"));
        // If remaining is EOL or header parsing saw EOH this is end of headers
        if eoh || eol {
            if eol {
                //Consume the EOL so it isn't included in data processing
                self.out_curr_data
                    .seek(SeekFrom::Current(data.len() as i64))?;
            } else if remaining.len() <= data.len() {
                self.out_curr_data
                    .seek(SeekFrom::Current((data.len() - remaining.len()) as i64))?;
            }
            // We've seen all response headers. At terminator.
            self.out_state =
                if self.out_tx_mut_ok()?.response_progress == HtpResponseProgress::HEADERS {
                    // Response headers.
                    // The next step is to determine if this response has a body.
                    State::BODY_DETERMINE
                } else {
                    // Response trailer.
                    // Finalize sending raw trailer data.
                    self.res_receiver_finalize_clear()?;
                    // Run hook response_TRAILER.
                    let cfg = self.cfg.clone();
                    let tx_ptr = self.out_tx_mut_ptr();
                    cfg.hook_response_trailer
                        .run_all(self, unsafe { &mut *tx_ptr })?;
                    // The next step is to finalize this response.
                    State::FINALIZE
                };
            Ok(())
        } else {
            self.out_curr_data
                .seek(SeekFrom::Current(data.len() as i64))?;
            self.check_out_buffer_limit(remaining.len())?;
            let remaining = Bstr::from(remaining);
            self.out_header.replace(remaining);
            Err(HtpStatus::DATA_BUFFER)
        }
    }

    /// Parses response line.
    ///
    /// Returns HtpStatus::OK on state change, HtpStatus::ERROR on error, or HtpStatus::DATA
    /// when more data is needed.
    pub fn res_line(&mut self, data: &[u8]) -> Result<()> {
        let line = match take_till_lf(data) {
            Ok((_, line)) => {
                self.out_curr_data
                    .seek(SeekFrom::Current(line.len() as i64))?;
                line
            }
            _ => {
                if self.out_status == HtpStreamState::CLOSED {
                    self.out_curr_data.seek(SeekFrom::End(0))?;
                    data
                } else {
                    return self.handle_out_absent_lf(data);
                }
            }
        };

        if !self.out_buf.is_empty() {
            self.check_out_buffer_limit(data.len())?;
        }
        let mut data = take(&mut self.out_buf);
        data.add(line);

        if is_line_ignorable(self.cfg.server_personality, &data) {
            if self.out_status == HtpStreamState::CLOSED {
                self.out_state = State::FINALIZE
            }
            // We have an empty/whitespace line, which we'll note, ignore and move on
            self.out_tx_mut_ok()?.response_ignored_lines =
                self.out_tx_mut_ok()?.response_ignored_lines.wrapping_add(1);
            // TODO How many lines are we willing to accept?
            // Start again
            return Ok(());
        }
        // Deallocate previous response line allocations, which we would have on a 100 response.
        self.out_tx_mut_ok()?.response_line = None;
        self.out_tx_mut_ok()?.response_protocol = None;
        self.out_tx_mut_ok()?.response_status = None;
        self.out_tx_mut_ok()?.response_message = None;
        // Process response line.
        let data = chomp(&data);
        // If the response line is invalid, determine if it _looks_ like
        // a response line. If it does not look like a line, process the
        // data as a response body because that is what browsers do.
        if treat_response_line_as_body(data) {
            self.out_tx_mut_ok()?.response_content_encoding_processing = HtpContentEncoding::NONE;
            self.res_process_body_data_ex(Some(data))?;
            // Continue to process response body. Because we don't have
            // any headers to parse, we assume the body continues until
            // the end of the stream.
            // Have we seen the entire response body?
            if self.out_curr_len() <= self.out_curr_data.position() as i64 {
                self.out_tx_mut_ok()?.response_transfer_coding = HtpTransferCoding::IDENTITY;
                self.out_tx_mut_ok()?.response_progress = HtpResponseProgress::BODY;
                self.out_body_data_left = -1;
                self.out_state = State::FINALIZE
            }
            return Ok(());
        }
        self.parse_response_line(data)?;
        self.state_response_line()?;
        // Move on to the next phase.
        self.out_state = State::HEADERS;
        self.out_tx_mut_ok()?.response_progress = HtpResponseProgress::HEADERS;
        Ok(())
    }

    /// Finalizes response parsing.
    pub fn res_finalize(&mut self, data: &[u8]) -> Result<()> {
        let mut work = data;
        if self.out_status != HtpStreamState::CLOSED {
            let out_next_byte = self
                .out_curr_data
                .get_ref()
                .get(self.out_curr_data.position() as usize);
            if out_next_byte.is_none() {
                return self.state_response_complete_ex(0);
            }
            let lf = out_next_byte.map(|byte| *byte == b'\n').unwrap_or(false);
            if !lf {
                if let Ok((_, line)) = take_till_lf(data) {
                    self.out_curr_data
                        .seek(SeekFrom::Current(line.len() as i64))?;
                    work = line;
                } else {
                    return self.handle_out_absent_lf(data);
                }
            } else {
                self.out_curr_data
                    .seek(SeekFrom::Current(data.len() as i64))?;
            }
        }
        if !self.out_buf.is_empty() {
            self.check_out_buffer_limit(work.len())?;
        }
        let mut data = take(&mut self.out_buf);
        let buf_len = data.len();
        data.add(work);

        if data.is_empty() {
            //closing
            return self.state_response_complete_ex(0);
        }
        if treat_response_line_as_body(&data) {
            // Interpret remaining bytes as body data
            htp_warn!(
                self,
                HtpLogCode::RESPONSE_BODY_UNEXPECTED,
                "Unexpected response body"
            );
            return self.res_process_body_data_ex(Some(&data));
        }
        // didnt use data, restore
        self.out_buf.add(&data[0..buf_len]);
        //unread last end of line so that RES_LINE works
        if self.out_curr_data.position() < data.len() as u64 {
            self.out_curr_data.seek(SeekFrom::Start(0))?;
        } else {
            self.out_curr_data
                .seek(SeekFrom::Current(-(data.len() as i64)))?;
        }
        self.state_response_complete_ex(0)
    }

    /// The response idle state will initialize response processing, as well as
    /// finalize each transactions after we are done with it.
    ///
    /// Returns HtpStatus::OK on state change, HtpStatus::ERROR on error, or HtpStatus::DATA
    /// when more data is needed.
    pub fn res_idle(&mut self) -> Result<()> {
        // We want to start parsing the next response (and change
        // the state from IDLE) only if there's at least one
        // byte of data available. Otherwise we could be creating
        // new structures even if there's no more data on the
        // connection.
        if self.out_curr_data.position() as i64 >= self.out_curr_len() {
            return Err(HtpStatus::DATA);
        }
        // Parsing a new response
        // Find the next outgoing transaction
        // If there is none, we just create one so that responses without
        // request can still be processed.
        self.set_out_tx_id(self.conn.tx(self.out_next_tx_index).map(|tx| tx.index));

        if self.out_tx().is_none() {
            htp_error!(
                self,
                HtpLogCode::UNABLE_TO_MATCH_RESPONSE_TO_REQUEST,
                "Unable to match response to request"
            );
            // finalize dangling request waiting for next request or body
            if self.in_state == State::FINALIZE {
                // Ignore result.
                let _ = self.state_request_complete();
            }
            let tx_id = self.create_tx()?;
            self.set_in_tx_id(Some(tx_id));
            self.in_reset();
            self.set_out_tx_id(Some(tx_id));
            let out_tx = self.out_tx_mut_ok()?;

            let mut uri = Uri::default();
            uri.path = Some(Bstr::from("/libhtp::request_uri_not_seen"));
            out_tx.request_uri = uri.path.clone();
            out_tx.parsed_uri = Some(uri);
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
        self.state_response_start()
    }

    /// Run the RESPONSE_BODY_DATA hook.
    pub fn res_run_hook_body_data(&mut self, d: &mut Data) -> Result<()> {
        // Do not invoke callbacks with an empty data chunk.
        if d.is_empty() {
            return Ok(());
        }
        if let Some(out_tx) = self.out_tx() {
            // Run transaction hooks first
            out_tx.hook_response_body_data.run_all(self, d)?;
            // Run configuration hooks second
            self.cfg.hook_response_body_data.run_all(self, d)?;
        }
        Ok(())
    }

    /// Process a chunk of outbound (server or response) data.
    pub fn res_data(
        &mut self,
        timestamp: Option<DateTime<Utc>>,
        data: *const core::ffi::c_void,
        len: usize,
    ) -> HtpStreamState {
        // Return if the connection is in stop state
        if self.out_status == HtpStreamState::STOP {
            htp_info!(
                self,
                HtpLogCode::PARSER_STATE_ERROR,
                "Outbound parser is in HTP_STREAM_STATE_STOP"
            );
            return HtpStreamState::STOP;
        }
        // Return if the connection has had a fatal error
        if self.out_status == HtpStreamState::ERROR {
            htp_error!(
                self,
                HtpLogCode::PARSER_STATE_ERROR,
                "Outbound parser is in HTP_STREAM_STATE_ERROR"
            );
            return HtpStreamState::ERROR;
        }
        // Sanity check: we must have a transaction pointer if the state is not IDLE (no outbound transaction)
        if self.out_tx().is_none() && self.out_state != State::IDLE {
            self.out_status = HtpStreamState::ERROR;
            htp_error!(
                self,
                HtpLogCode::MISSING_OUTBOUND_TRANSACTION_DATA,
                "Missing outbound transaction data"
            );
            return HtpStreamState::ERROR;
        }
        // If the length of the supplied data chunk is zero, proceed
        // only if the stream has been closed. We do not allow zero-sized
        // chunks in the API, but we use it internally to force the parsers
        // to finalize parsing.
        if (data.is_null() || len == 0) && self.out_status != HtpStreamState::CLOSED {
            htp_error!(
                self,
                HtpLogCode::ZERO_LENGTH_DATA_CHUNKS,
                "Zero-length data chunks are not allowed"
            );
            return HtpStreamState::CLOSED;
        }
        // Remember the timestamp of the current response data chunk
        if let Some(timestamp) = timestamp {
            self.out_timestamp = timestamp;
        }
        // Store the current chunk information
        let chunk = unsafe { std::slice::from_raw_parts(data as *mut u8, len) };
        self.out_curr_data = Cursor::new(chunk.to_vec());
        self.out_current_receiver_offset = 0;
        self.conn.track_outbound_data(len);
        // Return without processing any data if the stream is in tunneling
        // mode (which it would be after an initial CONNECT transaction.
        if self.out_status == HtpStreamState::TUNNEL {
            return HtpStreamState::TUNNEL;
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
                        rc = self.handle_out_state(chunk)
                    }
                    State::FINALIZE => {
                        rc = self.state_response_complete_ex(0);
                    }
                    _ => {
                        htp_error!(
                            self,
                            HtpLogCode::INVALID_GAP,
                            "Gaps are not allowed during this state"
                        );
                        return HtpStreamState::CLOSED;
                    }
                }
            } else {
                rc = self.handle_out_state(chunk);
            }

            if rc.is_ok() {
                if self.out_status == HtpStreamState::TUNNEL {
                    return HtpStreamState::TUNNEL;
                }
                rc = self.res_handle_state_change();
            }
            match rc {
                // Continue looping.
                Ok(_) => {}
                // Do we need more data?
                Err(HtpStatus::DATA) | Err(HtpStatus::DATA_BUFFER) => {
                    // Ignore result.
                    let _ = self.res_receiver_send_data(false);
                    self.out_status = HtpStreamState::DATA;
                    return HtpStreamState::DATA;
                }
                // Check for stop
                Err(HtpStatus::STOP) => {
                    self.out_status = HtpStreamState::STOP;
                    return HtpStreamState::STOP;
                }
                // Check for suspended parsing
                Err(HtpStatus::DATA_OTHER) => {
                    // We might have actually consumed the entire data chunk?
                    if self.out_curr_data.position() as i64 >= self.out_curr_len() {
                        self.out_status = HtpStreamState::DATA;
                        // Do not send STREAM_DATE_DATA_OTHER if we've
                        // consumed the entire chunk
                        return HtpStreamState::DATA;
                    } else {
                        self.out_status = HtpStreamState::DATA_OTHER;
                        // Partial chunk consumption
                        return HtpStreamState::DATA_OTHER;
                    }
                }
                // Permanent stream error.
                Err(_) => {
                    self.out_status = HtpStreamState::ERROR;
                    return HtpStreamState::ERROR;
                }
            }
        }
    }

    /// Advance out buffer cursor and buffer data.
    pub fn handle_out_absent_lf(&mut self, data: &[u8]) -> Result<()> {
        self.out_curr_data.seek(SeekFrom::End(0))?;
        self.check_out_buffer_limit(data.len())?;
        self.out_buf.add(data);
        Err(HtpStatus::DATA_BUFFER)
    }

    /// Return total length of out buffer data.
    pub fn out_curr_len(&self) -> i64 {
        self.out_curr_data.get_ref().len() as i64
    }
}
