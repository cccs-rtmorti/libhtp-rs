use crate::{
    bstr::Bstr,
    connection::Flags as ConnectionFlags,
    connection_parser::{ConnectionParser, HtpStreamState, ParserData, State},
    decompressors::{Decompressor, HtpContentEncoding},
    error::{NomError, Result},
    hook::DataHook,
    parsers::parse_chunked_length,
    transaction::{Data, HtpRequestProgress, HtpResponseProgress, HtpTransferCoding, Transaction},
    util::{
        chomp, is_line_ignorable, is_space, is_valid_chunked_length_data, nom_take_is_space,
        take_is_space, take_not_is_space, take_till_lf, take_till_lf_null, FlagOperations,
        HtpFlags,
    },
    HtpStatus,
};
use chrono::{DateTime, Utc};
use nom::{
    branch::alt, bytes::complete::take_until, character::complete::char,
    character::is_space as nom_is_space, sequence::tuple,
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
    fn request_receiver_send_data(&mut self, is_last: bool) -> Result<()> {
        let tx = self.request_mut() as *mut Transaction;
        if let Some(hook) = &self.request_data_receiver_hook {
            hook.run_all(
                self,
                &mut Data::new(
                    tx,
                    &ParserData::from(
                        &self.request_curr_data.get_ref()[self.request_current_receiver_offset
                            as usize
                            ..self.request_curr_data.position() as usize],
                    ),
                    is_last,
                ),
            )?;
        } else {
            return Ok(());
        };
        self.request_current_receiver_offset = self.request_curr_data.position();
        Ok(())
    }

    /// Configures the data receiver hook. If there is a previous hook, it will be finalized and cleared.
    fn request_receiver_set(&mut self, data_receiver_hook: Option<DataHook>) -> Result<()> {
        // Ignore result.
        let _ = self.request_receiver_finalize_clear();
        self.request_data_receiver_hook = data_receiver_hook;
        self.request_current_receiver_offset = self.request_curr_data.position();
        Ok(())
    }

    /// Finalizes an existing data receiver hook by sending any outstanding data to it. The
    /// hook is then removed so that it receives no more data.
    pub fn request_receiver_finalize_clear(&mut self) -> Result<()> {
        if self.request_data_receiver_hook.is_none() {
            return Ok(());
        }
        let rc = self.request_receiver_send_data(true);
        self.request_data_receiver_hook = None;
        rc
    }

    /// Handles request parser state changes. At the moment, this function is used only
    /// to configure data receivers, which are sent raw connection data.
    fn request_handle_state_change(&mut self) -> Result<()> {
        if self.request_state_previous == self.request_state {
            return Ok(());
        }
        if self.request_state == State::HEADERS {
            let header_fn = Some(self.request().cfg.hook_request_header_data.clone());
            let trailer_fn = Some(self.request().cfg.hook_request_trailer_data.clone());

            match self.request().request_progress {
                HtpRequestProgress::HEADERS => self.request_receiver_set(header_fn),
                HtpRequestProgress::TRAILER => self.request_receiver_set(trailer_fn),
                _ => Ok(()),
            }?;
        }
        // Initially, I had the finalization of raw data sending here, but that
        // caused the last REQUEST_HEADER_DATA hook to be invoked after the
        // REQUEST_HEADERS hook -- which I thought made no sense. For that reason,
        // the finalization is now initiated from the request header processing code,
        // which is less elegant but provides a better user experience. Having some
        // (or all) hooks to be invoked on state change might work better.
        self.request_state_previous = self.request_state;
        Ok(())
    }

    /// If there is any data left in the inbound data chunk, this function will preserve
    /// it for later consumption. The maximum amount accepted for buffering is controlled
    /// by Config::field_limit.
    fn check_request_buffer_limit(&mut self, len: usize) -> Result<()> {
        if len == 0 {
            return Ok(());
        }
        // Check the hard (buffering) limit.
        let mut newlen: usize = self.request_buf.len().wrapping_add(len);
        // When calculating the size of the buffer, take into account the
        // space we're using for the request header buffer.
        if let Some(header) = &self.request_header {
            newlen = newlen.wrapping_add(header.len())
        }
        let field_limit = self.cfg.field_limit;
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
    pub fn request_connect_check(&mut self) -> Result<()> {
        // If the request uses the CONNECT method, then there will
        // not be a request body, but first we need to wait to see the
        // response in order to determine if the tunneling request
        // was a success.
        if self.request().request_method_number == HtpMethod::CONNECT {
            self.request_state = State::CONNECT_WAIT_RESPONSE;
            self.request_status = HtpStreamState::DATA_OTHER;
            return Err(HtpStatus::DATA_OTHER);
        }
        // Continue to the next step to determine
        // the presence of request body
        self.request_state = State::BODY_DETERMINE;
        Ok(())
    }

    /// Determines whether inbound parsing needs to continue or stop. In
    /// case the data appears to be plain text HTTP, we try to continue.
    ///
    /// Returns OK if the parser can resume parsing, HtpStatus::DATA_BUFFER if
    /// we need more data.
    pub fn request_connect_probe_data(&mut self, line: &[u8]) -> Result<()> {
        let data = if let Ok((_, data)) = take_till_lf_null(line) {
            data
        } else {
            return self.handle_request_absent_lf(line);
        };

        if !self.request_buf.is_empty() {
            self.check_request_buffer_limit(data.len())?;
        }
        // copy, will still need buffer data for next state.
        let mut buffered = self.request_buf.clone();
        buffered.add(data);

        // The request method starts at the beginning of the
        // line and ends with the first whitespace character.
        // We skip leading whitespace as IIS allows this.
        let res = tuple((take_is_space, take_not_is_space))(buffered.as_slice());
        if let Ok((_, (_, method))) = res {
            if HtpMethod::new(method) == HtpMethod::UNKNOWN {
                self.request_status = HtpStreamState::TUNNEL;
                self.response_status = HtpStreamState::TUNNEL
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
    pub fn request_connect_wait_response(&mut self) -> Result<()> {
        // Check that we saw the response line of the current inbound transaction.
        if self.request().response_progress <= HtpResponseProgress::LINE {
            return Err(HtpStatus::DATA_OTHER);
        }
        // A 2xx response means a tunnel was established. Anything
        // else means we continue to follow the HTTP stream.
        if self.request().response_status_number.in_range(200, 299) {
            // TODO Check that the server did not accept a connection to itself.
            // The requested tunnel was established: we are going
            // to probe the remaining data on this stream to see
            // if we need to ignore it or parse it
            self.request_state = State::CONNECT_PROBE_DATA;
        } else {
            // No tunnel; continue to the next transaction
            self.request_state = State::FINALIZE
        }
        Ok(())
    }

    /// Consumes bytes until the end of the current line.
    ///
    /// Returns OK on state change, ERROR on error, or HtpStatus::DATA_BUFFER
    /// when more data is needed.
    pub fn request_body_chunked_data_end(&mut self, data: &[u8]) -> Result<()> {
        // TODO We shouldn't really see anything apart from CR and LF,
        //      so we should warn about anything else.
        if let Ok((_, parsed)) = take_till_lf(data) {
            let len = parsed.len() as i64;
            self.request_curr_data.seek(SeekFrom::Current(len))?;
            self.request_mut().request_message_len += len;
            self.request_state = State::BODY_CHUNKED_LENGTH;
            Ok(())
        } else {
            self.request_mut().request_message_len += data.len() as i64;
            self.handle_request_absent_lf(data)
        }
    }

    /// Processes a chunk of data.
    ///
    /// Returns OK on state change, ERROR on error, or HtpStatus::DATA_BUFFER
    /// when more data is needed.
    pub fn request_body_chunked_data(&mut self, data: &[u8]) -> Result<()> {
        // Determine how many bytes we can consume.
        let bytes_to_consume: usize = min(
            data.len(),
            self.request_chunked_length.unwrap_or(0) as usize,
        );
        // If the input buffer is empty, ask for more data.
        if bytes_to_consume == 0 {
            return Err(HtpStatus::DATA);
        }
        // Consume the data.
        self.request_body_data(Some(&data[0..bytes_to_consume]))?;
        // Adjust counters.
        self.request_curr_data
            .seek(SeekFrom::Current(bytes_to_consume as i64))?;
        if let Some(len) = self.request_chunked_length.as_mut() {
            *len = len.wrapping_sub(bytes_to_consume as i32);
            if *len == 0 {
                // End of the chunk.
                self.request_state = State::BODY_CHUNKED_DATA_END;
                return Ok(());
            }
        }
        // Ask for more data.
        Err(HtpStatus::DATA)
    }

    /// Extracts chunk length.
    /// Returns OK on state change, ERROR on error, or HtpStatus::DATA_BUFFER
    /// when more data is needed.
    pub fn request_body_chunked_length(&mut self, mut data: &[u8]) -> Result<()> {
        loop {
            if let Ok((remaining, line)) = take_till_lf(data) {
                self.request_curr_data
                    .seek(SeekFrom::Current(line.len() as i64))?;
                if !self.request_buf.is_empty() {
                    self.check_request_buffer_limit(line.len())?;
                }
                if line.eq(b"\n") {
                    self.request_mut().request_message_len =
                        (self.request().request_message_len as u64).wrapping_add(line.len() as u64)
                            as i64;
                    //Empty chunk len. Try to continue parsing.
                    data = remaining;
                    continue;
                }
                let mut data = self.request_buf.clone();
                data.add(line);
                self.request_mut().request_message_len = (self.request().request_message_len as u64)
                    .wrapping_add(data.len() as u64)
                    as i64;
                // Handle chunk length.
                let len = parse_chunked_length(&data)?;
                self.request_chunked_length = len;
                let len = len.as_ref().ok_or(HtpStatus::ERROR).map_err(|e| {
                    // Invalid chunk length
                    htp_error!(
                        self.logger,
                        HtpLogCode::INVALID_REQUEST_CHUNK_LEN,
                        "Request chunk encoding: Invalid chunk length"
                    );
                    e
                })?;
                match len.cmp(&0) {
                    Ordering::Equal => {
                        // End of data
                        self.request_state = State::HEADERS;
                        self.request_mut().request_progress = HtpRequestProgress::TRAILER
                    }
                    Ordering::Greater => {
                        // More data available.
                        self.request_state = State::BODY_CHUNKED_DATA
                    }
                    _ => {}
                }
                return Ok(());
            } else {
                // Check if the data we have seen so far is invalid
                return if !is_valid_chunked_length_data(data) {
                    // Contains leading junk non hex_ascii data
                    // Invalid chunk length
                    htp_error!(
                        self.logger,
                        HtpLogCode::INVALID_REQUEST_CHUNK_LEN,
                        "Request chunk encoding: Invalid chunk length"
                    );
                    Err(HtpStatus::ERROR)
                } else {
                    self.handle_request_absent_lf(data)
                };
            }
        }
    }

    /// Processes identity request body.
    ///
    /// Returns OK on state change, ERROR on error, or HtpStatus::DATA_BUFFER
    /// when more data is needed.
    pub fn request_body_identity(&mut self, data: &mut ParserData) -> Result<()> {
        // Determine how many bytes we can consume.
        let bytes_to_consume: usize = min(data.len(), self.request_body_data_left as usize);
        // If the input buffer is empty, ask for more data.
        if bytes_to_consume == 0 {
            return Err(HtpStatus::DATA);
        }
        if data.is_gap() {
            self.request_mut().request_message_len = self
                .request()
                .request_message_len
                .wrapping_add(bytes_to_consume as i64);
            // Send the gap to the data hooks
            let mut tx_data = Data::new(self.request_mut(), data, false);
            self.request_run_hook_body_data(&mut tx_data)?;
        } else {
            // Consume the data.
            self.request_body_data(Some(&data.as_slice()[0..bytes_to_consume]))?;
            self.request_curr_data
                .seek(SeekFrom::Current(bytes_to_consume as i64))?;
        }
        // Adjust the counters.
        self.request_body_data_left =
            (self.request_body_data_left as u64).wrapping_sub(bytes_to_consume as u64) as i64;
        // Have we seen the entire request body?
        if self.request_body_data_left == 0 {
            // End of request body.
            self.request_state = State::FINALIZE;
            // Sends close signal to decompressors
            return self.request_body_data(None);
        }
        // Ask for more data.
        Err(HtpStatus::DATA)
    }

    /// Determines presence (and encoding) of a request body.
    ///
    /// Returns OK on state change, ERROR on error, or HtpStatus::DATA_BUFFER
    /// when more data is needed.
    pub fn request_body_determine(&mut self) -> Result<()> {
        // Determine the next state based on the presence of the request
        // body, and the coding used.
        match self.request().request_transfer_coding {
            HtpTransferCoding::CHUNKED => {
                self.request_state = State::BODY_CHUNKED_LENGTH;
                self.request_mut().request_progress = HtpRequestProgress::BODY
            }
            HtpTransferCoding::IDENTITY => {
                self.request_content_length = self.request().request_content_length;
                self.request_body_data_left = self.request_content_length;
                if self.request_content_length != 0 {
                    self.request_state = State::BODY_IDENTITY;
                    self.request_mut().request_progress = HtpRequestProgress::BODY
                } else {
                    self.request_state = State::FINALIZE
                }
            }
            HtpTransferCoding::NO_BODY => {
                // This request does not have a body, which
                // means that we're done with it
                self.request_state = State::FINALIZE
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
    pub fn request_headers(&mut self, data: &[u8]) -> Result<()> {
        if self.request_status == HtpStreamState::CLOSED {
            self.request_mut().request_header_parser.set_complete(true);
            // Parse previous header, if any.
            if let Some(request_header) = self.request_header.take() {
                self.parse_request_headers(request_header.as_slice())?;
            }
            self.request_buf.clear();
            self.request_mut().request_progress = HtpRequestProgress::TRAILER;
            // We've seen all the request headers.
            return self.state_request_headers();
        }
        let request_header = if let Some(mut request_header) = self.request_header.take() {
            request_header.add(data);
            request_header
        } else {
            Bstr::from(data)
        };

        let (remaining, eoh) = self.parse_request_headers(request_header.as_slice())?;
        //TODO: Update the request state machine so that we don't have to have this EOL check
        let eol = remaining.len() == request_header.len()
            && (remaining.starts_with(b"\r\n") || remaining.starts_with(b"\n"));
        if eoh
            //If the input started with an EOL, we assume this is the end of the headers
            || eol
        {
            if remaining.len() < data.len() {
                self.request_curr_data
                    .seek(SeekFrom::Current((data.len() - remaining.len()) as i64))?;
            } else if eol {
                if remaining.starts_with(b"\r\n") {
                    self.request_curr_data
                        .seek(SeekFrom::Current(min(data.len() as i64, 2)))?;
                } else if remaining.starts_with(b"\n") {
                    self.request_curr_data
                        .seek(SeekFrom::Current(min(data.len() as i64, 1)))?;
                }
            }
            // We've seen all the request headers.
            self.state_request_headers()
        } else {
            self.request_curr_data
                .seek(SeekFrom::Current(data.len() as i64))?;
            self.check_request_buffer_limit(remaining.len())?;
            let remaining = Bstr::from(remaining);
            self.request_header.replace(remaining);
            Err(HtpStatus::DATA_BUFFER)
        }
    }

    /// Determines request protocol.
    /// Returns OK on state change, ERROR on error, or HtpStatus::DATA_BUFFER
    /// when more data is needed.
    pub fn request_protocol(&mut self, data: &[u8]) -> Result<()> {
        // Is this a short-style HTTP/0.9 request? If it is,
        // we will not want to parse request headers.
        if !self.request().is_protocol_0_9 {
            // Switch to request header parsing.
            self.request_state = State::HEADERS;
            self.request_mut().request_progress = HtpRequestProgress::HEADERS
        } else {
            let mut parser = tuple::<_, _, NomError<&[u8]>, _>((take_until(":"), char(':')));
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
                    self.request_mut().is_protocol_0_9 = false;
                    // Switch to request header parsing.
                    self.request_state = State::HEADERS;
                    self.request_mut().request_progress = HtpRequestProgress::HEADERS;
                    return Ok(());
                }
                Err(_) => {
                    // We're done with this request.
                    self.request_state = State::FINALIZE;
                }
            }
        }
        Ok(())
    }

    /// Parse the request line.
    ///
    /// Returns OK on state change, ERROR on error, or HtpStatus::DATA_BUFFER
    /// when more data is needed.
    pub fn request_line_complete(&mut self, line: &[u8]) -> Result<()> {
        self.check_request_buffer_limit(line.len())?;
        if line.is_empty() {
            return Err(HtpStatus::DATA);
        }
        // Is this a line that should be ignored?
        if is_line_ignorable(self.cfg.server_personality, line) {
            // We have an empty/whitespace line, which we'll note, ignore and move on.
            self.request_mut().request_ignored_lines =
                self.request().request_ignored_lines.wrapping_add(1);
            return Ok(());
        }
        // Process request line.
        let data = chomp(line);
        self.request_mut().request_line = Some(Bstr::from(data));
        self.parse_request_line(data)?;
        // Finalize request line parsing.
        self.state_request_line()?;
        Ok(())
    }

    /// Parses request line.
    ///
    /// Returns OK on state change, ERROR on error, or HtpStatus::DATA_BUFFER
    /// when more data is needed.
    pub fn request_line(&mut self, input: &[u8]) -> Result<()> {
        let mut data = take(&mut self.request_buf);
        let data_len = data.len();
        data.add(input);
        match take_till_lf(data.as_slice()) {
            Ok((_, line)) => {
                if data_len > line.len() {
                    // Store the peeked ahead data
                    self.request_buf.add(&data[line.len()..data_len]);
                } else {
                    self.request_curr_data
                        .seek(SeekFrom::Current(line.len() as i64 - data_len as i64))?;
                }
                self.request_line_complete(line)
            }
            _ => {
                if self.request_status == HtpStreamState::CLOSED {
                    self.request_curr_data.seek(SeekFrom::End(0))?;
                    self.request_line_complete(data.as_slice())
                } else {
                    self.handle_request_absent_lf(data.as_slice())
                }
            }
        }
    }

    /// Consumes request body data.
    /// This function assumes that handling of chunked encoding is implemented
    /// by the container. When you're done submitting body data, invoke a state
    /// change (to REQUEST) to finalize any processing that might be pending.
    /// The supplied data is fully consumed and there is no expectation that it
    /// will be available afterwards. The protocol parsing code makes no copies
    /// of the data, but some parsers might.
    ///
    /// Returns HtpStatus::OK on success or HtpStatus::ERROR if the request transaction
    /// is invalid or response body data hook fails.
    pub fn request_body_data(&mut self, data: Option<&[u8]>) -> Result<()> {
        // None data is used to indicate the end of request body.
        // Keep track of body size before decompression.
        self.request_mut().request_message_len += data.unwrap_or(b"").len() as i64;
        match self.request().request_content_encoding_processing {
            HtpContentEncoding::GZIP
            | HtpContentEncoding::DEFLATE
            | HtpContentEncoding::ZLIB
            | HtpContentEncoding::LZMA => {
                // Send data buffer to the decompressor if it exists
                if self.request().request_decompressor.is_none() && data.is_none() {
                    return Ok(());
                }
                let mut decompressor = self
                    .request_mut()
                    .request_decompressor
                    .take()
                    .ok_or(HtpStatus::ERROR)?;
                if let Some(data) = data {
                    decompressor
                        .decompress(data)
                        .map_err(|_| HtpStatus::ERROR)?;
                    if decompressor.time_spent()
                        > self.cfg.compression_options.get_time_limit() as u64
                    {
                        htp_log!(
                            self.logger,
                            HtpLogLevel::ERROR,
                            HtpLogCode::COMPRESSION_BOMB,
                            format!(
                                "Compression bomb: spent {} us decompressing",
                                decompressor.time_spent(),
                            )
                        );
                        return Err(HtpStatus::ERROR);
                    }
                    // put the decompressor back in its slot
                    self.request_mut()
                        .request_decompressor
                        .replace(decompressor);
                } else {
                    // don't put the decompressor back in its slot
                    // ignore errors
                    let _ = decompressor.finish();
                }
            }
            HtpContentEncoding::NONE => {
                // When there's no decompression, request_entity_len.
                // is identical to request_message_len.
                // None data is used to indicate the end of request body.
                // Keep track of the body length.
                self.request_mut().request_entity_len += data.unwrap_or(b"").len() as i64;
                let _ = self.request_mut().request_process_multipart_data(data);
                let _ = self.request_mut().request_process_urlencoded_data(data);
                // Send data to the callbacks.
                let data = ParserData::from(data);
                let mut data = Data::new(self.request_mut(), &data, false);
                self.request_run_hook_body_data(&mut data).map_err(|e| {
                    htp_error!(
                        self.logger,
                        HtpLogCode::REQUEST_BODY_DATA_CALLBACK_ERROR,
                        format!("Request body data callback returned error ({:?})", e)
                    );
                    e
                })?
            }
            HtpContentEncoding::ERROR => {
                htp_error!(
                    self.logger,
                    HtpLogCode::INVALID_CONTENT_ENCODING,
                    "Expected a valid content encoding"
                );
                return Err(HtpStatus::ERROR);
            }
        }
        Ok(())
    }

    /// Initialize the request decompression engine. We can deal with three
    /// scenarios:
    ///
    /// 1. Decompression is enabled, compression indicated in headers, and we decompress.
    ///
    /// 2. As above, but the user disables decompression by setting response_content_encoding
    ///    to COMPRESSION_NONE.
    ///
    /// 3. Decompression is disabled and we do not attempt to enable it, but the user
    ///    forces decompression by setting response_content_encoding to one of the
    ///    supported algorithms.
    pub fn request_initialize_decompressors(&mut self) -> Result<()> {
        let ce = (*self.request_mut())
            .request_headers
            .get_nocase_nozero("content-encoding")
            .map(|(_, val)| (&val.value).clone());
        // Process multiple encodings if there is no match on fast path
        let mut slow_path = false;

        // Fast path - try to match directly on the encoding value
        self.request_mut().request_content_encoding = if let Some(ce) = &ce {
            if ce.cmp_nocase_nozero(b"gzip") == Ordering::Equal
                || ce.cmp_nocase_nozero(b"x-gzip") == Ordering::Equal
            {
                HtpContentEncoding::GZIP
            } else if ce.cmp_nocase_nozero(b"deflate") == Ordering::Equal
                || ce.cmp_nocase_nozero(b"x-deflate") == Ordering::Equal
            {
                HtpContentEncoding::DEFLATE
            } else if ce.cmp_nocase_nozero(b"lzma") == Ordering::Equal {
                HtpContentEncoding::LZMA
            } else if ce.cmp_nocase_nozero(b"inflate") == Ordering::Equal {
                HtpContentEncoding::NONE
            } else {
                slow_path = true;
                HtpContentEncoding::NONE
            }
        } else {
            HtpContentEncoding::NONE
        };

        // Configure decompression, if enabled in the configuration.
        self.request_mut().request_content_encoding_processing =
            if self.cfg.request_decompression_enabled {
                self.request().request_content_encoding
            } else {
                slow_path = false;
                HtpContentEncoding::NONE
            };

        let request_content_encoding_processing =
            self.request_mut().request_content_encoding_processing;
        let compression_options = self.cfg.compression_options;
        match &request_content_encoding_processing {
            HtpContentEncoding::GZIP
            | HtpContentEncoding::DEFLATE
            | HtpContentEncoding::ZLIB
            | HtpContentEncoding::LZMA => {
                self.request_prepend_decompressor(request_content_encoding_processing)?;
            }
            HtpContentEncoding::NONE => {
                if slow_path {
                    if let Some(ce) = &ce {
                        let mut layers = 0;
                        let mut lzma_layers = 0;
                        for encoding in ce.split(|c| *c == b',' || *c == b' ') {
                            if encoding.is_empty() {
                                continue;
                            }
                            layers += 1;

                            if let Some(limit) = compression_options.get_layer_limit() {
                                // decompression layer depth check
                                if layers > limit {
                                    htp_warn!(
                                        self.logger,
                                        HtpLogCode::TOO_MANY_ENCODING_LAYERS,
                                        "Too many request content encoding layers"
                                    );
                                    break;
                                }
                            }

                            let encoding = Bstr::from(encoding);
                            let encoding = if encoding.index_of_nocase(b"gzip").is_some() {
                                if !(encoding.cmp_slice(b"gzip") == Ordering::Equal
                                    || encoding.cmp_slice(b"x-gzip") == Ordering::Equal)
                                {
                                    htp_warn!(
                                        self.logger,
                                        HtpLogCode::ABNORMAL_CE_HEADER,
                                        "C-E gzip has abnormal value"
                                    );
                                }
                                HtpContentEncoding::GZIP
                            } else if encoding.index_of_nocase(b"deflate").is_some() {
                                if !(encoding.cmp_slice(b"deflate") == Ordering::Equal
                                    || encoding.cmp_slice(b"x-deflate") == Ordering::Equal)
                                {
                                    htp_warn!(
                                        self.logger,
                                        HtpLogCode::ABNORMAL_CE_HEADER,
                                        "C-E deflate has abnormal value"
                                    );
                                }
                                HtpContentEncoding::DEFLATE
                            } else if encoding.cmp_slice(b"lzma") == Ordering::Equal {
                                lzma_layers += 1;
                                if let Some(limit) = compression_options.get_lzma_layers() {
                                    // LZMA decompression layer depth check
                                    if lzma_layers > limit {
                                        htp_warn!(
                                            self.logger,
                                            HtpLogCode::REQUEST_TOO_MANY_LZMA_LAYERS,
                                            "Too many request content encoding lzma layers"
                                        );
                                        break;
                                    }
                                }
                                HtpContentEncoding::LZMA
                            } else if encoding.cmp_slice(b"inflate") == Ordering::Equal {
                                HtpContentEncoding::NONE
                            } else {
                                htp_warn!(
                                    self.logger,
                                    HtpLogCode::ABNORMAL_CE_HEADER,
                                    "C-E unknown setting"
                                );
                                HtpContentEncoding::NONE
                            };
                            self.request_prepend_decompressor(encoding)?;
                        }
                    }
                }
            }
            HtpContentEncoding::ERROR => {
                htp_error!(
                    self.logger,
                    HtpLogCode::INVALID_CONTENT_ENCODING,
                    "Expected a valid content encoding"
                );
                return Err(HtpStatus::ERROR);
            }
        }
        Ok(())
    }

    /// Prepend a decompressor to the request
    pub fn request_prepend_decompressor(&mut self, encoding: HtpContentEncoding) -> Result<()> {
        let compression_options = self.cfg.compression_options;
        if encoding != HtpContentEncoding::NONE {
            if let Some(decompressor) = self.request_mut().request_decompressor.take() {
                self.request_mut()
                    .request_decompressor
                    .replace(decompressor.prepend(encoding, compression_options)?);
            } else {
                // The processing encoding will be the first one encountered
                (*self.request_mut()).request_content_encoding_processing = encoding;

                // Add the callback first because it will be called last in
                // the chain of writers

                // TODO: fix lifetime error and remove this line!
                let connp_ptr: *mut ConnectionParser = self as *mut ConnectionParser;
                let decompressor = unsafe {
                    Decompressor::new_with_callback(
                        encoding,
                        Box::new(move |data: Option<&[u8]>| -> std::io::Result<usize> {
                            (*connp_ptr).request_decompressor_callback(data)
                        }),
                        compression_options,
                    )?
                };
                self.request_mut()
                    .request_decompressor
                    .replace(decompressor);
            }
        }
        Ok(())
    }

    fn request_decompressor_callback(&mut self, data: Option<&[u8]>) -> std::io::Result<usize> {
        // If no data is passed, call the hooks with NULL to signify the end of the
        // request body.
        let parser_data = ParserData::from(data);
        let mut tx_data = Data::new(
            self.request_mut(),
            &parser_data,
            // is_last is not used in this callback
            false,
        );

        // Keep track of actual request body length.
        self.request_mut().request_entity_len += tx_data.len() as i64;

        // Invoke all callbacks.
        self.request_run_hook_body_data(&mut tx_data)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "body data hook failed"))?;

        let compression_options = self.cfg.compression_options;
        if let Some(decompressor) = &mut self.request_mut().request_decompressor {
            if decompressor.callback_inc() % compression_options.get_time_test_freq() == 0 {
                if let Some(time_spent) = decompressor.timer_reset() {
                    if time_spent > compression_options.get_time_limit() as u64 {
                        htp_log!(
                            self.logger,
                            HtpLogLevel::ERROR,
                            HtpLogCode::COMPRESSION_BOMB,
                            format!("Compression bomb: spent {} us decompressing", time_spent)
                        );
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "compression_time_limit reached",
                        ));
                    }
                }
            }
        }

        // output > ratio * input ?
        let ratio = compression_options.get_bomb_ratio();
        let exceeds_ratio =
            if let Some(ratio) = self.request_mut().request_message_len.checked_mul(ratio) {
                self.request().request_entity_len > ratio
            } else {
                // overflow occured
                true
            };

        let bomb_limit = compression_options.get_bomb_limit();
        let request_entity_len = self.request().request_entity_len;
        let request_message_len = self.request().request_message_len;
        if request_entity_len > bomb_limit as i64 && exceeds_ratio {
            htp_log!(
                self.logger,
                HtpLogLevel::ERROR,
                HtpLogCode::COMPRESSION_BOMB,
                format!(
                    "Compression bomb: decompressed {} bytes out of {}",
                    request_entity_len, request_message_len,
                )
            );
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "compression_bomb_limit reached",
            ));
        }
        Ok(tx_data.len())
    }

    /// Finalizes request.
    ///
    /// Returns OK on state change, ERROR on error, or HtpStatus::DATA_BUFFER
    /// when more data is needed.
    pub fn request_finalize(&mut self, data: &ParserData) -> Result<()> {
        if data.is_gap() {
            return self.state_request_complete();
        }
        let mut work = data.as_slice();
        if self.request_status != HtpStreamState::CLOSED {
            let request_next_byte = self
                .request_curr_data
                .get_ref()
                .get(self.request_curr_data.position() as usize);
            if request_next_byte.is_none() {
                return self.state_request_complete();
            }

            if let Ok((_, line)) = take_till_lf(work) {
                self.request_curr_data
                    .seek(SeekFrom::Current(line.len() as i64))?;
                work = line;
            } else {
                return self.handle_request_absent_lf(work);
            }
        }

        if !self.request_buf.is_empty() {
            self.check_request_buffer_limit(work.len())?;
        }
        let mut data = take(&mut self.request_buf);
        let buf_len = data.len();
        data.add(work);

        if data.is_empty() {
            //closing
            return self.state_request_complete();
        }
        let res = tuple((take_is_space, take_not_is_space))(&data);

        if let Ok((_, (_, method))) = res {
            if method.is_empty() {
                // empty whitespace line
                let rc = self.request_body_data(Some(&data));
                self.request_buf.clear();
                return rc;
            }
            if HtpMethod::new(method) == HtpMethod::UNKNOWN {
                if self.request_body_data_left <= 0 {
                    // log only once per transaction
                    htp_warn!(
                        self.logger,
                        HtpLogCode::REQUEST_BODY_UNEXPECTED,
                        "Unexpected request body"
                    );
                } else {
                    self.request_body_data_left = 1;
                }
                // Interpret remaining bytes as body data
                let rc = self.request_body_data(Some(&data));
                self.request_buf.clear();
                return rc;
            } // else continue
            self.request_body_data_left = -1;
        }
        // didnt use data, restore
        self.request_buf.add(&data[0..buf_len]);
        //unread last end of line so that request_line works
        if self.request_curr_data.position() < data.len() as u64 {
            self.request_curr_data.set_position(0);
        } else {
            self.request_curr_data
                .seek(SeekFrom::Current(-(data.len() as i64)))?;
        }
        self.state_request_complete()
    }

    /// Consumes whatever is left in the buffer after detecting an http/0.9 session.
    pub fn request_ignore_data_after_http_0_9(&mut self) -> Result<()> {
        let bytes_left = self.request_curr_len() - self.request_curr_data.position() as i64;

        if bytes_left > 0 {
            self.conn.flags.set(ConnectionFlags::HTTP_0_9_EXTRA)
        }
        self.request_curr_data.seek(SeekFrom::End(0))?;
        Err(HtpStatus::DATA)
    }

    /// The idle state is where the parser will end up after a transaction is processed.
    /// If there is more data available, a new request will be started.
    ///
    /// Returns OK on state change, ERROR on error, or HTP_DATA when more data is needed.
    pub fn request_idle(&mut self) -> Result<()> {
        // We want to start parsing the next request (and change
        // the state from IDLE) only if there's at least one
        // byte of data available. Otherwise we could be creating
        // new structures even if there's no more data on the
        // connection.
        if self.request_curr_data.position() as i64 >= self.request_curr_len() {
            // we may have buffered some data, if we are closing, we want to process it
            if self.request_status != HtpStreamState::CLOSED || self.request_buf.is_empty() {
                return Err(HtpStatus::DATA);
            }
        }
        self.request_reset();
        // Change state to TRANSACTION_START
        // Ignore the result.
        let _ = self.state_request_start();
        Ok(())
    }

    /// Buffer incomplete request data and verify that field_limit
    /// constraint is met.
    pub fn handle_request_absent_lf(&mut self, data: &[u8]) -> Result<()> {
        self.request_curr_data.seek(SeekFrom::End(0))?;
        self.check_request_buffer_limit(data.len())?;
        self.request_buf.add(data);
        Err(HtpStatus::DATA_BUFFER)
    }

    /// Run the REQUEST_BODY_DATA hook.
    pub fn request_run_hook_body_data(&mut self, d: &mut Data) -> Result<()> {
        // Do not invoke callbacks with an empty data chunk
        if !d.data().is_null() && d.is_empty() {
            return Ok(());
        }
        self.request()
            .hook_request_body_data
            .clone()
            .run_all(self, d)?;
        // Run configuration hooks second
        self.cfg.hook_request_body_data.run_all(self, d)?;
        // Treat request body as file
        if let Some(file) = &mut self.request_file {
            file.handle_file_data(self.cfg.hook_request_file_data.clone(), d.data(), d.len())?;
        }
        Ok(())
    }

    /// Process a chunk of inbound (client or request) data.
    pub fn request_data(
        &mut self,
        mut chunk: ParserData,
        timestamp: Option<DateTime<Utc>>,
    ) -> HtpStreamState {
        // Return if the connection is in stop state.
        if self.request_status == HtpStreamState::STOP {
            htp_info!(
                self.logger,
                HtpLogCode::PARSER_STATE_ERROR,
                "Inbound parser is in STOP state"
            );
            return HtpStreamState::STOP;
        }
        // Return if the connection had a fatal error earlier
        if self.request_status == HtpStreamState::ERROR {
            htp_error!(
                self.logger,
                HtpLogCode::PARSER_STATE_ERROR,
                "Inbound parser is in ERROR state"
            );
            return HtpStreamState::ERROR;
        }

        // If the length of the supplied data chunk is zero, proceed
        // only if the stream has been closed. We do not allow zero-sized
        // chunks in the API, but we use them internally to force the parsers
        // to finalize parsing.
        if chunk.is_empty() && self.request_status != HtpStreamState::CLOSED {
            htp_error!(
                self.logger,
                HtpLogCode::ZERO_LENGTH_DATA_CHUNKS,
                "Zero-length data chunks are not allowed"
            );
            return HtpStreamState::CLOSED;
        }
        // Remember the timestamp of the current request data chunk
        if let Some(timestamp) = timestamp {
            self.request_timestamp = timestamp;
        }

        // Store the current chunk information
        self.request_curr_data = Cursor::new(chunk.as_slice().to_vec());
        self.request_current_receiver_offset = 0;
        self.request_chunk_count = self.request_chunk_count.wrapping_add(1);
        self.conn.track_inbound_data(chunk.len());
        // Return without processing any data if the stream is in tunneling
        // mode (which it would be after an initial CONNECT transaction).
        if self.request_status == HtpStreamState::TUNNEL {
            return HtpStreamState::TUNNEL;
        }
        if self.response_status == HtpStreamState::DATA_OTHER {
            self.response_status = HtpStreamState::DATA
        }
        //handle gap
        if chunk.is_gap() {
            // Mark the transaction as having a gap
            self.request_mut()
                .flags
                .set(HtpFlags::REQUEST_MISSING_BYTES);

            if self.request().request_progress == HtpRequestProgress::NOT_STARTED {
                // Force the parser to start if it hasn't already
                self.request_mut().request_progress = HtpRequestProgress::GAP;
                if self.request_index() == 0 {
                    // We have a leading gap on the first transaction.
                    self.request_status = HtpStreamState::ERROR;
                    return HtpStreamState::ERROR;
                }
            }

            if self.request_state != State::BODY_IDENTITY
                && self.request_state != State::IGNORE_DATA_AFTER_HTTP_0_9
            {
                // go to request_connect_probe_data ?
                htp_error!(
                    self.logger,
                    HtpLogCode::INVALID_GAP,
                    "Gaps are not allowed during this state"
                );
                return HtpStreamState::CLOSED;
            }
        }

        loop
        // Invoke a processor, in a loop, until an error
        // occurs or until we run out of data. Many processors
        // will process a request, each pointing to the next
        // processor that needs to run.
        // Return if there's been an error or if we've run out of data. We are relying
        // on processors to supply error messages, so we'll keep quiet here.
        {
            let mut rc = self.handle_request_state(&mut chunk);

            if rc.is_ok() {
                if self.request_status == HtpStreamState::TUNNEL {
                    return HtpStreamState::TUNNEL;
                }
                rc = self.request_handle_state_change()
            }
            match rc {
                // Continue looping.
                Ok(_) => {}
                // Do we need more data?
                Err(HtpStatus::DATA) | Err(HtpStatus::DATA_BUFFER) => {
                    // Ignore result.
                    let _ = self.request_receiver_send_data(false);
                    self.request_status = HtpStreamState::DATA;
                    return HtpStreamState::DATA;
                }
                // Check for suspended parsing.
                Err(HtpStatus::DATA_OTHER) => {
                    // We might have actually consumed the entire data chunk?
                    if (self.request_curr_data.position() as i64) >= self.request_curr_len() {
                        // Do not send STREAM_DATE_DATA_OTHER if we've consumed the entire chunk.
                        self.request_status = HtpStreamState::DATA;
                        return HtpStreamState::DATA;
                    } else {
                        // Partial chunk consumption.
                        self.request_status = HtpStreamState::DATA_OTHER;
                        return HtpStreamState::DATA_OTHER;
                    }
                }
                // Check for the stop signal.
                Err(HtpStatus::STOP) => {
                    self.request_status = HtpStreamState::STOP;
                    return HtpStreamState::STOP;
                }
                // Permanent stream error.
                Err(_) => {
                    self.request_status = HtpStreamState::ERROR;
                    return HtpStreamState::ERROR;
                }
            }
        }
    }

    /// Return length of the current request chunk.
    pub fn request_curr_len(&self) -> i64 {
        self.request_curr_data.get_ref().len() as i64
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(b"GET", HtpMethod::GET)]
    #[case(b"PUT", HtpMethod::PUT)]
    #[case(b"POST", HtpMethod::POST)]
    #[case(b"PoST", HtpMethod::UNKNOWN)]
    #[case(b"post", HtpMethod::UNKNOWN)]
    #[case(b"NOT_METHOD", HtpMethod::UNKNOWN)]
    fn test_method(#[case] input: &[u8], #[case] expected: HtpMethod) {
        assert_eq!(HtpMethod::new(input), expected);
    }
}
