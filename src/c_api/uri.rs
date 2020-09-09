use crate::bstr::bstr_t;
use crate::htp_util::htp_uri_t;

/// Get the scheme of a uri.
///
/// Returns the scheme for uri or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_uri_scheme(uri: *const htp_uri_t) -> *const bstr_t {
    uri.as_ref()
        .and_then(|uri| uri.scheme.as_ref())
        .map(|scheme| scheme as *const bstr_t)
        .unwrap_or(std::ptr::null())
}

/// Get the username of a uri.
///
/// Returns the username for uri or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_uri_username(uri: *const htp_uri_t) -> *const bstr_t {
    uri.as_ref()
        .and_then(|uri| uri.username.as_ref())
        .map(|username| username as *const bstr_t)
        .unwrap_or(std::ptr::null())
}

/// Get the password of a uri.
///
/// Returns the password for uri or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_uri_password(uri: *const htp_uri_t) -> *const bstr_t {
    uri.as_ref()
        .and_then(|uri| uri.password.as_ref())
        .map(|password| password as *const bstr_t)
        .unwrap_or(std::ptr::null())
}

/// Get the hostname of a uri.
///
/// Returns the hostname for uri or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_uri_hostname(uri: *const htp_uri_t) -> *const bstr_t {
    uri.as_ref()
        .and_then(|uri| uri.hostname.as_ref())
        .map(|hostname| hostname as *const bstr_t)
        .unwrap_or(std::ptr::null())
}

/// Get the port of a uri.
///
/// Returns the port for uri or NULL on error.
#[no_mangle]
#[no_mangle]
pub unsafe extern "C" fn htp_uri_port(uri: *const htp_uri_t) -> *const bstr_t {
    uri.as_ref()
        .and_then(|uri| uri.port.as_ref())
        .map(|port| port as *const bstr_t)
        .unwrap_or(std::ptr::null())
}

/// Get the port_number of a uri.
///
/// Returns the port_number for uri or -1 on error.
#[no_mangle]
pub unsafe extern "C" fn htp_uri_port_number(uri: *const htp_uri_t) -> i32 {
    uri.as_ref()
        .and_then(|uri| uri.port_number)
        .map(|port| port as i32)
        .unwrap_or(-1)
}

/// Get the path of a uri.
///
/// Returns the path for uri or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_uri_path(uri: *const htp_uri_t) -> *const bstr_t {
    uri.as_ref()
        .and_then(|uri| uri.path.as_ref())
        .map(|path| path as *const bstr_t)
        .unwrap_or(std::ptr::null())
}

/// Get the query of a uri.
///
/// Returns the query for uri or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_uri_query(uri: *const htp_uri_t) -> *const bstr_t {
    uri.as_ref()
        .and_then(|uri| uri.query.as_ref())
        .map(|query| query as *const bstr_t)
        .unwrap_or(std::ptr::null())
}

/// Get the fragment of a uri.
///
/// Returns the fragment for uri or NULL on error.
#[no_mangle]
pub unsafe extern "C" fn htp_uri_fragment(uri: *const htp_uri_t) -> *const bstr_t {
    uri.as_ref()
        .and_then(|uri| uri.fragment.as_ref())
        .map(|fragment| fragment as *const bstr_t)
        .unwrap_or(std::ptr::null())
}
