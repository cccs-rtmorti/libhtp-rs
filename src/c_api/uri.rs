use crate::bstr::bstr_t;
use crate::htp_util::htp_uri_t;

/// Get the scheme of a uri.
///
/// Returns the scheme for uri or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_uri_scheme(uri: *const htp_uri_t) -> *const bstr_t {
    uri.as_ref()
        .map(|uri| uri.scheme)
        .unwrap_or(std::ptr::null_mut())
}

/// Get the username of a uri.
///
/// Returns the username for uri or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_uri_username(uri: *const htp_uri_t) -> *const bstr_t {
    uri.as_ref()
        .map(|uri| uri.username)
        .unwrap_or(std::ptr::null_mut())
}

/// Get the password of a uri.
///
/// Returns the password for uri or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_uri_password(uri: *const htp_uri_t) -> *const bstr_t {
    uri.as_ref()
        .map(|uri| uri.password)
        .unwrap_or(std::ptr::null_mut())
}

/// Get the hostname of a uri.
///
/// Returns the hostname for uri or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_uri_hostname(uri: *const htp_uri_t) -> *const bstr_t {
    uri.as_ref()
        .map(|uri| uri.hostname)
        .unwrap_or(std::ptr::null_mut())
}

/// Get the port of a uri.
///
/// Returns the port for uri or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_uri_port(uri: *const htp_uri_t) -> *const bstr_t {
    uri.as_ref()
        .map(|uri| uri.port)
        .unwrap_or(std::ptr::null_mut())
}

/// Get the port_number of a uri.
///
/// Returns the port_number for uri or -1 on error.
#[no_mangle]
pub unsafe extern "C" fn htp_uri_port_number(uri: *const htp_uri_t) -> i32 {
    uri.as_ref().map(|uri| uri.port_number).unwrap_or(-1)
}

/// Get the path of a uri.
///
/// Returns the path for uri or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_uri_path(uri: *const htp_uri_t) -> *const bstr_t {
    uri.as_ref()
        .map(|uri| uri.path)
        .unwrap_or(std::ptr::null_mut())
}

/// Get the query of a uri.
///
/// Returns the query for uri or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_uri_query(uri: *const htp_uri_t) -> *const bstr_t {
    uri.as_ref()
        .map(|uri| uri.query)
        .unwrap_or(std::ptr::null_mut())
}

/// Get the fragment of a uri.
///
/// Returns the fragment for uri or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_uri_fragment(uri: *const htp_uri_t) -> *const bstr_t {
    uri.as_ref()
        .map(|uri| uri.fragment)
        .unwrap_or(std::ptr::null_mut())
}
