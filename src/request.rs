use crate::{
    bstr::Bstr,
    connection::Flags as ConnectionFlags,
    connection_parser::{ConnectionParser, HtpStreamState, State},
    error::Result,
    hook::DataHook,
    parsers::parse_chunked_length,
    transaction::{Data, HtpRequestProgress, HtpResponseProgress, HtpTransferCoding},
    util::{
        chomp, is_line_ignorable, is_space, nom_take_is_space, take_is_space, take_not_is_space,
        take_till_lf, take_till_lf_null, FlagOperations,
    },
    HtpStatus,
};
use chrono::{DateTime, Utc};
use nom::{
    branch::alt, bytes::complete::take_until, character::complete::char,
    character::is_space as nom_is_space, error::ErrorKind, sequence::tuple,
};
use std::{
    cmp::{min, Ordering},
    io::{Cursor, Seek, SeekFrom},
    mem::take,
};

/// Enumerate HTTP methods.
/// cbindgen:rename-all=QualifiedScreamingSnakeCase
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum HtpMethod {
    /// Used by default, until the method is determined (e.g., before
    /// the request line is processed.
    UNKNOWN,
    /// HEAD
    HEAD,
    /// GET
    GET,
    /// PUT
    PUT,
    /// POST
    POST,
    /// DELETE
    DELETE,
    /// CONNECT
    CONNECT,
    /// OPTIONS
    OPTIONS,
    /// TRACE
    TRACE,
    /// PATCH
    PATCH,
    /// PROPFIND
    PROPFIND,
    /// PROPPATCH
    PROPPATCH,
    /// MKCOL
    MKCOL,
    /// COPY
    COPY,
    /// MOVE
    MOVE,
    /// LOCK
    LOCK,
    /// UNLOCK
    UNLOCK,
    /// VERSION_CONTROL
    VERSION_CONTROL,
    /// CHECKOUT
    CHECKOUT,
    /// UNCHECKOUT
    UNCHECKOUT,
    /// CHECKIN
    CHECKIN,
    /// UPDATE
    UPDATE,
    /// LABEL
    LABEL,
    /// REPORT
    REPORT,
    /// MKWORKSPACE
    MKWORKSPACE,
    /// MKACTIVITY
    MKACTIVITY,
    /// BASELINE_CONTROL
    BASELINE_CONTROL,
    /// MERGE
    MERGE,
    /// INVALID
    INVALID,
    /// ERROR
    ERROR,
}

impl HtpMethod {
    /// Creates a new HtpMethod from the slice.
    pub fn new(method: &[u8]) -> Self {
        match method {
            b"GET" => HtpMethod::GET,
            b"PUT" => HtpMethod::PUT,
            b"POST" => HtpMethod::POST,
            b"DELETE" => HtpMethod::DELETE,
            b"CONNECT" => HtpMethod::CONNECT,
            b"OPTIONS" => HtpMethod::OPTIONS,
            b"TRACE" => HtpMethod::TRACE,
            b"PATCH" => HtpMethod::PATCH,
            b"PROPFIND" => HtpMethod::PROPFIND,
            b"PROPPATCH" => HtpMethod::PROPPATCH,
            b"MKCOL" => HtpMethod::MKCOL,
            b"COPY" => HtpMethod::COPY,
            b"MOVE" => HtpMethod::MOVE,
            b"LOCK" => HtpMethod::LOCK,
            b"UNLOCK" => HtpMethod::UNLOCK,
            b"VERSION-CONTROL" => HtpMethod::VERSION_CONTROL,
            b"CHECKOUT" => HtpMethod::CHECKOUT,
            b"UNCHECKOUT" => HtpMethod::UNCHECKOUT,
            b"CHECKIN" => HtpMethod::CHECKIN,
            b"UPDATE" => HtpMethod::UPDATE,
            b"LABEL" => HtpMethod::LABEL,
            b"REPORT" => HtpMethod::REPORT,
            b"MKWORKSPACE" => HtpMethod::MKWORKSPACE,
            b"MKACTIVITY" => HtpMethod::MKACTIVITY,
            b"BASELINE-CONTROL" => HtpMethod::BASELINE_CONTROL,
            b"MERGE" => HtpMethod::MERGE,
            b"INVALID" => HtpMethod::INVALID,
            b"HEAD" => HtpMethod::HEAD,
            _ => HtpMethod::UNKNOWN,
        }
    }
}
impl ConnectionParser {
    /// Sends outstanding connection data to the currently active data receiver hook.
    fn req_receiver_send_data(&mut self, is_last: bool) -> Result<()> {
        let mut data = Data::new(
            self.in_tx_mut_ptr(),
            Some(
                &self.in_curr_data.get_ref()[self.in_current_receiver_offset as usize
                    ..self.in_curr_data.position() as usize],
            ),
            is_last,
        );
        if let Some(hook) = &self.in_data_receiver_hook {
            hook.run_all(self, &mut data)?;
        } else {
            return Ok(());
        };
        self.in_current_receiver_offset = self.in_curr_data.position();
        Ok(())
    }

