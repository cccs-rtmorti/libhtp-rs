use crate::error::Result;
use crate::htp_transaction::Protocol;
use crate::htp_util::Flags;
use crate::{bstr, htp_connection_parser, htp_parsers, htp_transaction, htp_util, Status};
use nom::{error::ErrorKind, sequence::tuple, Err::Error};
use std::cmp::Ordering;

impl htp_connection_parser::htp_connp_t {
    /// Generic response line parser.
    pub unsafe fn parse_response_line_generic(&mut self) -> Result<()> {
        let tx = self.out_tx_mut_ok()?;
        let data = if let Some(data) = tx.response_line.clone() {
            data
        } else {
            return Err(Status::ERROR);
        };
        (*tx).response_protocol_number = Protocol::INVALID;
        (*tx).response_status = None;
        (*tx).response_status_number = -1;
        (*tx).response_message = None;

        let response_line_parser = tuple::<_, _, (_, ErrorKind), _>((
            htp_util::take_htp_is_space,
            htp_util::take_not_htp_is_space,
            htp_util::take_htp_is_space,
            htp_util::take_not_htp_is_space,
            htp_util::take_ascii_whitespace(),
        ));

        if let Ok((message, (_ls, response_protocol, ws1, status_code, ws2))) =
            response_line_parser(data.as_slice())
        {
            if response_protocol.is_empty() {
                return Ok(());
            }

            let out_tx = self.out_tx_mut_ok()?;
            out_tx.response_protocol = Some(bstr::bstr_t::from(response_protocol));
            self.out_tx_mut_ok()?.response_protocol_number =
                htp_parsers::htp_parse_protocol(response_protocol, self);

            if ws1.is_empty() || status_code.is_empty() {
                return Ok(());
            }

            let out_tx = self.out_tx_mut_ok()?;
            out_tx.response_status = Some(bstr::bstr_t::from(status_code));

            if let Some(status_number) = htp_parsers::htp_parse_status(status_code) {
                out_tx.response_status_number = status_number as i32;
            } else {
                out_tx.response_status_number = -1;
            }

            if ws2.is_empty() {
                return Ok(());
            }

            out_tx.response_message = Some(bstr::bstr_t::from(message));
        } else {
            return Err(Status::ERROR);
        }
        Ok(())
    }

    /// Generic response header parser.
    pub unsafe fn parse_response_header_generic(
        &mut self,
        data: &[u8],
    ) -> Result<htp_transaction::htp_header_t> {
        let data = htp_util::htp_chomp(&data);
        let mut flags = Flags::empty();

        let mut header: &[u8] = b"";
        let mut value: &[u8] = b"";

        match htp_util::split_by_colon(data) {
            Ok((mut name, val)) => {
                // Colon present
                // Log empty header name
                let name_len = name.len();
                if name_len == 0 {
                    flags |= Flags::HTP_FIELD_INVALID;
                    if !self
                        .out_tx_mut_ok()?
                        .flags
                        .contains(Flags::HTP_FIELD_INVALID)
                    {
                        // Only once per transaction.
                        self.out_tx_mut_ok()?.flags |= Flags::HTP_FIELD_INVALID;
                        htp_warn!(
                            self as *mut htp_connection_parser::htp_connp_t,
                            htp_log_code::RESPONSE_INVALID_EMPTY_NAME,
                            "Response field invalid: empty name."
                        );
                    }
                }

                let mut unprintable = 0;
                // Ignore unprintable after field-name
                for item in name.iter().rev() {
                    if item <= &0x20 {
                        flags |= Flags::HTP_FIELD_INVALID;
                        if !self
                            .out_tx_mut_ok()?
                            .flags
                            .contains(Flags::HTP_FIELD_INVALID)
                        {
                            // Only once per transaction.
                            self.out_tx_mut_ok()?.flags |= Flags::HTP_FIELD_INVALID;
                            htp_log!(
                                self as *mut htp_connection_parser::htp_connp_t,
                                htp_log_level_t::HTP_LOG_WARNING,
                                htp_log_code::RESPONSE_INVALID_LWS_AFTER_NAME,
                                "Response field invalid: LWS after name"
                            );
                        }
                        unprintable += 1;
                    } else {
                        break;
                    }
                }

                if unprintable > 0 {
                    name = &name[0..name_len - unprintable];
                }

                // Check header is a token
                if !htp_util::is_word_token(name) {
                    flags |= Flags::HTP_FIELD_INVALID;
                    if !self
                        .out_tx_mut_ok()?
                        .flags
                        .contains(Flags::HTP_FIELD_INVALID)
                    {
                        self.out_tx_mut_ok()?.flags |= Flags::HTP_FIELD_INVALID;
                        htp_warn!(
                            self as *mut htp_connection_parser::htp_connp_t,
                            htp_log_code::RESPONSE_HEADER_NAME_NOT_TOKEN,
                            "Response header name is not a token."
                        );
                    }
                }

                header = name;
                value = val;
            }
            Err(Error(_)) => {
                // No colon
                flags |= Flags::HTP_FIELD_UNPARSEABLE;
                flags |= Flags::HTP_FIELD_INVALID;
                // clean up
                if !self
                    .out_tx_mut_ok()?
                    .flags
                    .contains(Flags::HTP_FIELD_UNPARSEABLE)
                {
                    // Only once per transaction.
                    self.out_tx_mut_ok()?.flags |= Flags::HTP_FIELD_UNPARSEABLE;
                    self.out_tx_mut_ok()?.flags |= Flags::HTP_FIELD_INVALID;
                    htp_warn!(
                        self as *mut htp_connection_parser::htp_connp_t,
                        htp_log_code::RESPONSE_FIELD_MISSING_COLON,
                        "Response field invalid: missing colon."
                    );
                }
                value = data;
            }
            _ => (),
        }

        // No null char in val
        if value.contains(&0) {
            htp_log!(
                self as *mut htp_connection_parser::htp_connp_t,
                htp_log_level_t::HTP_LOG_WARNING,
                htp_log_code::REQUEST_HEADER_INVALID,
                "Response header value contains null."
            );
        }

        Ok(htp_transaction::htp_header_t::new_with_flags(
            header.into(),
            value.into(),
            flags,
        ))
    }

