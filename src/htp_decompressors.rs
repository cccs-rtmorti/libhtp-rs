use ::libc;
extern "C" {
    pub type internal_state;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn inflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn inflateEnd(strm: z_streamp) -> libc::c_int;
    #[no_mangle]
    fn crc32(crc: uLong, buf: *const Bytef, len: uInt) -> uLong;
    #[no_mangle]
    fn inflateInit2_(strm: z_streamp, windowBits: libc::c_int,
                     version: *const libc::c_char, stream_size: libc::c_int)
     -> libc::c_int;
    #[no_mangle]
    fn LzmaDec_Init(p: *mut CLzmaDec);
    #[no_mangle]
    fn LzmaDec_Allocate(p: *mut CLzmaDec, props: *const Byte,
                        propsSize: libc::c_uint, alloc: ISzAllocPtr) -> SRes;
    #[no_mangle]
    fn LzmaDec_Free(p: *mut CLzmaDec, alloc: ISzAllocPtr);
    /* ---------- Buffer Interface ---------- */
    /* It's zlib-like interface.
   See LzmaDec_DecodeToDic description for information about STEPS and return results,
   but you must use LzmaDec_DecodeToBuf instead of LzmaDec_DecodeToDic and you don't need
   to work with CLzmaDec variables manually.

finishMode:
  It has meaning only if the decoding reaches output limit (*destLen).
  LZMA_FINISH_ANY - Decode just destLen bytes.
  LZMA_FINISH_END - Stream must be finished after (*destLen).
*/
    #[no_mangle]
    fn LzmaDec_DecodeToBuf(p: *mut CLzmaDec, dest: *mut Byte,
                           destLen: *mut SizeT, src: *const Byte,
                           srcLen: *mut SizeT, finishMode: ELzmaFinishMode,
                           status: *mut ELzmaStatus, memlimit: SizeT) -> SRes;
    /* *
 * Frees all data contained in the uri, and then the uri itself.
 * 
 * @param[in] uri
 */
    /* *
 * Allocates and initializes a new htp_uri_t structure.
 *
 * @return New structure, or NULL on memory allocation failure.
 */
    /* *
 * Creates a new log entry and stores it with the connection. The file and line
 * parameters are typically auto-generated using the HTP_LOG_MARK macro.
*
 * @param[in] connp
 * @param[in] file
 * @param[in] line
 * @param[in] level
 * @param[in] code
 * @param[in] fmt
 * @param[in] ...
 */
    #[no_mangle]
    fn htp_log(connp: *mut htp_connp_t, file: *const libc::c_char,
               line: libc::c_int, level: htp_log_level_t, code: libc::c_int,
               fmt: *const libc::c_char, _: ...);
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
// Below are all htp_status_t return codes used by LibHTP. Enum is not
// used here to allow applications to define their own codes.
/* *
 * The lowest htp_status_t value LibHTP will use internally.
 */
/* * General-purpose error code. */
/* *
 * No processing or work was done. This is typically used by callbacks
 * to indicate that they were not interested in doing any work in the
 * given context.
 */
/* * Returned by a function when its work was successfully completed. */
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
/* *
     * This is the default value that is used before
     * the presence of authentication is determined (e.g.,
     * before request headers are seen).
     */
/* * No authentication. */
/* * HTTP Basic authentication used. */
/* * HTTP Digest authentication used. */
/* * Unrecognized authentication method. */
/* *
     * This is the default value, which is used until the presence
     * of content encoding is determined (e.g., before request headers
     * are seen.
     */
/* * No compression. */
/* * Gzip compression. */
/* * Deflate compression. */
/* * LZMA compression. */
/* *
 * Enumerates the possible request and response body codings.
 */
/* * Body coding not determined yet. */
/* * No body. */
/* * Identity coding is used, which means that the body was sent as is. */
/* * Chunked encoding. */
/* * We could not recognize the encoding. */
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
pub const HTP_COMPRESSION_LZMA: htp_content_encoding_t = 4;
pub const HTP_COMPRESSION_DEFLATE: htp_content_encoding_t = 3;
pub const HTP_COMPRESSION_GZIP: htp_content_encoding_t = 2;
pub const HTP_COMPRESSION_NONE: htp_content_encoding_t = 1;
pub const HTP_COMPRESSION_UNKNOWN: htp_content_encoding_t = 0;
pub type htp_transfer_coding_t = libc::c_uint;
pub const HTP_CODING_INVALID: htp_transfer_coding_t = 4;
pub const HTP_CODING_CHUNKED: htp_transfer_coding_t = 3;
pub const HTP_CODING_IDENTITY: htp_transfer_coding_t = 2;
pub const HTP_CODING_NO_BODY: htp_transfer_coding_t = 1;
pub const HTP_CODING_UNKNOWN: htp_transfer_coding_t = 0;
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
/* * This is the default value, used only until the first element is added. */
/* * Keys are copied.*/
/* * Keys are adopted and freed when the table is destroyed. */
/* * Keys are only referenced; the caller is still responsible for freeing them after the table is destroyed. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_table_t {
    pub list: htp_list_array_t,
    pub alloc_type: htp_table_alloc_t,
}
pub type htp_table_alloc_t = libc::c_uint;
pub const HTP_TABLE_KEYS_REFERENCED: htp_table_alloc_t = 3;
pub const HTP_TABLE_KEYS_ADOPTED: htp_table_alloc_t = 2;
pub const HTP_TABLE_KEYS_COPIED: htp_table_alloc_t = 1;
pub const HTP_TABLE_KEYS_ALLOC_UKNOWN: htp_table_alloc_t = 0;
pub type htp_auth_type_t = libc::c_uint;
pub const HTP_AUTH_UNRECOGNIZED: htp_auth_type_t = 9;
pub const HTP_AUTH_DIGEST: htp_auth_type_t = 3;
pub const HTP_AUTH_BASIC: htp_auth_type_t = 2;
pub const HTP_AUTH_NONE: htp_auth_type_t = 1;
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
pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
pub type voidpf = *mut libc::c_void;
pub type alloc_func
    =
    Option<unsafe extern "C" fn(_: voidpf, _: uInt, _: uInt) -> voidpf>;
pub type free_func = Option<unsafe extern "C" fn(_: voidpf, _: voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut libc::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: libc::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;
/* 7zTypes.h -- Basic types
2018-08-04 : Igor Pavlov : Public domain */
pub type SRes = libc::c_int;
pub type UInt16 = libc::c_ushort;
pub type UInt32 = libc::c_uint;
pub type SizeT = size_t;
/* Returns: result. (result != SZ_OK) means break.
       Value (UInt64)(Int64)-1 for size means unknown value. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ISzAlloc {
    pub Alloc: Option<unsafe extern "C" fn(_: ISzAllocPtr, _: size_t)
                          -> *mut libc::c_void>,
    pub Free: Option<unsafe extern "C" fn(_: ISzAllocPtr,
                                          _: *mut libc::c_void) -> ()>,
}
pub type ISzAllocPtr = *const ISzAlloc;
/* LzmaDec.h -- LZMA Decoder
2018-04-21 : Igor Pavlov : Public domain */
/* #define _LZMA_PROB32 */
/* _LZMA_PROB32 can increase the speed on some CPUs,
   but memory usage for CLzmaDec::probs will be doubled in that case */
pub type CLzmaProb = UInt16;
/* ---------- LZMA Properties ---------- */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _CLzmaProps {
    pub lc: Byte,
    pub lp: Byte,
    pub pb: Byte,
    pub _pad_: Byte,
    pub dicSize: UInt32,
}
pub type CLzmaProps = _CLzmaProps;
/* ---------- LZMA Decoder state ---------- */
/* LZMA_REQUIRED_INPUT_MAX = number of required input bytes for worst case.
   Num bits = log2((2^11 / 31) ^ 22) + 26 < 134 + 26 = 160; */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CLzmaDec {
    pub prop: CLzmaProps,
    pub probs: *mut CLzmaProb,
    pub probs_1664: *mut CLzmaProb,
    pub dic: *mut Byte,
    pub dicBufSize: SizeT,
    pub dicPos: SizeT,
    pub buf: *const Byte,
    pub range: UInt32,
    pub code: UInt32,
    pub processedPos: UInt32,
    pub checkDicSize: UInt32,
    pub reps: [UInt32; 4],
    pub state: UInt32,
    pub remainLen: UInt32,
    pub numProbs: UInt32,
    pub tempBufSize: libc::c_uint,
    pub tempBuf: [Byte; 20],
}
/* There are two types of LZMA streams:
     - Stream with end mark. That end mark adds about 6 bytes to compressed size.
     - Stream without end mark. You must know exact uncompressed size to decompress such stream. */
pub type ELzmaFinishMode = libc::c_uint;
/* block must be finished at the end */
/* finish at any point */
pub const LZMA_FINISH_END: ELzmaFinishMode = 1;
pub const LZMA_FINISH_ANY: ELzmaFinishMode = 0;
/* ELzmaFinishMode has meaning only if the decoding reaches output limit !!!

   You must use LZMA_FINISH_END, when you know that current output buffer
   covers last bytes of block. In other cases you must use LZMA_FINISH_ANY.

   If LZMA decoder sees end marker before reaching output limit, it returns SZ_OK,
   and output value of destLen will be less than output buffer size limit.
   You can check status result also.

   You can use multiple checks to test data integrity after full decompression:
     1) Check Result and "status" variable.
     2) Check that output(destLen) = uncompressedSize, if you know real uncompressedSize.
     3) Check that output(srcLen) = compressedSize, if you know real compressedSize.
        You must use correct finish mode in that case. */
pub type ELzmaStatus = libc::c_uint;
/* there is probability that stream was finished without end mark */
/* you must provide more input bytes */
pub const LZMA_STATUS_MAYBE_FINISHED_WITHOUT_MARK: ELzmaStatus = 4;
/* stream was not finished */
pub const LZMA_STATUS_NEEDS_MORE_INPUT: ELzmaStatus = 3;
/* stream was finished with end mark. */
pub const LZMA_STATUS_NOT_FINISHED: ELzmaStatus = 2;
/* use main error code instead */
pub const LZMA_STATUS_FINISHED_WITH_MARK: ELzmaStatus = 1;
pub const LZMA_STATUS_NOT_SPECIFIED: ELzmaStatus = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_decompressor_gzip_t {
    pub super_0: htp_decompressor_t,
    pub zlib_initialized: libc::c_int,
    pub restart: uint8_t,
    pub passthrough: uint8_t,
    pub stream: z_stream,
    pub header: [uint8_t; 13],
    pub header_len: uint8_t,
    pub state: CLzmaDec,
    pub buffer: *mut libc::c_uchar,
    pub crc: libc::c_ulong,
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
unsafe extern "C" fn SzAlloc(mut _p: ISzAllocPtr, mut size: size_t)
 -> *mut libc::c_void {
    return malloc(size);
}
unsafe extern "C" fn SzFree(mut _p: ISzAllocPtr,
                            mut address: *mut libc::c_void) {
    free(address);
}
#[no_mangle]
pub static mut lzma_Alloc: ISzAlloc =
        {
            let mut init =
                ISzAlloc{Alloc:
                             Some(SzAlloc as
                                      unsafe extern "C" fn(_: ISzAllocPtr,
                                                           _: size_t)
                                          -> *mut libc::c_void),
                         Free:
                             Some(SzFree as
                                      unsafe extern "C" fn(_: ISzAllocPtr,
                                                           _:
                                                               *mut libc::c_void)
                                          -> ()),};
            init
        };
/* *
 *  @brief See if the header has extensions
 *  @return number of bytes to skip
 */
unsafe extern "C" fn htp_gzip_decompressor_probe(mut data:
                                                     *const libc::c_uchar,
                                                 mut data_len: size_t)
 -> size_t {
    if data_len < 4 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as size_t
    }
    let mut consumed: size_t = 0 as libc::c_int as size_t;
    if *data.offset(0 as libc::c_int as isize) as libc::c_int ==
           0x1f as libc::c_int &&
           *data.offset(1 as libc::c_int as isize) as libc::c_int ==
               0x8b as libc::c_int &&
           *data.offset(3 as libc::c_int as isize) as libc::c_int !=
               0 as libc::c_int {
        if *data.offset(3 as libc::c_int as isize) as libc::c_int &
               (1 as libc::c_int) << 3 as libc::c_int != 0 ||
               *data.offset(3 as libc::c_int as isize) as libc::c_int &
                   (1 as libc::c_int) << 4 as libc::c_int != 0 {
            /* skip past
             * - FNAME extension, which is a name ended in a NUL terminator
             * or
             * - FCOMMENT extension, which is a commend ended in a NULL terminator
             */
            let mut len: size_t = 0;
            len = 10 as libc::c_int as size_t;
            while len < data_len &&
                      *data.offset(len as isize) as libc::c_int !=
                          '\u{0}' as i32 {
                len = len.wrapping_add(1)
            }
            consumed = len.wrapping_add(1 as libc::c_int as libc::c_ulong)
        } else if *data.offset(3 as libc::c_int as isize) as libc::c_int &
                      (1 as libc::c_int) << 1 as libc::c_int != 0 {
            consumed = 12 as libc::c_int as size_t
            //printf("skipped %u bytes for FHCRC header (GZIP)\n", 12);
        } else {
            //printf("GZIP unknown/unsupported flags %02X\n", data[3]);
            consumed = 10 as libc::c_int as size_t
        }
    }
    if consumed > data_len { return 0 as libc::c_int as size_t }
    return consumed;
}
/* *
 *  @brief restart the decompressor
 *  @return 1 if it restarted, 0 otherwise
 */
unsafe extern "C" fn htp_gzip_decompressor_restart(mut drec:
                                                       *mut htp_decompressor_gzip_t,
                                                   mut data:
                                                       *const libc::c_uchar,
                                                   mut data_len: size_t,
                                                   mut consumed_back:
                                                       *mut size_t)
 -> libc::c_int {
    let mut current_block: u64;
    let mut consumed: size_t = 0 as libc::c_int as size_t;
    let mut rc: libc::c_int = 0 as libc::c_int;
    if ((*drec).restart as libc::c_int) < 3 as libc::c_int {
        // first retry with the existing type, but now consider the
        // extensions
        if (*drec).restart as libc::c_int == 0 as libc::c_int {
            consumed = htp_gzip_decompressor_probe(data, data_len);
            if (*drec).zlib_initialized == HTP_COMPRESSION_GZIP as libc::c_int
               {
                // if that still fails, try the other method we support
                //printf("GZIP restart, consumed %u\n", (uint)consumed);
                rc =
                    inflateInit2_(&mut (*drec).stream,
                                  15 as libc::c_int + 32 as libc::c_int,
                                  b"1.2.11\x00" as *const u8 as
                                      *const libc::c_char,
                                  ::std::mem::size_of::<z_stream>() as
                                      libc::c_ulong as libc::c_int)
            } else {
                //printf("DEFLATE restart, consumed %u\n", (uint)consumed);
                rc =
                    inflateInit2_(&mut (*drec).stream, -(15 as libc::c_int),
                                  b"1.2.11\x00" as *const u8 as
                                      *const libc::c_char,
                                  ::std::mem::size_of::<z_stream>() as
                                      libc::c_ulong as libc::c_int)
            }
            if rc != 0 as libc::c_int { return 0 as libc::c_int }
            current_block = 5272667214186690925;
        } else if (*drec).zlib_initialized ==
                      HTP_COMPRESSION_DEFLATE as libc::c_int {
            rc =
                inflateInit2_(&mut (*drec).stream,
                              15 as libc::c_int + 32 as libc::c_int,
                              b"1.2.11\x00" as *const u8 as
                                  *const libc::c_char,
                              ::std::mem::size_of::<z_stream>() as
                                  libc::c_ulong as libc::c_int);
            if rc != 0 as libc::c_int { return 0 as libc::c_int }
            (*drec).zlib_initialized = HTP_COMPRESSION_GZIP as libc::c_int;
            consumed = htp_gzip_decompressor_probe(data, data_len);
            current_block = 5272667214186690925;
        } else if (*drec).zlib_initialized ==
                      HTP_COMPRESSION_GZIP as libc::c_int {
            rc =
                inflateInit2_(&mut (*drec).stream, -(15 as libc::c_int),
                              b"1.2.11\x00" as *const u8 as
                                  *const libc::c_char,
                              ::std::mem::size_of::<z_stream>() as
                                  libc::c_ulong as libc::c_int);
            if rc != 0 as libc::c_int { return 0 as libc::c_int }
            (*drec).zlib_initialized = HTP_COMPRESSION_DEFLATE as libc::c_int;
            consumed = htp_gzip_decompressor_probe(data, data_len);
            current_block = 5272667214186690925;
        } else { current_block = 14401909646449704462; }
        match current_block {
            14401909646449704462 => { }
            _ => {
                *consumed_back = consumed;
                (*drec).restart = (*drec).restart.wrapping_add(1);
                return 1 as libc::c_int
            }
        }
    }
    return 0 as libc::c_int;
}
/* *
 * Ends decompressor.
 *
 * @param[in] drec
 */
unsafe extern "C" fn htp_gzip_decompressor_end(mut drec:
                                                   *mut htp_decompressor_gzip_t) {
    if (*drec).zlib_initialized == HTP_COMPRESSION_LZMA as libc::c_int {
        LzmaDec_Free(&mut (*drec).state, &lzma_Alloc);
        (*drec).zlib_initialized = 0 as libc::c_int
    } else if (*drec).zlib_initialized != 0 {
        inflateEnd(&mut (*drec).stream);
        (*drec).zlib_initialized = 0 as libc::c_int
    };
}
/* *
 * Decompress a chunk of gzip-compressed data.
 * If we have more than one decompressor, call this function recursively.
 *
 * @param[in] drec
 * @param[in] d
 * @return HTP_OK on success, HTP_ERROR or some other negative integer on failure.
 */
unsafe extern "C" fn htp_gzip_decompressor_decompress(mut drec:
                                                          *mut htp_decompressor_gzip_t,
                                                      mut d:
                                                          *mut htp_tx_data_t)
 -> htp_status_t {
    let mut consumed: size_t = 0 as libc::c_int as size_t;
    let mut rc: libc::c_int = 0 as libc::c_int;
    let mut callback_rc: htp_status_t = 0;
    // Pass-through the NULL chunk, which indicates the end of the stream.
    if (*drec).passthrough != 0 {
        let mut d2: htp_tx_data_t =
            htp_tx_data_t{tx: 0 as *mut htp_tx_t,
                          data: 0 as *const libc::c_uchar,
                          len: 0,
                          is_last: 0,};
        d2.tx = (*d).tx;
        d2.data = (*d).data;
        d2.len = (*d).len;
        d2.is_last = (*d).is_last;
        callback_rc =
            (*drec).super_0.callback.expect("non-null function pointer")(&mut d2);
        if callback_rc != 1 as libc::c_int { return -(1 as libc::c_int) }
        return 1 as libc::c_int
    }
    if (*d).data.is_null() {
        // Prepare data for callback.
        let mut dout: htp_tx_data_t =
            htp_tx_data_t{tx: 0 as *mut htp_tx_t,
                          data: 0 as *const libc::c_uchar,
                          len: 0,
                          is_last: 0,};
        dout.tx = (*d).tx;
        // This is last call, so output uncompressed data so far
        dout.len =
            (8192 as libc::c_int as
                 libc::c_uint).wrapping_sub((*drec).stream.avail_out) as
                size_t;
        if dout.len > 0 as libc::c_int as libc::c_ulong {
            dout.data = (*drec).buffer
        } else { dout.data = 0 as *const libc::c_uchar }
        dout.is_last = (*d).is_last;
        if !(*drec).super_0.next.is_null() && (*drec).zlib_initialized != 0 {
            return htp_gzip_decompressor_decompress((*drec).super_0.next as
                                                        *mut htp_decompressor_gzip_t,
                                                    &mut dout)
        } else {
            // Send decompressed data to the callback.
            callback_rc =
                (*drec).super_0.callback.expect("non-null function pointer")(&mut dout);
            if callback_rc != 1 as libc::c_int {
                htp_gzip_decompressor_end(drec);
                return callback_rc
            }
        }
        return 1 as libc::c_int
    }
    'c_5645:
        loop 
             // we'll be restarting the compressor
             {
            if consumed > (*d).len {
                htp_log((*(*d).tx).connp,
                        b"htp_decompressors.c\x00" as *const u8 as
                            *const libc::c_char, 235 as libc::c_int,
                        HTP_LOG_ERROR, 0 as libc::c_int,
                        b"GZip decompressor: consumed > d->len\x00" as
                            *const u8 as *const libc::c_char);
                return -(1 as libc::c_int)
            }
            (*drec).stream.next_in =
                (*d).data.offset(consumed as isize) as *mut libc::c_uchar;
            (*drec).stream.avail_in = (*d).len.wrapping_sub(consumed) as uInt;
            while (*drec).stream.avail_in != 0 as libc::c_int as libc::c_uint
                  {
                // If there's no more data left in the
        // buffer, send that information out.
                if (*drec).stream.avail_out ==
                       0 as libc::c_int as libc::c_uint {
                    (*drec).crc =
                        crc32((*drec).crc, (*drec).buffer,
                              8192 as libc::c_int as uInt);
                    // Prepare data for callback.
                    let mut d2_0: htp_tx_data_t =
                        htp_tx_data_t{tx: 0 as *mut htp_tx_t,
                                      data: 0 as *const libc::c_uchar,
                                      len: 0,
                                      is_last: 0,};
                    d2_0.tx = (*d).tx;
                    d2_0.data = (*drec).buffer;
                    d2_0.len = 8192 as libc::c_int as size_t;
                    d2_0.is_last = (*d).is_last;
                    if !(*drec).super_0.next.is_null() &&
                           (*drec).zlib_initialized != 0 {
                        callback_rc =
                            htp_gzip_decompressor_decompress((*drec).super_0.next
                                                                 as
                                                                 *mut htp_decompressor_gzip_t,
                                                             &mut d2_0)
                    } else {
                        // Send decompressed data to callback.
                        callback_rc =
                            (*drec).super_0.callback.expect("non-null function pointer")(&mut d2_0)
                    }
                    if callback_rc != 1 as libc::c_int {
                        htp_gzip_decompressor_end(drec);
                        return callback_rc
                    }
                    (*drec).stream.next_out = (*drec).buffer;
                    (*drec).stream.avail_out = 8192 as libc::c_int as uInt
                }
                if (*drec).zlib_initialized ==
                       HTP_COMPRESSION_LZMA as libc::c_int {
                    if ((*drec).header_len as libc::c_int) <
                           5 as libc::c_int + 8 as libc::c_int {
                        consumed =
                            (5 as libc::c_int + 8 as libc::c_int -
                                 (*drec).header_len as libc::c_int) as size_t;
                        if consumed > (*drec).stream.avail_in as libc::c_ulong
                           {
                            consumed = (*drec).stream.avail_in as size_t
                        }
                        memcpy((*drec).header.as_mut_ptr().offset((*drec).header_len
                                                                      as
                                                                      libc::c_int
                                                                      as
                                                                      isize)
                                   as *mut libc::c_void,
                               (*drec).stream.next_in as *const libc::c_void,
                               consumed);
                        (*drec).stream.next_in =
                            (*d).data.offset(consumed as isize) as
                                *mut libc::c_uchar;
                        (*drec).stream.avail_in =
                            (*d).len.wrapping_sub(consumed) as uInt;
                        (*drec).header_len =
                            ((*drec).header_len as
                                 libc::c_ulong).wrapping_add(consumed) as
                                uint8_t as uint8_t
                    }
                    if (*drec).header_len as libc::c_int ==
                           5 as libc::c_int + 8 as libc::c_int {
                        rc =
                            LzmaDec_Allocate(&mut (*drec).state,
                                             (*drec).header.as_mut_ptr(),
                                             5 as libc::c_int as libc::c_uint,
                                             &lzma_Alloc);
                        if rc != 0 as libc::c_int { return rc }
                        LzmaDec_Init(&mut (*drec).state);
                        // hacky to get to next step end retry allocate in case of failure
                        (*drec).header_len =
                            (*drec).header_len.wrapping_add(1)
                    }
                    if (*drec).header_len as libc::c_int >
                           5 as libc::c_int + 8 as libc::c_int {
                        let mut inprocessed: size_t =
                            (*drec).stream.avail_in as size_t;
                        let mut outprocessed: size_t =
                            (*drec).stream.avail_out as size_t;
                        let mut status: ELzmaStatus =
                            LZMA_STATUS_NOT_SPECIFIED;
                        rc =
                            LzmaDec_DecodeToBuf(&mut (*drec).state,
                                                (*drec).stream.next_out,
                                                &mut outprocessed,
                                                (*drec).stream.next_in,
                                                &mut inprocessed,
                                                LZMA_FINISH_ANY, &mut status,
                                                (*(*(*d).tx).cfg).lzma_memlimit);
                        (*drec).stream.avail_in =
                            ((*drec).stream.avail_in as
                                 libc::c_ulong).wrapping_sub(inprocessed) as
                                uInt as uInt;
                        (*drec).stream.next_in =
                            (*drec).stream.next_in.offset(inprocessed as
                                                              isize);
                        (*drec).stream.avail_out =
                            ((*drec).stream.avail_out as
                                 libc::c_ulong).wrapping_sub(outprocessed) as
                                uInt as uInt;
                        (*drec).stream.next_out =
                            (*drec).stream.next_out.offset(outprocessed as
                                                               isize);
                        let mut current_block_82: u64;
                        match rc {
                            0 => {
                                rc = 0 as libc::c_int;
                                if status as libc::c_uint ==
                                       LZMA_STATUS_FINISHED_WITH_MARK as
                                           libc::c_int as libc::c_uint {
                                    rc = 1 as libc::c_int
                                }
                                current_block_82 = 17019156190352891614;
                            }
                            2 => {
                                htp_log((*(*d).tx).connp,
                                        b"htp_decompressors.c\x00" as
                                            *const u8 as *const libc::c_char,
                                        306 as libc::c_int, HTP_LOG_WARNING,
                                        0 as libc::c_int,
                                        b"LZMA decompressor: memory limit reached\x00"
                                            as *const u8 as
                                            *const libc::c_char);
                                current_block_82 = 1497605668091507245;
                            }
                            _ => { current_block_82 = 1497605668091507245; }
                        }
                        match current_block_82 {
                            1497605668091507245 =>
                            // fall through
                            {
                                rc = -(3 as libc::c_int)
                            }
                            _ => { }
                        }
                    }
                } else if (*drec).zlib_initialized != 0 {
                    rc = inflate(&mut (*drec).stream, 0 as libc::c_int)
                } else {
                    // no initialization means previous error on stream
                    return -(1 as libc::c_int)
                }
                if 8192 as libc::c_int as libc::c_uint >
                       (*drec).stream.avail_out {
                    if rc == -(3 as libc::c_int) {
                        // There is data even if there is an error
                // So use this data and log a warning
                        htp_log((*(*d).tx).connp,
                                b"htp_decompressors.c\x00" as *const u8 as
                                    *const libc::c_char, 322 as libc::c_int,
                                HTP_LOG_WARNING, 0 as libc::c_int,
                                b"GZip decompressor: inflate failed with %d\x00"
                                    as *const u8 as *const libc::c_char, rc);
                        rc = 1 as libc::c_int
                    }
                }
                if rc == 1 as libc::c_int {
                    // How many bytes do we have?
                    let mut len: size_t =
                        (8192 as libc::c_int as
                             libc::c_uint).wrapping_sub((*drec).stream.avail_out)
                            as size_t;
                    // Update CRC
                    // Prepare data for the callback.
                    let mut d2_1: htp_tx_data_t =
                        htp_tx_data_t{tx: 0 as *mut htp_tx_t,
                                      data: 0 as *const libc::c_uchar,
                                      len: 0,
                                      is_last: 0,};
                    d2_1.tx = (*d).tx;
                    d2_1.data = (*drec).buffer;
                    d2_1.len = len;
                    d2_1.is_last = (*d).is_last;
                    if !(*drec).super_0.next.is_null() &&
                           (*drec).zlib_initialized != 0 {
                        callback_rc =
                            htp_gzip_decompressor_decompress((*drec).super_0.next
                                                                 as
                                                                 *mut htp_decompressor_gzip_t,
                                                             &mut d2_1)
                    } else {
                        // Send decompressed data to the callback.
                        callback_rc =
                            (*drec).super_0.callback.expect("non-null function pointer")(&mut d2_1)
                    }
                    if callback_rc != 1 as libc::c_int {
                        htp_gzip_decompressor_end(drec);
                        return callback_rc
                    }
                    (*drec).stream.avail_out = 8192 as libc::c_int as uInt;
                    (*drec).stream.next_out = (*drec).buffer;
                    // TODO Handle trailer.
                    return 1 as libc::c_int
                } else {
                    if !(rc != 0 as libc::c_int) { continue ; }
                    htp_log((*(*d).tx).connp,
                            b"htp_decompressors.c\x00" as *const u8 as
                                *const libc::c_char, 356 as libc::c_int,
                            HTP_LOG_WARNING, 0 as libc::c_int,
                            b"GZip decompressor: inflate failed with %d\x00"
                                as *const u8 as *const libc::c_char, rc);
                    if (*drec).zlib_initialized ==
                           HTP_COMPRESSION_LZMA as libc::c_int {
                        LzmaDec_Free(&mut (*drec).state, &lzma_Alloc);
                        // so as to clean zlib ressources after restart
                        (*drec).zlib_initialized =
                            HTP_COMPRESSION_NONE as libc::c_int
                    } else { inflateEnd(&mut (*drec).stream); }
                    // see if we want to restart the decompressor
                    if htp_gzip_decompressor_restart(drec, (*d).data,
                                                     (*d).len, &mut consumed)
                           == 1 as libc::c_int {
                        continue 'c_5645 ;
                    }
                    (*drec).zlib_initialized = 0 as libc::c_int;
                    // all our inflate attempts have failed, simply
            // pass the raw data on to the callback in case
            // it's not compressed at all
                    let mut d2_2: htp_tx_data_t =
                        htp_tx_data_t{tx: 0 as *mut htp_tx_t,
                                      data: 0 as *const libc::c_uchar,
                                      len: 0,
                                      is_last: 0,};
                    d2_2.tx = (*d).tx;
                    d2_2.data = (*d).data;
                    d2_2.len = (*d).len;
                    d2_2.is_last = (*d).is_last;
                    callback_rc =
                        (*drec).super_0.callback.expect("non-null function pointer")(&mut d2_2);
                    if callback_rc != 1 as libc::c_int {
                        return -(1 as libc::c_int)
                    }
                    (*drec).stream.avail_out = 8192 as libc::c_int as uInt;
                    (*drec).stream.next_out = (*drec).buffer;
                    /* successfully passed through, lets continue doing that */
                    (*drec).passthrough = 1 as libc::c_int as uint8_t;
                    return 1 as libc::c_int
                }
            }
            return 1 as libc::c_int
        };
}
/* *
 * Shut down gzip decompressor.
 *
 * @param[in] drec
 */
unsafe extern "C" fn htp_gzip_decompressor_destroy(mut drec:
                                                       *mut htp_decompressor_gzip_t) {
    if drec.is_null() { return }
    htp_gzip_decompressor_end(drec);
    free((*drec).buffer as *mut libc::c_void);
    free(drec as *mut libc::c_void);
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
/* *< deflate restarted to try rfc1950 instead of 1951 */
/* *< decompression failed, pass through raw data */
/* *
 * Create a new decompressor instance.
 *
 * @param[in] connp
 * @param[in] format
 * @return New htp_decompressor_t instance on success, or NULL on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_gzip_decompressor_create(mut connp:
                                                          *mut htp_connp_t,
                                                      mut format:
                                                          htp_content_encoding_t)
 -> *mut htp_decompressor_t {
    let mut drec: *mut htp_decompressor_gzip_t =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<htp_decompressor_gzip_t>() as
                   libc::c_ulong) as *mut htp_decompressor_gzip_t;
    if drec.is_null() { return 0 as *mut htp_decompressor_t }
    (*drec).super_0.decompress =
        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                *mut htp_decompressor_gzip_t,
                                                            _:
                                                                *mut htp_tx_data_t)
                                           -> htp_status_t>,
                                Option<unsafe extern "C" fn(_:
                                                                *mut htp_decompressor_t,
                                                            _:
                                                                *mut htp_tx_data_t)
                                           ->
                                               libc::c_int>>(Some(htp_gzip_decompressor_decompress
                                                                      as
                                                                      unsafe extern "C" fn(_:
                                                                                               *mut htp_decompressor_gzip_t,
                                                                                           _:
                                                                                               *mut htp_tx_data_t)
                                                                          ->
                                                                              htp_status_t));
    (*drec).super_0.destroy =
        ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                *mut htp_decompressor_gzip_t)
                                           -> ()>,
                                Option<unsafe extern "C" fn(_:
                                                                *mut htp_decompressor_t)
                                           ->
                                               ()>>(Some(htp_gzip_decompressor_destroy
                                                             as
                                                             unsafe extern "C" fn(_:
                                                                                      *mut htp_decompressor_gzip_t)
                                                                 -> ()));
    (*drec).super_0.next = 0 as *mut htp_decompressor_t;
    (*drec).buffer =
        malloc(8192 as libc::c_int as libc::c_ulong) as *mut libc::c_uchar;
    if (*drec).buffer.is_null() {
        free(drec as *mut libc::c_void);
        return 0 as *mut htp_decompressor_t
    }
    // Initialize zlib.
    let mut rc: libc::c_int = 0;
    match format as libc::c_uint {
        4 => {
            if (*(*connp).cfg).lzma_memlimit >
                   0 as libc::c_int as libc::c_ulong {
                (*drec).state.dic = 0 as *mut Byte;
                (*drec).state.probs = 0 as *mut CLzmaProb
            } else {
                htp_log(connp,
                        b"htp_decompressors.c\x00" as *const u8 as
                            *const libc::c_char, 445 as libc::c_int,
                        HTP_LOG_WARNING, 0 as libc::c_int,
                        b"LZMA decompression disabled\x00" as *const u8 as
                            *const libc::c_char);
                (*drec).passthrough = 1 as libc::c_int as uint8_t
            }
            rc = 0 as libc::c_int
        }
        3 => {
            // Negative values activate raw processing,
            // which is what we need for deflate.
            rc =
                inflateInit2_(&mut (*drec).stream, -(15 as libc::c_int),
                              b"1.2.11\x00" as *const u8 as
                                  *const libc::c_char,
                              ::std::mem::size_of::<z_stream>() as
                                  libc::c_ulong as libc::c_int)
        }
        2 => {
            // Increased windows size activates gzip header processing.
            rc =
                inflateInit2_(&mut (*drec).stream,
                              15 as libc::c_int + 32 as libc::c_int,
                              b"1.2.11\x00" as *const u8 as
                                  *const libc::c_char,
                              ::std::mem::size_of::<z_stream>() as
                                  libc::c_ulong as libc::c_int)
        }
        _ => {
            // do nothing
            rc = -(3 as libc::c_int)
        }
    }
    if rc != 0 as libc::c_int {
        htp_log(connp,
                b"htp_decompressors.c\x00" as *const u8 as
                    *const libc::c_char, 465 as libc::c_int, HTP_LOG_ERROR,
                0 as libc::c_int,
                b"GZip decompressor: inflateInit2 failed with code %d\x00" as
                    *const u8 as *const libc::c_char, rc);
        if format as libc::c_uint ==
               HTP_COMPRESSION_DEFLATE as libc::c_int as libc::c_uint ||
               format as libc::c_uint ==
                   HTP_COMPRESSION_GZIP as libc::c_int as libc::c_uint {
            inflateEnd(&mut (*drec).stream);
        }
        free((*drec).buffer as *mut libc::c_void);
        free(drec as *mut libc::c_void);
        return 0 as *mut htp_decompressor_t
    }
    (*drec).zlib_initialized = format as libc::c_int;
    (*drec).stream.avail_out = 8192 as libc::c_int as uInt;
    (*drec).stream.next_out = (*drec).buffer;
    return drec as *mut htp_decompressor_t;
}