    /// Configures the data receiver hook. If there is a previous hook, it will be finalized and cleared.
    fn req_receiver_set(&mut self, data_receiver_hook: Option<DataHook>) -> Result<()> {
        // Ignore result.
        let _ = self.req_receiver_finalize_clear();
        self.in_data_receiver_hook = data_receiver_hook;
        self.in_current_receiver_offset = self.in_curr_data.position();
        Ok(())
    }

    /// Finalizes an existing data receiver hook by sending any outstanding data to it. The
    /// hook is then removed so that it receives no more data.
    pub fn req_receiver_finalize_clear(&mut self) -> Result<()> {
        if self.in_data_receiver_hook.is_none() {
            return Ok(());
        }
        let rc = self.req_receiver_send_data(true);
        self.in_data_receiver_hook = None;
        rc
    }

    /// Handles request parser state changes. At the moment, this function is used only
    /// to configure data receivers, which are sent raw connection data.
    fn req_handle_state_change(&mut self) -> Result<()> {
        if self.in_state_previous == self.in_state {
            return Ok(());
        }
        if self.in_state == State::HEADERS {
            let header_fn = Some(self.in_tx_mut_ok()?.cfg.hook_request_header_data.clone());
            let trailer_fn = Some(self.in_tx_mut_ok()?.cfg.hook_request_trailer_data.clone());

            match self.in_tx_mut_ok()?.request_progress {
                HtpRequestProgress::HEADERS => self.req_receiver_set(header_fn),
                HtpRequestProgress::TRAILER => self.req_receiver_set(trailer_fn),
                _ => Ok(()),
            }?;
        }
        // Initially, I had the finalization of raw data sending here, but that
        // caused the last REQUEST_HEADER_DATA hook to be invoked after the
        // REQUEST_HEADERS hook -- which I thought made no sense. For that reason,
        // the finalization is now initiated from the request header processing code,
        // which is less elegant but provides a better user experience. Having some
        // (or all) hooks to be invoked on state change might work better.
        self.in_state_previous = self.in_state;
        Ok(())
    }

    /// If there is any data left in the inbound data chunk, this function will preserve
    /// it for later consumption. The maximum amount accepted for buffering is controlled
    /// by Config::field_limit.
    fn check_in_buffer_limit(&mut self, len: usize) -> Result<()> {
        if len == 0 {
            return Ok(());
        }
        // Check the hard (buffering) limit.
        let mut newlen: usize = self.in_buf.len().wrapping_add(len);
        // When calculating the size of the buffer, take into account the
        // space we're using for the request header buffer.
        if let Some(header) = &self.in_header {
            newlen = newlen.wrapping_add(header.len())
        }
        let field_limit = self.in_tx_mut_ok()?.cfg.field_limit;
        if newlen > field_limit {
            htp_error!(
                self.logger,
                HtpLogCode::REQUEST_FIELD_TOO_LONG,
                format!(
                    "Request buffer over the limit: size {} limit {}.",
                    newlen, field_limit
                )
            );
            return Err(HtpStatus::ERROR);
        }
        Ok(())
    }

