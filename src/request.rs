use crate::connection_parser::State;
use crate::error::Result;
use crate::hook::DataHook;
use crate::util::{
    nom_take_is_space, take_is_space, take_not_is_space, take_till_lf, take_till_lf_null, Flags,
};
use crate::{bstr, connection_parser, transaction, util, Status};
use nom::{
    branch::alt, bytes::complete::take_until, character::complete::char,
    character::is_space as nom_is_space, error::ErrorKind, sequence::tuple,
};
use std::io::{Cursor, Seek, SeekFrom};

/// HTTP methods.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum htp_method_t {
    /// Used by default, until the method is determined (e.g., before
    /// the request line is processed.
    HTP_M_UNKNOWN,
    HTP_M_HEAD,
    HTP_M_GET,
    HTP_M_PUT,
    HTP_M_POST,
    HTP_M_DELETE,
    HTP_M_CONNECT,
    HTP_M_OPTIONS,
    HTP_M_TRACE,
    HTP_M_PATCH,
    HTP_M_PROPFIND,
    HTP_M_PROPPATCH,
    HTP_M_MKCOL,
    HTP_M_COPY,
    HTP_M_MOVE,
    HTP_M_LOCK,
    HTP_M_UNLOCK,
    HTP_M_VERSION_CONTROL,
    HTP_M_CHECKOUT,
    HTP_M_UNCHECKOUT,
    HTP_M_CHECKIN,
    HTP_M_UPDATE,
    HTP_M_LABEL,
    HTP_M_REPORT,
    HTP_M_MKWORKSPACE,
    HTP_M_MKACTIVITY,
    HTP_M_BASELINE_CONTROL,
    HTP_M_MERGE,
    HTP_M_INVALID,
    HTP_M_ERROR,
}

pub type htp_time_t = libc::timeval;

impl connection_parser::ConnectionParser {
    /// Sends outstanding connection data to the currently active data receiver hook.
    ///
    /// Returns HTP_OK, or a value returned from a callback.
    fn req_receiver_send_data(&mut self, is_last: bool) -> Result<()> {
        let mut data = transaction::Data::new(
            self.in_tx_mut_ptr(),
            Some(
                &self.in_curr_data.get_ref()[self.in_current_receiver_offset as usize
                    ..self.in_curr_data.position() as usize],
            ),
            is_last,
        );
        if let Some(hook) = &self.in_data_receiver_hook {
            hook.run_all(&mut data)?;
        } else {
            return Ok(());
        };
        self.in_current_receiver_offset = self.in_curr_data.position();
        Ok(())
    }

    /// Configures the data receiver hook. If there is a previous hook, it will be finalized and cleared.
    ///
    /// Returns HTP_OK, or a value returned from a callback.
    fn req_receiver_set(&mut self, data_receiver_hook: Option<DataHook>) -> Result<()> {
        // Ignore result.
        let _ = self.req_receiver_finalize_clear();
        self.in_data_receiver_hook = data_receiver_hook;
        self.in_current_receiver_offset = self.in_curr_data.position();
        Ok(())
    }

