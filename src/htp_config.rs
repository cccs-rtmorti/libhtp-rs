use ::libc;
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    /* **************************************************************************
 * Copyright (c) 2009-2010 Open Information Security Foundation
 * Copyright (c) 2010-2013 Qualys, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.

 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.

 * - Neither the name of the Qualys, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 ***************************************************************************/
    /* *
 * @file
 * @author Ivan Ristic <ivanr@webkreator.com>
 */
    // 1048576 is 1 Mbyte
    //deflate max ratio is about 1000
    // Parser states, in the order in which they are
// used as a single transaction is processed.
    // Parsing functions
    #[no_mangle]
    fn htp_process_response_header_generic(connp: *mut htp_connp_t,
                                           data: *mut libc::c_uchar,
                                           len: size_t) -> htp_status_t;
    #[no_mangle]
    fn htp_parse_response_line_generic(connp: *mut htp_connp_t)
     -> htp_status_t;
    #[no_mangle]
    fn htp_process_request_header_generic(_: *mut htp_connp_t,
                                          data: *mut libc::c_uchar,
                                          len: size_t) -> htp_status_t;
    #[no_mangle]
    fn htp_parse_request_line_generic(connp: *mut htp_connp_t)
     -> htp_status_t;
    #[no_mangle]
    fn htp_process_request_header_apache_2_2(_: *mut htp_connp_t,
                                             data: *mut libc::c_uchar,
                                             len: size_t) -> htp_status_t;
    #[no_mangle]
    fn htp_parse_request_line_apache_2_2(connp: *mut htp_connp_t)
     -> htp_status_t;
    /* *
 * Creates a copy of the provided hook. The hook is allowed to be NULL,
 * in which case this function simply returns a NULL.
 *
 * @param[in] hook
 * @return A copy of the hook, or NULL (if the provided hook was NULL
 *         or, if it wasn't, if there was a memory allocation problem while
 *         constructing a copy).
 */
    /* *
 * Creates a new hook.
 *
 * @return New htp_hook_t structure on success, NULL on failure.
 */
    /* *
 * Destroys an existing hook. It is all right to send a NULL
 * to this method because it will simply return straight away.
 *
 * @param[in] hook
 */
    #[no_mangle]
    fn htp_hook_destroy(hook: *mut htp_hook_t);
    #[no_mangle]
    fn htp_hook_copy(hook: *const htp_hook_t) -> *mut htp_hook_t;
    /* *
 * Registers a new callback with the hook.
 *
 * @param[in] hook
 * @param[in] callback_fn
 * @return HTP_OK on success, HTP_ERROR on memory allocation error.
 */
    #[no_mangle]
    fn htp_hook_register(hook: *mut *mut htp_hook_t,
                         callback_fn: htp_callback_fn_t) -> htp_status_t;
    // Private transaction functions
    // Utility functions
    #[no_mangle]
    fn htp_ch_multipart_callback_request_headers(tx: *mut htp_tx_t)
     -> htp_status_t;
    #[no_mangle]
    fn htp_ch_urlencoded_callback_request_headers(tx: *mut htp_tx_t)
     -> htp_status_t;
    #[no_mangle]
    fn htp_ch_urlencoded_callback_request_line(tx: *mut htp_tx_t)
     -> htp_status_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
/* **************************************************************************
 * Copyright (c) 2009-2010 Open Information Security Foundation
 * Copyright (c) 2010-2013 Qualys, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.

 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.

 * - Neither the name of the Qualys, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 ***************************************************************************/
/* *
 * @file
 * @author Ivan Ristic <ivanr@webkreator.com>
 */
pub type htp_status_t = libc::c_int;
/* **************************************************************************
 * Copyright (c) 2009-2010 Open Information Security Foundation
 * Copyright (c) 2010-2013 Qualys, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.

 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.

 * - Neither the name of the Qualys, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 ***************************************************************************/
/* *
 * @file
 * @author Ivan Ristic <ivanr@webkreator.com>
 */
// Path-specific decoding options.
/* * Convert backslash characters to slashes. */
/* * Convert to lowercase. */
/* * Compress slash characters. */
/* * Should we URL-decode encoded path segment separators? */
/* * Should we decode '+' characters to spaces? */
/* * Reaction to encoded path separators. */
// Special characters options.
/* * Controls how raw NUL bytes are handled. */
/* * Determines server response to a raw NUL byte in the path. */
/* * Reaction to control characters. */
// URL encoding options.
/* * Should we decode %u-encoded characters? */
/* * Reaction to %u encoding. */
/* * Handling of invalid URL encodings. */
/* * Reaction to invalid URL encoding. */
/* * Controls how encoded NUL bytes are handled. */
/* * How are we expected to react to an encoded NUL byte? */
// UTF-8 options.
/* * Controls how invalid UTF-8 characters are handled. */
/* * Convert UTF-8 characters into bytes using best-fit mapping. */
// Best-fit mapping options.
/* * The best-fit map to use to decode %u-encoded characters. */
/* * The replacement byte used when there is no best-fit mapping. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_cfg_t {
    pub field_limit_hard: size_t,
    pub field_limit_soft: size_t,
    pub log_level: htp_log_level_t,
    pub tx_auto_destroy: libc::c_int,
    pub server_personality: htp_server_personality_t,
    pub parse_request_line: Option<unsafe extern "C" fn(_: *mut htp_connp_t)
                                       -> libc::c_int>,
    pub parse_response_line: Option<unsafe extern "C" fn(_: *mut htp_connp_t)
                                        -> libc::c_int>,
    pub process_request_header: Option<unsafe extern "C" fn(_:
                                                                *mut htp_connp_t,
                                                            _:
                                                                *mut libc::c_uchar,
                                                            _: size_t)
                                           -> libc::c_int>,
    pub process_response_header: Option<unsafe extern "C" fn(_:
                                                                 *mut htp_connp_t,
                                                             _:
                                                                 *mut libc::c_uchar,
                                                             _: size_t)
                                            -> libc::c_int>,
    pub parameter_processor: Option<unsafe extern "C" fn(_: *mut htp_param_t)
                                        -> libc::c_int>,
    pub decoder_cfgs: [htp_decoder_cfg_t; 3],
    pub generate_request_uri_normalized: libc::c_int,
    pub response_decompression_enabled: libc::c_int,
    pub request_encoding: *mut libc::c_char,
    pub internal_encoding: *mut libc::c_char,
    pub parse_request_cookies: libc::c_int,
    pub parse_request_auth: libc::c_int,
    pub extract_request_files: libc::c_int,
    pub extract_request_files_limit: libc::c_int,
    pub tmpdir: *mut libc::c_char,
    pub hook_request_start: *mut htp_hook_t,
    pub hook_request_line: *mut htp_hook_t,
    pub hook_request_uri_normalize: *mut htp_hook_t,
    pub hook_request_header_data: *mut htp_hook_t,
    pub hook_request_headers: *mut htp_hook_t,
    pub hook_request_body_data: *mut htp_hook_t,
    pub hook_request_file_data: *mut htp_hook_t,
    pub hook_request_trailer_data: *mut htp_hook_t,
    pub hook_request_trailer: *mut htp_hook_t,
    pub hook_request_complete: *mut htp_hook_t,
    pub hook_response_start: *mut htp_hook_t,
    pub hook_response_line: *mut htp_hook_t,
    pub hook_response_header_data: *mut htp_hook_t,
    pub hook_response_headers: *mut htp_hook_t,
    pub hook_response_body_data: *mut htp_hook_t,
    pub hook_response_trailer_data: *mut htp_hook_t,
    pub hook_response_trailer: *mut htp_hook_t,
    pub hook_response_complete: *mut htp_hook_t,
    pub hook_transaction_complete: *mut htp_hook_t,
    pub hook_log: *mut htp_hook_t,
    pub user_data: *mut libc::c_void,
    pub requestline_leading_whitespace_unwanted: htp_unwanted_t,
    pub response_decompression_layer_limit: libc::c_int,
    pub lzma_memlimit: size_t,
    pub compression_bomb_limit: int32_t,
}
/* **************************************************************************
 * Copyright (c) 2009-2010 Open Information Security Foundation
 * Copyright (c) 2010-2013 Qualys, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.

 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.

 * - Neither the name of the Qualys, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 ***************************************************************************/
/* *
 * @file
 * @author Ivan Ristic <ivanr@webkreator.com>
 */
/* *
 * Decoder contexts.
 */
/* * Default settings. Settings applied to this context are propagated to all other contexts. */
/* * Urlencoded decoder settings. */
/* * URL path decoder settings. */
/* *
 * Enumerates the possible server personalities.
 */
/* *
     * Minimal personality that performs at little work as possible. All optional
     * features are disabled. This personality is a good starting point for customization.
     */
/* * A generic personality that aims to work reasonably well for all server types. */
/* * The IDS personality tries to perform as much decoding as possible. */
/* * Mimics the behavior of IIS 4.0, as shipped with Windows NT 4.0. */
/* * Mimics the behavior of IIS 5.0, as shipped with Windows 2000. */
/* * Mimics the behavior of IIS 5.1, as shipped with Windows XP Professional. */
/* * Mimics the behavior of IIS 6.0, as shipped with Windows 2003. */
/* * Mimics the behavior of IIS 7.0, as shipped with Windows 2008. */
/* Mimics the behavior of IIS 7.5, as shipped with Windows 7. */
/* Mimics the behavior of Apache 2.x. */
/* *
 * Enumerates the ways in which servers respond to malformed data.
 */
pub type htp_unwanted_t = libc::c_uint;
/* * Responds with HTTP 404 status code. */
pub const HTP_UNWANTED_404: htp_unwanted_t = 404;
/* * Responds with HTTP 400 status code. */
pub const HTP_UNWANTED_400: htp_unwanted_t = 400;
/* * Ignores problem. */
pub const HTP_UNWANTED_IGNORE: htp_unwanted_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_hook_t {
    pub callbacks: *mut htp_list_array_t,
}
/* **************************************************************************
 * Copyright (c) 2009-2010 Open Information Security Foundation
 * Copyright (c) 2010-2013 Qualys, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.

 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.

 * - Neither the name of the Qualys, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 ***************************************************************************/
/* *
 * @file
 * @author Ivan Ristic <ivanr@webkreator.com>
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_list_array_t {
    pub first: size_t,
    pub last: size_t,
    pub max_size: size_t,
    pub current_size: size_t,
    pub elements: *mut *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_decoder_cfg_t {
    pub backslash_convert_slashes: libc::c_int,
    pub convert_lowercase: libc::c_int,
    pub path_separators_compress: libc::c_int,
    pub path_separators_decode: libc::c_int,
    pub plusspace_decode: libc::c_int,
    pub path_separators_encoded_unwanted: htp_unwanted_t,
    pub nul_raw_terminates: libc::c_int,
    pub nul_raw_unwanted: htp_unwanted_t,
    pub control_chars_unwanted: htp_unwanted_t,
    pub u_encoding_decode: libc::c_int,
    pub u_encoding_unwanted: htp_unwanted_t,
    pub url_encoding_invalid_handling: htp_url_encoding_handling_t,
    pub url_encoding_invalid_unwanted: htp_unwanted_t,
    pub nul_encoded_terminates: libc::c_int,
    pub nul_encoded_unwanted: htp_unwanted_t,
    pub utf8_invalid_unwanted: htp_unwanted_t,
    pub utf8_convert_bestfit: libc::c_int,
    pub bestfit_map: *mut libc::c_uchar,
    pub bestfit_replacement_byte: libc::c_uchar,
}
/* *
 * Enumerates the possible approaches to handling invalid URL-encodings.
 */
pub type htp_url_encoding_handling_t = libc::c_uint;
/* * Decode invalid URL encodings. */
pub const HTP_URL_DECODE_PROCESS_INVALID: htp_url_encoding_handling_t = 2;
/* * Ignore invalid URL encodings, but remove the % from the data. */
pub const HTP_URL_DECODE_REMOVE_PERCENT: htp_url_encoding_handling_t = 1;
/* * Ignore invalid URL encodings and leave the % in the data. */
pub const HTP_URL_DECODE_PRESERVE_PERCENT: htp_url_encoding_handling_t = 0;
/* **************************************************************************
 * Copyright (c) 2009-2010 Open Information Security Foundation
 * Copyright (c) 2010-2013 Qualys, Inc.
 * All rights reserved.
 * 
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 * 
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.

 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.

 * - Neither the name of the Qualys, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 * 
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 ***************************************************************************/
/* *
 * @file
 * @author Ivan Ristic <ivanr@webkreator.com>
 */
/* *
 * Represents a single TCP connection.
 */
/* * Client IP address. */
/* * Client port. */
/* * Server IP address. */
/* * Server port. */
/* *
     * Transactions carried out on this connection. The list may contain
     * NULL elements when some of the transactions are deleted (and then
     * removed from a connection by calling htp_conn_remove_tx().
     */
/* * Log messages associated with this connection. */
/* * Parsing flags: HTP_CONN_PIPELINED. */
/* * When was this connection opened? Can be NULL. */
/* * When was this connection closed? Can be NULL. */
/* * Inbound data counter. */
/* * Outbound data counter. */
/* *
 * Used to represent files that are seen during the processing of HTTP traffic. Most
 * commonly this refers to files seen in multipart/form-data payloads. In addition, PUT
 * request bodies can be treated as files.
 */
/* * Where did this file come from? Possible values: HTP_FILE_MULTIPART and HTP_FILE_PUT. */
/* * File name, as provided (e.g., in the Content-Disposition multipart part header. */
/* * File length. */
/* * The unique filename in which this file is stored on the filesystem, when applicable.*/
/* * The file descriptor used for external storage, or -1 if unused. */
/* *
 * Represents a chunk of file data.
 */
/* * File information. */
/* * Pointer to the data buffer. */
/* * Buffer length. */
/* *
 * Represents a single log entry.
 */
/* * The connection parser associated with this log message. */
/* * The transaction associated with this log message, if any. */
/* * Log message. */
/* * Message level. */
/* * Message code. */
/* * File in which the code that emitted the message resides. */
/* * Line number on which the code that emitted the message resides. */
/* *
 * Represents a single request or response header.
 */
/* * Header name. */
/* * Header value. */
/* * Parsing flags; a combination of: HTP_FIELD_INVALID, HTP_FIELD_FOLDED, HTP_FIELD_REPEATED. */
/* *
 * Represents a single request parameter.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_param_t {
    pub name: *mut bstr,
    pub value: *mut bstr,
    pub source: htp_data_source_t,
    pub parser_id: htp_parser_id_t,
    pub parser_data: *mut libc::c_void,
}
// Various flag bits. Even though we have a flag field in several places
// (header, transaction, connection), these fields are all in the same namespace
// because we may want to set the same flag in several locations. For example, we
// may set HTP_FIELD_FOLDED on the actual folded header, but also on the transaction
// that contains the header. Both uses are useful.
// Connection flags are 8 bits wide.
// All other flags are 64 bits wide.
/* At least one valid UTF-8 character and no invalid ones. */
/* Range U+FF00 - U+FFEF detected. */
/* Host in the URI. */
/* Host in the Host header. */
/* Range U+FF00 - U+FFEF detected. */
// Logging-related constants.
/* *
 * Enumerates all log levels.
 */
/* *
 * HTTP methods.
 */
/* *
     * Used by default, until the method is determined (e.g., before
     * the request line is processed.
     */
// A collection of unique parser IDs.
pub type htp_parser_id_t = libc::c_uint;
/* * multipart/form-data parser. */
pub const HTP_PARSER_MULTIPART: htp_parser_id_t = 1;
/* * application/x-www-form-urlencoded parser. */
pub const HTP_PARSER_URLENCODED: htp_parser_id_t = 0;
// Protocol version constants; an enum cannot be
// used here because we allow any properly-formatted protocol
// version (e.g., 1.3), even those that do not actually exist.
// A collection of possible data sources.
pub type htp_data_source_t = libc::c_uint;
/* * Transported in the request body. */
pub const HTP_SOURCE_BODY: htp_data_source_t = 3;
/* * Cookies. */
pub const HTP_SOURCE_COOKIE: htp_data_source_t = 2;
/* * Transported in the query string. */
pub const HTP_SOURCE_QUERY_STRING: htp_data_source_t = 1;
/* * Embedded in the URL. */
pub const HTP_SOURCE_URL: htp_data_source_t = 0;
/* **************************************************************************
 * Copyright (c) 2009-2010 Open Information Security Foundation
 * Copyright (c) 2010-2013 Qualys, Inc.
 * All rights reserved.
 * 
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 * 
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.

 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.

 * - Neither the name of the Qualys, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 * 
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 ***************************************************************************/
/* *
 * @file
 * @author Ivan Ristic <ivanr@webkreator.com>
 */