    /// Performs a check for a CONNECT transaction to decide whether inbound
    /// parsing needs to be suspended.
    ///
    /// Returns OK if the request does not use CONNECT, or HtpStatus::DATA_OTHER if
    /// inbound parsing needs to be suspended until we hear from the
    /// other side.
    pub fn req_connect_check(&mut self) -> Result<()> {
        // If the request uses the CONNECT method, then there will
        // not be a request body, but first we need to wait to see the
        // response in order to determine if the tunneling request
        // was a success.
        if self.in_tx_mut_ok()?.request_method_number == HtpMethod::CONNECT {
            self.in_state = State::CONNECT_WAIT_RESPONSE;
            self.in_status = HtpStreamState::DATA_OTHER;
            return Err(HtpStatus::DATA_OTHER);
        }
        // Continue to the next step to determine
        // the presence of request body
        self.in_state = State::BODY_DETERMINE;
        Ok(())
    }

    /// Determines whether inbound parsing needs to continue or stop. In
    /// case the data appears to be plain text HTTP, we try to continue.
    ///
    /// Returns OK if the parser can resume parsing, HtpStatus::DATA_BUFFER if
    /// we need more data.
    pub fn req_connect_probe_data(&mut self, line: &[u8]) -> Result<()> {
        let data = if let Ok((_, data)) = take_till_lf_null(line) {
            data
        } else {
            return self.handle_in_absent_lf(line);
        };

        if !self.in_buf.is_empty() {
            self.check_in_buffer_limit(data.len())?;
        }
        // copy, will still need buffer data for next state.
        let mut buffered = self.in_buf.clone();
        buffered.add(data);

        // The request method starts at the beginning of the
        // line and ends with the first whitespace character.
        // We skip leading whitespace as IIS allows this.
        let res = tuple::<_, _, (_, ErrorKind), _>((take_is_space, take_not_is_space))(
            buffered.as_slice(),
        );
        if let Ok((_, (_, method))) = res {
            if HtpMethod::new(method) == HtpMethod::UNKNOWN {
                self.in_status = HtpStreamState::TUNNEL;
                self.out_status = HtpStreamState::TUNNEL
            } else {
                return self.state_request_complete();
            }
        };
        Ok(())
    }

    /// Determines whether inbound parsing, which was suspended after
    /// encountering a CONNECT transaction, can proceed (after receiving
    /// the response).
    ///
    /// Returns OK if the parser can resume parsing, HtpStatus::DATA_OTHER if
    /// it needs to continue waiting.
    pub fn req_connect_wait_response(&mut self) -> Result<()> {
        // Check that we saw the response line of the current inbound transaction.
        if self.in_tx_mut_ok()?.response_progress <= HtpResponseProgress::LINE {
            return Err(HtpStatus::DATA_OTHER);
        }
        // A 2xx response means a tunnel was established. Anything
        // else means we continue to follow the HTTP stream.
        if self
            .in_tx_mut_ok()?
            .response_status_number
            .in_range(200, 299)
        {
            // TODO Check that the server did not accept a connection to itself.
            // The requested tunnel was established: we are going
            // to probe the remaining data on this stream to see
            // if we need to ignore it or parse it
            self.in_state = State::CONNECT_PROBE_DATA;
        } else {
            // No tunnel; continue to the next transaction
            self.in_state = State::FINALIZE
        }
        Ok(())
    }

    /// Consumes bytes until the end of the current line.
    ///
    /// Returns OK on state change, ERROR on error, or HtpStatus::DATA_BUFFER
    /// when more data is needed.
    pub fn req_body_chunked_data_end(&mut self, data: &[u8]) -> Result<()> {
        // TODO We shouldn't really see anything apart from CR and LF,
        //      so we should warn about anything else.
        if let Ok((_, parsed)) = take_till_lf(data) {
            let len = parsed.len() as i64;
            self.in_curr_data.seek(SeekFrom::Current(len))?;
            self.in_tx_mut_ok()?.request_message_len += len;
            self.in_state = State::BODY_CHUNKED_LENGTH;
            Ok(())
        } else {
            self.in_tx_mut_ok()?.request_message_len += data.len() as i64;
            self.handle_in_absent_lf(data)
        }
    }

