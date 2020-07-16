#[macro_export]
macro_rules! cstr {
    ( $x:expr ) => {{
        CString::new($x).unwrap().as_ptr()
    }};
}

/// Compares a transaction's header value to an expected value.
///
/// The `attr` argument is meant to be either `request_headers` or `response_headers`.
///
/// Example usage:
/// assert_header_eq!(tx, request_headers, "host", ""www.example.com");
#[allow(unused_macros)]
#[macro_export]
macro_rules! assert_header_eq {
    ($tx:expr, $attr:ident, $key:expr, $val:expr) => {{
        let header = &(*$tx).$attr
            .get_nocase_nozero($key)
            .expect(format!(
                "expected header '{}' to exist at {}:{}:{}",
                $key,
                file!(),
                line!(),
                column!()
            ).as_ref())
            .1;
        assert_eq!(*header.value, $val);
    }};
    ($tx:expr, $attr:ident, $key:expr, $val:expr,) => {{
        assert_header_eq!($tx, $attr, $key, $val);
    }};
    ($tx:expr, $attr:ident, $key:expr, $val:expr, $($arg:tt)+) => {{
        let header = (*(*$tx).$attr)
            .get_nocase_nozero($key)
            .expect(format!(
                "expected header '{}' to exist at {}:{}:{}",
                $key,
                file!(),
                line!(),
                column!()
            ).as_ref())
            .1
            .as_ref()
            .expect(format!(
                "expected header '{}' to exist at {}:{}:{}",
                $key,
                file!(),
                line!(),
                column!()
            ).as_ref());
        assert_eq!(*header.value, $val, $($arg)*);
    }};
}

/// Compares a transaction's request header value to an expected value.
///
/// Example usage:
/// assert_request_header_eq!(tx, "host", ""www.example.com");
#[macro_export]
macro_rules! assert_request_header_eq {
    ($tx:expr, $key:expr, $val:expr) => {{
        assert_header_eq!($tx, request_headers, $key, $val);
    }};
    ($tx:expr, $key:expr, $val:expr,) => {{
        assert_header_eq!($tx, request_headers, $key, $val);
    }};
    ($tx:expr, $key:expr, $val:expr, $($arg:tt)+) => {{
        assert_header_eq!($tx, request_headers, $val, $($arg)*);
    }};
}

/// Compares a transaction's response header value to an expected value.
///
/// Example usage:
/// assert_response_header_eq!(tx, "content-encoding", ""gzip");
#[macro_export]
macro_rules! assert_response_header_eq {
    ($tx:expr, $key:expr, $val:expr) => {{
        assert_header_eq!($tx, response_headers, $key, $val);
    }};
    ($tx:expr, $key:expr, $val:expr,) => {{
        assert_header_eq!($tx, response_headers, $key, $val);
    }};
    ($tx:expr, $key:expr, $val:expr, $($arg:tt)+) => {{
        assert_header_eq!($tx, response_headers, $val, $($arg)*);
    }};
}

/// Asserts that a transaction's response contains a flag.
///
/// Example usage:
/// assert_response_header_flag_contains!(tx, "Content-Length", Flags::HTP_FIELD_REPEATED);
#[macro_export]
macro_rules! assert_response_header_flag_contains {
    ($tx:expr, $key:expr, $val:expr) => {{
        let header = &(*$tx).response_headers
            .get_nocase_nozero($key)
            .expect(format!(
                "expected header '{}' to exist at {}:{}:{}",
                $key,
                file!(),
                line!(),
                column!()
            ).as_ref())
            .1;
        assert!(header.flags.contains($val));
        }};
    ($tx:expr, $key:expr, $val:expr,) => {{
        assert_response_header_flag_contains!($tx, response_headers, $key, $val);
    }};
    ($tx:expr, $key:expr, $val:expr, $($arg:tt)+) => {{
        let header = (*(*$tx).response_headers)
            .get_nocase_nozero($key)
            .expect(format!(
                "expected header '{}' to exist at {}:{}:{}",
                $key,
                file!(),
                line!(),
                column!()
            ).as_ref())
            .1
            .as_ref()
            .expect(format!(
                "expected header '{}' to exist at {}:{}:{}",
                $key,
                file!(),
                line!(),
                column!()
            ).as_ref());
        assert_eq!(*header.value, $val, $($arg)*);
        assert!((*header).flags.contains($val), $($arg)*);
    }};
}