pub type bstr = bstr_t;
// Data structures
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bstr_t {
    pub len: size_t,
    pub size: size_t,
    pub realptr: *mut libc::c_uchar,
}
/* **************************************************************************
 * Copyright (c) 2009-2010 Open Information Security Foundation
 * Copyright (c) 2010-2013 Qualys, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.

 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.

 * - Neither the name of the Qualys, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 ***************************************************************************/
/* *
 * @file
 * @author Ivan Ristic <ivanr@webkreator.com>
 */
/* *
 * Connection parser structure.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_connp_t {
    pub cfg: *mut htp_cfg_t,
    pub conn: *mut htp_conn_t,
    pub user_data: *const libc::c_void,
    pub last_error: *mut htp_log_t,
    pub in_status: htp_stream_state_t,
    pub out_status: htp_stream_state_t,
    pub out_data_other_at_tx_end: libc::c_uint,
    pub in_timestamp: htp_time_t,
    pub in_current_data: *mut libc::c_uchar,
    pub in_current_len: int64_t,
    pub in_current_read_offset: int64_t,
    pub in_current_consume_offset: int64_t,
    pub in_current_receiver_offset: int64_t,
    pub in_chunk_count: size_t,
    pub in_chunk_request_index: size_t,
    pub in_stream_offset: int64_t,
    pub in_next_byte: libc::c_int,
    pub in_buf: *mut libc::c_uchar,
    pub in_buf_size: size_t,
    pub in_header: *mut bstr,
    pub in_tx: *mut htp_tx_t,
    pub in_content_length: int64_t,
    pub in_body_data_left: int64_t,
    pub in_chunked_length: int64_t,
    pub in_state: Option<unsafe extern "C" fn(_: *mut htp_connp_t)
                             -> libc::c_int>,
    pub in_state_previous: Option<unsafe extern "C" fn(_: *mut htp_connp_t)
                                      -> libc::c_int>,
    pub in_data_receiver_hook: *mut htp_hook_t,
    pub out_next_tx_index: size_t,
    pub out_timestamp: htp_time_t,
    pub out_current_data: *mut libc::c_uchar,
    pub out_current_len: int64_t,
    pub out_current_read_offset: int64_t,
    pub out_current_consume_offset: int64_t,
    pub out_current_receiver_offset: int64_t,
    pub out_stream_offset: int64_t,
    pub out_next_byte: libc::c_int,
    pub out_buf: *mut libc::c_uchar,
    pub out_buf_size: size_t,
    pub out_header: *mut bstr,
    pub out_tx: *mut htp_tx_t,
    pub out_content_length: int64_t,
    pub out_body_data_left: int64_t,
    pub out_chunked_length: int64_t,
    pub out_state: Option<unsafe extern "C" fn(_: *mut htp_connp_t)
                              -> libc::c_int>,
    pub out_state_previous: Option<unsafe extern "C" fn(_: *mut htp_connp_t)
                                       -> libc::c_int>,
    pub out_data_receiver_hook: *mut htp_hook_t,
    pub out_decompressor: *mut htp_decompressor_t,
    pub put_file: *mut htp_file_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_file_t {
    pub source: htp_file_source_t,
    pub filename: *mut bstr,
    pub len: int64_t,
    pub tmpname: *mut libc::c_char,
    pub fd: libc::c_int,
}
pub type htp_file_source_t = libc::c_uint;
pub const HTP_FILE_PUT: htp_file_source_t = 2;
pub const HTP_FILE_MULTIPART: htp_file_source_t = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_decompressor_t {
    pub decompress: Option<unsafe extern "C" fn(_: *mut htp_decompressor_t,
                                                _: *mut htp_tx_data_t)
                               -> htp_status_t>,
    pub callback: Option<unsafe extern "C" fn(_: *mut htp_tx_data_t)
                             -> htp_status_t>,
    pub destroy: Option<unsafe extern "C" fn(_: *mut htp_decompressor_t)
                            -> ()>,
    pub next: *mut htp_decompressor_t,
}
/* *
 * Represents a single HTTP transaction, which is a combination of a request and a response.
 */
/* * The connection parser associated with this transaction. */
/* * The connection to which this transaction belongs. */
/* * The configuration structure associated with this transaction. */
/* *
     * Is the configuration structure shared with other transactions or connections? If
     * this field is set to HTP_CONFIG_PRIVATE, the transaction owns the configuration.
     */
/* * The user data associated with this transaction. */
// Request fields
/* * Contains a count of how many empty lines were skipped before the request line. */
/* * The first line of this request. */
/* * Request method. */
/* * Request method, as number. Available only if we were able to recognize the request method. */
/* *
     * Request URI, raw, as given to us on the request line. This field can take different forms,
     * for example authority for CONNECT methods, absolute URIs for proxy requests, and the query
     * string when one is provided. Use htp_tx_t::parsed_uri if you need to access to specific
     * URI elements. Can be NULL if the request line contains only a request method (which is
     * an extreme case of HTTP/0.9, but passes in practice.
     */
/* * Request protocol, as text. Can be NULL if no protocol was specified. */
/* *
     * Protocol version as a number. Multiply the high version number by 100, then add the low
     * version number. You should prefer to work the pre-defined HTP_PROTOCOL_* constants.
     */
/* *
     * Is this request using HTTP/0.9? We need a separate field for this purpose because
     * the protocol version alone is not sufficient to determine if HTTP/0.9 is used. For
     * example, if you submit "GET / HTTP/0.9" to Apache, it will not treat the request
     * as HTTP/0.9.
     */
/* *
     * This structure holds the individual components parsed out of the request URI, with
     * appropriate normalization and transformation applied, per configuration. No information
     * is added. In extreme cases when no URI is provided on the request line, all fields
     * will be NULL. (Well, except for port_number, which will be -1.) To inspect raw data, use
     * htp_tx_t::request_uri or htp_tx_t::parsed_uri_raw.
     */
/* *
     * This structure holds the individual components parsed out of the request URI, but
     * without any modification. The purpose of this field is to allow you to look at the data as it
     * was supplied on the request line. Fields can be NULL, depending on what data was supplied.
     * The port_number field is always -1.
     */
/* HTTP 1.1 RFC
     * 
     * 4.3 Message Body
     * 
     * The message-body (if any) of an HTTP message is used to carry the
     * entity-body associated with the request or response. The message-body
     * differs from the entity-body only when a transfer-coding has been
     * applied, as indicated by the Transfer-Encoding header field (section
     * 14.41).
     *
     *     message-body = entity-body
     *                  | <entity-body encoded as per Transfer-Encoding>
     */
/* *
     * The length of the request message-body. In most cases, this value
     * will be the same as request_entity_len. The values will be different
     * if request compression or chunking were applied. In that case,
     * request_message_len contains the length of the request body as it
     * has been seen over TCP; request_entity_len contains length after
     * de-chunking and decompression.
     */
/* *
     * The length of the request entity-body. In most cases, this value
     * will be the same as request_message_len. The values will be different
     * if request compression or chunking were applied. In that case,
     * request_message_len contains the length of the request body as it
     * has been seen over TCP; request_entity_len contains length after
     * de-chunking and decompression.
     */
/* * Parsed request headers. */
/* *
     * Request transfer coding. Can be one of HTP_CODING_UNKNOWN (body presence not
     * determined yet), HTP_CODING_IDENTITY, HTP_CODING_CHUNKED, HTP_CODING_NO_BODY,
     * and HTP_CODING_UNRECOGNIZED.
     */
/* * Request body compression. */
/* *
     * This field contain the request content type when that information is
     * available in request headers. The contents of the field will be converted
     * to lowercase and any parameters (e.g., character set information) removed.
     */
/* *
     * Contains the value specified in the Content-Length header. The value of this
     * field will be -1 from the beginning of the transaction and until request
     * headers are processed. It will stay -1 if the C-L header was not provided,
     * or if the value in it cannot be parsed.
     */
/* *
     * Transaction-specific REQUEST_BODY_DATA hook. Behaves as
     * the configuration hook with the same name.
     */
/* *
     * Transaction-specific RESPONSE_BODY_DATA hook. Behaves as
     * the configuration hook with the same name.
     */
/* *
     * Query string URLENCODED parser. Available only
     * when the query string is not NULL and not empty.
     */
/* *
     * Request body URLENCODED parser. Available only when the request body is in the
     * application/x-www-form-urlencoded format and the parser was configured to run.
     */
/* *
     * Request body MULTIPART parser. Available only when the body is in the
     * multipart/form-data format and the parser was configured to run.
     */
/* * Request parameters. */
/* * Request cookies */
/* * Authentication type used in the request. */
/* * Authentication username. */
/* * Authentication password. Available only when htp_tx_t::request_auth_type is HTP_AUTH_BASIC. */
/* *
     * Request hostname. Per the RFC, the hostname will be taken from the Host header
     * when available. If the host information is also available in the URI, it is used
     * instead of whatever might be in the Host header. Can be NULL. This field does
     * not contain port information.
     */
/* *
     * Request port number, if presented. The rules for htp_tx_t::request_host apply. Set to
     * -1 by default.
     */
// Response fields
/* * How many empty lines did we ignore before reaching the status line? */
/* * Response line. */
/* * Response protocol, as text. Can be NULL. */
/* *
     * Response protocol as number. Available only if we were able to parse the protocol version,
     * HTP_PROTOCOL_INVALID otherwise. HTP_PROTOCOL_UNKNOWN until parsing is attempted.
     */
/* *
     * Response status code, as text. Starts as NULL and can remain NULL on
     * an invalid response that does not specify status code.
     */
/* *
     * Response status code, available only if we were able to parse it, HTP_STATUS_INVALID
     * otherwise. HTP_STATUS_UNKNOWN until parsing is attempted.
     */
/* *
     * This field is set by the protocol decoder with it thinks that the
     * backend server will reject a request with a particular status code.
     */
/* * The message associated with the response status code. Can be NULL. */
/* * Have we seen the server respond with a 100 response? */
/* * Parsed response headers. Contains instances of htp_header_t. */
/* HTTP 1.1 RFC
     * 
     * 4.3 Message Body
     * 
     * The message-body (if any) of an HTTP message is used to carry the
     * entity-body associated with the request or response. The message-body
     * differs from the entity-body only when a transfer-coding has been
     * applied, as indicated by the Transfer-Encoding header field (section
     * 14.41).
     *
     *     message-body = entity-body
     *                  | <entity-body encoded as per Transfer-Encoding>
     */
/* *
     * The length of the response message-body. In most cases, this value
     * will be the same as response_entity_len. The values will be different
     * if response compression or chunking were applied. In that case,
     * response_message_len contains the length of the response body as it
     * has been seen over TCP; response_entity_len contains the length after
     * de-chunking and decompression.
     */
/* *
     * The length of the response entity-body. In most cases, this value
     * will be the same as response_message_len. The values will be different
     * if request compression or chunking were applied. In that case,
     * response_message_len contains the length of the response body as it
     * has been seen over TCP; response_entity_len contains length after
     * de-chunking and decompression.
     */
/* *
     * Contains the value specified in the Content-Length header. The value of this
     * field will be -1 from the beginning of the transaction and until response
     * headers are processed. It will stay -1 if the C-L header was not provided,
     * or if the value in it cannot be parsed.
     */
/* *
     * Response transfer coding, which indicates if there is a response body,
     * and how it is transported (e.g., as-is, or chunked).
     */
/* *
     * Response body compression, which indicates if compression is used
     * for the response body. This field is an interpretation of the information
     * available in response headers.
     */
/* *
     * Response body compression processing information, which is related to how
     * the library is going to process (or has processed) a response body. Changing
     * this field mid-processing can influence library actions. For example, setting
     * this field to HTP_COMPRESSION_NONE in a RESPONSE_HEADERS callback will prevent
     * decompression.
     */
/* *
     * This field will contain the response content type when that information
     * is available in response headers. The contents of the field will be converted
     * to lowercase and any parameters (e.g., character set information) removed.
     */
// Common fields
/* *
     * Parsing flags; a combination of: HTP_REQUEST_INVALID_T_E, HTP_INVALID_FOLDING,
     * HTP_REQUEST_SMUGGLING, HTP_MULTI_PACKET_HEAD, and HTP_FIELD_UNPARSEABLE.
     */
/* * Request progress. */
/* * Response progress. */
/* * Transaction index on the connection. */
/* * Total repetitions for headers in request. */
/* * Total repetitions for headers in response. */
/* *
 * This structure is used to pass transaction data (for example
 * request and response body buffers) to callbacks.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_tx_data_t {
    pub tx: *mut htp_tx_t,
    pub data: *const libc::c_uchar,
    pub len: size_t,
    pub is_last: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_tx_t {
    pub connp: *mut htp_connp_t,
    pub conn: *mut htp_conn_t,
    pub cfg: *mut htp_cfg_t,
    pub is_config_shared: libc::c_int,
    pub user_data: *mut libc::c_void,
    pub request_ignored_lines: libc::c_uint,
    pub request_line: *mut bstr,
    pub request_method: *mut bstr,
    pub request_method_number: htp_method_t,
    pub request_uri: *mut bstr,
    pub request_protocol: *mut bstr,
    pub request_protocol_number: libc::c_int,
    pub is_protocol_0_9: libc::c_int,
    pub parsed_uri: *mut htp_uri_t,
    pub parsed_uri_raw: *mut htp_uri_t,
    pub request_message_len: int64_t,
    pub request_entity_len: int64_t,
    pub request_headers: *mut htp_table_t,
    pub request_transfer_coding: htp_transfer_coding_t,
    pub request_content_encoding: htp_content_encoding_t,
    pub request_content_type: *mut bstr,
    pub request_content_length: int64_t,
    pub hook_request_body_data: *mut htp_hook_t,
    pub hook_response_body_data: *mut htp_hook_t,
    pub request_urlenp_query: *mut htp_urlenp_t,
    pub request_urlenp_body: *mut htp_urlenp_t,
    pub request_mpartp: *mut htp_mpartp_t,
    pub request_params: *mut htp_table_t,
    pub request_cookies: *mut htp_table_t,
    pub request_auth_type: htp_auth_type_t,
    pub request_auth_username: *mut bstr,
    pub request_auth_password: *mut bstr,
    pub request_hostname: *mut bstr,
    pub request_port_number: libc::c_int,
    pub response_ignored_lines: libc::c_uint,
    pub response_line: *mut bstr,
    pub response_protocol: *mut bstr,
    pub response_protocol_number: libc::c_int,
    pub response_status: *mut bstr,
    pub response_status_number: libc::c_int,
    pub response_status_expected_number: libc::c_int,
    pub response_message: *mut bstr,
    pub seen_100continue: libc::c_int,
    pub response_headers: *mut htp_table_t,
    pub response_message_len: int64_t,
    pub response_entity_len: int64_t,
    pub response_content_length: int64_t,
    pub response_transfer_coding: htp_transfer_coding_t,
    pub response_content_encoding: htp_content_encoding_t,
    pub response_content_encoding_processing: htp_content_encoding_t,
    pub response_content_type: *mut bstr,
    pub flags: uint64_t,
    pub request_progress: htp_tx_req_progress_t,
    pub response_progress: htp_tx_res_progress_t,
    pub index: size_t,
    pub req_header_repetitions: uint16_t,
    pub res_header_repetitions: uint16_t,
}
/* **************************************************************************
 * Copyright (c) 2009-2010 Open Information Security Foundation
 * Copyright (c) 2010-2013 Qualys, Inc.
 * All rights reserved.
 * 
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 * 
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.

 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.

 * - Neither the name of the Qualys, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 * 
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 ***************************************************************************/
/* 
 * @file
 * @author Ivan Ristic <ivanr@webkreator.com>
 */
/* *
 * Enumerate possible data handling strategies in hybrid parsing
 * mode. The two possibilities are to make copies of all data and
 * use bstr instances to wrap already available data.
 */
/* *
     * Make copies of all data. This strategy should be used when
     * the supplied buffers are transient and will go away after
     * the invoked function returns.
     */
/* *
     * Reuse buffers, without a change of ownership. We assume the
     * buffers will continue to be available until the transaction
     * is deleted by the container.
     */
/* *
 * Possible states of a progressing transaction. Internally, progress will change
 * to the next state when the processing activities associated with that state
 * begin. For example, when we start to process request line bytes, the request
 * state will change from HTP_REQUEST_NOT_STARTED to HTP_REQUEST_LINE.*
 */