    /// Processes a chunk of data.
    ///
    /// Returns OK on state change, ERROR on error, or HtpStatus::DATA_BUFFER
    /// when more data is needed.
    pub fn req_body_chunked_data(&mut self, data: &[u8]) -> Result<()> {
        // Determine how many bytes we can consume.
        let bytes_to_consume: usize = min(data.len(), self.in_chunked_length.unwrap_or(0) as usize);
        // If the input buffer is empty, ask for more data.
        if bytes_to_consume == 0 {
            return Err(HtpStatus::DATA);
        }
        // Consume the data.
        self.req_process_body_data_ex(&data[0..bytes_to_consume])?;
        // Adjust counters.
        self.in_curr_data
            .seek(SeekFrom::Current(bytes_to_consume as i64))?;
        self.in_tx_mut_ok()?.request_message_len = (self.in_tx_mut_ok()?.request_message_len as u64)
            .wrapping_add(bytes_to_consume as u64)
            as i64;
        if let Some(len) = &mut self.in_chunked_length {
            *len = len.wrapping_sub(bytes_to_consume as i32);
            if *len == 0 {
                // End of the chunk.
                self.in_state = State::BODY_CHUNKED_DATA_END;
                return Ok(());
            }
        }
        // Ask for more data.
        Err(HtpStatus::DATA)
    }

    /// Extracts chunk length.
    /// Returns OK on state change, ERROR on error, or HtpStatus::DATA_BUFFER
    /// when more data is needed.
    pub fn req_body_chunked_length(&mut self, data: &[u8]) -> Result<()> {
        if let Ok((_, line)) = take_till_lf(data) {
            self.in_curr_data
                .seek(SeekFrom::Current(line.len() as i64))?;
            if !self.in_buf.is_empty() {
                self.check_in_buffer_limit(line.len())?;
            }
            let mut data = take(&mut self.in_buf);
            data.add(line);

            self.in_tx_mut_ok()?.request_message_len =
                (self.in_tx_mut_ok()?.request_message_len as u64).wrapping_add(data.len() as u64)
                    as i64;
            // Handle chunk length.
            match parse_chunked_length(&data) {
                Ok(len) => {
                    self.in_chunked_length = len;
                    if let Some(len) = len {
                        match len.cmp(&0) {
                            Ordering::Equal => {
                                // End of data
                                self.in_state = State::HEADERS;
                                self.in_tx_mut_ok()?.request_progress = HtpRequestProgress::TRAILER
                            }
                            Ordering::Greater => {
                                // More data available.
                                self.in_state = State::BODY_CHUNKED_DATA
                            }
                            _ => {}
                        }
                        Ok(())
                    } else {
                        // Invalid chunk length
                        htp_error!(
                            self.logger,
                            HtpLogCode::INVALID_REQUEST_CHUNK_LEN,
                            "Request chunk encoding: Invalid chunk length"
                        );
                        Err(HtpStatus::ERROR)
                    }
                }
                Err(_) => Err(HtpStatus::ERROR),
            }
        } else {
            self.handle_in_absent_lf(data)
        }
    }

    /// Processes identity request body.
    ///
    /// Returns OK on state change, ERROR on error, or HtpStatus::DATA_BUFFER
    /// when more data is needed.
    pub fn req_body_identity(&mut self, data: &[u8]) -> Result<()> {
        // Determine how many bytes we can consume.
        let bytes_to_consume: usize = min(data.len(), self.in_body_data_left as usize);
        // If the input buffer is empty, ask for more data.
        if bytes_to_consume == 0 {
            return Err(HtpStatus::DATA);
        }
        // Consume data.
        self.req_process_body_data_ex(&data[0..bytes_to_consume])?;
        // Adjust counters.
        self.in_curr_data
            .seek(SeekFrom::Current(bytes_to_consume as i64))?;
        self.in_tx_mut_ok()?.request_message_len = (self.in_tx_mut_ok()?.request_message_len as u64)
            .wrapping_add(bytes_to_consume as u64)
            as i64;
        self.in_body_data_left =
            (self.in_body_data_left as u64).wrapping_sub(bytes_to_consume as u64) as i64;
        if self.in_body_data_left == 0 {
            // End of request body.
            self.in_state = State::FINALIZE;
            return Ok(());
        }
        // Ask for more data.
        Err(HtpStatus::DATA)
    }

