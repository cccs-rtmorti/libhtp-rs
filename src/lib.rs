#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![feature(extern_types)]
#![feature(ptr_wrapping_offset_from)]

#[repr(C)]
#[derive(PartialEq, Debug)]
/// Status codes used by LibHTP internally.
/// cbindgen:rename-all=QualifiedScreamingSnakeCase
pub enum Status {
    /// The lowest value LibHTP will use internally.
    ERROR_RESERVED = -1000,
    /// General-purpose error code.
    ERROR = -1,
    /// No processing or work was done. This is typically used by callbacks
    /// to indicate that they were not interested in doing any work in the
    /// given context.
    DECLINED = 0,
    /// Returned by a function when its work was successfully completed.
    OK = 1,
    ///  Returned when processing a connection stream, after consuming all
    ///  provided data. The caller should call again with more data.
    DATA = 2,
    /// Returned when processing a connection stream, after encountering
    /// a situation where processing needs to continue on the alternate
    /// stream (e.g., the inbound parser needs to observe some outbound
    /// data). The data provided was not completely consumed. On the next
    /// invocation the caller should supply only the data that has not
    /// been processed already. Use htp_connp_req_data_consumed() and
    /// htp_connp_res_data_consumed() to determine how much of the most
    /// recent data chunk was consumed.
    DATA_OTHER = 3,
    /// Used by callbacks to indicate that the processing should stop. For example,
    /// returning HTP_STOP from a connection callback indicates that LibHTP should
    /// stop following that particular connection.
    STOP = 4,
    /// Same as HTP_DATA, but indicates that any non-consumed part of the
    /// data chunk should be preserved (buffered) for later.
    DATA_BUFFER = 5,
    /// The highest value LibHTP will use internally.
    STATUS_RESERVED = 1000,
}

/// Convenience macro to check for null pointers and panic if found.
///
/// Useful for those times when the API assumes the pointer is valid,
/// but nobody is checking it.
///
/// # Examples
/// ```should_panic
/// # #[macro_use] extern crate htp;
/// fn foo(data: *const u8) {
///     nullcheck!(data);
///     // Do something with *data
/// }
/// # fn main() {
///     let data = std::ptr::null();
///     foo(data);
/// # }
#[macro_export]
macro_rules! nullcheck {
    ( $( $ptr:expr ),* ) => {
        $(
        if $ptr.is_null() {
            panic!(format!("{} is NULL in {}", stringify!($ptr), line!()));
        }
        )*
    }
}

#[macro_use]
pub mod log;
pub mod bstr;
pub mod c_api;
pub mod config;
mod connection;
pub mod connection_parser;
mod content_handlers;
pub mod decompressors;
pub mod hook;
#[macro_use]
mod unicode_bestfit_map;
pub mod list;
pub mod multipart;
mod parsers;
pub mod request;
mod request_generic;
pub mod response;
mod response_generic;
pub mod table;
pub mod transaction;
pub mod urlencoded;
mod utf8_decoder;
pub mod util;
pub mod lzma {
    pub mod LzmaDec;
} // mod lzma
pub mod error;