pub type htp_tx_res_progress_t = libc::c_uint;
pub const HTP_RESPONSE_COMPLETE: htp_tx_res_progress_t = 5;
pub const HTP_RESPONSE_TRAILER: htp_tx_res_progress_t = 4;
pub const HTP_RESPONSE_BODY: htp_tx_res_progress_t = 3;
pub const HTP_RESPONSE_HEADERS: htp_tx_res_progress_t = 2;
pub const HTP_RESPONSE_LINE: htp_tx_res_progress_t = 1;
pub const HTP_RESPONSE_NOT_STARTED: htp_tx_res_progress_t = 0;
pub type htp_tx_req_progress_t = libc::c_uint;
pub const HTP_REQUEST_COMPLETE: htp_tx_req_progress_t = 5;
pub const HTP_REQUEST_TRAILER: htp_tx_req_progress_t = 4;
pub const HTP_REQUEST_BODY: htp_tx_req_progress_t = 3;
pub const HTP_REQUEST_HEADERS: htp_tx_req_progress_t = 2;
pub const HTP_REQUEST_LINE: htp_tx_req_progress_t = 1;
pub const HTP_REQUEST_NOT_STARTED: htp_tx_req_progress_t = 0;
pub type htp_content_encoding_t = libc::c_uint;
/* * LZMA compression. */
pub const HTP_COMPRESSION_LZMA: htp_content_encoding_t = 4;
/* * Deflate compression. */
pub const HTP_COMPRESSION_DEFLATE: htp_content_encoding_t = 3;
/* * Gzip compression. */
pub const HTP_COMPRESSION_GZIP: htp_content_encoding_t = 2;
/* * No compression. */
pub const HTP_COMPRESSION_NONE: htp_content_encoding_t = 1;
/* *
     * This is the default value, which is used until the presence
     * of content encoding is determined (e.g., before request headers
     * are seen.
     */
pub const HTP_COMPRESSION_UNKNOWN: htp_content_encoding_t = 0;
/* *
 * Enumerates the possible request and response body codings.
 */
pub type htp_transfer_coding_t = libc::c_uint;
/* * We could not recognize the encoding. */
pub const HTP_CODING_INVALID: htp_transfer_coding_t = 4;
/* * Chunked encoding. */
pub const HTP_CODING_CHUNKED: htp_transfer_coding_t = 3;
/* * Identity coding is used, which means that the body was sent as is. */
pub const HTP_CODING_IDENTITY: htp_transfer_coding_t = 2;
/* * No body. */
pub const HTP_CODING_NO_BODY: htp_transfer_coding_t = 1;
/* * Body coding not determined yet. */
pub const HTP_CODING_UNKNOWN: htp_transfer_coding_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_table_t {
    pub list: htp_list_array_t,
    pub alloc_type: htp_table_alloc_t,
}
/* **************************************************************************
 * Copyright (c) 2009-2010 Open Information Security Foundation
 * Copyright (c) 2010-2013 Qualys, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.

 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.

 * - Neither the name of the Qualys, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 ***************************************************************************/
/* *
 * @file
 * @author Ivan Ristic <ivanr@webkreator.com>
 */
pub type htp_table_alloc_t = libc::c_uint;
/* * Keys are only referenced; the caller is still responsible for freeing them after the table is destroyed. */
pub const HTP_TABLE_KEYS_REFERENCED: htp_table_alloc_t = 3;
/* * Keys are adopted and freed when the table is destroyed. */
pub const HTP_TABLE_KEYS_ADOPTED: htp_table_alloc_t = 2;
/* * Keys are copied.*/
pub const HTP_TABLE_KEYS_COPIED: htp_table_alloc_t = 1;
/* * This is the default value, used only until the first element is added. */
pub const HTP_TABLE_KEYS_ALLOC_UKNOWN: htp_table_alloc_t = 0;
/* *
 * Returned when processing a connection stream, after consuming all
 * provided data. The caller should call again with more data.
 */
/* *
 * Returned when processing a connection stream, after encountering
 * a situation where processing needs to continue on the alternate
 * stream (e.g., the inbound parser needs to observe some outbound
 * data). The data provided was not completely consumed. On the next
 * invocation the caller should supply only the data that has not
 * been processed already. Use htp_connp_req_data_consumed() and
 * htp_connp_res_data_consumed() to determine how much of the most
 * recent data chunk was consumed.
 */
/* *
 * Used by callbacks to indicate that the processing should stop. For example,
 * returning HTP_STOP from a connection callback indicates that LibHTP should
 * stop following that particular connection.
 */
/* *
 * Same as HTP_DATA, but indicates that any non-consumed part of the
 * data chunk should be preserved (buffered) for later.
 */
/* *
 * The highest htp_status_t value LibHTP will use internally.
 */
/* *
 * Enumerates the possible values for authentication type.
 */
pub type htp_auth_type_t = libc::c_uint;
/* * Unrecognized authentication method. */
pub const HTP_AUTH_UNRECOGNIZED: htp_auth_type_t = 9;
/* * HTTP Digest authentication used. */
pub const HTP_AUTH_DIGEST: htp_auth_type_t = 3;
/* * HTTP Basic authentication used. */
pub const HTP_AUTH_BASIC: htp_auth_type_t = 2;
/* * No authentication. */
pub const HTP_AUTH_NONE: htp_auth_type_t = 1;
/* *
     * This is the default value that is used before
     * the presence of authentication is determined (e.g.,
     * before request headers are seen).
     */
pub const HTP_AUTH_UNKNOWN: htp_auth_type_t = 0;
/* **************************************************************************
 * Copyright (c) 2009-2010 Open Information Security Foundation
 * Copyright (c) 2010-2013 Qualys, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.

 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.

 * - Neither the name of the Qualys, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 ***************************************************************************/
/* *
 * @file
 * @author Ivan Ristic <ivanr@webkreator.com>
 */
/* * When in line mode, the parser is handling part headers. */
/* * When in data mode, the parser is consuming part data. */
/* * Initial state, after the parser has been created but before the boundary initialized. */
/* * Processing data, waiting for a new line (which might indicate a new boundary). */
/* * Testing a potential boundary. */
/* * Checking the first byte after a boundary. */
/* * Checking the second byte after a boundary. */
/* * Consuming linear whitespace after a boundary. */
/* * Used after a CR byte is detected in STATE_BOUNDARY_EAT_LWS. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_mpartp_t {
    pub multipart: htp_multipart_t,
    pub cfg: *mut htp_cfg_t,
    pub extract_files: libc::c_int,
    pub extract_limit: libc::c_int,
    pub extract_dir: *mut libc::c_char,
    pub file_count: libc::c_int,
    pub handle_data: Option<unsafe extern "C" fn(_: *mut htp_mpartp_t,
                                                 _: *const libc::c_uchar,
                                                 _: size_t, _: libc::c_int)
                                -> libc::c_int>,
    pub handle_boundary: Option<unsafe extern "C" fn(_: *mut htp_mpartp_t)
                                    -> libc::c_int>,
    pub parser_state: htp_multipart_state_t,
    pub boundary_match_pos: size_t,
    pub current_part: *mut htp_multipart_part_t,
    pub current_part_mode: htp_part_mode_t,
    pub boundary_pieces: *mut bstr_builder_t,
    pub part_header_pieces: *mut bstr_builder_t,
    pub pending_header_line: *mut bstr,
    pub part_data_pieces: *mut bstr_builder_t,
    pub boundary_candidate_pos: size_t,
    pub cr_aside: libc::c_int,
    pub gave_up_data: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bstr_builder_t {
    pub pieces: *mut htp_list_array_t,
}
pub type htp_part_mode_t = libc::c_uint;
pub const MODE_DATA: htp_part_mode_t = 1;
pub const MODE_LINE: htp_part_mode_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_multipart_part_t {
    pub parser: *mut htp_mpartp_t,
    pub type_0: htp_multipart_type_t,
    pub len: size_t,
    pub name: *mut bstr,
    pub value: *mut bstr,
    pub content_type: *mut bstr,
    pub headers: *mut htp_table_t,
    pub file: *mut htp_file_t,
}
pub type htp_multipart_type_t = libc::c_uint;
pub const MULTIPART_PART_EPILOGUE: htp_multipart_type_t = 4;
pub const MULTIPART_PART_PREAMBLE: htp_multipart_type_t = 3;
pub const MULTIPART_PART_FILE: htp_multipart_type_t = 2;
pub const MULTIPART_PART_TEXT: htp_multipart_type_t = 1;
pub const MULTIPART_PART_UNKNOWN: htp_multipart_type_t = 0;
pub type htp_multipart_state_t = libc::c_uint;
pub const STATE_BOUNDARY_EAT_LWS_CR: htp_multipart_state_t = 6;
pub const STATE_BOUNDARY_EAT_LWS: htp_multipart_state_t = 5;
pub const STATE_BOUNDARY_IS_LAST2: htp_multipart_state_t = 4;
pub const STATE_BOUNDARY_IS_LAST1: htp_multipart_state_t = 3;
pub const STATE_BOUNDARY: htp_multipart_state_t = 2;
pub const STATE_DATA: htp_multipart_state_t = 1;
pub const STATE_INIT: htp_multipart_state_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_multipart_t {
    pub boundary: *mut libc::c_char,
    pub boundary_len: size_t,
    pub boundary_count: libc::c_int,
    pub parts: *mut htp_list_array_t,
    pub flags: uint64_t,
}
// The MIME type that triggers the parser. Must be lowercase.
/* *
 * This is the main URLENCODED parser structure. It is used to store
 * parser configuration, temporary parsing data, as well as the parameters.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_urlenp_t {
    pub tx: *mut htp_tx_t,
    pub argument_separator: libc::c_uchar,
    pub decode_url_encoding: libc::c_int,
    pub params: *mut htp_table_t,
    pub _state: libc::c_int,
    pub _complete: libc::c_int,
    pub _name: *mut bstr,
    pub _bb: *mut bstr_builder_t,
}
/* *
 * URI structure. Each of the fields provides access to a single
 * URI element. Where an element is not present in a URI, the
 * corresponding field will be set to NULL or -1, depending on the
 * field type.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_uri_t {
    pub scheme: *mut bstr,
    pub username: *mut bstr,
    pub password: *mut bstr,
    pub hostname: *mut bstr,
    pub port: *mut bstr,
    pub port_number: libc::c_int,
    pub path: *mut bstr,
    pub query: *mut bstr,
    pub fragment: *mut bstr,
}
pub type htp_method_t = libc::c_uint;
pub const HTP_M_INVALID: htp_method_t = 28;
pub const HTP_M_MERGE: htp_method_t = 27;
pub const HTP_M_BASELINE_CONTROL: htp_method_t = 26;
pub const HTP_M_MKACTIVITY: htp_method_t = 25;
pub const HTP_M_MKWORKSPACE: htp_method_t = 24;
pub const HTP_M_REPORT: htp_method_t = 23;
pub const HTP_M_LABEL: htp_method_t = 22;
pub const HTP_M_UPDATE: htp_method_t = 21;
pub const HTP_M_CHECKIN: htp_method_t = 20;
pub const HTP_M_UNCHECKOUT: htp_method_t = 19;
pub const HTP_M_CHECKOUT: htp_method_t = 18;
pub const HTP_M_VERSION_CONTROL: htp_method_t = 17;
pub const HTP_M_UNLOCK: htp_method_t = 16;
pub const HTP_M_LOCK: htp_method_t = 15;
pub const HTP_M_MOVE: htp_method_t = 14;
pub const HTP_M_COPY: htp_method_t = 13;
pub const HTP_M_MKCOL: htp_method_t = 12;
pub const HTP_M_PROPPATCH: htp_method_t = 11;
pub const HTP_M_PROPFIND: htp_method_t = 10;
pub const HTP_M_PATCH: htp_method_t = 9;
pub const HTP_M_TRACE: htp_method_t = 8;
pub const HTP_M_OPTIONS: htp_method_t = 7;
pub const HTP_M_CONNECT: htp_method_t = 6;
pub const HTP_M_DELETE: htp_method_t = 5;
pub const HTP_M_POST: htp_method_t = 4;
pub const HTP_M_PUT: htp_method_t = 3;
pub const HTP_M_GET: htp_method_t = 2;
pub const HTP_M_HEAD: htp_method_t = 1;
pub const HTP_M_UNKNOWN: htp_method_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_conn_t {
    pub client_addr: *mut libc::c_char,
    pub client_port: libc::c_int,
    pub server_addr: *mut libc::c_char,
    pub server_port: libc::c_int,
    pub transactions: *mut htp_list_array_t,
    pub messages: *mut htp_list_array_t,
    pub flags: uint8_t,
    pub open_timestamp: htp_time_t,
    pub close_timestamp: htp_time_t,
    pub in_data_counter: int64_t,
    pub out_data_counter: int64_t,
}
pub type htp_time_t = timeval;
/* *
 * Enumerates all stream states. Each connection has two streams, one
 * inbound and one outbound. Their states are tracked separately.
 */
pub type htp_stream_state_t = libc::c_uint;
pub const HTP_STREAM_DATA: htp_stream_state_t = 9;
pub const HTP_STREAM_STOP: htp_stream_state_t = 6;
pub const HTP_STREAM_DATA_OTHER: htp_stream_state_t = 5;
pub const HTP_STREAM_TUNNEL: htp_stream_state_t = 4;
pub const HTP_STREAM_ERROR: htp_stream_state_t = 3;
pub const HTP_STREAM_CLOSED: htp_stream_state_t = 2;
pub const HTP_STREAM_OPEN: htp_stream_state_t = 1;
pub const HTP_STREAM_NEW: htp_stream_state_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_log_t {
    pub connp: *mut htp_connp_t,
    pub tx: *mut htp_tx_t,
    pub msg: *const libc::c_char,
    pub level: htp_log_level_t,
    pub code: libc::c_int,
    pub file: *const libc::c_char,
    pub line: libc::c_uint,
}
pub type htp_log_level_t = libc::c_uint;
pub const HTP_LOG_DEBUG2: htp_log_level_t = 6;
pub const HTP_LOG_DEBUG: htp_log_level_t = 5;
pub const HTP_LOG_INFO: htp_log_level_t = 4;
pub const HTP_LOG_NOTICE: htp_log_level_t = 3;
pub const HTP_LOG_WARNING: htp_log_level_t = 2;
pub const HTP_LOG_ERROR: htp_log_level_t = 1;
pub const HTP_LOG_NONE: htp_log_level_t = 0;
pub type htp_server_personality_t = libc::c_uint;
pub const HTP_SERVER_APACHE_2: htp_server_personality_t = 9;
pub const HTP_SERVER_IIS_7_5: htp_server_personality_t = 8;
pub const HTP_SERVER_IIS_7_0: htp_server_personality_t = 7;
pub const HTP_SERVER_IIS_6_0: htp_server_personality_t = 6;
pub const HTP_SERVER_IIS_5_1: htp_server_personality_t = 5;
pub const HTP_SERVER_IIS_5_0: htp_server_personality_t = 4;
pub const HTP_SERVER_IIS_4_0: htp_server_personality_t = 3;
pub const HTP_SERVER_IDS: htp_server_personality_t = 2;
pub const HTP_SERVER_GENERIC: htp_server_personality_t = 1;
pub const HTP_SERVER_MINIMAL: htp_server_personality_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_file_data_t {
    pub file: *mut htp_file_t,
    pub data: *const libc::c_uchar,
    pub len: size_t,
}
pub type htp_decoder_ctx_t = libc::c_uint;
pub const HTP_DECODER_URL_PATH: htp_decoder_ctx_t = 2;
pub const HTP_DECODER_URLENCODED: htp_decoder_ctx_t = 1;
pub const HTP_DECODER_DEFAULTS: htp_decoder_ctx_t = 0;
pub type htp_callback_fn_t
    =
    Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>;
/* **************************************************************************
 * Copyright (c) 2009-2010 Open Information Security Foundation
 * Copyright (c) 2010-2013 Qualys, Inc.
 * All rights reserved.
 * 
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 * 
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.

 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.

 * - Neither the name of the Qualys, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 * 
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 ***************************************************************************/
/* *
 * @file
 * @author Ivan Ristic <ivanr@webkreator.com>
 */
/* *
 * This map is used by default for best-fit mapping from the Unicode
 * values U+0100-FFFF.
 */
