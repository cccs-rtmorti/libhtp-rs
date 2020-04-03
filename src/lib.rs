#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(ptr_wrapping_offset_from)]
#![feature(register_tool)]
#![register_tool(c2rust)]

pub mod bstr;
pub mod bstr_builder;
pub mod htp_base64;
pub mod htp_config;
pub mod htp_connection;
pub mod htp_connection_parser;
pub mod htp_content_handlers;
pub mod htp_cookies;
pub mod htp_decompressors;
pub mod htp_hooks;
pub mod htp_list;
pub mod htp_multipart;
pub mod htp_parsers;
pub mod htp_php;
pub mod htp_request;
pub mod htp_request_apache_2_2;
pub mod htp_request_generic;
pub mod htp_response;
pub mod htp_response_generic;
pub mod htp_table;
pub mod htp_transaction;
pub mod htp_transcoder;
pub mod htp_urlencoded;
pub mod htp_utf8_decoder;
pub mod htp_util;
pub mod lzma {
    pub mod LzFind;
    pub mod LzmaDec;
} // mod lzma
pub mod strlcat;
pub mod strlcpy;
