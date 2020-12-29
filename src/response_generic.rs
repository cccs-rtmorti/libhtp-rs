use crate::{
    bstr::Bstr,
    connection_parser::ConnectionParser,
    error::Result,
    headers::{headers, Flags as HeaderFlags},
    parsers::{parse_content_length, parse_protocol, parse_status},
    transaction::{Header, HtpProtocol, HtpResponseNumber},
    util::{take_ascii_whitespace, take_is_space, take_not_is_space, Flags},
    HtpStatus,
};
use nom::{error::ErrorKind, sequence::tuple};
use std::cmp::Ordering;

impl ConnectionParser {
    /// Generic response line parser.
    pub fn parse_response_line_generic(&mut self, response_line: &[u8]) -> Result<()> {
        let out_tx = self.out_tx_mut_ok()?;
        out_tx.response_protocol_number = HtpProtocol::INVALID;
        out_tx.response_status = None;
        out_tx.response_status_number = HtpResponseNumber::INVALID;
        out_tx.response_message = None;

        let response_line_parser = tuple::<_, _, (_, ErrorKind), _>((
            take_is_space,
            take_not_is_space,
            take_is_space,
            take_not_is_space,
            take_ascii_whitespace(),
        ));

        if let Ok((message, (_ls, response_protocol, ws1, status_code, ws2))) =
            response_line_parser(response_line)
        {
            if response_protocol.is_empty() {
                return Ok(());
            }

            out_tx.response_protocol = Some(Bstr::from(response_protocol));
            self.out_tx_mut_ok()?.response_protocol_number =
                parse_protocol(response_protocol, self);

            if ws1.is_empty() || status_code.is_empty() {
                return Ok(());
            }

            let out_tx = self.out_tx_mut_ok()?;
            out_tx.response_status = Some(Bstr::from(status_code));
            out_tx.response_status_number = parse_status(status_code);

            if ws2.is_empty() {
                return Ok(());
            }

            out_tx.response_message = Some(Bstr::from(message));
        } else {
            return Err(HtpStatus::ERROR);
        }
        Ok(())
    }

    /// Generic response header parser.
    ///
    ///Returns a tuple of the unparsed data and a boolean indicating if the EOH was seen.
    pub fn process_response_headers_generic<'a>(
        &mut self,
        data: &'a [u8],
    ) -> Result<(&'a [u8], bool)> {
        let rc = headers(data);
        if let Ok((remaining, (headers, eoh))) = rc {
            for h in headers {
                let mut flags = Flags::empty();
                let name_flags = &h.name.flags;
                let value_flags = &h.value.flags;
                if value_flags.contains(HeaderFlags::DEFORMED_EOL)
                    || name_flags.contains(HeaderFlags::DEFORMED_EOL)
                {
                    htp_warn!(
                        self,
                        HtpLogCode::DEFORMED_EOL,
                        "Weird response end of lines mix"
                    );
                }
                // Ignore LWS after field-name.
                if name_flags.contains(HeaderFlags::NAME_TRAILING_WHITESPACE) {
                    htp_warn_once!(
                        self,
                        HtpLogCode::RESPONSE_INVALID_LWS_AFTER_NAME,
                        "Request field invalid: LWS after name",
                        self.out_tx_mut_ok()?.flags,
                        flags,
                        Flags::FIELD_INVALID
                    );
                }
                //If there was leading whitespace, probably was invalid folding.
                if name_flags.contains(HeaderFlags::NAME_LEADING_WHITESPACE) {
                    htp_warn_once!(
                        self,
                        HtpLogCode::INVALID_RESPONSE_FIELD_FOLDING,
                        "Invalid response field folding",
                        self.out_tx_mut_ok()?.flags,
                        flags,
                        Flags::INVALID_FOLDING
                    );
                    flags |= Flags::FIELD_INVALID;
                }
                // Check that field-name is a token
                if name_flags.contains(HeaderFlags::NAME_NON_TOKEN_CHARS) {
                    // Incorrectly formed header name.
                    htp_warn_once!(
                        self,
                        HtpLogCode::RESPONSE_HEADER_NAME_NOT_TOKEN,
                        "Response header name is not a token",
                        self.out_tx_mut_ok()?.flags,
                        flags,
                        Flags::FIELD_INVALID
                    );
                }
                // No colon?
                if name_flags.contains(HeaderFlags::MISSING_COLON) {
                    // We handle this case as a header with an empty name, with the value equal
                    // to the entire input string.
                    // TODO Apache will respond to this problem with a 400.
                    // Now extract the name and the value
                    htp_warn_once!(
                        self,
                        HtpLogCode::RESPONSE_FIELD_MISSING_COLON,
                        "Response field invalid: colon missing",
                        self.out_tx_mut_ok()?.flags,
                        flags,
                        Flags::FIELD_UNPARSEABLE
                    );
                    flags |= Flags::FIELD_INVALID;
                } else if name_flags.contains(HeaderFlags::NAME_EMPTY) {
                    // Empty header name.
                    htp_warn_once!(
                        self,
                        HtpLogCode::RESPONSE_INVALID_EMPTY_NAME,
                        "Response field invalid: empty name",
                        self.out_tx_mut_ok()?.flags,
                        flags,
                        Flags::FIELD_INVALID
                    );
                }
                self.process_response_header_generic(Header::new_with_flags(
                    h.name.name.into(),
                    h.value.value.into(),
                    flags,
                ))?;
            }
            Ok((remaining, eoh))
        } else {
            Ok((data, false))
        }
    }

    /// Generic response header line(s) processor, which assembles folded lines
    /// into a single buffer before invoking the parsing function.
    fn process_response_header_generic(&mut self, header: Header) -> Result<()> {
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
                let existing_cl = parse_content_length(&h_existing.value, None);
                let new_cl = parse_content_length(&(header.value), None);
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
