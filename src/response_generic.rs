use crate::error::Result;
use crate::transaction::HtpProtocol;
use crate::util::Flags;
use crate::{bstr, connection_parser, parsers, transaction, util, HtpStatus};
use nom::{error::ErrorKind, sequence::tuple, Err::Error};
use std::cmp::Ordering;

impl connection_parser::ConnectionParser {
    /// Generic response line parser.
    pub unsafe fn parse_response_line_generic(&mut self, response_line: &[u8]) -> Result<()> {
        let out_tx = self.out_tx_mut_ok()?;
        out_tx.response_protocol_number = HtpProtocol::INVALID;
        out_tx.response_status = None;
        out_tx.response_status_number = -1;
        out_tx.response_message = None;

        let response_line_parser = tuple::<_, _, (_, ErrorKind), _>((
            util::take_is_space,
            util::take_not_is_space,
            util::take_is_space,
            util::take_not_is_space,
            util::take_ascii_whitespace(),
        ));

        if let Ok((message, (_ls, response_protocol, ws1, status_code, ws2))) =
            response_line_parser(response_line)
        {
            if response_protocol.is_empty() {
                return Ok(());
            }

            out_tx.response_protocol = Some(bstr::Bstr::from(response_protocol));
            self.out_tx_mut_ok()?.response_protocol_number =
                parsers::parse_protocol(response_protocol, self);

            if ws1.is_empty() || status_code.is_empty() {
                return Ok(());
            }

            let out_tx = self.out_tx_mut_ok()?;
            out_tx.response_status = Some(bstr::Bstr::from(status_code));

            if let Some(status_number) = parsers::parse_status(status_code) {
                out_tx.response_status_number = status_number as i32;
            } else {
                out_tx.response_status_number = -1;
            }

            if ws2.is_empty() {
                return Ok(());
            }

            out_tx.response_message = Some(bstr::Bstr::from(message));
        } else {
            return Err(HtpStatus::ERROR);
        }
        Ok(())
    }

    /// Generic response header parser.
    pub unsafe fn parse_response_header_generic(
        &mut self,
        data: &[u8],
    ) -> Result<transaction::Header> {
        let data = util::chomp(&data);
        let mut flags = Flags::empty();

        let mut header: &[u8] = b"";
        let mut value: &[u8] = b"";

        match util::split_by_colon(data) {
            Ok((mut name, val)) => {
                // Colon present
                // Log empty header name
                let name_len = name.len();
                if name_len == 0 {
                    flags |= Flags::FIELD_INVALID;
                    if !self.out_tx_mut_ok()?.flags.contains(Flags::FIELD_INVALID) {
                        // Only once per transaction.
                        self.out_tx_mut_ok()?.flags |= Flags::FIELD_INVALID;
                        htp_warn!(
                            self,
                            HtpLogCode::RESPONSE_INVALID_EMPTY_NAME,
                            "Response field invalid: empty name."
                        );
                    }
                }

                let mut unprintable = 0;
                // Ignore unprintable after field-name
                for item in name.iter().rev() {
                    if item <= &0x20 {
                        flags |= Flags::FIELD_INVALID;
                        if !self.out_tx_mut_ok()?.flags.contains(Flags::FIELD_INVALID) {
                            // Only once per transaction.
                            self.out_tx_mut_ok()?.flags |= Flags::FIELD_INVALID;
                            htp_log!(
                                self,
                                HtpLogLevel::WARNING,
                                HtpLogCode::RESPONSE_INVALID_LWS_AFTER_NAME,
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
                if !util::is_word_token(name) {
                    flags |= Flags::FIELD_INVALID;
                    if !self.out_tx_mut_ok()?.flags.contains(Flags::FIELD_INVALID) {
                        self.out_tx_mut_ok()?.flags |= Flags::FIELD_INVALID;
                        htp_warn!(
                            self,
                            HtpLogCode::RESPONSE_HEADER_NAME_NOT_TOKEN,
                            "Response header name is not a token."
                        );
                    }
                }

                header = name;
                value = val;
            }
            Err(Error(_)) => {
                // No colon
                flags |= Flags::FIELD_UNPARSEABLE;
                flags |= Flags::FIELD_INVALID;
                // clean up
                if !self
                    .out_tx_mut_ok()?
                    .flags
                    .contains(Flags::FIELD_UNPARSEABLE)
                {
                    // Only once per transaction.
                    self.out_tx_mut_ok()?.flags |= Flags::FIELD_UNPARSEABLE;
                    self.out_tx_mut_ok()?.flags |= Flags::FIELD_INVALID;
                    htp_warn!(
                        self,
                        HtpLogCode::RESPONSE_FIELD_MISSING_COLON,
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
                self,
                HtpLogLevel::WARNING,
                HtpLogCode::REQUEST_HEADER_INVALID,
                "Response header value contains null."
            );
        }

        Ok(transaction::Header::new_with_flags(
            header.into(),
            value.into(),
            flags,
        ))
    }

    /// Generic response header line(s) processor, which assembles folded lines
    /// into a single buffer before invoking the parsing function.
    pub fn process_response_header_generic(&mut self, data: &[u8]) -> Result<()> {
        let header = unsafe { self.parse_response_header_generic(data)? };
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
            if !h_existing.flags.contains(Flags::FIELD_REPEATED) {
                // This is the second occurence for this header.
                repeated = true;
            } else if reps < 64 {
                update_reps = true;
            } else {
                return Ok(());
            }
            h_existing.flags |= Flags::FIELD_REPEATED;
            // For simplicity reasons, we count the repetitions of all headers
            // Having multiple C-L headers is against the RFC but many
            // browsers ignore the subsequent headers if the values are the same.
            if header.name.cmp_nocase("Content-Length") == Ordering::Equal {
                // Don't use string comparison here because we want to
                // ignore small formatting differences.
                let existing_cl = util::parse_content_length(&h_existing.value, None);
                let new_cl = util::parse_content_length(&(header.value), None);
                if existing_cl.is_none() || new_cl.is_none() || existing_cl != new_cl {
                    // Ambiguous response C-L value.
                    htp_warn!(
                        self,
                        HtpLogCode::DUPLICATE_CONTENT_LENGTH_FIELD_IN_RESPONSE,
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
                self,
                HtpLogCode::RESPONSE_HEADER_REPETITION,
                "Repetition for header"
            );
        }
        Ok(())
    }
}