    /// Determines presence (and encoding) of a request body.
    ///
    /// Returns OK on state change, ERROR on error, or HtpStatus::DATA_BUFFER
    /// when more data is needed.
    pub fn req_body_determine(&mut self) -> Result<()> {
        // Determine the next state based on the presence of the request
        // body, and the coding used.
        match self.in_tx_mut_ok()?.request_transfer_coding {
            HtpTransferCoding::CHUNKED => {
                self.in_state = State::BODY_CHUNKED_LENGTH;
                self.in_tx_mut_ok()?.request_progress = HtpRequestProgress::BODY
            }
            HtpTransferCoding::IDENTITY => {
                self.in_content_length = self.in_tx_mut_ok()?.request_content_length;
                self.in_body_data_left = self.in_content_length;
                if self.in_content_length != 0 {
                    self.in_state = State::BODY_IDENTITY;
                    self.in_tx_mut_ok()?.request_progress = HtpRequestProgress::BODY
                } else {
                    self.in_state = State::FINALIZE
                }
            }
            HtpTransferCoding::NO_BODY => {
                // This request does not have a body, which
                // means that we're done with it
                self.in_state = State::FINALIZE
            }
            _ => {
                // Should not be here
                return Err(HtpStatus::ERROR);
            }
        }
        Ok(())
    }

    /// Parses request headers.
    /// Returns OK on state change, ERROR on error, or HtpStatus::DATA_BUFFER
    /// when more data is needed.
    pub fn req_headers(&mut self, data: &[u8]) -> Result<()> {
        if self.in_status == HtpStreamState::CLOSED {
            // Parse previous header, if any.
            if let Some(in_header) = self.in_header.take() {
                self.process_request_headers(in_header.as_slice())?;
            }
            self.in_buf.clear();
            self.in_tx_mut_ok()?.request_progress = HtpRequestProgress::TRAILER;
            // We've seen all the request headers.
            return self.state_request_headers();
        }
        let in_header = if let Some(mut in_header) = self.in_header.take() {
            in_header.add(data);
            in_header
        } else {
            Bstr::from(data)
        };

        let (remaining, eoh) = self.process_request_headers(in_header.as_slice())?;
        //TODO: Update the request state machine so that we don't have to have this EOL check
        let eol = remaining.len() == in_header.len()
            && (remaining.starts_with(b"\r\n") || remaining.starts_with(b"\n"));
        if eoh
            //If the input started with an EOL, we assume this is the end of the headers
            || eol
        {
            if remaining.len() < data.len() {
                self.in_curr_data
                    .seek(SeekFrom::Current((data.len() - remaining.len()) as i64))?;
            } else if eol {
                if remaining.starts_with(b"\r\n") {
                    self.in_curr_data
                        .seek(SeekFrom::Current(min(data.len() as i64, 2)))?;
                } else if remaining.starts_with(b"\n") {
                    self.in_curr_data
                        .seek(SeekFrom::Current(min(data.len() as i64, 1)))?;
                }
            }
            // We've seen all the request headers.
            self.state_request_headers()
        } else {
            self.in_curr_data
                .seek(SeekFrom::Current(data.len() as i64))?;
            self.check_in_buffer_limit(remaining.len())?;
            let remaining = Bstr::from(remaining);
            self.in_header.replace(remaining);
            Err(HtpStatus::DATA_BUFFER)
        }
    }

    /// Determines request protocol.
    /// Returns OK on state change, ERROR on error, or HtpStatus::DATA_BUFFER
    /// when more data is needed.
    pub fn req_protocol(&mut self, data: &[u8]) -> Result<()> {
        // Is this a short-style HTTP/0.9 request? If it is,
        // we will not want to parse request headers.
        if !self.in_tx_mut_ok()?.is_protocol_0_9 {
            // Switch to request header parsing.
            self.in_state = State::HEADERS;
            self.in_tx_mut_ok()?.request_progress = HtpRequestProgress::HEADERS
        } else {
            let parser =
                tuple::<_, _, (_, ErrorKind), _>((take_until::<_, &[u8], _>(":"), char(':')));
            match parser(data) {
                Ok((_, (hdr, _))) => {
                    if let Ok((_, space)) = alt((nom_take_is_space, take_is_space))(hdr) {
                        let mut afterspace = false;
                        for c in space {
                            if nom_is_space(*c) {
                                afterspace = true;
                            } else if afterspace || is_space(*c) {
                                break;
                            }
                        }
                    }
                    htp_warn!(
                        self.logger,
                        HtpLogCode::REQUEST_LINE_NO_PROTOCOL,
                        "Request line: missing protocol"
                    );
                    self.in_tx_mut_ok()?.is_protocol_0_9 = false;
                    // Switch to request header parsing.
                    self.in_state = State::HEADERS;
                    self.in_tx_mut_ok()?.request_progress = HtpRequestProgress::HEADERS;
                    return Ok(());
                }
                Err(_) => {
                    // We're done with this request.
                    self.in_state = State::FINALIZE;
                }
            }
        }
        Ok(())
    }