    /// Finalizes an existing data receiver hook by sending any outstanding data to it. The
    /// hook is then removed so that it receives no more data.
    ///
    /// Returns HTP_OK, or a value returned from a callback.
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
    ///
    /// Returns HTP_OK, or a value returned from a callback.
    fn req_handle_state_change(&mut self) -> Result<()> {
        if self.in_state_previous == self.in_state {
            return Ok(());
        }
        if self.in_state == State::HEADERS {
            unsafe {
                let header_fn = Some((*self.in_tx_mut_ok()?.cfg).hook_request_header_data.clone());
                let trailer_fn = Some(
                    (*self.in_tx_mut_ok()?.cfg)
                        .hook_request_trailer_data
                        .clone(),
                );

                match self.in_tx_mut_ok()?.request_progress {
                    transaction::htp_tx_req_progress_t::HTP_REQUEST_HEADERS => {
                        self.req_receiver_set(header_fn)
                    }
                    transaction::htp_tx_req_progress_t::HTP_REQUEST_TRAILER => {
                        self.req_receiver_set(trailer_fn)
                    }
                    _ => Ok(()),
                }?;
            }
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
    /// by htp_config_t::field_limit.
    ///
    /// Returns HTP_OK, or HTP_ERROR on fatal failure.
    fn check_buffer_limit(&mut self, len: usize) -> Result<()> {
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
        unsafe {
            if newlen > (*self.in_tx_mut_ok()?.cfg).field_limit {
                htp_error!(
                    self as *mut connection_parser::ConnectionParser,
                    htp_log_code::REQUEST_FIELD_TOO_LONG,
                    format!(
                        "Request buffer over the limit: size {} limit {}.",
                        newlen,
                        (*self.in_tx_mut_ok()?.cfg).field_limit
                    )
                );
                return Err(Status::ERROR);
            }
        }
        Ok(())
    }

    /// Performs a check for a CONNECT transaction to decide whether inbound
    /// parsing needs to be suspended.
    ///
    /// Returns HTP_OK if the request does not use CONNECT, HTP_DATA_OTHER if
    ///          inbound parsing needs to be suspended until we hear from the
    ///          other side
    pub fn REQ_CONNECT_CHECK(&mut self) -> Result<()> {
        // If the request uses the CONNECT method, then there will
        // not be a request body, but first we need to wait to see the
        // response in order to determine if the tunneling request
        // was a success.
        if self.in_tx_mut_ok()?.request_method_number == htp_method_t::HTP_M_CONNECT {
            self.in_state = State::CONNECT_WAIT_RESPONSE;
            self.in_status = connection_parser::htp_stream_state_t::HTP_STREAM_DATA_OTHER;
            return Err(Status::DATA_OTHER);
        }
        // Continue to the next step to determine
        // the presence of request body
        self.in_state = State::BODY_DETERMINE;
        Ok(())
    }

    /// Determines whether inbound parsing needs to continue or stop. In
    /// case the data appears to be plain text HTTP, we try to continue.
    ///
    /// Returns HTP_OK if the parser can resume parsing, HTP_DATA_BUFFER if
    ///         we need more data.
    pub fn REQ_CONNECT_PROBE_DATA(&mut self, line: &[u8]) -> Result<()> {
        let data = if let Ok((_, data)) = take_till_lf_null(line) {
            data
        } else {
            return self.handle_absent_lf(line);
        };

        if !self.in_buf.is_empty() {
            self.check_buffer_limit(data.len())?;
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
            let method_type = util::convert_to_method(method);
            if method_type == htp_method_t::HTP_M_UNKNOWN {
                self.in_status = connection_parser::htp_stream_state_t::HTP_STREAM_TUNNEL;
                self.out_status = connection_parser::htp_stream_state_t::HTP_STREAM_TUNNEL
            } else {
                return self.state_request_complete().into();
            }
        };
        Ok(())
    }

    /// Determines whether inbound parsing, which was suspended after
    /// encountering a CONNECT transaction, can proceed (after receiving
    /// the response).
    ///
    /// Returns HTP_OK if the parser can resume parsing, HTP_DATA_OTHER if
    ///         it needs to continue waiting.
    pub fn REQ_CONNECT_WAIT_RESPONSE(&mut self) -> Result<()> {
        // Check that we saw the response line of the current inbound transaction.
        if self.in_tx_mut_ok()?.response_progress
            <= transaction::htp_tx_res_progress_t::HTP_RESPONSE_LINE
        {
            return Err(Status::DATA_OTHER);
        }
        // A 2xx response means a tunnel was established. Anything
        // else means we continue to follow the HTTP stream.
        if self.in_tx_mut_ok()?.response_status_number >= 200
            && self.in_tx_mut_ok()?.response_status_number <= 299
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
    /// Returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
    pub fn REQ_BODY_CHUNKED_DATA_END(&mut self, data: &[u8]) -> Result<()> {
        // TODO We shouldn't really see anything apart from CR and LF,
        //      so we should warn about anything else.
        if let Ok((_, parsed)) = take_till_lf(data) {
            let len = parsed.len() as i64;
            self.in_curr_data.seek(SeekFrom::Current(len))?;
            self.in_tx_mut_ok()?.request_message_len += len;
            self.in_state = State::BODY_CHUNKED_LENGTH;
            return Ok(());
        } else {
            self.in_tx_mut_ok()?.request_message_len += data.len() as i64;
            self.handle_absent_lf(data)
        }
    }

    /// Processes a chunk of data.
    ///
    /// Returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
    pub fn REQ_BODY_CHUNKED_DATA(&mut self, data: &[u8]) -> Result<()> {
        // Determine how many bytes we can consume.
        let bytes_to_consume: usize = std::cmp::min(data.len(), self.in_chunked_length as usize);
        // If the input buffer is empty, ask for more data.
        if bytes_to_consume == 0 {
            return Err(Status::DATA);
        }
        // Consume the data.
        self.req_process_body_data_ex(&data[0..bytes_to_consume])?;
        // Adjust counters.
        self.in_curr_data
            .seek(SeekFrom::Current(bytes_to_consume as i64))?;
        self.in_tx_mut_ok()?.request_message_len = (self.in_tx_mut_ok()?.request_message_len as u64)
            .wrapping_add(bytes_to_consume as u64)
            as i64;
        self.in_chunked_length =
            (self.in_chunked_length as u64).wrapping_sub(bytes_to_consume as u64) as i64;
        if self.in_chunked_length == 0 {
            // End of the chunk.
            self.in_state = State::BODY_CHUNKED_DATA_END;
            return Ok(());
        }
        // Ask for more data.
        Err(Status::DATA)
    }

    /// Extracts chunk length.
    ///
    /// Returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
    pub fn REQ_BODY_CHUNKED_LENGTH(&mut self, data: &[u8]) -> Result<()> {
        if let Ok((_, line)) = take_till_lf(data) {
            self.in_curr_data
                .seek(SeekFrom::Current(line.len() as i64))?;
            if !self.in_buf.is_empty() {
                self.check_buffer_limit(line.len())?;
            }
            let mut data = std::mem::take(&mut self.in_buf);
            data.add(line);

            self.in_tx_mut_ok()?.request_message_len =
                (self.in_tx_mut_ok()?.request_message_len as u64).wrapping_add(data.len() as u64)
                    as i64;
            if let Ok(Some(chunked_len)) = util::parse_chunked_length(&data) {
                self.in_chunked_length = chunked_len as i64;
            } else {
                self.in_chunked_length = -1;
            }

            // Handle chunk length.
            if self.in_chunked_length > 0 {
                // More data available.
                self.in_state = State::BODY_CHUNKED_DATA
            } else if self.in_chunked_length == 0 {
                // End of data.
                self.in_state = State::HEADERS;
                self.in_tx_mut_ok()?.request_progress =
                    transaction::htp_tx_req_progress_t::HTP_REQUEST_TRAILER
            } else {
                // Invalid chunk length.
                unsafe {
                    htp_error!(
                        self as *mut connection_parser::ConnectionParser,
                        htp_log_code::INVALID_REQUEST_CHUNK_LEN,
                        "Request chunk encoding: Invalid chunk length"
                    );
                }
                return Err(Status::ERROR);
            }
            Ok(())
        } else {
            return self.handle_absent_lf(data);
        }
    }

    /// Processes identity request body.
    ///
    /// Returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
    pub fn REQ_BODY_IDENTITY(&mut self, data: &[u8]) -> Result<()> {
        // Determine how many bytes we can consume.
        let bytes_to_consume: usize = std::cmp::min(data.len(), self.in_body_data_left as usize);
        // If the input buffer is empty, ask for more data.
        if bytes_to_consume == 0 {
            return Err(Status::DATA);
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
        Err(Status::DATA)
    }

    /// Determines presence (and encoding) of a request body.
    ///
    /// Returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
    pub fn REQ_BODY_DETERMINE(&mut self) -> Result<()> {
        // Determine the next state based on the presence of the request
        // body, and the coding used.
        match self.in_tx_mut_ok()?.request_transfer_coding as u32 {
            3 => {
                self.in_state = State::BODY_CHUNKED_LENGTH;
                self.in_tx_mut_ok()?.request_progress =
                    transaction::htp_tx_req_progress_t::HTP_REQUEST_BODY
            }
            2 => {
                self.in_content_length = self.in_tx_mut_ok()?.request_content_length;
                self.in_body_data_left = self.in_content_length;
                if self.in_content_length != 0 {
                    self.in_state = State::BODY_IDENTITY;
                    self.in_tx_mut_ok()?.request_progress =
                        transaction::htp_tx_req_progress_t::HTP_REQUEST_BODY
                } else {
                    unsafe { (*self.in_tx_mut_ok()?.connp).in_state = State::FINALIZE }
                }
            }
            1 => {
                // This request does not have a body, which
                // means that we're done with it
                self.in_state = State::FINALIZE
            }
            _ => {
                // Should not be here
                return Err(Status::ERROR);
            }
        }
        Ok(())
    }

    /// Parses request headers.
    ///
    /// Returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
    pub fn REQ_HEADERS(&mut self, data: &[u8]) -> Result<()> {
        let mut rest = data;
        loop {
            if self.in_status == connection_parser::htp_stream_state_t::HTP_STREAM_CLOSED {
                // Parse previous header, if any.
                if let Some(in_header) = self.in_header.take() {
                    unsafe {
                        self.process_request_header(in_header.as_slice())?;
                    }
                }
                self.in_buf.clear();
                self.in_tx_mut_ok()?.request_progress =
                    transaction::htp_tx_req_progress_t::HTP_REQUEST_TRAILER;
                // We've seen all the request headers.
                unsafe { return self.state_request_headers().into() };
            }

            if let Ok((remaining, line)) = take_till_lf(rest) {
                self.in_curr_data
                    .seek(SeekFrom::Current(line.len() as i64))?;
                if !self.in_buf.is_empty() {
                    self.check_buffer_limit(line.len())?;
                }
                let mut data = std::mem::take(&mut self.in_buf);
                data.add(line);

                rest = remaining;
                unsafe {
                    if util::is_line_terminator((*self.cfg).server_personality, &data, false) {
                        // Parse previous header, if any.
                        if let Some(in_header) = self.in_header.take() {
                            self.process_request_header(in_header.as_slice())?;
                        }
                        // We've seen all the request headers.
                        return self.state_request_headers().into();
                    }
                }

                let chomped = util::chomp(&data);
                if !util::is_line_folded(chomped) {
                    // New header line.
                    // Parse previous header, if any.
                    if let Some(in_header) = self.in_header.take() {
                        unsafe {
                            self.process_request_header(in_header.as_slice())?;
                        }
                    }

                    if let Some(byte) = remaining.get(0) {
                        if !util::is_folding_char(*byte) {
                            // Because we know this header is not folded, we can process the buffer straight away.
                            unsafe {
                                self.process_request_header(chomped)?;
                            }
                        } else {
                            self.in_header = Some(bstr::Bstr::from(chomped));
                        }
                    } else {
                        // Keep the partial header data for parsing later.
                        self.in_header = Some(bstr::Bstr::from(chomped));
                    }
                } else if self.in_header.is_none() {
                    // Folding; check that there's a previous header line to add to.
                    // Invalid folding.
                    // Warn only once per transaction.
                    if !self
                        .in_tx_mut_ok()?
                        .flags
                        .contains(Flags::HTP_INVALID_FOLDING)
                    {
                        self.in_tx_mut_ok()?.flags |= Flags::HTP_INVALID_FOLDING;
                        unsafe {
                            htp_warn!(
                                self as *mut connection_parser::ConnectionParser,
                                htp_log_code::INVALID_REQUEST_FIELD_FOLDING,
                                "Invalid request field folding"
                            );
                        }
                    }
                    // Keep the header data for parsing later.
                    self.in_header = Some(bstr::Bstr::from(chomped));
                } else if let Some(header) = &mut self.in_header {
                    // Add to the existing header.
                    header.add(&chomped);
                }
            } else {
                self.handle_absent_lf(rest)?;
            }
        }
    }

    /// Determines request protocol.
    ///
    /// Returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
    pub fn REQ_PROTOCOL(&mut self, data: &[u8]) -> Result<()> {
        // Is this a short-style HTTP/0.9 request? If it is,
        // we will not want to parse request headers.
        if !self.in_tx_mut_ok()?.is_protocol_0_9 {
            // Switch to request header parsing.
            self.in_state = State::HEADERS;
            self.in_tx_mut_ok()?.request_progress =
                transaction::htp_tx_req_progress_t::HTP_REQUEST_HEADERS;
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
                            } else if afterspace || util::is_space(*c) {
                                break;
                            }
                        }
                    }
                    unsafe {
                        htp_warn!(
                            self as *mut connection_parser::ConnectionParser,
                            htp_log_code::REQUEST_LINE_NO_PROTOCOL,
                            "Request line: missing protocol"
                        );
                    }

                    self.in_tx_mut_ok()?.is_protocol_0_9 = false;
                    // Switch to request header parsing.
                    self.in_state = State::HEADERS;
                    self.in_tx_mut_ok()?.request_progress =
                        transaction::htp_tx_req_progress_t::HTP_REQUEST_HEADERS;
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
    /// Returns HTP_OK on succesful parse, HTP_ERROR on error.
    pub fn REQ_LINE_complete(&mut self, line: &[u8]) -> Result<()> {
        if !self.in_buf.is_empty() {
            self.check_buffer_limit(line.len())?;
        }
        let mut data = std::mem::take(&mut self.in_buf);
        data.add(line);
        if data.len() == 0 {
            return Err(Status::DATA);
        }
        // Is this a line that should be ignored?
        let ignore = util::is_line_ignorable(unsafe { (*self.cfg).server_personality }, &data);
        if ignore {
            // We have an empty/whitespace line, which we'll note, ignore and move on.
            self.in_tx_mut_ok()?.request_ignored_lines =
                self.in_tx_mut_ok()?.request_ignored_lines.wrapping_add(1);
            return Ok(());
        }
        // Process request line.
        let data = util::chomp(&data);
        self.in_tx_mut_ok()?.request_line = Some(bstr::Bstr::from(data));
        unsafe {
            self.parse_request_line(data)?;
            // Finalize request line parsing.
            self.state_request_line()?;
        }
        Ok(())
    }

    /// Parses request line.
    ///
    /// Returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
    pub fn REQ_LINE(&mut self, data: &[u8]) -> Result<()> {
        match take_till_lf(data) {
            Ok((_, read)) => {
                self.in_curr_data
                    .seek(SeekFrom::Current(read.len() as i64))?;
                self.REQ_LINE_complete(read)
            }
            _ => {
                if self.in_status == connection_parser::htp_stream_state_t::HTP_STREAM_CLOSED {
                    self.in_curr_data.seek(SeekFrom::End(0))?;
                    self.REQ_LINE_complete(data)
                } else {
                    return self.handle_absent_lf(data);
                }
            }
        }
    }

    pub fn REQ_FINALIZE(&mut self, data: &[u8]) -> Result<()> {
        let mut work = data;
        if self.in_status != connection_parser::htp_stream_state_t::HTP_STREAM_CLOSED {
            let in_next_byte = self
                .in_curr_data
                .get_ref()
                .get(self.in_curr_data.position() as usize);
            if in_next_byte.is_none() {
                return self.state_request_complete().into();
            }
            let lf = in_next_byte
                .map(|byte| *byte == '\n' as u8)
                .unwrap_or(false);
            if !lf {
                if let Ok((_, line)) = take_till_lf(data) {
                    self.in_curr_data
                        .seek(SeekFrom::Current(line.len() as i64))?;
                    work = line;
                } else {
                    return self.handle_absent_lf(data);
                }
            }
        }

        if !self.in_buf.is_empty() {
            self.check_buffer_limit(work.len())?;
        }
        self.in_buf.add(work);
        let mut data = std::mem::take(&mut self.in_buf);

        if data.is_empty() {
            return self.state_request_complete().into();
        }

        let res = tuple::<_, _, (&[u8], ErrorKind), _>((take_is_space, take_not_is_space))(&data);

        if let Ok((_, (_, method))) = res {
            if method.is_empty() {
                // empty whitespace line
                let rc = self
                    .in_tx_mut()
                    .ok_or(Status::ERROR)?
                    .req_process_body_data_ex(Some(&data));
                self.in_buf.clear();
                return rc;
            }

            let method_type = util::convert_to_method(method);
            if method_type == htp_method_t::HTP_M_UNKNOWN {
                if self.in_body_data_left <= 0 {
                    // log only once per transaction
                    unsafe {
                        htp_warn!(
                            self as *mut connection_parser::ConnectionParser,
                            htp_log_code::REQUEST_BODY_UNEXPECTED,
                            "Unexpected request body"
                        );
                    }
                } else {
                    self.in_body_data_left = 1;
                }
                // Interpret remaining bytes as body data
                let rc = self
                    .in_tx_mut()
                    .ok_or(Status::ERROR)?
                    .req_process_body_data_ex(Some(&data));
                self.in_buf.clear();
                return rc;
            } // else continue
            self.in_body_data_left = -1;
        }

        self.in_buf = std::mem::take(&mut data);
        if (self.in_curr_data.position() as i64) < self.in_buf.len() as i64 {
            self.in_curr_data.set_position(0);
        } else {
            self.in_curr_data
                .seek(SeekFrom::Current((self.in_buf.len() as i64) * -1))?;
        }
        return self.state_request_complete().into();
    }

    pub fn REQ_IGNORE_DATA_AFTER_HTTP_0_9(&mut self) -> Result<()> {
        // Consume whatever is left in the buffer.
        let bytes_left = self.in_curr_len() - self.in_curr_data.position() as i64;

        if bytes_left > 0 {
            self.conn.flags |= util::ConnectionFlags::HTP_CONN_HTTP_0_9_EXTRA
        }
        self.in_curr_data.seek(SeekFrom::End(0))?;
        Err(Status::DATA)
    }

    /// The idle state is where the parser will end up after a transaction is processed.
    /// If there is more data available, a new request will be started.
    ///
    /// Returns HTP_OK on state change, HTP_ERROR on error, or HTP_DATA when more data is needed.
    pub fn REQ_IDLE(&mut self) -> Result<()> {
        // We want to start parsing the next request (and change
        // the state from IDLE) only if there's at least one
        // byte of data available. Otherwise we could be creating
        // new structures even if there's no more data on the
        // connection.
        if self.in_curr_data.position() as i64 >= self.in_curr_len() {
            return Err(Status::DATA);
        }

        if let Ok(tx_id) = self.create_tx() {
            self.set_in_tx_id(Some(tx_id))
        } else {
            return Err(Status::ERROR);
        }

        // Change state to TRANSACTION_START
        // Ignore the result.
        unsafe {
            let _ = self.state_request_start();
        }
        Ok(())
    }

    pub fn handle_absent_lf(&mut self, data: &[u8]) -> Result<()> {
        self.in_curr_data.seek(SeekFrom::End(0))?;
        self.check_buffer_limit(data.len())?;
        self.in_buf.add(data);
        return Err(Status::DATA_BUFFER);
    }
    /// Run the REQUEST_BODY_DATA hook.
    pub unsafe fn req_run_hook_body_data(&mut self, d: *mut transaction::Data) -> Result<()> {
        // Do not invoke callbacks with an empty data chunk
        if !(*d).data().is_null() && (*d).len() == 0 {
            return Ok(());
        }
        // Do not invoke callbacks without a transaction.
        if let Some(in_tx) = self.in_tx() {
            // Run transaction hooks first
            in_tx.hook_request_body_data.run_all(d)?;
        }
        // Run configuration hooks second
        (*self.cfg).hook_request_body_data.run_all(d)?;
        // On PUT requests, treat request body as file
        if let Some(file) = &mut self.put_file {
            file.handle_file_data(self.cfg, (*d).data(), (*d).len())?;
        }
        Ok(())
    }

    /// Returns HTP_STREAM_DATA, HTP_STREAM_ERROR or STEAM_STATE_DATA_OTHER (see QUICK_START).
    ///         HTP_STREAM_CLOSED and HTP_STREAM_TUNNEL are also possible.
    pub unsafe fn req_data(
        &mut self,
        timestamp: Option<htp_time_t>,
        data: *const core::ffi::c_void,
        len: usize,
    ) -> connection_parser::htp_stream_state_t {
        // Return if the connection is in stop state.
        if self.in_status == connection_parser::htp_stream_state_t::HTP_STREAM_STOP {
            htp_info!(
                self as *mut connection_parser::ConnectionParser,
                htp_log_code::PARSER_STATE_ERROR,
                "Inbound parser is in HTP_STREAM_STOP"
            );
            return connection_parser::htp_stream_state_t::HTP_STREAM_STOP;
        }
        // Return if the connection had a fatal error earlier
        if self.in_status == connection_parser::htp_stream_state_t::HTP_STREAM_ERROR {
            htp_error!(
                self as *mut connection_parser::ConnectionParser,
                htp_log_code::PARSER_STATE_ERROR,
                "Inbound parser is in HTP_STREAM_ERROR"
            );
            return connection_parser::htp_stream_state_t::HTP_STREAM_ERROR;
        }
        // Sanity check: we must have a transaction pointer if the state is not IDLE (no inbound transaction)
        if self.in_tx().is_none() && self.in_state != State::IDLE {
            self.in_status = connection_parser::htp_stream_state_t::HTP_STREAM_ERROR;
            htp_error!(
                self as *mut connection_parser::ConnectionParser,
                htp_log_code::MISSING_INBOUND_TRANSACTION_DATA,
                "Missing inbound transaction data"
            );
            return connection_parser::htp_stream_state_t::HTP_STREAM_ERROR;
        }
        // If the length of the supplied data chunk is zero, proceed
        // only if the stream has been closed. We do not allow zero-sized
        // chunks in the API, but we use them internally to force the parsers
        // to finalize parsing.
        if len == 0 && self.in_status != connection_parser::htp_stream_state_t::HTP_STREAM_CLOSED {
            htp_error!(
                self as *mut connection_parser::ConnectionParser,
                htp_log_code::ZERO_LENGTH_DATA_CHUNKS,
                "Zero-length data chunks are not allowed"
            );
            return connection_parser::htp_stream_state_t::HTP_STREAM_CLOSED;
        }
        // Remember the timestamp of the current request data chunk
        if let Some(timestamp) = timestamp {
            self.in_timestamp = timestamp;
        }

        // Store the current chunk information
        let chunk = std::slice::from_raw_parts(data as *mut u8, len);
        self.in_curr_data = Cursor::new(chunk.to_vec());
        self.in_current_receiver_offset = 0;
        self.in_chunk_count = self.in_chunk_count.wrapping_add(1);
        self.conn.track_inbound_data(len);
        // Return without processing any data if the stream is in tunneling
        // mode (which it would be after an initial CONNECT transaction).
        if self.in_status == connection_parser::htp_stream_state_t::HTP_STREAM_TUNNEL {
            return connection_parser::htp_stream_state_t::HTP_STREAM_TUNNEL;
        }
        if self.out_status == connection_parser::htp_stream_state_t::HTP_STREAM_DATA_OTHER {
            self.out_status = connection_parser::htp_stream_state_t::HTP_STREAM_DATA
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
                    State::FINALIZE => rc = self.state_request_complete().into(),
                    _ => {
                        // go to htp_connp_REQ_CONNECT_PROBE_DATA ?
                        htp_error!(
                            self as *mut connection_parser::ConnectionParser,
                            htp_log_code::INVALID_GAP,
                            "Gaps are not allowed during this state"
                        );
                        return connection_parser::htp_stream_state_t::HTP_STREAM_CLOSED;
                    }
                }
            } else {
                rc = self.handle_in_state(chunk);
            }

            if rc.is_ok() {
                if self.in_status == connection_parser::htp_stream_state_t::HTP_STREAM_TUNNEL {
                    return connection_parser::htp_stream_state_t::HTP_STREAM_TUNNEL;
                }
                rc = self.req_handle_state_change()
            }
            match rc {
                // Continue looping.
                Ok(_) => {}
                // Do we need more data?
                Err(Status::DATA) | Err(Status::DATA_BUFFER) => {
                    // Ignore result.
                    let _ = self.req_receiver_send_data(false);
                    self.in_status = connection_parser::htp_stream_state_t::HTP_STREAM_DATA;
                    return connection_parser::htp_stream_state_t::HTP_STREAM_DATA;
                }
                // Check for suspended parsing.
                Err(Status::DATA_OTHER) => {
                    // We might have actually consumed the entire data chunk?
                    if (self.in_curr_data.position() as i64) >= self.in_curr_len() {
                        // Do not send STREAM_DATE_DATA_OTHER if we've consumed the entire chunk.
                        self.in_status = connection_parser::htp_stream_state_t::HTP_STREAM_DATA;
                        return connection_parser::htp_stream_state_t::HTP_STREAM_DATA;
                    } else {
                        // Partial chunk consumption.
                        self.in_status =
                            connection_parser::htp_stream_state_t::HTP_STREAM_DATA_OTHER;
                        return connection_parser::htp_stream_state_t::HTP_STREAM_DATA_OTHER;
                    }
                }
                // Check for the stop signal.
                Err(Status::STOP) => {
                    self.in_status = connection_parser::htp_stream_state_t::HTP_STREAM_STOP;
                    return connection_parser::htp_stream_state_t::HTP_STREAM_STOP;
                }
                // Permanent stream error.
                Err(_) => {
                    self.in_status = connection_parser::htp_stream_state_t::HTP_STREAM_ERROR;
                    return connection_parser::htp_stream_state_t::HTP_STREAM_ERROR;
                }
            }
        }
    }

    pub fn in_curr_len(&self) -> i64 {
        self.in_curr_data.get_ref().len() as i64
    }
}
