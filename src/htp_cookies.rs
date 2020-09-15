use crate::error::Result;
use crate::htp_util::take_ascii_whitespace;
use crate::{bstr, htp_table, htp_transaction};

/// Parses a single v0 request cookie.
///
/// Returns the (name, value).
pub fn htp_parse_single_cookie_v0(data: &[u8]) -> (&[u8], &[u8]) {
    let parts: Vec<&[u8]> = data.splitn(2, |&x| x == '=' as u8).collect();
    match parts.len() {
        1 => (data, b""),
        2 => (parts[0], parts[1]),
        _ => (b"", b""),
    }
}

/// Parses the Cookie request header in v0 format and places the results into tx->request_cookies.
///
/// Returns HTP_OK on success, HTP_ERROR on error
pub fn htp_parse_cookies_v0(in_tx: &mut htp_transaction::htp_tx_t) -> Result<()> {
    if let Some((_, cookie_header)) = in_tx.request_headers.get_nocase_nozero_mut("cookie") {
        let data: &[u8] = cookie_header.value.as_ref();
        // Create a new table to store cookies.
        in_tx.request_cookies = htp_table::htp_table_alloc(4);
        for cookie in data.split(|b| *b == ';' as u8) {
            if let Ok((cookie, _)) = take_ascii_whitespace()(cookie) {
                if cookie.is_empty() {
                    continue;
                }
                let (name, value) = htp_parse_single_cookie_v0(cookie);

                if !name.is_empty() {
                    unsafe {
                        (*in_tx.request_cookies)
                            .add(bstr::bstr_t::from(name), bstr::bstr_t::from(value));
                    }
                }
            }
        }
    }

    Ok(())
}