    /// Parse the request line.
    ///
    /// Returns OK on state change, ERROR on error, or HtpStatus::DATA_BUFFER
    /// when more data is needed.
    pub fn req_line_complete(&mut self, line: &[u8]) -> Result<()> {
        if !self.in_buf.is_empty() {
            self.check_in_buffer_limit(line.len())?;
        }
        let mut data = take(&mut self.in_buf);
        data.add(line);
        if data.is_empty() {
            return Err(HtpStatus::DATA);
        }
        // Is this a line that should be ignored?
        if is_line_ignorable(self.cfg.server_personality, &data) {
            // We have an empty/whitespace line, which we'll note, ignore and move on.
            self.in_tx_mut_ok()?.request_ignored_lines =
                self.in_tx_mut_ok()?.request_ignored_lines.wrapping_add(1);
            return Ok(());
        }
        // Process request line.
        let data = chomp(&data);
        self.in_tx_mut_ok()?.request_line = Some(Bstr::from(data));
        self.parse_request_line(data)?;
        // Finalize request line parsing.
        self.state_request_line()?;
        Ok(())
    }

    /// Parses request line.
    ///
    /// Returns OK on state change, ERROR on error, or HtpStatus::DATA_BUFFER
    /// when more data is needed.
    pub fn req_line(&mut self, data: &[u8]) -> Result<()> {
        match take_till_lf(data) {
            Ok((_, read)) => {
                self.in_curr_data
                    .seek(SeekFrom::Current(read.len() as i64))?;
                self.req_line_complete(read)
            }
            _ => {
                if self.in_status == HtpStreamState::CLOSED {
                    self.in_curr_data.seek(SeekFrom::End(0))?;
                    self.req_line_complete(data)
                } else {
                    self.handle_in_absent_lf(data)
                }
            }
        }
    }

    /// Finalizes request.
    ///
    /// Returns OK on state change, ERROR on error, or HtpStatus::DATA_BUFFER
    /// when more data is needed.
    pub fn req_finalize(&mut self, data: &[u8]) -> Result<()> {
        let mut work = data;
        if self.in_status != HtpStreamState::CLOSED {
            let in_next_byte = self
                .in_curr_data
                .get_ref()
                .get(self.in_curr_data.position() as usize);
            if in_next_byte.is_none() {
                return self.state_request_complete();
            }
            let lf = in_next_byte.map(|byte| *byte == b'\n').unwrap_or(false);
            if !lf {
                if let Ok((_, line)) = take_till_lf(data) {
                    self.in_curr_data
                        .seek(SeekFrom::Current(line.len() as i64))?;
                    work = line;
                } else {
                    return self.handle_in_absent_lf(data);
                }
            }
        }

        if !self.in_buf.is_empty() {
            self.check_in_buffer_limit(work.len())?;
        }
        self.in_buf.add(work);
        let mut data = take(&mut self.in_buf);
        if data.is_empty() {
            return self.state_request_complete();
        }

        let res = tuple::<_, _, (&[u8], ErrorKind), _>((take_is_space, take_not_is_space))(&data);

        if let Ok((_, (_, method))) = res {
            let connp_ptr: *mut Self = self as *mut Self;
            if method.is_empty() {
                // empty whitespace line
                let rc = self
                    .in_tx_mut()
                    .ok_or(HtpStatus::ERROR)?
                    .req_process_body_data(unsafe { &mut *connp_ptr }, Some(&data));
                self.in_buf.clear();
                return rc;
            }
            if HtpMethod::new(method) == HtpMethod::UNKNOWN {
                if self.in_body_data_left <= 0 {
                    // log only once per transaction
                    htp_warn!(
                        self.logger,
                        HtpLogCode::REQUEST_BODY_UNEXPECTED,
                        "Unexpected request body"
                    );
                } else {
                    self.in_body_data_left = 1;
                }
                // Interpret remaining bytes as body data
                let rc = self
                    .in_tx_mut()
                    .ok_or(HtpStatus::ERROR)?
                    .req_process_body_data(unsafe { &mut *connp_ptr }, Some(&data));
                self.in_buf.clear();
                return rc;
            } // else continue
            self.in_body_data_left = -1;
        }

        self.in_buf = take(&mut data);
        if (self.in_curr_data.position() as i64) < self.in_buf.len() as i64 {
            self.in_curr_data.set_position(0);
        } else {
            self.in_curr_data
                .seek(SeekFrom::Current(-(self.in_buf.len() as i64)))?;
        }
        self.state_request_complete()
    }