    /// Generic response header line(s) processor, which assembles folded lines
    /// into a single buffer before invoking the parsing function.
    pub unsafe fn process_response_header_generic(&mut self, data: &[u8]) -> Result<()> {
        let header = self.parse_response_header_generic(data)?;
        let mut repeated = false;
        let reps = self.out_tx_mut_ok()?.res_header_repetitions;
        let mut update_reps = false;
        // Do we already have a header with the same name?
        if let Some((_, h_existing)) = self
            .out_tx_mut_ok()?
            .response_headers
            .get_nocase_mut(header.name.as_slice())
        {
            // Keep track of repeated same-name headers.
            if !h_existing.flags.contains(Flags::HTP_FIELD_REPEATED) {
                // This is the second occurence for this header.
                repeated = true;
            } else if reps < 64 {
                update_reps = true;
            } else {
                return Ok(());
            }
            h_existing.flags |= Flags::HTP_FIELD_REPEATED;
            // For simplicity reasons, we count the repetitions of all headers
            // Having multiple C-L headers is against the RFC but many
            // browsers ignore the subsequent headers if the values are the same.
            if header.name.cmp_nocase("Content-Length") == Ordering::Equal {
                // Don't use string comparison here because we want to
                // ignore small formatting differences.
                let existing_cl = htp_util::htp_parse_content_length(&h_existing.value, None);
                let new_cl = htp_util::htp_parse_content_length(&(header.value), None);
                if existing_cl.is_none() || new_cl.is_none() || existing_cl != new_cl {
                    // Ambiguous response C-L value.
                    htp_warn!(
                        self as *mut htp_connection_parser::htp_connp_t,
                        htp_log_code::DUPLICATE_CONTENT_LENGTH_FIELD_IN_RESPONSE,
                        "Ambiguous response C-L value"
                    );
                }
            } else {
                // Add to the existing header.
                h_existing.value.extend_from_slice(b", ");
                h_existing.value.extend_from_slice(header.value.as_slice());
            }
        } else {
            self.out_tx_mut_ok()?
                .response_headers
                .add(header.name.clone(), header);
        }
        if update_reps {
            self.out_tx_mut_ok()?.res_header_repetitions =
                self.out_tx_mut_ok()?.res_header_repetitions.wrapping_add(1)
        }
        if repeated {
            htp_warn!(
                self as *mut htp_connection_parser::htp_connp_t,
                htp_log_code::RESPONSE_HEADER_REPETITION,
                "Repetition for header"
            );
        }
        Ok(())
    }
}