static mut bestfit_1252: [libc::c_uchar; 1173] =
    [0x1 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x41 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x1 as libc::c_int as libc::c_uchar,
     0x61 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x2 as libc::c_int as libc::c_uchar,
     0x41 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x3 as libc::c_int as libc::c_uchar,
     0x61 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x4 as libc::c_int as libc::c_uchar,
     0x41 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x5 as libc::c_int as libc::c_uchar,
     0x61 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x6 as libc::c_int as libc::c_uchar,
     0x43 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x7 as libc::c_int as libc::c_uchar,
     0x63 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x8 as libc::c_int as libc::c_uchar,
     0x43 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0x9 as libc::c_int as libc::c_uchar,
     0x63 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0xa as libc::c_int as libc::c_uchar,
     0x43 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0xb as libc::c_int as libc::c_uchar,
     0x63 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0xc as libc::c_int as libc::c_uchar,
     0x43 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0xd as libc::c_int as libc::c_uchar,
     0x63 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0xe as libc::c_int as libc::c_uchar,
     0x44 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar, 0xf as libc::c_int as libc::c_uchar,
     0x64 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x11 as libc::c_int as libc::c_uchar,
     0x64 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x12 as libc::c_int as libc::c_uchar,
     0x45 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x13 as libc::c_int as libc::c_uchar,
     0x65 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x14 as libc::c_int as libc::c_uchar,
     0x45 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x15 as libc::c_int as libc::c_uchar,
     0x65 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x16 as libc::c_int as libc::c_uchar,
     0x45 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x17 as libc::c_int as libc::c_uchar,
     0x65 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x18 as libc::c_int as libc::c_uchar,
     0x45 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x19 as libc::c_int as libc::c_uchar,
     0x65 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x1a as libc::c_int as libc::c_uchar,
     0x45 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x1b as libc::c_int as libc::c_uchar,
     0x65 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x1c as libc::c_int as libc::c_uchar,
     0x47 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x1d as libc::c_int as libc::c_uchar,
     0x67 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x1e as libc::c_int as libc::c_uchar,
     0x47 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x1f as libc::c_int as libc::c_uchar,
     0x67 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x47 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x67 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x22 as libc::c_int as libc::c_uchar,
     0x47 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x23 as libc::c_int as libc::c_uchar,
     0x67 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x24 as libc::c_int as libc::c_uchar,
     0x48 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x68 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x26 as libc::c_int as libc::c_uchar,
     0x48 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x27 as libc::c_int as libc::c_uchar,
     0x68 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x28 as libc::c_int as libc::c_uchar,
     0x49 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x29 as libc::c_int as libc::c_uchar,
     0x69 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x2a as libc::c_int as libc::c_uchar,
     0x49 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x2b as libc::c_int as libc::c_uchar,
     0x69 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x2c as libc::c_int as libc::c_uchar,
     0x49 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x2d as libc::c_int as libc::c_uchar,
     0x69 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x2e as libc::c_int as libc::c_uchar,
     0x49 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x2f as libc::c_int as libc::c_uchar,
     0x69 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x30 as libc::c_int as libc::c_uchar,
     0x49 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x31 as libc::c_int as libc::c_uchar,
     0x69 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x34 as libc::c_int as libc::c_uchar,
     0x4a as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x35 as libc::c_int as libc::c_uchar,
     0x6a as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x36 as libc::c_int as libc::c_uchar,
     0x4b as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x37 as libc::c_int as libc::c_uchar,
     0x6b as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x39 as libc::c_int as libc::c_uchar,
     0x4c as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x3a as libc::c_int as libc::c_uchar,
     0x6c as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x3b as libc::c_int as libc::c_uchar,
     0x4c as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x3c as libc::c_int as libc::c_uchar,
     0x6c as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x3d as libc::c_int as libc::c_uchar,
     0x4c as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x3e as libc::c_int as libc::c_uchar,
     0x6c as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x41 as libc::c_int as libc::c_uchar,
     0x4c as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x42 as libc::c_int as libc::c_uchar,
     0x6c as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x43 as libc::c_int as libc::c_uchar,
     0x4e as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x44 as libc::c_int as libc::c_uchar,
     0x6e as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x45 as libc::c_int as libc::c_uchar,
     0x4e as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x46 as libc::c_int as libc::c_uchar,
     0x6e as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x47 as libc::c_int as libc::c_uchar,
     0x4e as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x48 as libc::c_int as libc::c_uchar,
     0x6e as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x4c as libc::c_int as libc::c_uchar,
     0x4f as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x4d as libc::c_int as libc::c_uchar,
     0x6f as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x4e as libc::c_int as libc::c_uchar,
     0x4f as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x4f as libc::c_int as libc::c_uchar,
     0x6f as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x50 as libc::c_int as libc::c_uchar,
     0x4f as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x51 as libc::c_int as libc::c_uchar,
     0x6f as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x54 as libc::c_int as libc::c_uchar,
     0x52 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x55 as libc::c_int as libc::c_uchar,
     0x72 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x56 as libc::c_int as libc::c_uchar,
     0x52 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x57 as libc::c_int as libc::c_uchar,
     0x72 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x58 as libc::c_int as libc::c_uchar,
     0x52 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x59 as libc::c_int as libc::c_uchar,
     0x72 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x5a as libc::c_int as libc::c_uchar,
     0x53 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x5b as libc::c_int as libc::c_uchar,
     0x73 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x5c as libc::c_int as libc::c_uchar,
     0x53 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x5d as libc::c_int as libc::c_uchar,
     0x73 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x5e as libc::c_int as libc::c_uchar,
     0x53 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x5f as libc::c_int as libc::c_uchar,
     0x73 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x62 as libc::c_int as libc::c_uchar,
     0x54 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x63 as libc::c_int as libc::c_uchar,
     0x74 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x64 as libc::c_int as libc::c_uchar,
     0x54 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x65 as libc::c_int as libc::c_uchar,
     0x74 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x66 as libc::c_int as libc::c_uchar,
     0x54 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x67 as libc::c_int as libc::c_uchar,
     0x74 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x68 as libc::c_int as libc::c_uchar,
     0x55 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x69 as libc::c_int as libc::c_uchar,
     0x75 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x6a as libc::c_int as libc::c_uchar,
     0x55 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x6b as libc::c_int as libc::c_uchar,
     0x75 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x6c as libc::c_int as libc::c_uchar,
     0x55 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x6d as libc::c_int as libc::c_uchar,
     0x75 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x6e as libc::c_int as libc::c_uchar,
     0x55 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x6f as libc::c_int as libc::c_uchar,
     0x75 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x70 as libc::c_int as libc::c_uchar,
     0x55 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x71 as libc::c_int as libc::c_uchar,
     0x75 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x72 as libc::c_int as libc::c_uchar,
     0x55 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x73 as libc::c_int as libc::c_uchar,
     0x75 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x74 as libc::c_int as libc::c_uchar,
     0x57 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x75 as libc::c_int as libc::c_uchar,
     0x77 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x76 as libc::c_int as libc::c_uchar,
     0x59 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x77 as libc::c_int as libc::c_uchar,
     0x79 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x79 as libc::c_int as libc::c_uchar,
     0x5a as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x7b as libc::c_int as libc::c_uchar,
     0x5a as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x7c as libc::c_int as libc::c_uchar,
     0x7a as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x80 as libc::c_int as libc::c_uchar,
     0x62 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x97 as libc::c_int as libc::c_uchar,
     0x49 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x9a as libc::c_int as libc::c_uchar,
     0x6c as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x9f as libc::c_int as libc::c_uchar,
     0x4f as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xa0 as libc::c_int as libc::c_uchar,
     0x4f as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xa1 as libc::c_int as libc::c_uchar,
     0x6f as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xab as libc::c_int as libc::c_uchar,
     0x74 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xae as libc::c_int as libc::c_uchar,
     0x54 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xaf as libc::c_int as libc::c_uchar,
     0x55 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xb0 as libc::c_int as libc::c_uchar,
     0x75 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xb6 as libc::c_int as libc::c_uchar,
     0x7a as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xc0 as libc::c_int as libc::c_uchar,
     0x7c as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xc3 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xcd as libc::c_int as libc::c_uchar,
     0x41 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xce as libc::c_int as libc::c_uchar,
     0x61 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xcf as libc::c_int as libc::c_uchar,
     0x49 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xd0 as libc::c_int as libc::c_uchar,
     0x69 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xd1 as libc::c_int as libc::c_uchar,
     0x4f as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xd2 as libc::c_int as libc::c_uchar,
     0x6f as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xd3 as libc::c_int as libc::c_uchar,
     0x55 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xd4 as libc::c_int as libc::c_uchar,
     0x75 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xd5 as libc::c_int as libc::c_uchar,
     0x55 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xd6 as libc::c_int as libc::c_uchar,
     0x75 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xd7 as libc::c_int as libc::c_uchar,
     0x55 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xd8 as libc::c_int as libc::c_uchar,
     0x75 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xd9 as libc::c_int as libc::c_uchar,
     0x55 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xda as libc::c_int as libc::c_uchar,
     0x75 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xdb as libc::c_int as libc::c_uchar,
     0x55 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xdc as libc::c_int as libc::c_uchar,
     0x75 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xde as libc::c_int as libc::c_uchar,
     0x41 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xdf as libc::c_int as libc::c_uchar,
     0x61 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xe4 as libc::c_int as libc::c_uchar,
     0x47 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xe5 as libc::c_int as libc::c_uchar,
     0x67 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xe6 as libc::c_int as libc::c_uchar,
     0x47 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xe7 as libc::c_int as libc::c_uchar,
     0x67 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xe8 as libc::c_int as libc::c_uchar,
     0x4b as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xe9 as libc::c_int as libc::c_uchar,
     0x6b as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xea as libc::c_int as libc::c_uchar,
     0x4f as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xeb as libc::c_int as libc::c_uchar,
     0x6f as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xec as libc::c_int as libc::c_uchar,
     0x4f as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xed as libc::c_int as libc::c_uchar,
     0x6f as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0xf0 as libc::c_int as libc::c_uchar,
     0x6a as libc::c_int as libc::c_uchar,
     0x2 as libc::c_int as libc::c_uchar,
     0x61 as libc::c_int as libc::c_uchar,
     0x67 as libc::c_int as libc::c_uchar,
     0x2 as libc::c_int as libc::c_uchar,
     0xb9 as libc::c_int as libc::c_uchar,
     0x27 as libc::c_int as libc::c_uchar,
     0x2 as libc::c_int as libc::c_uchar,
     0xba as libc::c_int as libc::c_uchar,
     0x22 as libc::c_int as libc::c_uchar,
     0x2 as libc::c_int as libc::c_uchar,
     0xbc as libc::c_int as libc::c_uchar,
     0x27 as libc::c_int as libc::c_uchar,
     0x2 as libc::c_int as libc::c_uchar,
     0xc4 as libc::c_int as libc::c_uchar,
     0x5e as libc::c_int as libc::c_uchar,
     0x2 as libc::c_int as libc::c_uchar,
     0xc8 as libc::c_int as libc::c_uchar,
     0x27 as libc::c_int as libc::c_uchar,
     0x2 as libc::c_int as libc::c_uchar,
     0xcb as libc::c_int as libc::c_uchar,
     0x60 as libc::c_int as libc::c_uchar,
     0x2 as libc::c_int as libc::c_uchar,
     0xcd as libc::c_int as libc::c_uchar,
     0x5f as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x60 as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar, 0x2 as libc::c_int as libc::c_uchar,
     0x5e as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar, 0x3 as libc::c_int as libc::c_uchar,
     0x7e as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar, 0xe as libc::c_int as libc::c_uchar,
     0x22 as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar,
     0x31 as libc::c_int as libc::c_uchar,
     0x5f as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar,
     0x32 as libc::c_int as libc::c_uchar,
     0x5f as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar,
     0x7e as libc::c_int as libc::c_uchar,
     0x3b as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar,
     0x93 as libc::c_int as libc::c_uchar,
     0x47 as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar,
     0x98 as libc::c_int as libc::c_uchar,
     0x54 as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar,
     0xa3 as libc::c_int as libc::c_uchar,
     0x53 as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar,
     0xa6 as libc::c_int as libc::c_uchar,
     0x46 as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar,
     0xa9 as libc::c_int as libc::c_uchar,
     0x4f as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar,
     0xb1 as libc::c_int as libc::c_uchar,
     0x61 as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar,
     0xb4 as libc::c_int as libc::c_uchar,
     0x64 as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar,
     0xb5 as libc::c_int as libc::c_uchar,
     0x65 as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar,
     0xc0 as libc::c_int as libc::c_uchar,
     0x70 as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar,
     0xc3 as libc::c_int as libc::c_uchar,
     0x73 as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar,
     0xc4 as libc::c_int as libc::c_uchar,
     0x74 as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar,
     0xc6 as libc::c_int as libc::c_uchar,
     0x66 as libc::c_int as libc::c_uchar,
     0x4 as libc::c_int as libc::c_uchar,
     0xbb as libc::c_int as libc::c_uchar,
     0x68 as libc::c_int as libc::c_uchar,
     0x5 as libc::c_int as libc::c_uchar,
     0x89 as libc::c_int as libc::c_uchar,
     0x3a as libc::c_int as libc::c_uchar,
     0x6 as libc::c_int as libc::c_uchar,
     0x6a as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x2 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x4 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x5 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x6 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x10 as libc::c_int as libc::c_uchar,
     0x2d as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x11 as libc::c_int as libc::c_uchar,
     0x2d as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x17 as libc::c_int as libc::c_uchar,
     0x3d as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x32 as libc::c_int as libc::c_uchar,
     0x27 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x35 as libc::c_int as libc::c_uchar,
     0x60 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x44 as libc::c_int as libc::c_uchar,
     0x2f as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x74 as libc::c_int as libc::c_uchar,
     0x34 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x75 as libc::c_int as libc::c_uchar,
     0x35 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x76 as libc::c_int as libc::c_uchar,
     0x36 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x77 as libc::c_int as libc::c_uchar,
     0x37 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x78 as libc::c_int as libc::c_uchar,
     0x38 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x7f as libc::c_int as libc::c_uchar,
     0x6e as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x80 as libc::c_int as libc::c_uchar,
     0x30 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x81 as libc::c_int as libc::c_uchar,
     0x31 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x82 as libc::c_int as libc::c_uchar,
     0x32 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x83 as libc::c_int as libc::c_uchar,
     0x33 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x84 as libc::c_int as libc::c_uchar,
     0x34 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x85 as libc::c_int as libc::c_uchar,
     0x35 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x86 as libc::c_int as libc::c_uchar,
     0x36 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x87 as libc::c_int as libc::c_uchar,
     0x37 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x88 as libc::c_int as libc::c_uchar,
     0x38 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x89 as libc::c_int as libc::c_uchar,
     0x39 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0xa7 as libc::c_int as libc::c_uchar,
     0x50 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x2 as libc::c_int as libc::c_uchar,
     0x43 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x7 as libc::c_int as libc::c_uchar,
     0x45 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0xa as libc::c_int as libc::c_uchar,
     0x67 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0xb as libc::c_int as libc::c_uchar,
     0x48 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0xc as libc::c_int as libc::c_uchar,
     0x48 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0xd as libc::c_int as libc::c_uchar,
     0x48 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0xe as libc::c_int as libc::c_uchar,
     0x68 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x10 as libc::c_int as libc::c_uchar,
     0x49 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x11 as libc::c_int as libc::c_uchar,
     0x49 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x12 as libc::c_int as libc::c_uchar,
     0x4c as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x13 as libc::c_int as libc::c_uchar,
     0x6c as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x15 as libc::c_int as libc::c_uchar,
     0x4e as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x18 as libc::c_int as libc::c_uchar,
     0x50 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x19 as libc::c_int as libc::c_uchar,
     0x50 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x1a as libc::c_int as libc::c_uchar,
     0x51 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x1b as libc::c_int as libc::c_uchar,
     0x52 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x1c as libc::c_int as libc::c_uchar,
     0x52 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x1d as libc::c_int as libc::c_uchar,
     0x52 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x24 as libc::c_int as libc::c_uchar,
     0x5a as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x28 as libc::c_int as libc::c_uchar,
     0x5a as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x2a as libc::c_int as libc::c_uchar,
     0x4b as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x2c as libc::c_int as libc::c_uchar,
     0x42 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x2d as libc::c_int as libc::c_uchar,
     0x43 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x2e as libc::c_int as libc::c_uchar,
     0x65 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x2f as libc::c_int as libc::c_uchar,
     0x65 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x30 as libc::c_int as libc::c_uchar,
     0x45 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x31 as libc::c_int as libc::c_uchar,
     0x46 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x33 as libc::c_int as libc::c_uchar,
     0x4d as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x34 as libc::c_int as libc::c_uchar,
     0x6f as libc::c_int as libc::c_uchar,
     0x22 as libc::c_int as libc::c_uchar,
     0x12 as libc::c_int as libc::c_uchar,
     0x2d as libc::c_int as libc::c_uchar,
     0x22 as libc::c_int as libc::c_uchar,
     0x15 as libc::c_int as libc::c_uchar,
     0x2f as libc::c_int as libc::c_uchar,
     0x22 as libc::c_int as libc::c_uchar,
     0x16 as libc::c_int as libc::c_uchar,
     0x5c as libc::c_int as libc::c_uchar,
     0x22 as libc::c_int as libc::c_uchar,
     0x17 as libc::c_int as libc::c_uchar,
     0x2a as libc::c_int as libc::c_uchar,
     0x22 as libc::c_int as libc::c_uchar,
     0x1a as libc::c_int as libc::c_uchar,
     0x76 as libc::c_int as libc::c_uchar,
     0x22 as libc::c_int as libc::c_uchar,
     0x1e as libc::c_int as libc::c_uchar,
     0x38 as libc::c_int as libc::c_uchar,
     0x22 as libc::c_int as libc::c_uchar,
     0x23 as libc::c_int as libc::c_uchar,
     0x7c as libc::c_int as libc::c_uchar,
     0x22 as libc::c_int as libc::c_uchar,
     0x29 as libc::c_int as libc::c_uchar,
     0x6e as libc::c_int as libc::c_uchar,
     0x22 as libc::c_int as libc::c_uchar,
     0x36 as libc::c_int as libc::c_uchar,
     0x3a as libc::c_int as libc::c_uchar,
     0x22 as libc::c_int as libc::c_uchar,
     0x3c as libc::c_int as libc::c_uchar,
     0x7e as libc::c_int as libc::c_uchar,
     0x22 as libc::c_int as libc::c_uchar,
     0x61 as libc::c_int as libc::c_uchar,
     0x3d as libc::c_int as libc::c_uchar,
     0x22 as libc::c_int as libc::c_uchar,
     0x64 as libc::c_int as libc::c_uchar,
     0x3d as libc::c_int as libc::c_uchar,
     0x22 as libc::c_int as libc::c_uchar,
     0x65 as libc::c_int as libc::c_uchar,
     0x3d as libc::c_int as libc::c_uchar,
     0x23 as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar,
     0x5e as libc::c_int as libc::c_uchar,
     0x23 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x28 as libc::c_int as libc::c_uchar,
     0x23 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x29 as libc::c_int as libc::c_uchar,
     0x23 as libc::c_int as libc::c_uchar,
     0x29 as libc::c_int as libc::c_uchar,
     0x3c as libc::c_int as libc::c_uchar,
     0x23 as libc::c_int as libc::c_uchar,
     0x2a as libc::c_int as libc::c_uchar,
     0x3e as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x2d as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0xc as libc::c_int as libc::c_uchar,
     0x2b as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x10 as libc::c_int as libc::c_uchar,
     0x2b as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x14 as libc::c_int as libc::c_uchar,
     0x2b as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x18 as libc::c_int as libc::c_uchar,
     0x2b as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x1c as libc::c_int as libc::c_uchar,
     0x2b as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x2c as libc::c_int as libc::c_uchar,
     0x2d as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x34 as libc::c_int as libc::c_uchar,
     0x2d as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x3c as libc::c_int as libc::c_uchar,
     0x2b as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x50 as libc::c_int as libc::c_uchar,
     0x2d as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x52 as libc::c_int as libc::c_uchar,
     0x2b as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x53 as libc::c_int as libc::c_uchar,
     0x2b as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x54 as libc::c_int as libc::c_uchar,
     0x2b as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x55 as libc::c_int as libc::c_uchar,
     0x2b as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x56 as libc::c_int as libc::c_uchar,
     0x2b as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x57 as libc::c_int as libc::c_uchar,
     0x2b as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x58 as libc::c_int as libc::c_uchar,
     0x2b as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x59 as libc::c_int as libc::c_uchar,
     0x2b as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x5a as libc::c_int as libc::c_uchar,
     0x2b as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x5b as libc::c_int as libc::c_uchar,
     0x2b as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x5c as libc::c_int as libc::c_uchar,
     0x2b as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x5d as libc::c_int as libc::c_uchar,
     0x2b as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x64 as libc::c_int as libc::c_uchar,
     0x2d as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x65 as libc::c_int as libc::c_uchar,
     0x2d as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x66 as libc::c_int as libc::c_uchar,
     0x2d as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x67 as libc::c_int as libc::c_uchar,
     0x2d as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x68 as libc::c_int as libc::c_uchar,
     0x2d as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x69 as libc::c_int as libc::c_uchar,
     0x2d as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x6a as libc::c_int as libc::c_uchar,
     0x2b as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x6b as libc::c_int as libc::c_uchar,
     0x2b as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x6c as libc::c_int as libc::c_uchar,
     0x2b as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x84 as libc::c_int as libc::c_uchar,
     0x5f as libc::c_int as libc::c_uchar,
     0x27 as libc::c_int as libc::c_uchar,
     0x58 as libc::c_int as libc::c_uchar,
     0x7c as libc::c_int as libc::c_uchar,
     0x30 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x30 as libc::c_int as libc::c_uchar,
     0x8 as libc::c_int as libc::c_uchar,
     0x3c as libc::c_int as libc::c_uchar,
     0x30 as libc::c_int as libc::c_uchar,
     0x9 as libc::c_int as libc::c_uchar,
     0x3e as libc::c_int as libc::c_uchar,
     0x30 as libc::c_int as libc::c_uchar,
     0x1a as libc::c_int as libc::c_uchar,
     0x5b as libc::c_int as libc::c_uchar,
     0x30 as libc::c_int as libc::c_uchar,
     0x1b as libc::c_int as libc::c_uchar,
     0x5d as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x1 as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x2 as libc::c_int as libc::c_uchar,
     0x22 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x3 as libc::c_int as libc::c_uchar,
     0x23 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x4 as libc::c_int as libc::c_uchar,
     0x24 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x5 as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x6 as libc::c_int as libc::c_uchar,
     0x26 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x7 as libc::c_int as libc::c_uchar,
     0x27 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x8 as libc::c_int as libc::c_uchar,
     0x28 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x9 as libc::c_int as libc::c_uchar,
     0x29 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xa as libc::c_int as libc::c_uchar,
     0x2a as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xb as libc::c_int as libc::c_uchar,
     0x2b as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xc as libc::c_int as libc::c_uchar,
     0x2c as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xd as libc::c_int as libc::c_uchar,
     0x2d as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xe as libc::c_int as libc::c_uchar,
     0x2e as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0xf as libc::c_int as libc::c_uchar,
     0x2f as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x10 as libc::c_int as libc::c_uchar,
     0x30 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x11 as libc::c_int as libc::c_uchar,
     0x31 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x12 as libc::c_int as libc::c_uchar,
     0x32 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x13 as libc::c_int as libc::c_uchar,
     0x33 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x14 as libc::c_int as libc::c_uchar,
     0x34 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x15 as libc::c_int as libc::c_uchar,
     0x35 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x16 as libc::c_int as libc::c_uchar,
     0x36 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x17 as libc::c_int as libc::c_uchar,
     0x37 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x18 as libc::c_int as libc::c_uchar,
     0x38 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x19 as libc::c_int as libc::c_uchar,
     0x39 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x1a as libc::c_int as libc::c_uchar,
     0x3a as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x1b as libc::c_int as libc::c_uchar,
     0x3b as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x1c as libc::c_int as libc::c_uchar,
     0x3c as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x1d as libc::c_int as libc::c_uchar,
     0x3d as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x1e as libc::c_int as libc::c_uchar,
     0x3e as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x20 as libc::c_int as libc::c_uchar,
     0x40 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x21 as libc::c_int as libc::c_uchar,
     0x41 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x22 as libc::c_int as libc::c_uchar,
     0x42 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x23 as libc::c_int as libc::c_uchar,
     0x43 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x24 as libc::c_int as libc::c_uchar,
     0x44 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x25 as libc::c_int as libc::c_uchar,
     0x45 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x26 as libc::c_int as libc::c_uchar,
     0x46 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x27 as libc::c_int as libc::c_uchar,
     0x47 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x28 as libc::c_int as libc::c_uchar,
     0x48 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x29 as libc::c_int as libc::c_uchar,
     0x49 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x2a as libc::c_int as libc::c_uchar,
     0x4a as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x2b as libc::c_int as libc::c_uchar,
     0x4b as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x2c as libc::c_int as libc::c_uchar,
     0x4c as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x2d as libc::c_int as libc::c_uchar,
     0x4d as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x2e as libc::c_int as libc::c_uchar,
     0x4e as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x2f as libc::c_int as libc::c_uchar,
     0x4f as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x30 as libc::c_int as libc::c_uchar,
     0x50 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x31 as libc::c_int as libc::c_uchar,
     0x51 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x32 as libc::c_int as libc::c_uchar,
     0x52 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x33 as libc::c_int as libc::c_uchar,
     0x53 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x34 as libc::c_int as libc::c_uchar,
     0x54 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x35 as libc::c_int as libc::c_uchar,
     0x55 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x36 as libc::c_int as libc::c_uchar,
     0x56 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x37 as libc::c_int as libc::c_uchar,
     0x57 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x38 as libc::c_int as libc::c_uchar,
     0x58 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x39 as libc::c_int as libc::c_uchar,
     0x59 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x3a as libc::c_int as libc::c_uchar,
     0x5a as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x3b as libc::c_int as libc::c_uchar,
     0x5b as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x3c as libc::c_int as libc::c_uchar,
     0x5c as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x3d as libc::c_int as libc::c_uchar,
     0x5d as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x3e as libc::c_int as libc::c_uchar,
     0x5e as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x3f as libc::c_int as libc::c_uchar,
     0x5f as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x40 as libc::c_int as libc::c_uchar,
     0x60 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x41 as libc::c_int as libc::c_uchar,
     0x61 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x42 as libc::c_int as libc::c_uchar,
     0x62 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x43 as libc::c_int as libc::c_uchar,
     0x63 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x44 as libc::c_int as libc::c_uchar,
     0x64 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x45 as libc::c_int as libc::c_uchar,
     0x65 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x46 as libc::c_int as libc::c_uchar,
     0x66 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x47 as libc::c_int as libc::c_uchar,
     0x67 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x48 as libc::c_int as libc::c_uchar,
     0x68 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x49 as libc::c_int as libc::c_uchar,
     0x69 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x4a as libc::c_int as libc::c_uchar,
     0x6a as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x4b as libc::c_int as libc::c_uchar,
     0x6b as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x4c as libc::c_int as libc::c_uchar,
     0x6c as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x4d as libc::c_int as libc::c_uchar,
     0x6d as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x4e as libc::c_int as libc::c_uchar,
     0x6e as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x4f as libc::c_int as libc::c_uchar,
     0x6f as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x50 as libc::c_int as libc::c_uchar,
     0x70 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x51 as libc::c_int as libc::c_uchar,
     0x71 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x52 as libc::c_int as libc::c_uchar,
     0x72 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x53 as libc::c_int as libc::c_uchar,
     0x73 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x54 as libc::c_int as libc::c_uchar,
     0x74 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x55 as libc::c_int as libc::c_uchar,
     0x75 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x56 as libc::c_int as libc::c_uchar,
     0x76 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x57 as libc::c_int as libc::c_uchar,
     0x77 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x58 as libc::c_int as libc::c_uchar,
     0x78 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x59 as libc::c_int as libc::c_uchar,
     0x79 as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x5a as libc::c_int as libc::c_uchar,
     0x7a as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x5b as libc::c_int as libc::c_uchar,
     0x7b as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x5c as libc::c_int as libc::c_uchar,
     0x7c as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x5d as libc::c_int as libc::c_uchar,
     0x7d as libc::c_int as libc::c_uchar,
     0xff as libc::c_int as libc::c_uchar,
     0x5e as libc::c_int as libc::c_uchar,
     0x7e as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar,
     0 as libc::c_int as libc::c_uchar, 0 as libc::c_int as libc::c_uchar];