    /// Consumes whatever is left in the buffer after detecting an http/0.9 session.
    pub fn req_ignore_data_after_http_0_9(&mut self) -> Result<()> {
        let bytes_left = self.in_curr_len() - self.in_curr_data.position() as i64;

        if bytes_left > 0 {
            self.conn.flags.set(ConnectionFlags::HTTP_0_9_EXTRA)
        }
        self.in_curr_data.seek(SeekFrom::End(0))?;
        Err(HtpStatus::DATA)
    }

    /// The idle state is where the parser will end up after a transaction is processed.
    /// If there is more data available, a new request will be started.
    ///
    /// Returns OK on state change, ERROR on error, or HTP_DATA when more data is needed.
    pub fn req_idle(&mut self) -> Result<()> {
        // We want to start parsing the next request (and change
        // the state from IDLE) only if there's at least one
        // byte of data available. Otherwise we could be creating
        // new structures even if there's no more data on the
        // connection.
        if self.in_curr_data.position() as i64 >= self.in_curr_len() {
            return Err(HtpStatus::DATA);
        }
        let tx_id = self.create_tx()?;
        self.set_in_tx_id(Some(tx_id));
        self.in_reset();
        // Change state to TRANSACTION_START
        // Ignore the result.
        let _ = self.state_request_start();
        Ok(())
    }

    /// Buffer incomplete request data and verify that field_limit
    /// constraint is met.
    pub fn handle_in_absent_lf(&mut self, data: &[u8]) -> Result<()> {
        self.in_curr_data.seek(SeekFrom::End(0))?;
        self.check_in_buffer_limit(data.len())?;
        self.in_buf.add(data);
        Err(HtpStatus::DATA_BUFFER)
    }

    /// Run the REQUEST_BODY_DATA hook.
    pub fn req_run_hook_body_data(&mut self, d: &mut Data) -> Result<()> {
        // Do not invoke callbacks with an empty data chunk
        if !d.data().is_null() && d.is_empty() {
            return Ok(());
        }
        // Do not invoke callbacks without a transaction.
        if let Some(in_tx) = self.in_tx() {
            // Run transaction hooks first
            in_tx.hook_request_body_data.run_all(self, d)?;
        }
        // Run configuration hooks second
        self.cfg.hook_request_body_data.run_all(self, d)?;
        // On PUT requests, treat request body as file
        if let Some(file) = &mut self.put_file {
            file.handle_file_data(self.cfg.hook_request_file_data.clone(), d.data(), d.len())?;
        }
        Ok(())
    }

    /// Process a chunk of inbound (client or request) data.
    pub fn req_data(
        &mut self,
        timestamp: Option<DateTime<Utc>>,
        data: *const core::ffi::c_void,
        len: usize,
    ) -> HtpStreamState {
        // Return if the connection is in stop state.
        if self.in_status == HtpStreamState::STOP {
            htp_info!(
                self.logger,
                HtpLogCode::PARSER_STATE_ERROR,
                "Inbound parser is in STOP state"
            );
            return HtpStreamState::STOP;
        }
        // Return if the connection had a fatal error earlier
        if self.in_status == HtpStreamState::ERROR {
            htp_error!(
                self.logger,
                HtpLogCode::PARSER_STATE_ERROR,
                "Inbound parser is in ERROR state"
            );
            return HtpStreamState::ERROR;
        }
        // Sanity check: we must have a transaction pointer if the state is not IDLE (no inbound transaction)
        if self.in_tx().is_none() && self.in_state != State::IDLE {
            self.in_status = HtpStreamState::ERROR;
            htp_error!(
                self.logger,
                HtpLogCode::MISSING_INBOUND_TRANSACTION_DATA,
                "Missing inbound transaction data"
            );
            return HtpStreamState::ERROR;
        }
        // If the length of the supplied data chunk is zero, proceed
        // only if the stream has been closed. We do not allow zero-sized
        // chunks in the API, but we use them internally to force the parsers
        // to finalize parsing.
        if len == 0 && self.in_status != HtpStreamState::CLOSED {
            htp_error!(
                self.logger,
                HtpLogCode::ZERO_LENGTH_DATA_CHUNKS,
                "Zero-length data chunks are not allowed"
            );
            return HtpStreamState::CLOSED;
        }
        // Remember the timestamp of the current request data chunk
        if let Some(timestamp) = timestamp {
            self.in_timestamp = timestamp;
        }

        // Store the current chunk information
        let chunk = unsafe { std::slice::from_raw_parts(data as *mut u8, len) };
        self.in_curr_data = Cursor::new(chunk.to_vec());
        self.in_current_receiver_offset = 0;
        self.in_chunk_count = self.in_chunk_count.wrapping_add(1);
        self.conn.track_inbound_data(len);
        // Return without processing any data if the stream is in tunneling
        // mode (which it would be after an initial CONNECT transaction).
        if self.in_status == HtpStreamState::TUNNEL {
            return HtpStreamState::TUNNEL;
        }
        if self.out_status == HtpStreamState::DATA_OTHER {
            self.out_status = HtpStreamState::DATA
        }
        loop
        // Invoke a processor, in a loop, until an error
        // occurs or until we run out of data. Many processors
        // will process a request, each pointing to the next
        // processor that needs to run.
        // Return if there's been an error or if we've run out of data. We are relying
        // on processors to supply error messages, so we'll keep quiet here.
        {
            let mut rc;

            //handle gap
            if data.is_null() && len > 0 {
                match self.in_state {
                    State::BODY_IDENTITY | State::IGNORE_DATA_AFTER_HTTP_0_9 => {
                        rc = self.handle_in_state(chunk)
                    }
                    State::FINALIZE => rc = self.state_request_complete(),
                    _ => {
                        // go to req_connect_probe_data ?
                        htp_error!(
                            self.logger,
                            HtpLogCode::INVALID_GAP,
                            "Gaps are not allowed during this state"
                        );
                        return HtpStreamState::CLOSED;
                    }
                }
            } else {
                rc = self.handle_in_state(chunk);
            }

            if rc.is_ok() {
                if self.in_status == HtpStreamState::TUNNEL {
                    return HtpStreamState::TUNNEL;
                }
                rc = self.req_handle_state_change()
            }
            match rc {
                // Continue looping.
                Ok(_) => {}
                // Do we need more data?
                Err(HtpStatus::DATA) | Err(HtpStatus::DATA_BUFFER) => {
                    // Ignore result.
                    let _ = self.req_receiver_send_data(false);
                    self.in_status = HtpStreamState::DATA;
                    return HtpStreamState::DATA;
                }
                // Check for suspended parsing.
                Err(HtpStatus::DATA_OTHER) => {
                    // We might have actually consumed the entire data chunk?
                    if (self.in_curr_data.position() as i64) >= self.in_curr_len() {
                        // Do not send STREAM_DATE_DATA_OTHER if we've consumed the entire chunk.
                        self.in_status = HtpStreamState::DATA;
                        return HtpStreamState::DATA;
                    } else {
                        // Partial chunk consumption.
                        self.in_status = HtpStreamState::DATA_OTHER;
                        return HtpStreamState::DATA_OTHER;
                    }
                }
                // Check for the stop signal.
                Err(HtpStatus::STOP) => {
                    self.in_status = HtpStreamState::STOP;
                    return HtpStreamState::STOP;
                }
                // Permanent stream error.
                Err(_) => {
                    self.in_status = HtpStreamState::ERROR;
                    return HtpStreamState::ERROR;
                }
            }
        }
    }

    /// Return length of the current request chunk.
    pub fn in_curr_len(&self) -> i64 {
        self.in_curr_data.get_ref().len() as i64
    }
}

#[test]
fn Method() {
    let method = b"GET";
    assert_eq!(HtpMethod::GET, HtpMethod::new(method));
}