#[no_mangle]
pub unsafe extern "C" fn htp_config_create() -> *mut htp_cfg_t {
    let mut cfg: *mut htp_cfg_t =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<htp_cfg_t>() as libc::c_ulong) as
            *mut htp_cfg_t; // Use the parser default.
    if cfg.is_null() {
        return 0 as *mut htp_cfg_t
    } // 2 layers seem fairly common
    (*cfg).field_limit_hard = 18000 as libc::c_int as size_t;
    (*cfg).field_limit_soft = 9000 as libc::c_int as size_t;
    (*cfg).log_level = HTP_LOG_NOTICE;
    (*cfg).response_decompression_enabled = 1 as libc::c_int;
    (*cfg).parse_request_cookies = 1 as libc::c_int;
    (*cfg).parse_request_auth = 1 as libc::c_int;
    (*cfg).extract_request_files = 0 as libc::c_int;
    (*cfg).extract_request_files_limit = -(1 as libc::c_int);
    (*cfg).response_decompression_layer_limit = 2 as libc::c_int;
    (*cfg).lzma_memlimit = 1048576 as libc::c_int as size_t;
    (*cfg).compression_bomb_limit = 1048576 as libc::c_int;
    // Default settings for URL-encoded data.
    htp_config_set_bestfit_map(cfg, HTP_DECODER_DEFAULTS,
                               bestfit_1252.as_mut_ptr() as
                                   *mut libc::c_void);
    htp_config_set_bestfit_replacement_byte(cfg, HTP_DECODER_DEFAULTS,
                                            '?' as i32);
    htp_config_set_url_encoding_invalid_handling(cfg, HTP_DECODER_DEFAULTS,
                                                 HTP_URL_DECODE_PRESERVE_PERCENT);
    htp_config_set_nul_raw_terminates(cfg, HTP_DECODER_DEFAULTS,
                                      0 as libc::c_int);
    htp_config_set_nul_encoded_terminates(cfg, HTP_DECODER_DEFAULTS,
                                          0 as libc::c_int);
    htp_config_set_u_encoding_decode(cfg, HTP_DECODER_DEFAULTS,
                                     0 as libc::c_int);
    htp_config_set_plusspace_decode(cfg, HTP_DECODER_URLENCODED,
                                    1 as libc::c_int);
    htp_config_set_server_personality(cfg, HTP_SERVER_MINIMAL);
    return cfg;
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_copy(mut cfg: *mut htp_cfg_t)
 -> *mut htp_cfg_t {
    if cfg.is_null() { return 0 as *mut htp_cfg_t }
    // Start by making a copy of the entire structure,
    // which is essentially a shallow copy.
    let mut copy: *mut htp_cfg_t =
        malloc(::std::mem::size_of::<htp_cfg_t>() as libc::c_ulong) as
            *mut htp_cfg_t;
    if copy.is_null() { return 0 as *mut htp_cfg_t }
    memcpy(copy as *mut libc::c_void, cfg as *const libc::c_void,
           ::std::mem::size_of::<htp_cfg_t>() as libc::c_ulong);
    // Now create copies of the hooks' structures.
    if !(*cfg).hook_request_start.is_null() {
        (*copy).hook_request_start = htp_hook_copy((*cfg).hook_request_start);
        if (*copy).hook_request_start.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t
        }
    }
    if !(*cfg).hook_request_line.is_null() {
        (*copy).hook_request_line = htp_hook_copy((*cfg).hook_request_line);
        if (*copy).hook_request_line.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t
        }
    }
    if !(*cfg).hook_request_uri_normalize.is_null() {
        (*copy).hook_request_uri_normalize =
            htp_hook_copy((*cfg).hook_request_uri_normalize);
        if (*copy).hook_request_uri_normalize.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t
        }
    }
    if !(*cfg).hook_request_header_data.is_null() {
        (*copy).hook_request_header_data =
            htp_hook_copy((*cfg).hook_request_header_data);
        if (*copy).hook_request_header_data.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t
        }
    }
    if !(*cfg).hook_request_headers.is_null() {
        (*copy).hook_request_headers =
            htp_hook_copy((*cfg).hook_request_headers);
        if (*copy).hook_request_headers.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t
        }
    }
    if !(*cfg).hook_request_body_data.is_null() {
        (*copy).hook_request_body_data =
            htp_hook_copy((*cfg).hook_request_body_data);
        if (*copy).hook_request_body_data.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t
        }
    }
    if !(*cfg).hook_request_file_data.is_null() {
        (*copy).hook_request_file_data =
            htp_hook_copy((*cfg).hook_request_file_data);
        if (*copy).hook_request_file_data.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t
        }
    }
    if !(*cfg).hook_request_trailer.is_null() {
        (*copy).hook_request_trailer =
            htp_hook_copy((*cfg).hook_request_trailer);
        if (*copy).hook_request_trailer.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t
        }
    }
    if !(*cfg).hook_request_trailer_data.is_null() {
        (*copy).hook_request_trailer_data =
            htp_hook_copy((*cfg).hook_request_trailer_data);
        if (*copy).hook_request_trailer_data.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t
        }
    }
    if !(*cfg).hook_request_complete.is_null() {
        (*copy).hook_request_complete =
            htp_hook_copy((*cfg).hook_request_complete);
        if (*copy).hook_request_complete.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t
        }
    }
    if !(*cfg).hook_response_start.is_null() {
        (*copy).hook_response_start =
            htp_hook_copy((*cfg).hook_response_start);
        if (*copy).hook_response_start.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t
        }
    }
    if !(*cfg).hook_response_line.is_null() {
        (*copy).hook_response_line = htp_hook_copy((*cfg).hook_response_line);
        if (*copy).hook_response_line.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t
        }
    }
    if !(*cfg).hook_response_header_data.is_null() {
        (*copy).hook_response_header_data =
            htp_hook_copy((*cfg).hook_response_header_data);
        if (*copy).hook_response_header_data.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t
        }
    }
    if !(*cfg).hook_response_headers.is_null() {
        (*copy).hook_response_headers =
            htp_hook_copy((*cfg).hook_response_headers);
        if (*copy).hook_response_headers.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t
        }
    }
    if !(*cfg).hook_response_body_data.is_null() {
        (*copy).hook_response_body_data =
            htp_hook_copy((*cfg).hook_response_body_data);
        if (*copy).hook_response_body_data.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t
        }
    }
    if !(*cfg).hook_response_trailer.is_null() {
        (*copy).hook_response_trailer =
            htp_hook_copy((*cfg).hook_response_trailer);
        if (*copy).hook_response_trailer.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t
        }
    }
    if !(*cfg).hook_response_trailer_data.is_null() {
        (*copy).hook_response_trailer_data =
            htp_hook_copy((*cfg).hook_response_trailer_data);
        if (*copy).hook_response_trailer_data.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t
        }
    }
    if !(*cfg).hook_response_complete.is_null() {
        (*copy).hook_response_complete =
            htp_hook_copy((*cfg).hook_response_complete);
        if (*copy).hook_response_complete.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t
        }
    }
    if !(*cfg).hook_transaction_complete.is_null() {
        (*copy).hook_transaction_complete =
            htp_hook_copy((*cfg).hook_transaction_complete);
        if (*copy).hook_transaction_complete.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t
        }
    }
    if !(*cfg).hook_log.is_null() {
        (*copy).hook_log = htp_hook_copy((*cfg).hook_log);
        if (*copy).hook_log.is_null() {
            htp_config_destroy(copy);
            return 0 as *mut htp_cfg_t
        }
    }
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_destroy(mut cfg: *mut htp_cfg_t) {
    if cfg.is_null() { return }
    htp_hook_destroy((*cfg).hook_request_start);
    htp_hook_destroy((*cfg).hook_request_line);
    htp_hook_destroy((*cfg).hook_request_uri_normalize);
    htp_hook_destroy((*cfg).hook_request_header_data);
    htp_hook_destroy((*cfg).hook_request_headers);
    htp_hook_destroy((*cfg).hook_request_body_data);
    htp_hook_destroy((*cfg).hook_request_file_data);
    htp_hook_destroy((*cfg).hook_request_trailer);
    htp_hook_destroy((*cfg).hook_request_trailer_data);
    htp_hook_destroy((*cfg).hook_request_complete);
    htp_hook_destroy((*cfg).hook_response_start);
    htp_hook_destroy((*cfg).hook_response_line);
    htp_hook_destroy((*cfg).hook_response_header_data);
    htp_hook_destroy((*cfg).hook_response_headers);
    htp_hook_destroy((*cfg).hook_response_body_data);
    htp_hook_destroy((*cfg).hook_response_trailer);
    htp_hook_destroy((*cfg).hook_response_trailer_data);
    htp_hook_destroy((*cfg).hook_response_complete);
    htp_hook_destroy((*cfg).hook_transaction_complete);
    htp_hook_destroy((*cfg).hook_log);
    free(cfg as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_get_user_data(mut cfg: *mut htp_cfg_t)
 -> *mut libc::c_void {
    if cfg.is_null() { return 0 as *mut libc::c_void }
    return (*cfg).user_data;
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_log(mut cfg: *mut htp_cfg_t,
                                                 mut callback_fn:
                                                     Option<unsafe extern "C" fn(_:
                                                                                     *mut htp_log_t)
                                                                ->
                                                                    libc::c_int>) {
    if cfg.is_null() { return }
    htp_hook_register(&mut (*cfg).hook_log,
                      ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                              *mut htp_log_t)
                                                         -> libc::c_int>,
                                              htp_callback_fn_t>(callback_fn));
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_multipart_parser(mut cfg:
                                                                  *mut htp_cfg_t) {
    if cfg.is_null() { return }
    htp_config_register_request_headers(cfg,
                                        Some(htp_ch_multipart_callback_request_headers
                                                 as
                                                 unsafe extern "C" fn(_:
                                                                          *mut htp_tx_t)
                                                     -> htp_status_t));
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_complete(mut cfg:
                                                                  *mut htp_cfg_t,
                                                              mut callback_fn:
                                                                  Option<unsafe extern "C" fn(_:
                                                                                                  *mut htp_tx_t)
                                                                             ->
                                                                                 libc::c_int>) {
    if cfg.is_null() { return }
    htp_hook_register(&mut (*cfg).hook_request_complete,
                      ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                              *mut htp_tx_t)
                                                         -> libc::c_int>,
                                              htp_callback_fn_t>(callback_fn));
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_body_data(mut cfg:
                                                                   *mut htp_cfg_t,
                                                               mut callback_fn:
                                                                   Option<unsafe extern "C" fn(_:
                                                                                                   *mut htp_tx_data_t)
                                                                              ->
                                                                                  libc::c_int>) {
    if cfg.is_null() { return }
    htp_hook_register(&mut (*cfg).hook_request_body_data,
                      ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                              *mut htp_tx_data_t)
                                                         -> libc::c_int>,
                                              htp_callback_fn_t>(callback_fn));
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_file_data(mut cfg:
                                                                   *mut htp_cfg_t,
                                                               mut callback_fn:
                                                                   Option<unsafe extern "C" fn(_:
                                                                                                   *mut htp_file_data_t)
                                                                              ->
                                                                                  libc::c_int>) {
    if cfg.is_null() { return }
    htp_hook_register(&mut (*cfg).hook_request_file_data,
                      ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                              *mut htp_file_data_t)
                                                         -> libc::c_int>,
                                              htp_callback_fn_t>(callback_fn));
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_uri_normalize(mut cfg:
                                                                       *mut htp_cfg_t,
                                                                   mut callback_fn:
                                                                       Option<unsafe extern "C" fn(_:
                                                                                                       *mut htp_tx_t)
                                                                                  ->
                                                                                      libc::c_int>) {
    if cfg.is_null() { return }
    htp_hook_register(&mut (*cfg).hook_request_uri_normalize,
                      ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                              *mut htp_tx_t)
                                                         -> libc::c_int>,
                                              htp_callback_fn_t>(callback_fn));
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_header_data(mut cfg:
                                                                     *mut htp_cfg_t,
                                                                 mut callback_fn:
                                                                     Option<unsafe extern "C" fn(_:
                                                                                                     *mut htp_tx_data_t)
                                                                                ->
                                                                                    libc::c_int>) {
    if cfg.is_null() { return }
    htp_hook_register(&mut (*cfg).hook_request_header_data,
                      ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                              *mut htp_tx_data_t)
                                                         -> libc::c_int>,
                                              htp_callback_fn_t>(callback_fn));
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_headers(mut cfg:
                                                                 *mut htp_cfg_t,
                                                             mut callback_fn:
                                                                 Option<unsafe extern "C" fn(_:
                                                                                                 *mut htp_tx_t)
                                                                            ->
                                                                                libc::c_int>) {
    if cfg.is_null() { return }
    htp_hook_register(&mut (*cfg).hook_request_headers,
                      ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                              *mut htp_tx_t)
                                                         -> libc::c_int>,
                                              htp_callback_fn_t>(callback_fn));
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_line(mut cfg:
                                                              *mut htp_cfg_t,
                                                          mut callback_fn:
                                                              Option<unsafe extern "C" fn(_:
                                                                                              *mut htp_tx_t)
                                                                         ->
                                                                             libc::c_int>) {
    if cfg.is_null() { return }
    htp_hook_register(&mut (*cfg).hook_request_line,
                      ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                              *mut htp_tx_t)
                                                         -> libc::c_int>,
                                              htp_callback_fn_t>(callback_fn));
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_start(mut cfg:
                                                               *mut htp_cfg_t,
                                                           mut callback_fn:
                                                               Option<unsafe extern "C" fn(_:
                                                                                               *mut htp_tx_t)
                                                                          ->
                                                                              libc::c_int>) {
    if cfg.is_null() { return }
    htp_hook_register(&mut (*cfg).hook_request_start,
                      ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                              *mut htp_tx_t)
                                                         -> libc::c_int>,
                                              htp_callback_fn_t>(callback_fn));
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_trailer(mut cfg:
                                                                 *mut htp_cfg_t,
                                                             mut callback_fn:
                                                                 Option<unsafe extern "C" fn(_:
                                                                                                 *mut htp_tx_t)
                                                                            ->
                                                                                libc::c_int>) {
    if cfg.is_null() { return }
    htp_hook_register(&mut (*cfg).hook_request_trailer,
                      ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                              *mut htp_tx_t)
                                                         -> libc::c_int>,
                                              htp_callback_fn_t>(callback_fn));
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_request_trailer_data(mut cfg:
                                                                      *mut htp_cfg_t,
                                                                  mut callback_fn:
                                                                      Option<unsafe extern "C" fn(_:
                                                                                                      *mut htp_tx_data_t)
                                                                                 ->
                                                                                     libc::c_int>) {
    if cfg.is_null() { return }
    htp_hook_register(&mut (*cfg).hook_request_trailer_data,
                      ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                              *mut htp_tx_data_t)
                                                         -> libc::c_int>,
                                              htp_callback_fn_t>(callback_fn));
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_response_body_data(mut cfg:
                                                                    *mut htp_cfg_t,
                                                                mut callback_fn:
                                                                    Option<unsafe extern "C" fn(_:
                                                                                                    *mut htp_tx_data_t)
                                                                               ->
                                                                                   libc::c_int>) {
    if cfg.is_null() { return }
    htp_hook_register(&mut (*cfg).hook_response_body_data,
                      ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                              *mut htp_tx_data_t)
                                                         -> libc::c_int>,
                                              htp_callback_fn_t>(callback_fn));
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_response_complete(mut cfg:
                                                                   *mut htp_cfg_t,
                                                               mut callback_fn:
                                                                   Option<unsafe extern "C" fn(_:
                                                                                                   *mut htp_tx_t)
                                                                              ->
                                                                                  libc::c_int>) {
    if cfg.is_null() { return }
    htp_hook_register(&mut (*cfg).hook_response_complete,
                      ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                              *mut htp_tx_t)
                                                         -> libc::c_int>,
                                              htp_callback_fn_t>(callback_fn));
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_response_header_data(mut cfg:
                                                                      *mut htp_cfg_t,
                                                                  mut callback_fn:
                                                                      Option<unsafe extern "C" fn(_:
                                                                                                      *mut htp_tx_data_t)
                                                                                 ->
                                                                                     libc::c_int>) {
    if cfg.is_null() { return }
    htp_hook_register(&mut (*cfg).hook_response_header_data,
                      ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                              *mut htp_tx_data_t)
                                                         -> libc::c_int>,
                                              htp_callback_fn_t>(callback_fn));
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_response_headers(mut cfg:
                                                                  *mut htp_cfg_t,
                                                              mut callback_fn:
                                                                  Option<unsafe extern "C" fn(_:
                                                                                                  *mut htp_tx_t)
                                                                             ->
                                                                                 libc::c_int>) {
    if cfg.is_null() { return }
    htp_hook_register(&mut (*cfg).hook_response_headers,
                      ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                              *mut htp_tx_t)
                                                         -> libc::c_int>,
                                              htp_callback_fn_t>(callback_fn));
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_response_line(mut cfg:
                                                               *mut htp_cfg_t,
                                                           mut callback_fn:
                                                               Option<unsafe extern "C" fn(_:
                                                                                               *mut htp_tx_t)
                                                                          ->
                                                                              libc::c_int>) {
    if cfg.is_null() { return }
    htp_hook_register(&mut (*cfg).hook_response_line,
                      ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                              *mut htp_tx_t)
                                                         -> libc::c_int>,
                                              htp_callback_fn_t>(callback_fn));
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_response_start(mut cfg:
                                                                *mut htp_cfg_t,
                                                            mut callback_fn:
                                                                Option<unsafe extern "C" fn(_:
                                                                                                *mut htp_tx_t)
                                                                           ->
                                                                               libc::c_int>) {
    if cfg.is_null() { return }
    htp_hook_register(&mut (*cfg).hook_response_start,
                      ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                              *mut htp_tx_t)
                                                         -> libc::c_int>,
                                              htp_callback_fn_t>(callback_fn));
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_response_trailer(mut cfg:
                                                                  *mut htp_cfg_t,
                                                              mut callback_fn:
                                                                  Option<unsafe extern "C" fn(_:
                                                                                                  *mut htp_tx_t)
                                                                             ->
                                                                                 libc::c_int>) {
    if cfg.is_null() { return }
    htp_hook_register(&mut (*cfg).hook_response_trailer,
                      ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                              *mut htp_tx_t)
                                                         -> libc::c_int>,
                                              htp_callback_fn_t>(callback_fn));
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_response_trailer_data(mut cfg:
                                                                       *mut htp_cfg_t,
                                                                   mut callback_fn:
                                                                       Option<unsafe extern "C" fn(_:
                                                                                                       *mut htp_tx_data_t)
                                                                                  ->
                                                                                      libc::c_int>) {
    if cfg.is_null() { return }
    htp_hook_register(&mut (*cfg).hook_response_trailer_data,
                      ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                              *mut htp_tx_data_t)
                                                         -> libc::c_int>,
                                              htp_callback_fn_t>(callback_fn));
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_transaction_complete(mut cfg:
                                                                      *mut htp_cfg_t,
                                                                  mut callback_fn:
                                                                      Option<unsafe extern "C" fn(_:
                                                                                                      *mut htp_tx_t)
                                                                                 ->
                                                                                     libc::c_int>) {
    if cfg.is_null() { return }
    htp_hook_register(&mut (*cfg).hook_transaction_complete,
                      ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                              *mut htp_tx_t)
                                                         -> libc::c_int>,
                                              htp_callback_fn_t>(callback_fn));
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_register_urlencoded_parser(mut cfg:
                                                                   *mut htp_cfg_t) {
    if cfg.is_null() { return }
    htp_config_register_request_line(cfg,
                                     Some(htp_ch_urlencoded_callback_request_line
                                              as
                                              unsafe extern "C" fn(_:
                                                                       *mut htp_tx_t)
                                                  -> htp_status_t));
    htp_config_register_request_headers(cfg,
                                        Some(htp_ch_urlencoded_callback_request_headers
                                                 as
                                                 unsafe extern "C" fn(_:
                                                                          *mut htp_tx_t)
                                                     -> htp_status_t));
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_extract_request_files(mut cfg:
                                                                  *mut htp_cfg_t,
                                                              mut extract_request_files:
                                                                  libc::c_int,
                                                              mut limit:
                                                                  libc::c_int)
 -> htp_status_t {
    if cfg.is_null() { return -(1 as libc::c_int) }
    if (*cfg).tmpdir.is_null() { return -(1 as libc::c_int) }
    (*cfg).extract_request_files = extract_request_files;
    (*cfg).extract_request_files_limit = limit;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_field_limits(mut cfg: *mut htp_cfg_t,
                                                     mut soft_limit: size_t,
                                                     mut hard_limit: size_t) {
    if cfg.is_null() { return }
    (*cfg).field_limit_soft = soft_limit;
    (*cfg).field_limit_hard = hard_limit;
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_lzma_memlimit(mut cfg: *mut htp_cfg_t,
                                                      mut memlimit: size_t) {
    if cfg.is_null() { return }
    (*cfg).lzma_memlimit = memlimit;
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_compression_bomb_limit(mut cfg:
                                                                   *mut htp_cfg_t,
                                                               mut bomblimit:
                                                                   size_t) {
    if cfg.is_null() { return }
    if bomblimit > 2147483647 as libc::c_int as libc::c_ulong {
        (*cfg).compression_bomb_limit = 2147483647 as libc::c_int
    } else { (*cfg).compression_bomb_limit = bomblimit as int32_t };
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_log_level(mut cfg: *mut htp_cfg_t,
                                                  mut log_level:
                                                      htp_log_level_t) {
    if cfg.is_null() { return }
    (*cfg).log_level = log_level;
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_parse_request_auth(mut cfg:
                                                               *mut htp_cfg_t,
                                                           mut parse_request_auth:
                                                               libc::c_int) {
    if cfg.is_null() { return }
    (*cfg).parse_request_auth = parse_request_auth;
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_parse_request_cookies(mut cfg:
                                                                  *mut htp_cfg_t,
                                                              mut parse_request_cookies:
                                                                  libc::c_int) {
    if cfg.is_null() { return }
    (*cfg).parse_request_cookies = parse_request_cookies;
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_response_decompression(mut cfg:
                                                                   *mut htp_cfg_t,
                                                               mut enabled:
                                                                   libc::c_int) {
    if cfg.is_null() { return }
    (*cfg).response_decompression_enabled = enabled;
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_server_personality(mut cfg:
                                                               *mut htp_cfg_t,
                                                           mut personality:
                                                               htp_server_personality_t)
 -> htp_status_t {
    if cfg.is_null() { return -(1 as libc::c_int) }
    match personality as libc::c_uint {
        0 => {
            (*cfg).parse_request_line =
                Some(htp_parse_request_line_generic as
                         unsafe extern "C" fn(_: *mut htp_connp_t)
                             -> htp_status_t);
            (*cfg).process_request_header =
                Some(htp_process_request_header_generic as
                         unsafe extern "C" fn(_: *mut htp_connp_t,
                                              _: *mut libc::c_uchar,
                                              _: size_t) -> htp_status_t);
            (*cfg).parse_response_line =
                Some(htp_parse_response_line_generic as
                         unsafe extern "C" fn(_: *mut htp_connp_t)
                             -> htp_status_t);
            (*cfg).process_response_header =
                Some(htp_process_response_header_generic as
                         unsafe extern "C" fn(_: *mut htp_connp_t,
                                              _: *mut libc::c_uchar,
                                              _: size_t) -> htp_status_t)
        }
        1 => {
            (*cfg).parse_request_line =
                Some(htp_parse_request_line_generic as
                         unsafe extern "C" fn(_: *mut htp_connp_t)
                             -> htp_status_t);
            (*cfg).process_request_header =
                Some(htp_process_request_header_generic as
                         unsafe extern "C" fn(_: *mut htp_connp_t,
                                              _: *mut libc::c_uchar,
                                              _: size_t) -> htp_status_t);
            (*cfg).parse_response_line =
                Some(htp_parse_response_line_generic as
                         unsafe extern "C" fn(_: *mut htp_connp_t)
                             -> htp_status_t);
            (*cfg).process_response_header =
                Some(htp_process_response_header_generic as
                         unsafe extern "C" fn(_: *mut htp_connp_t,
                                              _: *mut libc::c_uchar,
                                              _: size_t) -> htp_status_t);
            htp_config_set_backslash_convert_slashes(cfg,
                                                     HTP_DECODER_URL_PATH,
                                                     1 as libc::c_int);
            htp_config_set_path_separators_decode(cfg, HTP_DECODER_URL_PATH,
                                                  1 as libc::c_int);
            htp_config_set_path_separators_compress(cfg, HTP_DECODER_URL_PATH,
                                                    1 as libc::c_int);
        }
        2 => {
            (*cfg).parse_request_line =
                Some(htp_parse_request_line_generic as
                         unsafe extern "C" fn(_: *mut htp_connp_t)
                             -> htp_status_t);
            (*cfg).process_request_header =
                Some(htp_process_request_header_generic as
                         unsafe extern "C" fn(_: *mut htp_connp_t,
                                              _: *mut libc::c_uchar,
                                              _: size_t) -> htp_status_t);
            (*cfg).parse_response_line =
                Some(htp_parse_response_line_generic as
                         unsafe extern "C" fn(_: *mut htp_connp_t)
                             -> htp_status_t);
            (*cfg).process_response_header =
                Some(htp_process_response_header_generic as
                         unsafe extern "C" fn(_: *mut htp_connp_t,
                                              _: *mut libc::c_uchar,
                                              _: size_t) -> htp_status_t);
            htp_config_set_backslash_convert_slashes(cfg,
                                                     HTP_DECODER_URL_PATH,
                                                     1 as libc::c_int);
            htp_config_set_path_separators_decode(cfg, HTP_DECODER_URL_PATH,
                                                  1 as libc::c_int);
            htp_config_set_path_separators_compress(cfg, HTP_DECODER_URL_PATH,
                                                    1 as libc::c_int);
            htp_config_set_convert_lowercase(cfg, HTP_DECODER_URL_PATH,
                                             1 as libc::c_int);
            htp_config_set_utf8_convert_bestfit(cfg, HTP_DECODER_URL_PATH,
                                                1 as libc::c_int);
            htp_config_set_u_encoding_decode(cfg, HTP_DECODER_URL_PATH,
                                             1 as libc::c_int);
            htp_config_set_requestline_leading_whitespace_unwanted(cfg,
                                                                   HTP_DECODER_DEFAULTS,
                                                                   HTP_UNWANTED_IGNORE);
        }
        9 => {
            (*cfg).parse_request_line =
                Some(htp_parse_request_line_apache_2_2 as
                         unsafe extern "C" fn(_: *mut htp_connp_t)
                             -> htp_status_t);
            (*cfg).process_request_header =
                Some(htp_process_request_header_apache_2_2 as
                         unsafe extern "C" fn(_: *mut htp_connp_t,
                                              _: *mut libc::c_uchar,
                                              _: size_t) -> htp_status_t);
            (*cfg).parse_response_line =
                Some(htp_parse_response_line_generic as
                         unsafe extern "C" fn(_: *mut htp_connp_t)
                             -> htp_status_t);
            (*cfg).process_response_header =
                Some(htp_process_response_header_generic as
                         unsafe extern "C" fn(_: *mut htp_connp_t,
                                              _: *mut libc::c_uchar,
                                              _: size_t) -> htp_status_t);
            htp_config_set_backslash_convert_slashes(cfg,
                                                     HTP_DECODER_URL_PATH,
                                                     0 as libc::c_int);
            htp_config_set_path_separators_decode(cfg, HTP_DECODER_URL_PATH,
                                                  0 as libc::c_int);
            htp_config_set_path_separators_compress(cfg, HTP_DECODER_URL_PATH,
                                                    1 as libc::c_int);
            htp_config_set_u_encoding_decode(cfg, HTP_DECODER_URL_PATH,
                                             0 as libc::c_int);
            htp_config_set_url_encoding_invalid_handling(cfg,
                                                         HTP_DECODER_URL_PATH,
                                                         HTP_URL_DECODE_PRESERVE_PERCENT);
            htp_config_set_url_encoding_invalid_unwanted(cfg,
                                                         HTP_DECODER_URL_PATH,
                                                         HTP_UNWANTED_400);
            htp_config_set_control_chars_unwanted(cfg, HTP_DECODER_URL_PATH,
                                                  HTP_UNWANTED_IGNORE);
            htp_config_set_requestline_leading_whitespace_unwanted(cfg,
                                                                   HTP_DECODER_DEFAULTS,
                                                                   HTP_UNWANTED_400);
        }
        5 => {
            (*cfg).parse_request_line =
                Some(htp_parse_request_line_generic as
                         unsafe extern "C" fn(_: *mut htp_connp_t)
                             -> htp_status_t);
            (*cfg).process_request_header =
                Some(htp_process_request_header_generic as
                         unsafe extern "C" fn(_: *mut htp_connp_t,
                                              _: *mut libc::c_uchar,
                                              _: size_t) -> htp_status_t);
            (*cfg).parse_response_line =
                Some(htp_parse_response_line_generic as
                         unsafe extern "C" fn(_: *mut htp_connp_t)
                             -> htp_status_t);
            (*cfg).process_response_header =
                Some(htp_process_response_header_generic as
                         unsafe extern "C" fn(_: *mut htp_connp_t,
                                              _: *mut libc::c_uchar,
                                              _: size_t) -> htp_status_t);
            htp_config_set_backslash_convert_slashes(cfg,
                                                     HTP_DECODER_URL_PATH,
                                                     1 as libc::c_int);
            htp_config_set_path_separators_decode(cfg, HTP_DECODER_URL_PATH,
                                                  1 as libc::c_int);
            htp_config_set_path_separators_compress(cfg, HTP_DECODER_URL_PATH,
                                                    1 as libc::c_int);
            htp_config_set_u_encoding_decode(cfg, HTP_DECODER_URL_PATH,
                                             0 as libc::c_int);
            htp_config_set_url_encoding_invalid_handling(cfg,
                                                         HTP_DECODER_URL_PATH,
                                                         HTP_URL_DECODE_PRESERVE_PERCENT);
            htp_config_set_control_chars_unwanted(cfg, HTP_DECODER_URL_PATH,
                                                  HTP_UNWANTED_IGNORE);
            htp_config_set_requestline_leading_whitespace_unwanted(cfg,
                                                                   HTP_DECODER_DEFAULTS,
                                                                   HTP_UNWANTED_IGNORE);
        }
        6 => {
            (*cfg).parse_request_line =
                Some(htp_parse_request_line_generic as
                         unsafe extern "C" fn(_: *mut htp_connp_t)
                             -> htp_status_t);
            (*cfg).process_request_header =
                Some(htp_process_request_header_generic as
                         unsafe extern "C" fn(_: *mut htp_connp_t,
                                              _: *mut libc::c_uchar,
                                              _: size_t) -> htp_status_t);
            (*cfg).parse_response_line =
                Some(htp_parse_response_line_generic as
                         unsafe extern "C" fn(_: *mut htp_connp_t)
                             -> htp_status_t);
            (*cfg).process_response_header =
                Some(htp_process_response_header_generic as
                         unsafe extern "C" fn(_: *mut htp_connp_t,
                                              _: *mut libc::c_uchar,
                                              _: size_t) -> htp_status_t);
            htp_config_set_backslash_convert_slashes(cfg,
                                                     HTP_DECODER_URL_PATH,
                                                     1 as libc::c_int);
            htp_config_set_path_separators_decode(cfg, HTP_DECODER_URL_PATH,
                                                  1 as libc::c_int);
            htp_config_set_path_separators_compress(cfg, HTP_DECODER_URL_PATH,
                                                    1 as libc::c_int);
            htp_config_set_u_encoding_decode(cfg, HTP_DECODER_URL_PATH,
                                             1 as libc::c_int);
            htp_config_set_url_encoding_invalid_handling(cfg,
                                                         HTP_DECODER_URL_PATH,
                                                         HTP_URL_DECODE_PRESERVE_PERCENT);
            htp_config_set_u_encoding_unwanted(cfg, HTP_DECODER_URL_PATH,
                                               HTP_UNWANTED_400);
            htp_config_set_control_chars_unwanted(cfg, HTP_DECODER_URL_PATH,
                                                  HTP_UNWANTED_400);
            htp_config_set_requestline_leading_whitespace_unwanted(cfg,
                                                                   HTP_DECODER_DEFAULTS,
                                                                   HTP_UNWANTED_IGNORE);
        }
        7 | 8 => {
            (*cfg).parse_request_line =
                Some(htp_parse_request_line_generic as
                         unsafe extern "C" fn(_: *mut htp_connp_t)
                             -> htp_status_t);
            (*cfg).process_request_header =
                Some(htp_process_request_header_generic as
                         unsafe extern "C" fn(_: *mut htp_connp_t,
                                              _: *mut libc::c_uchar,
                                              _: size_t) -> htp_status_t);
            (*cfg).parse_response_line =
                Some(htp_parse_response_line_generic as
                         unsafe extern "C" fn(_: *mut htp_connp_t)
                             -> htp_status_t);
            (*cfg).process_response_header =
                Some(htp_process_response_header_generic as
                         unsafe extern "C" fn(_: *mut htp_connp_t,
                                              _: *mut libc::c_uchar,
                                              _: size_t) -> htp_status_t);
            htp_config_set_backslash_convert_slashes(cfg,
                                                     HTP_DECODER_URL_PATH,
                                                     1 as libc::c_int);
            htp_config_set_path_separators_decode(cfg, HTP_DECODER_URL_PATH,
                                                  1 as libc::c_int);
            htp_config_set_path_separators_compress(cfg, HTP_DECODER_URL_PATH,
                                                    1 as libc::c_int);
            htp_config_set_u_encoding_decode(cfg, HTP_DECODER_URL_PATH,
                                             1 as libc::c_int);
            htp_config_set_url_encoding_invalid_handling(cfg,
                                                         HTP_DECODER_URL_PATH,
                                                         HTP_URL_DECODE_PRESERVE_PERCENT);
            htp_config_set_url_encoding_invalid_unwanted(cfg,
                                                         HTP_DECODER_URL_PATH,
                                                         HTP_UNWANTED_400);
            htp_config_set_control_chars_unwanted(cfg, HTP_DECODER_URL_PATH,
                                                  HTP_UNWANTED_400);
            htp_config_set_requestline_leading_whitespace_unwanted(cfg,
                                                                   HTP_DECODER_DEFAULTS,
                                                                   HTP_UNWANTED_IGNORE);
        }
        _ => { return -(1 as libc::c_int) }
    }
    // Remember the personality
    (*cfg).server_personality = personality;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_tmpdir(mut cfg: *mut htp_cfg_t,
                                               mut tmpdir:
                                                   *mut libc::c_char) {
    if cfg.is_null() { return }
    (*cfg).tmpdir = tmpdir;
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_tx_auto_destroy(mut cfg:
                                                            *mut htp_cfg_t,
                                                        mut tx_auto_destroy:
                                                            libc::c_int) {
    if cfg.is_null() { return }
    (*cfg).tx_auto_destroy = tx_auto_destroy;
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_user_data(mut cfg: *mut htp_cfg_t,
                                                  mut user_data:
                                                      *mut libc::c_void) {
    if cfg.is_null() { return }
    (*cfg).user_data = user_data;
}
unsafe extern "C" fn convert_to_0_or_1(mut b: libc::c_int) -> libc::c_int {
    if b != 0 { return 1 as libc::c_int }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_bestfit_map(mut cfg: *mut htp_cfg_t,
                                                    mut ctx:
                                                        htp_decoder_ctx_t,
                                                    mut map:
                                                        *mut libc::c_void) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint { return }
    (*cfg).decoder_cfgs[ctx as usize].bestfit_map = map as *mut libc::c_uchar;
    if ctx as libc::c_uint ==
           HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].bestfit_map =
                map as *mut libc::c_uchar;
            i = i.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_bestfit_replacement_byte(mut cfg:
                                                                     *mut htp_cfg_t,
                                                                 mut ctx:
                                                                     htp_decoder_ctx_t,
                                                                 mut b:
                                                                     libc::c_int) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint { return }
    (*cfg).decoder_cfgs[ctx as usize].bestfit_replacement_byte =
        b as libc::c_uchar;
    if ctx as libc::c_uint ==
           HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].bestfit_replacement_byte =
                b as libc::c_uchar;
            i = i.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_url_encoding_invalid_handling(mut cfg:
                                                                          *mut htp_cfg_t,
                                                                      mut ctx:
                                                                          htp_decoder_ctx_t,
                                                                      mut handling:
                                                                          htp_url_encoding_handling_t) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint { return }
    (*cfg).decoder_cfgs[ctx as usize].url_encoding_invalid_handling =
        handling;
    if ctx as libc::c_uint ==
           HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].url_encoding_invalid_handling =
                handling;
            i = i.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_nul_raw_terminates(mut cfg:
                                                               *mut htp_cfg_t,
                                                           mut ctx:
                                                               htp_decoder_ctx_t,
                                                           mut enabled:
                                                               libc::c_int) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint { return }
    (*cfg).decoder_cfgs[ctx as usize].nul_raw_terminates =
        convert_to_0_or_1(enabled);
    if ctx as libc::c_uint ==
           HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].nul_raw_terminates =
                convert_to_0_or_1(enabled);
            i = i.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_nul_encoded_terminates(mut cfg:
                                                                   *mut htp_cfg_t,
                                                               mut ctx:
                                                                   htp_decoder_ctx_t,
                                                               mut enabled:
                                                                   libc::c_int) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint { return }
    (*cfg).decoder_cfgs[ctx as usize].nul_encoded_terminates =
        convert_to_0_or_1(enabled);
    if ctx as libc::c_uint ==
           HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].nul_encoded_terminates =
                convert_to_0_or_1(enabled);
            i = i.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_u_encoding_decode(mut cfg:
                                                              *mut htp_cfg_t,
                                                          mut ctx:
                                                              htp_decoder_ctx_t,
                                                          mut enabled:
                                                              libc::c_int) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint { return }
    (*cfg).decoder_cfgs[ctx as usize].u_encoding_decode =
        convert_to_0_or_1(enabled);
    if ctx as libc::c_uint ==
           HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].u_encoding_decode =
                convert_to_0_or_1(enabled);
            i = i.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_backslash_convert_slashes(mut cfg:
                                                                      *mut htp_cfg_t,
                                                                  mut ctx:
                                                                      htp_decoder_ctx_t,
                                                                  mut enabled:
                                                                      libc::c_int) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint { return }
    (*cfg).decoder_cfgs[ctx as usize].backslash_convert_slashes =
        convert_to_0_or_1(enabled);
    if ctx as libc::c_uint ==
           HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].backslash_convert_slashes =
                convert_to_0_or_1(enabled);
            i = i.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_path_separators_decode(mut cfg:
                                                                   *mut htp_cfg_t,
                                                               mut ctx:
                                                                   htp_decoder_ctx_t,
                                                               mut enabled:
                                                                   libc::c_int) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint { return }
    (*cfg).decoder_cfgs[ctx as usize].path_separators_decode =
        convert_to_0_or_1(enabled);
    if ctx as libc::c_uint ==
           HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].path_separators_decode =
                convert_to_0_or_1(enabled);
            i = i.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_path_separators_compress(mut cfg:
                                                                     *mut htp_cfg_t,
                                                                 mut ctx:
                                                                     htp_decoder_ctx_t,
                                                                 mut enabled:
                                                                     libc::c_int) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint { return }
    (*cfg).decoder_cfgs[ctx as usize].path_separators_compress =
        convert_to_0_or_1(enabled);
    if ctx as libc::c_uint ==
           HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].path_separators_compress =
                convert_to_0_or_1(enabled);
            i = i.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_plusspace_decode(mut cfg:
                                                             *mut htp_cfg_t,
                                                         mut ctx:
                                                             htp_decoder_ctx_t,
                                                         mut enabled:
                                                             libc::c_int) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint { return }
    (*cfg).decoder_cfgs[ctx as usize].plusspace_decode =
        convert_to_0_or_1(enabled);
    if ctx as libc::c_uint ==
           HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].plusspace_decode =
                convert_to_0_or_1(enabled);
            i = i.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_convert_lowercase(mut cfg:
                                                              *mut htp_cfg_t,
                                                          mut ctx:
                                                              htp_decoder_ctx_t,
                                                          mut enabled:
                                                              libc::c_int) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint { return }
    (*cfg).decoder_cfgs[ctx as usize].convert_lowercase =
        convert_to_0_or_1(enabled);
    if ctx as libc::c_uint ==
           HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].convert_lowercase =
                convert_to_0_or_1(enabled);
            i = i.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_utf8_convert_bestfit(mut cfg:
                                                                 *mut htp_cfg_t,
                                                             mut ctx:
                                                                 htp_decoder_ctx_t,
                                                             mut enabled:
                                                                 libc::c_int) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint { return }
    (*cfg).decoder_cfgs[ctx as usize].utf8_convert_bestfit =
        convert_to_0_or_1(enabled);
    if ctx as libc::c_uint ==
           HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].utf8_convert_bestfit =
                convert_to_0_or_1(enabled);
            i = i.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_u_encoding_unwanted(mut cfg:
                                                                *mut htp_cfg_t,
                                                            mut ctx:
                                                                htp_decoder_ctx_t,
                                                            mut unwanted:
                                                                htp_unwanted_t) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint { return }
    (*cfg).decoder_cfgs[ctx as usize].u_encoding_unwanted = unwanted;
    if ctx as libc::c_uint ==
           HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].u_encoding_unwanted = unwanted;
            i = i.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_control_chars_unwanted(mut cfg:
                                                                   *mut htp_cfg_t,
                                                               mut ctx:
                                                                   htp_decoder_ctx_t,
                                                               mut unwanted:
                                                                   htp_unwanted_t) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint { return }
    (*cfg).decoder_cfgs[ctx as usize].u_encoding_unwanted = unwanted;
    if ctx as libc::c_uint ==
           HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].u_encoding_unwanted = unwanted;
            i = i.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_url_encoding_invalid_unwanted(mut cfg:
                                                                          *mut htp_cfg_t,
                                                                      mut ctx:
                                                                          htp_decoder_ctx_t,
                                                                      mut unwanted:
                                                                          htp_unwanted_t) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint { return }
    (*cfg).decoder_cfgs[ctx as usize].url_encoding_invalid_unwanted =
        unwanted;
    if ctx as libc::c_uint ==
           HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].url_encoding_invalid_unwanted =
                unwanted;
            i = i.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_nul_encoded_unwanted(mut cfg:
                                                                 *mut htp_cfg_t,
                                                             mut ctx:
                                                                 htp_decoder_ctx_t,
                                                             mut unwanted:
                                                                 htp_unwanted_t) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint { return }
    (*cfg).decoder_cfgs[ctx as usize].nul_encoded_unwanted = unwanted;
    if ctx as libc::c_uint ==
           HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].nul_encoded_unwanted = unwanted;
            i = i.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_nul_raw_unwanted(mut cfg:
                                                             *mut htp_cfg_t,
                                                         mut ctx:
                                                             htp_decoder_ctx_t,
                                                         mut unwanted:
                                                             htp_unwanted_t) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint { return }
    (*cfg).decoder_cfgs[ctx as usize].nul_raw_unwanted = unwanted;
    if ctx as libc::c_uint ==
           HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].nul_raw_unwanted = unwanted;
            i = i.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_path_separators_encoded_unwanted(mut cfg:
                                                                             *mut htp_cfg_t,
                                                                         mut ctx:
                                                                             htp_decoder_ctx_t,
                                                                         mut unwanted:
                                                                             htp_unwanted_t) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint { return }
    (*cfg).decoder_cfgs[ctx as usize].path_separators_encoded_unwanted =
        unwanted;
    if ctx as libc::c_uint ==
           HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].path_separators_encoded_unwanted =
                unwanted;
            i = i.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_utf8_invalid_unwanted(mut cfg:
                                                                  *mut htp_cfg_t,
                                                              mut ctx:
                                                                  htp_decoder_ctx_t,
                                                              mut unwanted:
                                                                  htp_unwanted_t) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint { return }
    (*cfg).decoder_cfgs[ctx as usize].utf8_invalid_unwanted = unwanted;
    if ctx as libc::c_uint ==
           HTP_DECODER_DEFAULTS as libc::c_int as libc::c_uint {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < 3 as libc::c_int as libc::c_ulong {
            (*cfg).decoder_cfgs[i as usize].utf8_invalid_unwanted = unwanted;
            i = i.wrapping_add(1)
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_requestline_leading_whitespace_unwanted(mut cfg:
                                                                                    *mut htp_cfg_t,
                                                                                mut ctx:
                                                                                    htp_decoder_ctx_t,
                                                                                mut unwanted:
                                                                                    htp_unwanted_t) {
    if ctx as libc::c_uint >= 3 as libc::c_int as libc::c_uint { return }
    (*cfg).requestline_leading_whitespace_unwanted = unwanted;
}
/* *
 * Creates a new configuration structure. Configuration structures created at
 * configuration time must not be changed afterwards in order to support lock-less
 * copying.
 *
 * @return New configuration structure.
 */
/* *
 * Creates a copy of the supplied configuration structure. The idea is to create
 * one or more configuration objects at configuration-time, but to use this
 * function to create per-connection copies. That way it will be possible to
 * adjust per-connection configuration as necessary, without affecting the
 * global configuration. Make sure no other thread changes the configuration
 * object while this function is operating.
 *
 * @param[in] cfg
 * @return A copy of the configuration structure.
 */
/* *
 * Destroy a configuration structure.
 *
 * @param[in] cfg
 */
/* *
 * Retrieves user data associated with this configuration.
 *
 * @param[in] cfg
 * @return User data pointer, or NULL if not set.
 */
/* *
 * Registers a callback that is invoked every time there is a log message with
 * severity equal and higher than the configured log level.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
/* *
 * Adds the built-in Multipart parser to the configuration. This parser will extract information
 * stored in request bodies, when they are in multipart/form-data format.
 *
 * @param[in] cfg
 */
/* *
 * Registers a REQUEST_START callback, which is invoked every time a new
 * request begins and before any parsing is done.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
/* *
 * Registers a REQUEST_BODY_DATA callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
/* *
 * Registers a REQUEST_COMPLETE callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
/* *
 * Registers a REQUEST_FILE_DATA callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
/* *
 * Registers a REQUEST_HEADER_DATA callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
/* *
 * Registers a REQUEST_HEADERS callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
/* *
 * Registers a REQUEST_LINE callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
/* *
 * Registers a REQUEST_URI_NORMALIZE callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
/* *
 * Registers a HTP_REQUEST_TRAILER callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
/* *
 * Registers a REQUEST_TRAILER_DATA callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
/* *
 * Registers a RESPONSE_BODY_DATA callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
/* *
 * Registers a RESPONSE_COMPLETE callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
/* *
 * Registers a RESPONSE_HEADER_DATA callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
/* *
 * Registers a RESPONSE_HEADERS callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
/* *
 * Registers a RESPONSE_LINE callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
/* *
 * Registers a RESPONSE_START callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
/* *
 * Registers a RESPONSE_TRAILER callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
/* *
 * Registers a RESPONSE_TRAILER_DATA callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
/* *
 * Registers a TRANSACTION_COMPLETE callback.
 *
 * @param[in] cfg
 * @param[in] callback_fn
 */
/* *
 * Adds the built-in Urlencoded parser to the configuration. The parser will
 * parse query strings and request bodies with the appropriate MIME type.
 *
 * @param[in] cfg
 */
/* *
 * Configures whether backslash characters are treated as path segment separators. They
 * are not on Unix systems, but are on Windows systems. If this setting is enabled, a path
 * such as "/one\two/three" will be converted to "/one/two/three". Implemented only for HTP_DECODER_URL_PATH.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] enabled
 */
/* *
 * Configures a best-fit map, which is used whenever characters longer than one byte
 * need to be converted to a single-byte. By default a Windows 1252 best-fit map is used.
 * The map is an list of triplets, the first 2 bytes being an UCS-2 character to map from,
 * and the third byte being the single byte to map to. Make sure that your map contains
 * the mappings to cover the full-width and half-width form characters (U+FF00-FFEF). The
 * last triplet in the map must be all zeros (3 NUL bytes).
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] map
 */
/* *
 * Sets the replacement character that will be used to in the lossy best-fit
 * mapping from multi-byte to single-byte streams. The question mark character
 * is used as the default replacement byte.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] replacement_byte
 */
/* *
 * Controls reaction to raw control characters in the data.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] unwanted
 */
/* *
 * Configures whether input data will be converted to lowercase. Useful when set on the
 * HTP_DECODER_URL_PATH context, in order to handle servers with case-insensitive filesystems.
 * Implemented only for HTP_DECODER_URL_PATH.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] enabled
 */
/* *
 * Enables or disables Multipart file extraction. This function can be invoked only
 * after a previous htp_config_set_tmpdir() invocation. Otherwise, the configuration
 * change will fail, and extraction will not be enabled. Disabled by default. Please
 * note that the built-in file extraction implementation uses synchronous I/O, which
 * means that it is not suitable for use in an event-driven container. There's an
 * upper limit to how many files can be created on the filesystem during a single
 * request. The limit exists in order to mitigate against a DoS attack with a
 * Multipart payload that contains hundreds and thousands of files (it's cheap for the
 * attacker to do this, but costly for the server to support it). The default limit
 * may be pretty conservative.
 *
 * @param[in] cfg
 * @param[in] extract_files 1 if you wish extraction to be enabled, 0 otherwise
 * @param[in] limit the maximum number of files allowed; use -1 to use the parser default.
 */
/* *
 * Configures the maximum size of the buffer LibHTP will use when all data is not available
 * in the current buffer (e.g., a very long header line that might span several packets). This
 * limit is controlled by the hard_limit parameter. The soft_limit parameter is not implemented.
 * 
 * @param[in] cfg
 * @param[in] soft_limit NOT IMPLEMENTED.
 * @param[in] hard_limit
 */
/* *
 * Configures the maximum memlimit LibHTP will pass to liblzma.
 *
 * @param[in] cfg
 * @param[in] memlimit
 */
/* *
 * Configures the maximum compression bomb size LibHTP will decompress.
 *
 * @param[in] cfg
 * @param[in] bomblimit
 */
/* *
 * Configures the desired log level.
 * 
 * @param[in] cfg
 * @param[in] log_level
 */
/* *
 * Configures how the server reacts to encoded NUL bytes. Some servers will stop at
 * at NUL, while some will respond with 400 or 404. When the termination option is not
 * used, the NUL byte will remain in the path.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] enabled
 */
/* *
 * Configures reaction to encoded NUL bytes in input data.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] unwanted
 */
/* *
 * Configures the handling of raw NUL bytes. If enabled, raw NUL terminates strings.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] enabled
 */
/* *
 * Configures how the server reacts to raw NUL bytes. Some servers will terminate
 * path at NUL, while some will respond with 400 or 404. When the termination option
 * is not used, the NUL byte will remain in the data.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] unwanted
 */
/* *
 * Enable or disable request HTTP Authentication parsing. Enabled by default.
 *
 * @param[in] cfg
 * @param[in] parse_request_auth
 */
/* *
 * Enable or disable request cookie parsing. Enabled by default.
 *
 * @param[in] cfg
 * @param[in] parse_request_cookies
 */
/* *
 * Configures whether consecutive path segment separators will be compressed. When enabled, a path
 * such as "/one//two" will be normalized to "/one/two". Backslash conversion and path segment separator
 * decoding are carried out before compression. For example, the path "/one\\/two\/%5cthree/%2f//four"
 * will be converted to "/one/two/three/four" (assuming all 3 options are enabled). Implemented only for
 * HTP_DECODER_URL_PATH.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] enabled
 */
/* *
 * Configures whether encoded path segment separators will be decoded. Apache does not do
 * this by default, but IIS does. If enabled, a path such as "/one%2ftwo" will be normalized
 * to "/one/two". If the backslash_separators option is also enabled, encoded backslash
 * characters will be converted too (and subsequently normalized to forward slashes). Implemented
 * only for HTP_DECODER_URL_PATH.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] enabled
 */
/* *
 * Configures reaction to encoded path separator characters (e.g., %2f). Implemented only for HTP_DECODER_URL_PATH.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] unwanted
 */
/* *
 * Configures whether plus characters are converted to spaces when decoding URL-encoded strings. This
 * is appropriate to do for parameters, but not for URLs. Only applies to contexts where decoding
 * is taking place.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] enabled
 */
/* *
 * Controls whether compressed response bodies will be automatically decompressed.
 *
 * @param[in] cfg
 * @param[in] enabled set to 1 to enable decompression, 0 otherwise
 */
/* *
 * Configure desired server personality.
 *
 * @param[in] cfg
 * @param[in] personality
 * @return HTP_OK if the personality is supported, HTP_ERROR if it isn't.
 */
/* *
 * Configures the path where temporary files should be stored. Must be set
 * in order to use the Multipart file extraction functionality.
 * 
 * @param[in] cfg
 * @param[in] tmpdir
 */
/* *
 * Configures whether transactions will be automatically destroyed once they
 * are processed and all callbacks invoked. This option is appropriate for
 * programs that process transactions as they are processed.
 *
 * @param[in] cfg
 * @param[in] tx_auto_destroy
 */
/* *
 * Associates provided opaque user data with the configuration.
 * 
 * @param[in] cfg
 * @param[in] user_data
 */
/* *
 * Configures whether %u-encoded sequences are decoded. Such sequences
 * will be treated as invalid URL encoding if decoding is not desirable.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] enabled
 */
/* *
 * Configures reaction to %u-encoded sequences in input data.
 * 
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] unwanted
 */
/* *
 * Configures how the server handles to invalid URL encoding.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] handling
 */
/* *
 * Configures how the server reacts to invalid URL encoding.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] unwanted
 */
/* *
 * Controls whether the data should be treated as UTF-8 and converted to a single-byte
 * stream using best-fit mapping. Implemented only for HTP_DECODER_URL_PATH.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] enabled
 */
/* *
 * Configures how the server reacts to invalid UTF-8 characters. This setting does
 * not affect path normalization; it only controls what response status will be expect for
 * a request that contains invalid UTF-8 characters. Implemented only for HTP_DECODER_URL_PATH.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] unwanted
 */
/* *
 * Configures how the server reacts to leading whitespace on the request line.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] unwanted
 */
/* *
 * Configures many layers of compression we try to decompress.
 *
 * @param[in] cfg
 * @param[in] limit 0 disables limit
 */
#[no_mangle]
pub unsafe extern "C" fn htp_config_set_response_decompression_layer_limit(mut cfg:
                                                                               *mut htp_cfg_t,
                                                                           mut limit:
                                                                               libc::c_int) {
    if cfg.is_null() { return }
    (*cfg).response_decompression_layer_limit = limit;
}
