use ::libc;
extern "C" {
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn tolower(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong,
                _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn vsnprintf(_: *mut libc::c_char, _: libc::c_ulong,
                 _: *const libc::c_char, _: ::std::ffi::VaList)
     -> libc::c_int;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memchr(_: *const libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    /* *
 * Add new element to the end of the list, expanding the list as necessary.
 *
 * @param[in] l
 * @param[in] e
 * @return HTP_OK on success or HTP_ERROR on failure.
 *
 */
    #[no_mangle]
    fn htp_list_array_push(l: *mut htp_list_array_t, e: *mut libc::c_void)
     -> htp_status_t;
    /* *
 * Append as many bytes from the source to destination bstring. The
 * destination storage will not be expanded if there is not enough space in it
 * already to accommodate all of the data.
 *
 * @param[in] b
 * @param[in] cstr
 * @return The destination bstring.
 */
    #[no_mangle]
    fn bstr_add_c_noex(b: *mut bstr, cstr: *const libc::c_char) -> *mut bstr;
    /* *
 * Append as many bytes from the source bstring to destination bstring. The
 * destination storage will not be expanded if there is not enough space in it
 * already to accommodate all of the data.
 *
 * @param[in] bdestination
 * @param[in] bsource
 * @return The destination bstring.
 */
    #[no_mangle]
    fn bstr_add_noex(bdestination: *mut bstr, bsource: *const bstr)
     -> *mut bstr;
    /* *
 * Adjust bstring length. You will need to use this method whenever
 * you work directly with the string contents, and end up changing
 * its length by direct structure manipulation.
 *
 * @param[in] b
 * @param[in] newlen
 */
    #[no_mangle]
    fn bstr_adjust_len(b: *mut bstr, newlen: size_t);
    /* *
 * Allocate a zero-length bstring, reserving space for at least size bytes.
 *
 * @param[in] size
 * @return New string instance
 */
    #[no_mangle]
    fn bstr_alloc(size: size_t) -> *mut bstr;
    /* *
 * Return the byte at the given position, counting from the end of the string (e.g.,
 * byte at position 0 is the last byte in the string.)
 *
 * @param[in] b
 * @param[in] pos
 * @return The byte at the given location, or -1 if the position is out of range.
 */
    #[no_mangle]
    fn bstr_char_at_end(b: *const bstr, pos: size_t) -> libc::c_int;
    /* *
 * Remove the last byte from bstring, assuming it contains at least one byte. This
 * function will not reduce the storage that backs the string, only the amount
 * of data used.
 *
 * @param[in] b
 */
    #[no_mangle]
    fn bstr_chop(b: *mut bstr);
    /* *
 * Case-sensitive comparison of a bstring and a NUL-terminated string.
 *
 * @param[in] b
 * @param[in] cstr
 * @return Zero on string match, 1 if b is greater than cstr, and -1 if cstr is
 *         greater than b.
 */
    #[no_mangle]
    fn bstr_cmp_c(b: *const bstr, cstr: *const libc::c_char) -> libc::c_int;
    /* *
 * Create a new bstring by copying the provided bstring.
 *
 * @param[in] b
 * @return New bstring, or NULL if memory allocation failed.
 */
    #[no_mangle]
    fn bstr_dup(b: *const bstr) -> *mut bstr;
    /* *
 * Create a new bstring by copying a part of the provided bstring.
 *
 * @param[in] b
 * @param[in] offset
 * @param[in] len
 * @return New bstring, or NULL if memory allocation failed.
 */
    #[no_mangle]
    fn bstr_dup_ex(b: *const bstr, offset: size_t, len: size_t) -> *mut bstr;
    /* *
 * Create a copy of the provided bstring, then convert it to lowercase.
 *
 * @param[in] b
 * @return New bstring, or NULL if memory allocation failed
 */
    #[no_mangle]
    fn bstr_dup_lower(b: *const bstr) -> *mut bstr;
    /* *
 * Create a new bstring by copying the provided memory region.
 *
 * @param[in] data
 * @param[in] len
 * @return New bstring, or NULL if memory allocation failed
 */
    #[no_mangle]
    fn bstr_dup_mem(data: *const libc::c_void, len: size_t) -> *mut bstr;
    /* *
 * Deallocate the supplied bstring instance and set it to NULL. Allows NULL on
 * input.
 *
 * @param[in] b
 */
    #[no_mangle]
    fn bstr_free(b: *mut bstr);
    /* *
 * Convert bstring to lowercase. This function converts the supplied string,
 * it does not create a new string.
 *
 * @param[in] b
 * @return The same bstring received on input
 */
    #[no_mangle]
    fn bstr_to_lowercase(b: *mut bstr) -> *mut bstr;
    /* *
 * Convert contents of a memory region to a positive integer.
 *
 * @param[in] data
 * @param[in] len
 * @param[in] base The desired number base.
 * @param[in] lastlen Points to the first unused byte in the region
 * @return If the conversion was successful, this function returns the
 *         number. When the conversion fails, -1 will be returned when not
 *         one valid digit was found, and -2 will be returned if an overflow
 *         occurred.
 */
    #[no_mangle]
    fn bstr_util_mem_to_pint(data: *const libc::c_void, len: size_t,
                             base: libc::c_int, lastlen: *mut size_t)
     -> int64_t;
    /* *
 * Removes whitespace from the beginning and the end of a memory region. The data
 * itself is not modified; this function only adjusts the provided pointers.
 *
 * @param[in,out] data
 * @param[in,out] len
 */
    #[no_mangle]
    fn bstr_util_mem_trim(data: *mut *mut libc::c_uchar, len: *mut size_t);
    /* *
 * Runs all the callbacks associated with a given hook. Only stops if
 * one of the callbacks returns an error (HTP_ERROR) or stop (HTP_STOP).
 *
 * @param[in] hook
 * @param[in] user_data
 * @return HTP_OK if at least one hook ran successfully, HTP_STOP if there was
 *         no error but processing should stop, and HTP_ERROR or any other value
 *         less than zero on error.
 */
    #[no_mangle]
    fn htp_hook_run_all(hook: *mut htp_hook_t, user_data: *mut libc::c_void)
     -> htp_status_t;
    #[no_mangle]
    fn htp_utf8_decode_allow_overlong(state: *mut uint32_t,
                                      codep: *mut uint32_t, byte: uint32_t)
     -> uint32_t;
    #[no_mangle]
    fn htp_connp_REQ_IDLE(connp: *mut htp_connp_t) -> htp_status_t;
    #[no_mangle]
    fn htp_connp_REQ_LINE(connp: *mut htp_connp_t) -> htp_status_t;
    #[no_mangle]
    fn htp_connp_REQ_PROTOCOL(connp: *mut htp_connp_t) -> htp_status_t;
    #[no_mangle]
    fn htp_connp_REQ_HEADERS(connp: *mut htp_connp_t) -> htp_status_t;
    #[no_mangle]
    fn htp_connp_REQ_CONNECT_CHECK(connp: *mut htp_connp_t) -> htp_status_t;
    #[no_mangle]
    fn htp_connp_REQ_CONNECT_WAIT_RESPONSE(connp: *mut htp_connp_t)
     -> htp_status_t;
    #[no_mangle]
    fn htp_connp_REQ_BODY_DETERMINE(connp: *mut htp_connp_t) -> htp_status_t;
    #[no_mangle]
    fn htp_connp_REQ_BODY_IDENTITY(connp: *mut htp_connp_t) -> htp_status_t;
    #[no_mangle]
    fn htp_connp_REQ_BODY_CHUNKED_LENGTH(connp: *mut htp_connp_t)
     -> htp_status_t;
    #[no_mangle]
    fn htp_connp_REQ_BODY_CHUNKED_DATA(connp: *mut htp_connp_t)
     -> htp_status_t;
    #[no_mangle]
    fn htp_connp_REQ_BODY_CHUNKED_DATA_END(connp: *mut htp_connp_t)
     -> htp_status_t;
    #[no_mangle]
    fn htp_connp_REQ_FINALIZE(connp: *mut htp_connp_t) -> htp_status_t;
    #[no_mangle]
    fn htp_connp_REQ_IGNORE_DATA_AFTER_HTTP_0_9(connp: *mut htp_connp_t)
     -> htp_status_t;
    #[no_mangle]
    fn htp_connp_RES_IDLE(connp: *mut htp_connp_t) -> htp_status_t;
    #[no_mangle]
    fn htp_connp_RES_LINE(connp: *mut htp_connp_t) -> htp_status_t;
    #[no_mangle]
    fn htp_connp_RES_HEADERS(connp: *mut htp_connp_t) -> htp_status_t;
    #[no_mangle]
    fn htp_connp_RES_BODY_DETERMINE(connp: *mut htp_connp_t) -> htp_status_t;
    #[no_mangle]
    fn htp_connp_RES_BODY_IDENTITY_CL_KNOWN(connp: *mut htp_connp_t)
     -> htp_status_t;
    #[no_mangle]
    fn htp_connp_RES_BODY_IDENTITY_STREAM_CLOSE(connp: *mut htp_connp_t)
     -> htp_status_t;
    #[no_mangle]
    fn htp_connp_RES_BODY_CHUNKED_LENGTH(connp: *mut htp_connp_t)
     -> htp_status_t;
    #[no_mangle]
    fn htp_connp_RES_BODY_CHUNKED_DATA(connp: *mut htp_connp_t)
     -> htp_status_t;
    #[no_mangle]
    fn htp_connp_RES_BODY_CHUNKED_DATA_END(connp: *mut htp_connp_t)
     -> htp_status_t;
    #[no_mangle]
    fn htp_connp_RES_FINALIZE(connp: *mut htp_connp_t) -> htp_status_t;
    #[no_mangle]
    fn strlcat(dst: *mut libc::c_char, src: *const libc::c_char, size: size_t)
     -> size_t;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type size_t = libc::c_ulong;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
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
/* *
 * Used to represent files that are seen during the processing of HTTP traffic. Most
 * commonly this refers to files seen in multipart/form-data payloads. In addition, PUT
 * request bodies can be treated as files.
 */
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
/* *
 * Represents a single HTTP transaction, which is a combination of a request and a response.
 */
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
pub type htp_part_mode_t = libc::c_uint;
/* * When in data mode, the parser is consuming part data. */
pub const MODE_DATA: htp_part_mode_t = 1;
/* * When in line mode, the parser is handling part headers. */
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
/* * Used after a CR byte is detected in STATE_BOUNDARY_EAT_LWS. */
pub const STATE_BOUNDARY_EAT_LWS_CR: htp_multipart_state_t = 6;
/* * Consuming linear whitespace after a boundary. */
pub const STATE_BOUNDARY_EAT_LWS: htp_multipart_state_t = 5;
/* * Checking the second byte after a boundary. */
pub const STATE_BOUNDARY_IS_LAST2: htp_multipart_state_t = 4;
/* * Checking the first byte after a boundary. */
pub const STATE_BOUNDARY_IS_LAST1: htp_multipart_state_t = 3;
/* * Testing a potential boundary. */
pub const STATE_BOUNDARY: htp_multipart_state_t = 2;
/* * Processing data, waiting for a new line (which might indicate a new boundary). */
pub const STATE_DATA: htp_multipart_state_t = 1;
/* * Initial state, after the parser has been created but before the boundary initialized. */
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
/* *
 * Represents a single log entry.
 */
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
/* *
 * Represents a chunk of file data.
 */
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
 * Is character a linear white space character?
 *
 * @param[in] c
 * @return 0 or 1
 */
#[no_mangle]
pub unsafe extern "C" fn htp_is_lws(mut c: libc::c_int) -> libc::c_int {
    if c == ' ' as i32 || c == '\t' as i32 {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
/* *
 * Is character a separator character?
 *
 * @param[in] c
 * @return 0 or 1
 */
#[no_mangle]
pub unsafe extern "C" fn htp_is_separator(mut c: libc::c_int) -> libc::c_int {
    /* separators = "(" | ")" | "<" | ">" | "@"
                  | "," | ";" | ":" | "\" | <">
                  | "/" | "[" | "]" | "?" | "="
                  | "{" | "}" | SP | HT         */
    match c {
        40 | 41 | 60 | 62 | 64 | 44 | 59 | 58 | 92 | 34 | 47 | 91 | 93 | 63 |
        61 | 123 | 125 | 32 | 9 => {
            return 1 as libc::c_int
        }
        _ => { return 0 as libc::c_int }
    };
}
/* *
 * Is character a text character?
 *
 * @param[in] c
 * @return 0 or 1
 */
#[no_mangle]
pub unsafe extern "C" fn htp_is_text(mut c: libc::c_int) -> libc::c_int {
    if c == '\t' as i32 { return 1 as libc::c_int }
    if c < 32 as libc::c_int { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
/* *
 * Is character a token character?
 *
 * @param[in] c
 * @return 0 or 1
 */
#[no_mangle]
pub unsafe extern "C" fn htp_is_token(mut c: libc::c_int) -> libc::c_int {
    /* token = 1*<any CHAR except CTLs or separators> */
    /* CHAR  = <any US-ASCII character (octets 0 - 127)> */
    if c < 32 as libc::c_int || c > 126 as libc::c_int {
        return 0 as libc::c_int
    }
    if htp_is_separator(c) != 0 { return 0 as libc::c_int }
    return 1 as libc::c_int;
}
/* *
 * Remove all line terminators (LF, CR or CRLF) from
 * the end of the line provided as input.
 *
 * @return 0 if nothing was removed, 1 if one or more LF characters were removed, or
 *         2 if one or more CR and/or LF characters were removed.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_chomp(mut data: *mut libc::c_uchar,
                                   mut len: *mut size_t) -> libc::c_int {
    let mut r: libc::c_int = 0 as libc::c_int;
    // Loop until there's no more stuff in the buffer
    while *len > 0 as libc::c_int as libc::c_ulong {
        // Try one LF first
        if *data.offset((*len).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as isize) as libc::c_int == '\n' as i32 {
            *len = (*len).wrapping_sub(1);
            r = 1 as libc::c_int;
            if *len == 0 as libc::c_int as libc::c_ulong { return r }
            // A CR is allowed before LF
            if *data.offset((*len).wrapping_sub(1 as libc::c_int as
                                                    libc::c_ulong) as isize)
                   as libc::c_int == '\r' as i32 {
                *len = (*len).wrapping_sub(1);
                r = 2 as libc::c_int
            }
        } else if *data.offset((*len).wrapping_sub(1 as libc::c_int as
                                                       libc::c_ulong) as
                                   isize) as libc::c_int == '\r' as i32 {
            *len = (*len).wrapping_sub(1);
            r = 1 as libc::c_int
        } else { return r }
    }
    return r;
}
/* *
 * Is character a white space character?
 *
 * @param[in] c
 * @return 0 or 1
 */
#[no_mangle]
pub unsafe extern "C" fn htp_is_space(mut c: libc::c_int) -> libc::c_int {
    match c {
        32 | 12 | 11 | 9 | 13 | 10 => { return 1 as libc::c_int }
        _ => { return 0 as libc::c_int }
    };
}
/* *
 * Converts request method, given as a string, into a number.
 *
 * @param[in] method
 * @return Method number of M_UNKNOWN
 */
#[no_mangle]
pub unsafe extern "C" fn htp_convert_method_to_number(mut method: *mut bstr)
 -> libc::c_int {
    if method.is_null() { return HTP_M_UNKNOWN as libc::c_int }
    // TODO Optimize using parallel matching, or something similar.
    if bstr_cmp_c(method, b"GET\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        return HTP_M_GET as libc::c_int
    }
    if bstr_cmp_c(method, b"PUT\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        return HTP_M_PUT as libc::c_int
    }
    if bstr_cmp_c(method, b"POST\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        return HTP_M_POST as libc::c_int
    }
    if bstr_cmp_c(method, b"DELETE\x00" as *const u8 as *const libc::c_char)
           == 0 as libc::c_int {
        return HTP_M_DELETE as libc::c_int
    }
    if bstr_cmp_c(method, b"CONNECT\x00" as *const u8 as *const libc::c_char)
           == 0 as libc::c_int {
        return HTP_M_CONNECT as libc::c_int
    }
    if bstr_cmp_c(method, b"OPTIONS\x00" as *const u8 as *const libc::c_char)
           == 0 as libc::c_int {
        return HTP_M_OPTIONS as libc::c_int
    }
    if bstr_cmp_c(method, b"TRACE\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        return HTP_M_TRACE as libc::c_int
    }
    if bstr_cmp_c(method, b"PATCH\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        return HTP_M_PATCH as libc::c_int
    }
    if bstr_cmp_c(method, b"PROPFIND\x00" as *const u8 as *const libc::c_char)
           == 0 as libc::c_int {
        return HTP_M_PROPFIND as libc::c_int
    }
    if bstr_cmp_c(method,
                  b"PROPPATCH\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        return HTP_M_PROPPATCH as libc::c_int
    }
    if bstr_cmp_c(method, b"MKCOL\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        return HTP_M_MKCOL as libc::c_int
    }
    if bstr_cmp_c(method, b"COPY\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        return HTP_M_COPY as libc::c_int
    }
    if bstr_cmp_c(method, b"MOVE\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        return HTP_M_MOVE as libc::c_int
    }
    if bstr_cmp_c(method, b"LOCK\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        return HTP_M_LOCK as libc::c_int
    }
    if bstr_cmp_c(method, b"UNLOCK\x00" as *const u8 as *const libc::c_char)
           == 0 as libc::c_int {
        return HTP_M_UNLOCK as libc::c_int
    }
    if bstr_cmp_c(method,
                  b"VERSION-CONTROL\x00" as *const u8 as *const libc::c_char)
           == 0 as libc::c_int {
        return HTP_M_VERSION_CONTROL as libc::c_int
    }
    if bstr_cmp_c(method, b"CHECKOUT\x00" as *const u8 as *const libc::c_char)
           == 0 as libc::c_int {
        return HTP_M_CHECKOUT as libc::c_int
    }
    if bstr_cmp_c(method,
                  b"UNCHECKOUT\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        return HTP_M_UNCHECKOUT as libc::c_int
    }
    if bstr_cmp_c(method, b"CHECKIN\x00" as *const u8 as *const libc::c_char)
           == 0 as libc::c_int {
        return HTP_M_CHECKIN as libc::c_int
    }
    if bstr_cmp_c(method, b"UPDATE\x00" as *const u8 as *const libc::c_char)
           == 0 as libc::c_int {
        return HTP_M_UPDATE as libc::c_int
    }
    if bstr_cmp_c(method, b"LABEL\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        return HTP_M_LABEL as libc::c_int
    }
    if bstr_cmp_c(method, b"REPORT\x00" as *const u8 as *const libc::c_char)
           == 0 as libc::c_int {
        return HTP_M_REPORT as libc::c_int
    }
    if bstr_cmp_c(method,
                  b"MKWORKSPACE\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        return HTP_M_MKWORKSPACE as libc::c_int
    }
    if bstr_cmp_c(method,
                  b"MKACTIVITY\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        return HTP_M_MKACTIVITY as libc::c_int
    }
    if bstr_cmp_c(method,
                  b"BASELINE-CONTROL\x00" as *const u8 as *const libc::c_char)
           == 0 as libc::c_int {
        return HTP_M_BASELINE_CONTROL as libc::c_int
    }
    if bstr_cmp_c(method, b"MERGE\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        return HTP_M_MERGE as libc::c_int
    }
    if bstr_cmp_c(method, b"INVALID\x00" as *const u8 as *const libc::c_char)
           == 0 as libc::c_int {
        return HTP_M_INVALID as libc::c_int
    }
    if bstr_cmp_c(method, b"HEAD\x00" as *const u8 as *const libc::c_char) ==
           0 as libc::c_int {
        return HTP_M_HEAD as libc::c_int
    }
    return HTP_M_UNKNOWN as libc::c_int;
}
/* *
 * Is the given line empty?
 *
 * @param[in] data
 * @param[in] len
 * @return 0 or 1
 */
#[no_mangle]
pub unsafe extern "C" fn htp_is_line_empty(mut data: *mut libc::c_uchar,
                                           mut len: size_t) -> libc::c_int {
    if len == 1 as libc::c_int as libc::c_ulong ||
           len == 2 as libc::c_int as libc::c_ulong &&
               *data.offset(0 as libc::c_int as isize) as libc::c_int ==
                   '\r' as i32 &&
               *data.offset(1 as libc::c_int as isize) as libc::c_int ==
                   '\n' as i32 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* *
 * Does line consist entirely of whitespace characters?
 * 
 * @param[in] data
 * @param[in] len
 * @return 0 or 1
 */
#[no_mangle]
pub unsafe extern "C" fn htp_is_line_whitespace(mut data: *mut libc::c_uchar,
                                                mut len: size_t)
 -> libc::c_int {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < len {
        if *(*__ctype_b_loc()).offset(*data.offset(i as isize) as libc::c_int
                                          as isize) as libc::c_int &
               _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0 {
            return 0 as libc::c_int
        }
        i = i.wrapping_add(1)
    }
    return 1 as libc::c_int;
}
/* *
 * Parses Content-Length string (positive decimal number).
 * White space is allowed before and after the number.
 *
 * @param[in] b
 * @return Content-Length as a number, or -1 on error.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_parse_content_length(mut b: *mut bstr,
                                                  mut connp: *mut htp_connp_t)
 -> int64_t {
    let mut len: size_t = (*b).len;
    let mut data: *mut libc::c_uchar =
        if (*b).realptr.is_null() {
            (b as
                 *mut libc::c_uchar).offset(::std::mem::size_of::<bstr>() as
                                                libc::c_ulong as isize)
        } else { (*b).realptr };
    let mut pos: size_t = 0 as libc::c_int as size_t;
    let mut r: int64_t = 0 as libc::c_int as int64_t;
    if len == 0 as libc::c_int as libc::c_ulong {
        return -(1003 as libc::c_int) as int64_t
    }
    // Ignore junk before
    while pos < len &&
              ((*data.offset(pos as isize) as libc::c_int) < '0' as i32 ||
                   *data.offset(pos as isize) as libc::c_int > '9' as i32) {
        if htp_is_lws(*data.offset(pos as isize) as libc::c_int) == 0 &&
               !connp.is_null() && r == 0 as libc::c_int as libc::c_long {
            htp_log(connp,
                    b"htp_util.c\x00" as *const u8 as *const libc::c_char,
                    267 as libc::c_int, HTP_LOG_WARNING, 0 as libc::c_int,
                    b"C-L value with extra data in the beginnning\x00" as
                        *const u8 as *const libc::c_char);
            r = -(1 as libc::c_int) as int64_t
        }
        pos = pos.wrapping_add(1)
    }
    if pos == len { return -(1001 as libc::c_int) as int64_t }
    r =
        bstr_util_mem_to_pint(data.offset(pos as isize) as
                                  *const libc::c_void, len.wrapping_sub(pos),
                              10 as libc::c_int, &mut pos);
    // Ok to have junk afterwards
    if pos < len && !connp.is_null() {
        htp_log(connp, b"htp_util.c\x00" as *const u8 as *const libc::c_char,
                278 as libc::c_int, HTP_LOG_WARNING, 0 as libc::c_int,
                b"C-L value with extra data in the end\x00" as *const u8 as
                    *const libc::c_char);
    }
    return r;
}
/* *
 * Parses chunk length (positive hexadecimal number). White space is allowed before
 * and after the number. An error will be returned if the chunk length is greater than
 * INT32_MAX.
 *
 * @param[in] data
 * @param[in] len
 * @return Chunk length, or a negative number on error.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_parse_chunked_length(mut data:
                                                      *mut libc::c_uchar,
                                                  mut len: size_t)
 -> int64_t {
    // skip leading line feeds and other control chars
    while len != 0 {
        let mut c: libc::c_uchar = *data;
        if !(c as libc::c_int == 0xd as libc::c_int ||
                 c as libc::c_int == 0xa as libc::c_int ||
                 c as libc::c_int == 0x20 as libc::c_int ||
                 c as libc::c_int == 0x9 as libc::c_int ||
                 c as libc::c_int == 0xb as libc::c_int ||
                 c as libc::c_int == 0xc as libc::c_int) {
            break ;
        }
        data = data.offset(1);
        len = len.wrapping_sub(1)
    }
    if len == 0 as libc::c_int as libc::c_ulong {
        return -(1004 as libc::c_int) as int64_t
    }
    // find how much of the data is correctly formatted
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < len {
        let mut c_0: libc::c_uchar = *data.offset(i as isize);
        if !(*(*__ctype_b_loc()).offset(c_0 as libc::c_int as isize) as
                 libc::c_int &
                 _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                 ||
                 c_0 as libc::c_int >= 'a' as i32 &&
                     c_0 as libc::c_int <= 'f' as i32 ||
                 c_0 as libc::c_int >= 'A' as i32 &&
                     c_0 as libc::c_int <= 'F' as i32) {
            break ;
        }
        i = i.wrapping_add(1)
    }
    // cut off trailing junk
    if i != len { len = i }
    let mut chunk_len: int64_t =
        htp_parse_positive_integer_whitespace(data, len, 16 as libc::c_int);
    if chunk_len < 0 as libc::c_int as libc::c_long { return chunk_len }
    if chunk_len > 2147483647 as libc::c_int as libc::c_long {
        return -(1 as libc::c_int) as int64_t
    }
    return chunk_len;
}
/* *
 * A somewhat forgiving parser for a positive integer in a given base.
 * Only LWS is allowed before and after the number.
 * 
 * @param[in] data
 * @param[in] len
 * @param[in] base
 * @return The parsed number on success; a negative number on error.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_parse_positive_integer_whitespace(mut data:
                                                                   *mut libc::c_uchar,
                                                               mut len:
                                                                   size_t,
                                                               mut base:
                                                                   libc::c_int)
 -> int64_t {
    if len == 0 as libc::c_int as libc::c_ulong {
        return -(1003 as libc::c_int) as int64_t
    }
    let mut last_pos: size_t = 0;
    let mut pos: size_t = 0 as libc::c_int as size_t;
    // Ignore LWS before
    while pos < len &&
              htp_is_lws(*data.offset(pos as isize) as libc::c_int) != 0 {
        pos = pos.wrapping_add(1)
    }
    if pos == len { return -(1001 as libc::c_int) as int64_t }
    let mut r: int64_t =
        bstr_util_mem_to_pint(data.offset(pos as isize) as
                                  *const libc::c_void, len.wrapping_sub(pos),
                              base, &mut last_pos);
    if r < 0 as libc::c_int as libc::c_long { return r }
    // Move after the last digit
    pos = (pos as libc::c_ulong).wrapping_add(last_pos) as size_t as size_t;
    // Ignore LWS after
    while pos < len {
        if htp_is_lws(*data.offset(pos as isize) as libc::c_int) == 0 {
            return -(1002 as libc::c_int) as int64_t
        }
        pos = pos.wrapping_add(1)
    }
    return r;
}
/* *
 * Records one log message.
 * 
 * @param[in] connp
 * @param[in] file
 * @param[in] line
 * @param[in] level
 * @param[in] code
 * @param[in] fmt
 */
#[no_mangle]
pub unsafe extern "C" fn htp_log(mut connp: *mut htp_connp_t,
                                 mut file: *const libc::c_char,
                                 mut line: libc::c_int,
                                 mut level: htp_log_level_t,
                                 mut code: libc::c_int,
                                 mut fmt: *const libc::c_char,
                                 mut args: ...) {
    if connp.is_null() { return }
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut args_0: ::std::ffi::VaListImpl;
    // Ignore messages below our log level.
    if ((*(*connp).cfg).log_level as libc::c_uint) < level as libc::c_uint {
        return
    }
    args_0 = args.clone();
    let mut r: libc::c_int =
        vsnprintf(buf.as_mut_ptr(), 1024 as libc::c_int as libc::c_ulong, fmt,
                  args_0.as_va_list());
    if r < 0 as libc::c_int {
        snprintf(buf.as_mut_ptr(), 1024 as libc::c_int as libc::c_ulong,
                 b"[vnsprintf returned error %d]\x00" as *const u8 as
                     *const libc::c_char, r);
    } else if r >= 1024 as libc::c_int {
        // Indicate overflow with a '+' at the end.
        buf[1022 as libc::c_int as usize] = '+' as i32 as libc::c_char;
        buf[1023 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char
    }
    // Create a new log entry.
    let mut log: *mut htp_log_t =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<htp_log_t>() as libc::c_ulong) as
            *mut htp_log_t;
    if log.is_null() { return }
    (*log).connp = connp;
    (*log).file = file;
    (*log).line = line as libc::c_uint;
    (*log).level = level;
    (*log).code = code;
    (*log).msg = strdup(buf.as_mut_ptr());
    htp_list_array_push((*(*connp).conn).messages, log as *mut libc::c_void);
    if level as libc::c_uint == HTP_LOG_ERROR as libc::c_int as libc::c_uint {
        (*connp).last_error = log
    }
    /* coverity[check_return] */
    htp_hook_run_all((*(*connp).cfg).hook_log, log as *mut libc::c_void);
}
/* *
 * Determines if the given line is a continuation (of some previous line).
 * 
 * @param[in] data
 * @param[in] len
 * @return 0 or 1 for false and true, respectively. Returns -1 on error (NULL pointer or length zero).
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_is_line_folded(mut data:
                                                      *mut libc::c_uchar,
                                                  mut len: size_t)
 -> libc::c_int {
    if data.is_null() || len == 0 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int)
    }
    return htp_is_folding_char(*data.offset(0 as libc::c_int as isize) as
                                   libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn htp_is_folding_char(mut c: libc::c_int)
 -> libc::c_int {
    if htp_is_lws(c) != 0 || c == 0 as libc::c_int {
        return 1 as libc::c_int
    } else { return 0 as libc::c_int };
}
/* *
 * Determines if the given line is a request terminator.
 *
 * @param[in] connp
 * @param[in] data
 * @param[in] len
 * @return 0 or 1
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_is_line_terminator(mut connp:
                                                          *mut htp_connp_t,
                                                      mut data:
                                                          *mut libc::c_uchar,
                                                      mut len: size_t)
 -> libc::c_int {
    // Is this the end of request headers?
    match (*(*connp).cfg).server_personality as libc::c_uint {
        5 => {
            // IIS 5 will accept a whitespace line as a terminator
            if htp_is_line_whitespace(data, len) != 0 {
                return 1 as libc::c_int
            }
        }
        _ => { }
    }
    // Fall through
    // Treat an empty line as terminator
    if htp_is_line_empty(data, len) != 0 { return 1 as libc::c_int }
    if len == 2 as libc::c_int as libc::c_ulong &&
           htp_is_lws(*data.offset(0 as libc::c_int as isize) as libc::c_int)
               != 0 &&
           *data.offset(1 as libc::c_int as isize) as libc::c_int ==
               '\n' as i32 {
        if (*connp).out_current_read_offset < (*connp).out_current_len &&
               *(*connp).out_current_data.offset((*connp).out_current_read_offset
                                                     as isize) as libc::c_int
                   != '\n' as i32 {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
// Only space is terminator if terminator does not follow right away
/* *
 * Determines if the given line can be ignored when it appears before a request.
 *
 * @param[in] connp
 * @param[in] data
 * @param[in] len
 * @return 0 or 1
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_is_line_ignorable(mut connp:
                                                         *mut htp_connp_t,
                                                     mut data:
                                                         *mut libc::c_uchar,
                                                     mut len: size_t)
 -> libc::c_int {
    return htp_connp_is_line_terminator(connp, data, len);
}
unsafe extern "C" fn htp_parse_port(mut data: *mut libc::c_uchar,
                                    mut len: size_t,
                                    mut port: *mut libc::c_int,
                                    mut invalid: *mut libc::c_int)
 -> htp_status_t {
    if len == 0 as libc::c_int as libc::c_ulong {
        *port = -(1 as libc::c_int);
        *invalid = 1 as libc::c_int;
        return 1 as libc::c_int
    }
    let mut port_parsed: int64_t =
        htp_parse_positive_integer_whitespace(data, len, 10 as libc::c_int);
    if port_parsed < 0 as libc::c_int as libc::c_long {
        // Failed to parse the port number.
        *port = -(1 as libc::c_int);
        *invalid = 1 as libc::c_int
    } else if port_parsed > 0 as libc::c_int as libc::c_long &&
                  port_parsed < 65536 as libc::c_int as libc::c_long {
        // Valid port number.
        *port = port_parsed as libc::c_int
    } else {
        // Port number out of range.
        *port = -(1 as libc::c_int);
        *invalid = 1 as libc::c_int
    }
    return 1 as libc::c_int;
}
/* *
 * Parses an authority string, which consists of a hostname with an optional port number; username
 * and password are not allowed and will not be handled.
 *
 * @param[in] hostport
 * @param[out] hostname A bstring containing the hostname, or NULL if the hostname is invalid. If this value
 *                      is not NULL, the caller assumes responsibility for memory management.
 * @param[out] port Port as text, or NULL if not provided.
 * @param[out] port_number Port number, or -1 if the port is not present or invalid.
 * @param[out] invalid Set to 1 if any part of the authority is invalid.
 * @return HTP_OK on success, HTP_ERROR on memory allocation failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_parse_hostport(mut hostport: *mut bstr,
                                            mut hostname: *mut *mut bstr,
                                            mut port: *mut *mut bstr,
                                            mut port_number: *mut libc::c_int,
                                            mut invalid: *mut libc::c_int)
 -> htp_status_t {
    if hostport.is_null() || hostname.is_null() || port_number.is_null() ||
           invalid.is_null() {
        return -(1 as libc::c_int)
    }
    *hostname = 0 as *mut bstr;
    if !port.is_null() { *port = 0 as *mut bstr }
    *port_number = -(1 as libc::c_int);
    *invalid = 0 as libc::c_int;
    let mut data: *mut libc::c_uchar =
        if (*hostport).realptr.is_null() {
            (hostport as
                 *mut libc::c_uchar).offset(::std::mem::size_of::<bstr>() as
                                                libc::c_ulong as isize)
        } else { (*hostport).realptr };
    let mut len: size_t = (*hostport).len;
    bstr_util_mem_trim(&mut data, &mut len);
    if len == 0 as libc::c_int as libc::c_ulong {
        *invalid = 1 as libc::c_int;
        return 1 as libc::c_int
    }
    // Check for an IPv6 address.
    if *data.offset(0 as libc::c_int as isize) as libc::c_int == '[' as i32 {
        // IPv6 host.
        // Find the end of the IPv6 address.
        let mut pos: size_t = 0 as libc::c_int as size_t;
        while pos < len &&
                  *data.offset(pos as isize) as libc::c_int != ']' as i32 {
            pos = pos.wrapping_add(1)
        }
        if pos == len { *invalid = 1 as libc::c_int; return 1 as libc::c_int }
        *hostname =
            bstr_dup_mem(data as *const libc::c_void,
                         pos.wrapping_add(1 as libc::c_int as libc::c_ulong));
        if (*hostname).is_null() { return -(1 as libc::c_int) }
        // Over the ']'.
        pos = pos.wrapping_add(1);
        if pos == len { return 1 as libc::c_int }
        // Handle port.
        if *data.offset(pos as isize) as libc::c_int == ':' as i32 {
            if !port.is_null() {
                *port =
                    bstr_dup_mem(data.offset(pos as
                                                 isize).offset(1 as
                                                                   libc::c_int
                                                                   as isize)
                                     as *const libc::c_void,
                                 len.wrapping_sub(pos).wrapping_sub(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong));
                if (*port).is_null() {
                    bstr_free(*hostname);
                    return -(1 as libc::c_int)
                }
            }
            return htp_parse_port(data.offset(pos as
                                                  isize).offset(1 as
                                                                    libc::c_int
                                                                    as isize),
                                  len.wrapping_sub(pos).wrapping_sub(1 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong),
                                  port_number, invalid)
        } else { *invalid = 1 as libc::c_int; return 1 as libc::c_int }
    } else {
        // Not IPv6 host.
        // Is there a colon?
        let mut colon: *mut libc::c_uchar =
            memchr(data as *const libc::c_void, ':' as i32, len) as
                *mut libc::c_uchar;
        if colon.is_null() {
            // Hostname alone, no port.
            *hostname = bstr_dup_mem(data as *const libc::c_void, len);
            if (*hostname).is_null() { return -(1 as libc::c_int) }
            bstr_to_lowercase(*hostname);
        } else {
            // Hostname and port.
            // Ignore whitespace at the end of hostname.
            let mut hostend: *mut libc::c_uchar = colon;
            while hostend > data &&
                      *(*__ctype_b_loc()).offset(*hostend.offset(-(1 as
                                                                       libc::c_int
                                                                       as
                                                                       isize))
                                                     as libc::c_int as isize)
                          as libc::c_int &
                          _ISspace as libc::c_int as libc::c_ushort as
                              libc::c_int != 0 {
                hostend = hostend.offset(-1)
            }
            *hostname =
                bstr_dup_mem(data as *const libc::c_void,
                             hostend.wrapping_offset_from(data) as
                                 libc::c_long as size_t);
            if (*hostname).is_null() { return -(1 as libc::c_int) }
            if !port.is_null() {
                *port =
                    bstr_dup_mem(colon.offset(1 as libc::c_int as isize) as
                                     *const libc::c_void,
                                 len.wrapping_sub(colon.offset(1 as
                                                                   libc::c_int
                                                                   as
                                                                   isize).wrapping_offset_from(data)
                                                      as libc::c_long as
                                                      libc::c_ulong));
                if (*port).is_null() {
                    bstr_free(*hostname);
                    return -(1 as libc::c_int)
                }
            }
            return htp_parse_port(colon.offset(1 as libc::c_int as isize),
                                  len.wrapping_sub(colon.offset(1 as
                                                                    libc::c_int
                                                                    as
                                                                    isize).wrapping_offset_from(data)
                                                       as libc::c_long as
                                                       libc::c_ulong),
                                  port_number, invalid)
        }
    }
    return 1 as libc::c_int;
}
/* *
 * Parses hostport provided in the URI.
 *
 * @param[in] connp
 * @param[in] hostport
 * @param[in] uri
 * @return HTP_OK on success or HTP_ERROR error.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_parse_uri_hostport(mut connp: *mut htp_connp_t,
                                                mut hostport: *mut bstr,
                                                mut uri: *mut htp_uri_t)
 -> libc::c_int {
    let mut invalid: libc::c_int = 0;
    let mut rc: htp_status_t =
        htp_parse_hostport(hostport, &mut (*uri).hostname, &mut (*uri).port,
                           &mut (*uri).port_number, &mut invalid);
    if rc != 1 as libc::c_int { return rc }
    if invalid != 0 {
        (*(*connp).in_tx).flags =
            ((*(*connp).in_tx).flags as libc::c_ulonglong |
                 0x2000000 as libc::c_ulonglong) as uint64_t
    }
    if !(*uri).hostname.is_null() {
        if htp_validate_hostname((*uri).hostname) == 0 as libc::c_int {
            (*(*connp).in_tx).flags =
                ((*(*connp).in_tx).flags as libc::c_ulonglong |
                     0x2000000 as libc::c_ulonglong) as uint64_t
        }
    }
    return 1 as libc::c_int;
}
/* *
 * Parses hostport provided in the Host header.
 * 
 * @param[in] hostport
 * @param[out] hostname
 * @param[out] port
 * @param[out] port_number
 * @param[out] flags
 * @return HTP_OK on success or HTP_ERROR error.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_parse_header_hostport(mut hostport: *mut bstr,
                                                   mut hostname:
                                                       *mut *mut bstr,
                                                   mut port: *mut *mut bstr,
                                                   mut port_number:
                                                       *mut libc::c_int,
                                                   mut flags: *mut uint64_t)
 -> htp_status_t {
    let mut invalid: libc::c_int = 0;
    let mut rc: htp_status_t =
        htp_parse_hostport(hostport, hostname, port, port_number,
                           &mut invalid);
    if rc != 1 as libc::c_int { return rc }
    if invalid != 0 {
        *flags =
            (*flags as libc::c_ulonglong | 0x4000000 as libc::c_ulonglong) as
                uint64_t
    }
    if !(*hostname).is_null() {
        if htp_validate_hostname(*hostname) == 0 as libc::c_int {
            *flags =
                (*flags as libc::c_ulonglong | 0x4000000 as libc::c_ulonglong)
                    as uint64_t
        }
    }
    return 1 as libc::c_int;
}
/* *
 * Parses request URI, making no attempt to validate the contents.
 * 
 * @param[in] input
 * @param[in] uri
 * @return HTP_ERROR on memory allocation failure, HTP_OK otherwise
 */
#[no_mangle]
pub unsafe extern "C" fn htp_parse_uri(mut input: *mut bstr,
                                       mut uri: *mut *mut htp_uri_t)
 -> libc::c_int {
    // Allow a htp_uri_t structure to be provided on input,
    // but allocate a new one if the structure is NULL.
    if (*uri).is_null() {
        *uri =
            calloc(1 as libc::c_int as libc::c_ulong,
                   ::std::mem::size_of::<htp_uri_t>() as libc::c_ulong) as
                *mut htp_uri_t;
        if (*uri).is_null() { return -(1 as libc::c_int) }
    }
    if input.is_null() {
        // The input might be NULL on requests that don't actually
        // contain the URI. We allow that.
        return 1 as libc::c_int
    }
    let mut data: *mut libc::c_uchar =
        if (*input).realptr.is_null() {
            (input as
                 *mut libc::c_uchar).offset(::std::mem::size_of::<bstr>() as
                                                libc::c_ulong as isize)
        } else { (*input).realptr };
    let mut len: size_t = (*input).len;
    let mut start: size_t = 0;
    let mut pos: size_t = 0;
    if len == 0 as libc::c_int as libc::c_ulong {
        // Empty string.
        return 1 as libc::c_int
    }
    pos = 0 as libc::c_int as size_t;
    // Scheme test: if it doesn't start with a forward slash character (which it must
    // for the contents to be a path or an authority, then it must be the scheme part
    if *data.offset(0 as libc::c_int as isize) as libc::c_int != '/' as i32 {
        // Parse scheme        
        // Find the colon, which marks the end of the scheme part
        start = pos;
        while pos < len &&
                  *data.offset(pos as isize) as libc::c_int != ':' as i32 {
            pos = pos.wrapping_add(1)
        }
        if pos >= len {
            // We haven't found a colon, which means that the URI
            // is invalid. Apache will ignore this problem and assume
            // the URI contains an invalid path so, for the time being,
            // we are going to do the same.
            pos = 0 as libc::c_int as size_t
        } else {
            // Make a copy of the scheme
            (**uri).scheme =
                bstr_dup_mem(data.offset(start as isize) as
                                 *const libc::c_void,
                             pos.wrapping_sub(start));
            if (**uri).scheme.is_null() { return -(1 as libc::c_int) }
            // Go over the colon
            pos = pos.wrapping_add(1)
        }
    }
    // Authority test: two forward slash characters and it's an authority.
    // One, three or more slash characters, and it's a path. We, however,
    // only attempt to parse authority if we've seen a scheme.
    if !(**uri).scheme.is_null() {
        if pos.wrapping_add(2 as libc::c_int as libc::c_ulong) < len &&
               *data.offset(pos as isize) as libc::c_int == '/' as i32 &&
               *data.offset(pos.wrapping_add(1 as libc::c_int as
                                                 libc::c_ulong) as isize) as
                   libc::c_int == '/' as i32 &&
               *data.offset(pos.wrapping_add(2 as libc::c_int as
                                                 libc::c_ulong) as isize) as
                   libc::c_int != '/' as i32 {
            // Parse authority
            // Go over the two slash characters
            pos = pos.wrapping_add(2 as libc::c_int as libc::c_ulong);
            start = pos;
            // Authority ends with a question mark, forward slash or hash
            while pos < len &&
                      *data.offset(pos as isize) as libc::c_int != '?' as i32
                      &&
                      *data.offset(pos as isize) as libc::c_int != '/' as i32
                      &&
                      *data.offset(pos as isize) as libc::c_int != '#' as i32
                  {
                pos = pos.wrapping_add(1)
            }
            let mut hostname_start: *mut libc::c_uchar =
                0 as *mut libc::c_uchar;
            let mut hostname_len: size_t = 0;
            // Are the credentials included in the authority?
            let mut m: *mut libc::c_uchar =
                memchr(data.offset(start as isize) as *const libc::c_void,
                       '@' as i32, pos.wrapping_sub(start)) as
                    *mut libc::c_uchar;
            if !m.is_null() {
                // Credentials present
                let mut credentials_start: *mut libc::c_uchar =
                    data.offset(start as isize);
                let mut credentials_len: size_t =
                    (m.wrapping_offset_from(data) as libc::c_long as
                         libc::c_ulong).wrapping_sub(start);
                // Figure out just the hostname part
                hostname_start =
                    data.offset(start as
                                    isize).offset(credentials_len as
                                                      isize).offset(1 as
                                                                        libc::c_int
                                                                        as
                                                                        isize);
                hostname_len =
                    pos.wrapping_sub(start).wrapping_sub(credentials_len).wrapping_sub(1
                                                                                           as
                                                                                           libc::c_int
                                                                                           as
                                                                                           libc::c_ulong);
                // Extract the username and the password
                m =
                    memchr(credentials_start as *const libc::c_void,
                           ':' as i32, credentials_len) as *mut libc::c_uchar;
                if !m.is_null() {
                    // Username and password
                    (**uri).username =
                        bstr_dup_mem(credentials_start as *const libc::c_void,
                                     m.wrapping_offset_from(credentials_start)
                                         as libc::c_long as size_t);
                    if (**uri).username.is_null() {
                        return -(1 as libc::c_int)
                    }
                    (**uri).password =
                        bstr_dup_mem(m.offset(1 as libc::c_int as isize) as
                                         *const libc::c_void,
                                     credentials_len.wrapping_sub(m.wrapping_offset_from(credentials_start)
                                                                      as
                                                                      libc::c_long
                                                                      as
                                                                      libc::c_ulong).wrapping_sub(1
                                                                                                      as
                                                                                                      libc::c_int
                                                                                                      as
                                                                                                      libc::c_ulong));
                    if (**uri).password.is_null() {
                        return -(1 as libc::c_int)
                    }
                } else {
                    // Username alone
                    (**uri).username =
                        bstr_dup_mem(credentials_start as *const libc::c_void,
                                     credentials_len);
                    if (**uri).username.is_null() {
                        return -(1 as libc::c_int)
                    }
                }
            } else {
                // No credentials
                hostname_start = data.offset(start as isize);
                hostname_len = pos.wrapping_sub(start)
            }
            // Parsing authority without credentials.
            if hostname_len > 0 as libc::c_int as libc::c_ulong &&
                   *hostname_start.offset(0 as libc::c_int as isize) as
                       libc::c_int == '[' as i32 {
                // IPv6 address.
                m =
                    memchr(hostname_start as *const libc::c_void, ']' as i32,
                           hostname_len) as *mut libc::c_uchar;
                if m.is_null() {
                    // Invalid IPv6 address; use the entire string as hostname.
                    (**uri).hostname =
                        bstr_dup_mem(hostname_start as *const libc::c_void,
                                     hostname_len);
                    if (**uri).hostname.is_null() {
                        return -(1 as libc::c_int)
                    }
                } else {
                    (**uri).hostname =
                        bstr_dup_mem(hostname_start as *const libc::c_void,
                                     (m.wrapping_offset_from(hostname_start)
                                          as libc::c_long +
                                          1 as libc::c_int as libc::c_long) as
                                         size_t);
                    if (**uri).hostname.is_null() {
                        return -(1 as libc::c_int)
                    }
                    // Is there a port?
                    hostname_len =
                        hostname_len.wrapping_sub((m.wrapping_offset_from(hostname_start)
                                                       as libc::c_long +
                                                       1 as libc::c_int as
                                                           libc::c_long) as
                                                      libc::c_ulong);
                    hostname_start = m.offset(1 as libc::c_int as isize);
                    // Port string
                    m =
                        memchr(hostname_start as *const libc::c_void,
                               ':' as i32, hostname_len) as
                            *mut libc::c_uchar;
                    if !m.is_null() {
                        let mut port_len: size_t =
                            hostname_len.wrapping_sub(m.wrapping_offset_from(hostname_start)
                                                          as libc::c_long as
                                                          libc::c_ulong).wrapping_sub(1
                                                                                          as
                                                                                          libc::c_int
                                                                                          as
                                                                                          libc::c_ulong);
                        (**uri).port =
                            bstr_dup_mem(m.offset(1 as libc::c_int as isize)
                                             as *const libc::c_void,
                                         port_len);
                        if (**uri).port.is_null() {
                            return -(1 as libc::c_int)
                        }
                    }
                }
            } else {
                // Not IPv6 address.
                m =
                    memchr(hostname_start as *const libc::c_void, ':' as i32,
                           hostname_len) as *mut libc::c_uchar;
                if !m.is_null() {
                    let mut port_len_0: size_t =
                        hostname_len.wrapping_sub(m.wrapping_offset_from(hostname_start)
                                                      as libc::c_long as
                                                      libc::c_ulong).wrapping_sub(1
                                                                                      as
                                                                                      libc::c_int
                                                                                      as
                                                                                      libc::c_ulong);
                    hostname_len =
                        hostname_len.wrapping_sub(port_len_0).wrapping_sub(1
                                                                               as
                                                                               libc::c_int
                                                                               as
                                                                               libc::c_ulong);
                    // Port string
                    (**uri).port =
                        bstr_dup_mem(m.offset(1 as libc::c_int as isize) as
                                         *const libc::c_void, port_len_0);
                    if (**uri).port.is_null() { return -(1 as libc::c_int) }
                }
                // Hostname
                (**uri).hostname =
                    bstr_dup_mem(hostname_start as *const libc::c_void,
                                 hostname_len);
                if (**uri).hostname.is_null() { return -(1 as libc::c_int) }
            }
        }
    }
    // Path
    start = pos;
    // The path part will end with a question mark or a hash character, which
    // mark the beginning of the query part or the fragment part, respectively.
    while pos < len && *data.offset(pos as isize) as libc::c_int != '?' as i32
              && *data.offset(pos as isize) as libc::c_int != '#' as i32 {
        pos = pos.wrapping_add(1)
    }
    // Path
    (**uri).path =
        bstr_dup_mem(data.offset(start as isize) as *const libc::c_void,
                     pos.wrapping_sub(start));
    if (**uri).path.is_null() { return -(1 as libc::c_int) }
    if pos == len { return 1 as libc::c_int }
    // Query
    if *data.offset(pos as isize) as libc::c_int == '?' as i32 {
        // Step over the question mark
        start = pos.wrapping_add(1 as libc::c_int as libc::c_ulong);
        // The query part will end with the end of the input
        // or the beginning of the fragment part
        while pos < len &&
                  *data.offset(pos as isize) as libc::c_int != '#' as i32 {
            pos = pos.wrapping_add(1)
        }
        // Query string
        (**uri).query =
            bstr_dup_mem(data.offset(start as isize) as *const libc::c_void,
                         pos.wrapping_sub(start));
        if (**uri).query.is_null() { return -(1 as libc::c_int) }
        if pos == len { return 1 as libc::c_int }
    }
    // Fragment
    if *data.offset(pos as isize) as libc::c_int == '#' as i32 {
        // Step over the hash character
        start = pos.wrapping_add(1 as libc::c_int as libc::c_ulong);
        // Fragment; ends with the end of the input
        (**uri).fragment =
            bstr_dup_mem(data.offset(start as isize) as *const libc::c_void,
                         len.wrapping_sub(start));
        if (**uri).fragment.is_null() { return -(1 as libc::c_int) }
    }
    return 1 as libc::c_int;
}
/* *
 * Convert two input bytes, pointed to by the pointer parameter,
 * into a single byte by assuming the input consists of hexadecimal
 * characters. This function will happily convert invalid input.
 *
 * @param[in] what
 * @return hex-decoded byte
 */
unsafe extern "C" fn x2c(mut what: *mut libc::c_uchar) -> libc::c_uchar {
    let mut digit: libc::c_uchar = 0;
    digit =
        if *what.offset(0 as libc::c_int as isize) as libc::c_int >=
               'A' as i32 {
            ((*what.offset(0 as libc::c_int as isize) as libc::c_int &
                  0xdf as libc::c_int) - 'A' as i32) + 10 as libc::c_int
        } else {
            (*what.offset(0 as libc::c_int as isize) as libc::c_int) -
                '0' as i32
        } as libc::c_uchar;
    digit = (digit as libc::c_int * 16 as libc::c_int) as libc::c_uchar;
    digit =
        (digit as libc::c_int +
             if *what.offset(1 as libc::c_int as isize) as libc::c_int >=
                    'A' as i32 {
                 ((*what.offset(1 as libc::c_int as isize) as libc::c_int &
                       0xdf as libc::c_int) - 'A' as i32) + 10 as libc::c_int
             } else {
                 (*what.offset(1 as libc::c_int as isize) as libc::c_int) -
                     '0' as i32
             }) as libc::c_uchar;
    return digit;
}
/* *
 * Convert a Unicode codepoint into a single-byte, using best-fit
 * mapping (as specified in the provided configuration structure).
 *
 * @param[in] cfg
 * @param[in] codepoint
 * @return converted single byte
 */
unsafe extern "C" fn bestfit_codepoint(mut cfg: *mut htp_cfg_t,
                                       mut ctx: htp_decoder_ctx_t,
                                       mut codepoint: uint32_t) -> uint8_t {
    // Is it a single-byte codepoint?
    if codepoint < 0x100 as libc::c_int as libc::c_uint {
        return codepoint as uint8_t
    }
    // Our current implementation converts only the 2-byte codepoints.
    if codepoint > 0xffff as libc::c_int as libc::c_uint {
        return (*cfg).decoder_cfgs[ctx as usize].bestfit_replacement_byte
    }
    let mut p: *mut uint8_t = (*cfg).decoder_cfgs[ctx as usize].bestfit_map;
    loop 
         // TODO Optimize lookup.
         {
        let mut x: uint32_t =
            (((*p.offset(0 as libc::c_int as isize) as libc::c_int) <<
                  8 as libc::c_int) +
                 *p.offset(1 as libc::c_int as isize) as libc::c_int) as
                uint32_t;
        if x == 0 as libc::c_int as libc::c_uint {
            return (*cfg).decoder_cfgs[ctx as usize].bestfit_replacement_byte
        }
        if x == codepoint { return *p.offset(2 as libc::c_int as isize) }
        // Move to the next triplet
        p = p.offset(3 as libc::c_int as isize)
    };
}
/* *
 * Decode a UTF-8 encoded path. Overlong characters will be decoded, invalid
 * characters will be left as-is. Best-fit mapping will be used to convert
 * UTF-8 into a single-byte stream.
 *
 * @param[in] cfg
 * @param[in] tx
 * @param[in] path
 */
#[no_mangle]
pub unsafe extern "C" fn htp_utf8_decode_path_inplace(mut cfg: *mut htp_cfg_t,
                                                      mut tx: *mut htp_tx_t,
                                                      mut path: *mut bstr) {
    if path.is_null() { return }
    let mut data: *mut uint8_t =
        if (*path).realptr.is_null() {
            (path as
                 *mut libc::c_uchar).offset(::std::mem::size_of::<bstr>() as
                                                libc::c_ulong as isize)
        } else { (*path).realptr };
    if data.is_null() { return }
    let mut len: size_t = (*path).len;
    let mut rpos: size_t = 0 as libc::c_int as size_t;
    let mut wpos: size_t = 0 as libc::c_int as size_t;
    let mut codepoint: uint32_t = 0 as libc::c_int as uint32_t;
    let mut state: uint32_t = 0 as libc::c_int as uint32_t;
    let mut counter: uint32_t = 0 as libc::c_int as uint32_t;
    let mut seen_valid: uint8_t = 0 as libc::c_int as uint8_t;
    while rpos < len && wpos < len {
        counter = counter.wrapping_add(1);
        match htp_utf8_decode_allow_overlong(&mut state, &mut codepoint,
                                             *data.offset(rpos as isize) as
                                                 uint32_t) {
            0 => {
                if counter == 1 as libc::c_int as libc::c_uint {
                    // ASCII character, which we just copy.
                    let fresh0 = wpos;
                    wpos = wpos.wrapping_add(1);
                    *data.offset(fresh0 as isize) = codepoint as uint8_t
                } else {
                    // A valid UTF-8 character, which we need to convert.
                    seen_valid = 1 as libc::c_int as uint8_t;
                    // Check for overlong characters and set the flag accordingly.
                    match counter {
                        2 => {
                            if codepoint < 0x80 as libc::c_int as libc::c_uint
                               {
                                (*tx).flags =
                                    ((*tx).flags as libc::c_ulonglong |
                                         0x400000 as libc::c_ulonglong) as
                                        uint64_t
                            }
                        }
                        3 => {
                            if codepoint <
                                   0x800 as libc::c_int as libc::c_uint {
                                (*tx).flags =
                                    ((*tx).flags as libc::c_ulonglong |
                                         0x400000 as libc::c_ulonglong) as
                                        uint64_t
                            }
                        }
                        4 => {
                            if codepoint <
                                   0x10000 as libc::c_int as libc::c_uint {
                                (*tx).flags =
                                    ((*tx).flags as libc::c_ulonglong |
                                         0x400000 as libc::c_ulonglong) as
                                        uint64_t
                            }
                        }
                        _ => { }
                    }
                    // Special flag for half-width/full-width evasion.
                    if codepoint >= 0xff00 as libc::c_int as libc::c_uint &&
                           codepoint <= 0xffef as libc::c_int as libc::c_uint
                       {
                        (*tx).flags =
                            ((*tx).flags as libc::c_ulonglong |
                                 0x800000 as libc::c_ulonglong) as uint64_t
                    }
                    // Use best-fit mapping to convert to a single byte.
                    let fresh1 = wpos;
                    wpos = wpos.wrapping_add(1);
                    *data.offset(fresh1 as isize) =
                        bestfit_codepoint(cfg, HTP_DECODER_URL_PATH,
                                          codepoint)
                }
                // Advance over the consumed byte and reset the byte counter.
                rpos = rpos.wrapping_add(1);
                counter = 0 as libc::c_int as uint32_t
            }
            1 => {
                // Invalid UTF-8 character.
                (*tx).flags =
                    ((*tx).flags as libc::c_ulonglong |
                         0x200000 as libc::c_ulonglong) as uint64_t;
                // Is the server expected to respond with 400?
                if (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as libc::c_int as
                                           usize].utf8_invalid_unwanted as
                       libc::c_uint !=
                       HTP_UNWANTED_IGNORE as libc::c_int as libc::c_uint {
                    (*tx).response_status_expected_number =
                        (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as
                                                libc::c_int as
                                                usize].utf8_invalid_unwanted
                            as libc::c_int
                }
                // Output the replacement byte, replacing one or more invalid bytes.
                let fresh2 = wpos;
                wpos = wpos.wrapping_add(1);
                *data.offset(fresh2 as isize) =
                    (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as libc::c_int as
                                            usize].bestfit_replacement_byte;
                // If the invalid byte was first in a sequence, consume it. Otherwise,
                // assume it's the starting byte of the next character.
                if counter == 1 as libc::c_int as libc::c_uint {
                    rpos = rpos.wrapping_add(1)
                }
                // Reset the decoder state and continue decoding.
                state = 0 as libc::c_int as uint32_t;
                codepoint = 0 as libc::c_int as uint32_t;
                counter = 0 as libc::c_int as uint32_t
            }
            _ => {
                // Keep going; the character is not yet formed.
                rpos = rpos.wrapping_add(1)
            }
        }
    }
    // Did the input stream seem like a valid UTF-8 string?
    if seen_valid as libc::c_int != 0 &&
           (*tx).flags as libc::c_ulonglong & 0x200000 as libc::c_ulonglong ==
               0 {
        (*tx).flags =
            ((*tx).flags as libc::c_ulonglong | 0x100000 as libc::c_ulonglong)
                as uint64_t
    }
    // Adjust the length of the string, because
    // we're doing in-place decoding.
    bstr_adjust_len(path, wpos);
}
/* *
 * Validate a path that is quite possibly UTF-8 encoded.
 * 
 * @param[in] tx
 * @param[in] path
 */
#[no_mangle]
pub unsafe extern "C" fn htp_utf8_validate_path(mut tx: *mut htp_tx_t,
                                                mut path: *mut bstr) {
    let mut data: *mut libc::c_uchar =
        if (*path).realptr.is_null() {
            (path as
                 *mut libc::c_uchar).offset(::std::mem::size_of::<bstr>() as
                                                libc::c_ulong as isize)
        } else {
            (*path).realptr
        }; // How many bytes used by a UTF-8 character.
    let mut len: size_t = (*path).len;
    let mut rpos: size_t = 0 as libc::c_int as size_t;
    let mut codepoint: uint32_t = 0 as libc::c_int as uint32_t;
    let mut state: uint32_t = 0 as libc::c_int as uint32_t;
    let mut counter: uint32_t = 0 as libc::c_int as uint32_t;
    let mut seen_valid: uint8_t = 0 as libc::c_int as uint8_t;
    while rpos < len {
        counter = counter.wrapping_add(1);
        match htp_utf8_decode_allow_overlong(&mut state, &mut codepoint,
                                             *data.offset(rpos as isize) as
                                                 uint32_t) {
            0 => {
                // We have a valid character.
                if counter > 1 as libc::c_int as libc::c_uint {
                    // A valid UTF-8 character, consisting of 2 or more bytes.
                    seen_valid = 1 as libc::c_int as uint8_t;
                    // Check for overlong characters and set the flag accordingly.
                    match counter {
                        2 => {
                            if codepoint < 0x80 as libc::c_int as libc::c_uint
                               {
                                (*tx).flags =
                                    ((*tx).flags as libc::c_ulonglong |
                                         0x400000 as libc::c_ulonglong) as
                                        uint64_t
                            }
                        }
                        3 => {
                            if codepoint <
                                   0x800 as libc::c_int as libc::c_uint {
                                (*tx).flags =
                                    ((*tx).flags as libc::c_ulonglong |
                                         0x400000 as libc::c_ulonglong) as
                                        uint64_t
                            }
                        }
                        4 => {
                            if codepoint <
                                   0x10000 as libc::c_int as libc::c_uint {
                                (*tx).flags =
                                    ((*tx).flags as libc::c_ulonglong |
                                         0x400000 as libc::c_ulonglong) as
                                        uint64_t
                            }
                        }
                        _ => { }
                    }
                }
                // Special flag for half-width/full-width evasion.
                if codepoint > 0xfeff as libc::c_int as libc::c_uint &&
                       codepoint < 0x10000 as libc::c_int as libc::c_uint {
                    (*tx).flags =
                        ((*tx).flags as libc::c_ulonglong |
                             0x800000 as libc::c_ulonglong) as uint64_t
                }
                // Advance over the consumed byte and reset the byte counter.
                rpos = rpos.wrapping_add(1);
                counter = 0 as libc::c_int as uint32_t
            }
            1 => {
                // Invalid UTF-8 character.
                (*tx).flags =
                    ((*tx).flags as libc::c_ulonglong |
                         0x200000 as libc::c_ulonglong) as uint64_t;
                // Override the decoder state because we want to continue decoding.
                state = 0 as libc::c_int as uint32_t;
                // Advance over the consumed byte and reset the byte counter.
                rpos = rpos.wrapping_add(1);
                counter = 0 as libc::c_int as uint32_t
            }
            _ => {
                // Keep going; the character is not yet formed.
                rpos = rpos.wrapping_add(1)
            }
        }
    }
    // Did the input stream seem like a valid UTF-8 string?
    if seen_valid as libc::c_int != 0 &&
           (*tx).flags as libc::c_ulonglong & 0x200000 as libc::c_ulonglong ==
               0 {
        (*tx).flags =
            ((*tx).flags as libc::c_ulonglong | 0x100000 as libc::c_ulonglong)
                as uint64_t
    };
}
/* *
 * Decode a %u-encoded character, using best-fit mapping as necessary. Path version.
 *
 * @param[in] cfg
 * @param[in] tx
 * @param[in] data
 * @return decoded byte
 */
unsafe extern "C" fn decode_u_encoding_path(mut cfg: *mut htp_cfg_t,
                                            mut tx: *mut htp_tx_t,
                                            mut data: *mut libc::c_uchar)
 -> libc::c_int {
    let mut c1: libc::c_uint = x2c(data) as libc::c_uint;
    let mut c2: libc::c_uint =
        x2c(data.offset(2 as libc::c_int as isize)) as libc::c_uint;
    let mut r: libc::c_int =
        (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as libc::c_int as
                                usize].bestfit_replacement_byte as
            libc::c_int;
    if c1 == 0 as libc::c_int as libc::c_uint {
        r = c2 as libc::c_int;
        (*tx).flags =
            ((*tx).flags as libc::c_ulonglong | 0x40000 as libc::c_ulonglong)
                as uint64_t
    } else {
        // Check for fullwidth form evasion
        if c1 == 0xff as libc::c_int as libc::c_uint {
            (*tx).flags =
                ((*tx).flags as libc::c_ulonglong |
                     0x800000 as libc::c_ulonglong) as uint64_t
        }
        if (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as libc::c_int as
                                   usize].u_encoding_unwanted as libc::c_uint
               != HTP_UNWANTED_IGNORE as libc::c_int as libc::c_uint {
            (*tx).response_status_expected_number =
                (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as libc::c_int as
                                        usize].u_encoding_unwanted as
                    libc::c_int
        }
        // Use best-fit mapping
        let mut p: *mut libc::c_uchar =
            (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as libc::c_int as
                                    usize].bestfit_map;
        // TODO Optimize lookup.
        // Have we reached the end of the map?
        while !(*p.offset(0 as libc::c_int as isize) as libc::c_int ==
                    0 as libc::c_int &&
                    *p.offset(1 as libc::c_int as isize) as libc::c_int ==
                        0 as libc::c_int) {
            // Have we found the mapping we're looking for?
            if *p.offset(0 as libc::c_int as isize) as libc::c_uint == c1 &&
                   *p.offset(1 as libc::c_int as isize) as libc::c_uint == c2
               {
                r = *p.offset(2 as libc::c_int as isize) as libc::c_int;
                break ;
            } else {
                // Move to the next triplet
                p = p.offset(3 as libc::c_int as isize)
            }
        }
    }
    // Check for encoded path separators
    if r == '/' as i32 ||
           (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as libc::c_int as
                                   usize].backslash_convert_slashes != 0 &&
               r == '\\' as i32 {
        (*tx).flags =
            ((*tx).flags as libc::c_ulonglong | 0x80000 as libc::c_ulonglong)
                as uint64_t
    }
    return r;
}
/* *
 * Decode a %u-encoded character, using best-fit mapping as necessary. Params version.
 *
 * @param[in] cfg
 * @param[in] tx
 * @param[in] data
 * @return decoded byte
 */
unsafe extern "C" fn decode_u_encoding_params(mut cfg: *mut htp_cfg_t,
                                              mut ctx: htp_decoder_ctx_t,
                                              mut data: *mut libc::c_uchar,
                                              mut flags: *mut uint64_t)
 -> libc::c_int {
    let mut c1: libc::c_uint = x2c(data) as libc::c_uint;
    let mut c2: libc::c_uint =
        x2c(data.offset(2 as libc::c_int as isize)) as libc::c_uint;
    // Check for overlong usage first.
    if c1 == 0 as libc::c_int as libc::c_uint {
        *flags =
            (*flags as libc::c_ulonglong | 0x20000000 as libc::c_ulonglong) as
                uint64_t;
        return c2 as libc::c_int
    }
    // Both bytes were used.
    // Detect half-width and full-width range.
    if c1 == 0xff as libc::c_int as libc::c_uint &&
           c2 <= 0xef as libc::c_int as libc::c_uint {
        *flags =
            (*flags as libc::c_ulonglong | 0x40000000 as libc::c_ulonglong) as
                uint64_t
    }
    // Use best-fit mapping.
    let mut p: *mut libc::c_uchar =
        (*cfg).decoder_cfgs[ctx as usize].bestfit_map;
    let mut r: libc::c_int =
        (*cfg).decoder_cfgs[ctx as usize].bestfit_replacement_byte as
            libc::c_int;
    // TODO Optimize lookup.
    // Have we reached the end of the map?
    while !(*p.offset(0 as libc::c_int as isize) as libc::c_int ==
                0 as libc::c_int &&
                *p.offset(1 as libc::c_int as isize) as libc::c_int ==
                    0 as libc::c_int) {
        // Have we found the mapping we're looking for?
        if *p.offset(0 as libc::c_int as isize) as libc::c_uint == c1 &&
               *p.offset(1 as libc::c_int as isize) as libc::c_uint == c2 {
            r = *p.offset(2 as libc::c_int as isize) as libc::c_int;
            break ;
        } else {
            // Move to the next triplet
            p = p.offset(3 as libc::c_int as isize)
        }
    }
    return r;
}
/* *
 * Decode a request path according to the settings in the
 * provided configuration structure.
 *
 * @param[in] cfg
 * @param[in] tx
 * @param[in] path
 */
#[no_mangle]
pub unsafe extern "C" fn htp_decode_path_inplace(mut tx: *mut htp_tx_t,
                                                 mut path: *mut bstr)
 -> libc::c_int {
    if path.is_null() { return -(1 as libc::c_int) }
    let mut data: *mut libc::c_uchar =
        if (*path).realptr.is_null() {
            (path as
                 *mut libc::c_uchar).offset(::std::mem::size_of::<bstr>() as
                                                libc::c_ulong as isize)
        } else { (*path).realptr };
    if data.is_null() { return -(1 as libc::c_int) }
    let mut len: size_t = (*path).len;
    let mut cfg: *mut htp_cfg_t = (*tx).cfg;
    let mut rpos: size_t = 0 as libc::c_int as size_t;
    let mut wpos: size_t = 0 as libc::c_int as size_t;
    let mut previous_was_separator: libc::c_int = 0 as libc::c_int;
    let mut current_block_104: u64;
    while rpos < len && wpos < len {
        let mut c: libc::c_int = *data.offset(rpos as isize) as libc::c_int;
        // Decode encoded characters
        if c == '%' as i32 {
            if rpos.wrapping_add(2 as libc::c_int as libc::c_ulong) < len {
                let mut handled: libc::c_int = 0 as libc::c_int;
                if (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as libc::c_int as
                                           usize].u_encoding_decode != 0 {
                    // Check for the %u encoding
                    if *data.offset(rpos.wrapping_add(1 as libc::c_int as
                                                          libc::c_ulong) as
                                        isize) as libc::c_int == 'u' as i32 ||
                           *data.offset(rpos.wrapping_add(1 as libc::c_int as
                                                              libc::c_ulong)
                                            as isize) as libc::c_int ==
                               'U' as i32 {
                        handled = 1 as libc::c_int;
                        if (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as
                                                   libc::c_int as
                                                   usize].u_encoding_unwanted
                               as libc::c_uint !=
                               HTP_UNWANTED_IGNORE as libc::c_int as
                                   libc::c_uint {
                            (*tx).response_status_expected_number =
                                (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as
                                                        libc::c_int as
                                                        usize].u_encoding_unwanted
                                    as libc::c_int
                        }
                        if rpos.wrapping_add(5 as libc::c_int as
                                                 libc::c_ulong) < len {
                            if *(*__ctype_b_loc()).offset(*data.offset(rpos.wrapping_add(2
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong)
                                                                           as
                                                                           isize)
                                                              as libc::c_int
                                                              as isize) as
                                   libc::c_int &
                                   _ISxdigit as libc::c_int as libc::c_ushort
                                       as libc::c_int != 0 &&
                                   *(*__ctype_b_loc()).offset(*data.offset(rpos.wrapping_add(3
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong)
                                                                               as
                                                                               isize)
                                                                  as
                                                                  libc::c_int
                                                                  as isize) as
                                       libc::c_int &
                                       _ISxdigit as libc::c_int as
                                           libc::c_ushort as libc::c_int != 0
                                   &&
                                   *(*__ctype_b_loc()).offset(*data.offset(rpos.wrapping_add(4
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong)
                                                                               as
                                                                               isize)
                                                                  as
                                                                  libc::c_int
                                                                  as isize) as
                                       libc::c_int &
                                       _ISxdigit as libc::c_int as
                                           libc::c_ushort as libc::c_int != 0
                                   &&
                                   *(*__ctype_b_loc()).offset(*data.offset(rpos.wrapping_add(5
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong)
                                                                               as
                                                                               isize)
                                                                  as
                                                                  libc::c_int
                                                                  as isize) as
                                       libc::c_int &
                                       _ISxdigit as libc::c_int as
                                           libc::c_ushort as libc::c_int != 0
                               {
                                // Decode a valid %u encoding
                                c =
                                    decode_u_encoding_path(cfg, tx,
                                                           &mut *data.offset(rpos.wrapping_add(2
                                                                                                   as
                                                                                                   libc::c_int
                                                                                                   as
                                                                                                   libc::c_ulong)
                                                                                 as
                                                                                 isize));
                                rpos =
                                    (rpos as
                                         libc::c_ulong).wrapping_add(6 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong)
                                        as size_t as size_t;
                                if c == 0 as libc::c_int {
                                    (*tx).flags =
                                        ((*tx).flags as libc::c_ulonglong |
                                             0x4000 as libc::c_ulonglong) as
                                            uint64_t;
                                    if (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH
                                                               as libc::c_int
                                                               as
                                                               usize].nul_encoded_unwanted
                                           as libc::c_uint !=
                                           HTP_UNWANTED_IGNORE as libc::c_int
                                               as libc::c_uint {
                                        (*tx).response_status_expected_number
                                            =
                                            (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH
                                                                    as
                                                                    libc::c_int
                                                                    as
                                                                    usize].nul_encoded_unwanted
                                                as libc::c_int
                                    }
                                }
                            } else {
                                // Invalid %u encoding
                                (*tx).flags =
                                    ((*tx).flags as libc::c_ulonglong |
                                         0x10000 as libc::c_ulonglong) as
                                        uint64_t;
                                if (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as
                                                           libc::c_int as
                                                           usize].url_encoding_invalid_unwanted
                                       as libc::c_uint !=
                                       HTP_UNWANTED_IGNORE as libc::c_int as
                                           libc::c_uint {
                                    (*tx).response_status_expected_number =
                                        (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH
                                                                as libc::c_int
                                                                as
                                                                usize].url_encoding_invalid_unwanted
                                            as libc::c_int
                                }
                                match (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH
                                                              as libc::c_int
                                                              as
                                                              usize].url_encoding_invalid_handling
                                          as libc::c_uint {
                                    1 => {
                                        current_block_104 =
                                            5193467589189724848;
                                        match current_block_104 {
                                            15044848815912959287 => {
                                                // Leave the percent character in output
                                                rpos = rpos.wrapping_add(1)
                                            }
                                            5193467589189724848 => {
                                                // Do not place anything in output; eat
                                        // the percent character
                                                rpos = rpos.wrapping_add(1);
                                                continue ;
                                            }
                                            _ => {
                                                // Decode invalid %u encoding
                                                c =
                                                    decode_u_encoding_path(cfg,
                                                                           tx,
                                                                           &mut *data.offset(rpos.wrapping_add(2
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_ulong)
                                                                                                 as
                                                                                                 isize));
                                                rpos =
                                                    (rpos as
                                                         libc::c_ulong).wrapping_add(6
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong)
                                                        as size_t as size_t
                                            }
                                        }
                                    }
                                    0 => {
                                        current_block_104 =
                                            15044848815912959287;
                                        match current_block_104 {
                                            15044848815912959287 => {
                                                rpos = rpos.wrapping_add(1)
                                            }
                                            5193467589189724848 => {
                                                rpos = rpos.wrapping_add(1);
                                                continue ;
                                            }
                                            _ => {
                                                c =
                                                    decode_u_encoding_path(cfg,
                                                                           tx,
                                                                           &mut *data.offset(rpos.wrapping_add(2
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_ulong)
                                                                                                 as
                                                                                                 isize));
                                                rpos =
                                                    (rpos as
                                                         libc::c_ulong).wrapping_add(6
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong)
                                                        as size_t as size_t
                                            }
                                        }
                                    }
                                    2 => {
                                        current_block_104 =
                                            3531489836707249550;
                                        match current_block_104 {
                                            15044848815912959287 => {
                                                rpos = rpos.wrapping_add(1)
                                            }
                                            5193467589189724848 => {
                                                rpos = rpos.wrapping_add(1);
                                                continue ;
                                            }
                                            _ => {
                                                c =
                                                    decode_u_encoding_path(cfg,
                                                                           tx,
                                                                           &mut *data.offset(rpos.wrapping_add(2
                                                                                                                   as
                                                                                                                   libc::c_int
                                                                                                                   as
                                                                                                                   libc::c_ulong)
                                                                                                 as
                                                                                                 isize));
                                                rpos =
                                                    (rpos as
                                                         libc::c_ulong).wrapping_add(6
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong)
                                                        as size_t as size_t
                                            }
                                        }
                                    }
                                    _ => { }
                                }
                            }
                        } else {
                            // Invalid %u encoding (not enough data)
                            (*tx).flags =
                                ((*tx).flags as libc::c_ulonglong |
                                     0x10000 as libc::c_ulonglong) as
                                    uint64_t;
                            if (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as
                                                       libc::c_int as
                                                       usize].url_encoding_invalid_unwanted
                                   as libc::c_uint !=
                                   HTP_UNWANTED_IGNORE as libc::c_int as
                                       libc::c_uint {
                                (*tx).response_status_expected_number =
                                    (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH
                                                            as libc::c_int as
                                                            usize].url_encoding_invalid_unwanted
                                        as libc::c_int
                            }
                            match (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as
                                                          libc::c_int as
                                                          usize].url_encoding_invalid_handling
                                      as libc::c_uint {
                                1 => {
                                    current_block_104 = 15984154738040588190;
                                    match current_block_104 {
                                        11934984557441853882 => {
                                            // Leave the percent character in output
                                            rpos = rpos.wrapping_add(1)
                                        }
                                        15984154738040588190 => {
                                            // Do not place anything in output; eat
                                    // the percent character
                                            rpos = rpos.wrapping_add(1);
                                            continue ;
                                        }
                                        _ => {
                                            // Cannot decode, because there's not enough data.
                                    // Leave the percent character in output
                                            rpos = rpos.wrapping_add(1)
                                        }
                                    }
                                }
                                0 => {
                                    current_block_104 = 11934984557441853882;
                                    match current_block_104 {
                                        11934984557441853882 => {
                                            rpos = rpos.wrapping_add(1)
                                        }
                                        15984154738040588190 => {
                                            rpos = rpos.wrapping_add(1);
                                            continue ;
                                        }
                                        _ => { rpos = rpos.wrapping_add(1) }
                                    }
                                }
                                2 => {
                                    current_block_104 = 14856184476078576297;
                                    match current_block_104 {
                                        11934984557441853882 => {
                                            rpos = rpos.wrapping_add(1)
                                        }
                                        15984154738040588190 => {
                                            rpos = rpos.wrapping_add(1);
                                            continue ;
                                        }
                                        _ => { rpos = rpos.wrapping_add(1) }
                                    }
                                }
                                _ => { }
                            }
                        }
                    }
                }
                // Handle standard URL encoding
                if handled == 0 {
                    if *(*__ctype_b_loc()).offset(*data.offset(rpos.wrapping_add(1
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong)
                                                                   as isize)
                                                      as libc::c_int as isize)
                           as libc::c_int &
                           _ISxdigit as libc::c_int as libc::c_ushort as
                               libc::c_int != 0 &&
                           *(*__ctype_b_loc()).offset(*data.offset(rpos.wrapping_add(2
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong)
                                                                       as
                                                                       isize)
                                                          as libc::c_int as
                                                          isize) as
                               libc::c_int &
                               _ISxdigit as libc::c_int as libc::c_ushort as
                                   libc::c_int != 0 {
                        c =
                            x2c(&mut *data.offset(rpos.wrapping_add(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong)
                                                      as isize)) as
                                libc::c_int;
                        if c == 0 as libc::c_int {
                            (*tx).flags =
                                ((*tx).flags as libc::c_ulonglong |
                                     0x4000 as libc::c_ulonglong) as uint64_t;
                            if (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as
                                                       libc::c_int as
                                                       usize].nul_encoded_unwanted
                                   as libc::c_uint !=
                                   HTP_UNWANTED_IGNORE as libc::c_int as
                                       libc::c_uint {
                                (*tx).response_status_expected_number =
                                    (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH
                                                            as libc::c_int as
                                                            usize].nul_encoded_unwanted
                                        as libc::c_int
                            }
                            if (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as
                                                       libc::c_int as
                                                       usize].nul_encoded_terminates
                                   != 0 {
                                bstr_adjust_len(path, wpos);
                                return 1 as libc::c_int
                            }
                        }
                        if c == '/' as i32 ||
                               (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as
                                                       libc::c_int as
                                                       usize].backslash_convert_slashes
                                   != 0 && c == '\\' as i32 {
                            (*tx).flags =
                                ((*tx).flags as libc::c_ulonglong |
                                     0x80000 as libc::c_ulonglong) as
                                    uint64_t;
                            if (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as
                                                       libc::c_int as
                                                       usize].path_separators_encoded_unwanted
                                   as libc::c_uint !=
                                   HTP_UNWANTED_IGNORE as libc::c_int as
                                       libc::c_uint {
                                (*tx).response_status_expected_number =
                                    (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH
                                                            as libc::c_int as
                                                            usize].path_separators_encoded_unwanted
                                        as libc::c_int
                            }
                            if (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as
                                                       libc::c_int as
                                                       usize].path_separators_decode
                                   != 0 {
                                // Decode
                                rpos =
                                    (rpos as
                                         libc::c_ulong).wrapping_add(3 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong)
                                        as size_t as size_t
                            } else {
                                // Leave encoded
                                c = '%' as i32;
                                rpos = rpos.wrapping_add(1)
                            }
                        } else {
                            // Decode
                            rpos =
                                (rpos as
                                     libc::c_ulong).wrapping_add(3 as
                                                                     libc::c_int
                                                                     as
                                                                     libc::c_ulong)
                                    as size_t as size_t
                        }
                    } else {
                        // Invalid encoding
                        (*tx).flags =
                            ((*tx).flags as libc::c_ulonglong |
                                 0x10000 as libc::c_ulonglong) as uint64_t;
                        if (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as
                                                   libc::c_int as
                                                   usize].url_encoding_invalid_unwanted
                               as libc::c_uint !=
                               HTP_UNWANTED_IGNORE as libc::c_int as
                                   libc::c_uint {
                            (*tx).response_status_expected_number =
                                (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as
                                                        libc::c_int as
                                                        usize].url_encoding_invalid_unwanted
                                    as libc::c_int
                        }
                        match (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as
                                                      libc::c_int as
                                                      usize].url_encoding_invalid_handling
                                  as libc::c_uint {
                            1 => {
                                // Do not place anything in output; eat
                                // the percent character
                                rpos = rpos.wrapping_add(1);
                                continue ;
                            }
                            0 => {
                                // Leave the percent character in output
                                rpos = rpos.wrapping_add(1)
                            }
                            2 => {
                                // Decode
                                c =
                                    x2c(&mut *data.offset(rpos.wrapping_add(1
                                                                                as
                                                                                libc::c_int
                                                                                as
                                                                                libc::c_ulong)
                                                              as isize)) as
                                        libc::c_int;
                                rpos =
                                    (rpos as
                                         libc::c_ulong).wrapping_add(3 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong)
                                        as size_t as size_t
                            }
                            _ => {
                                // Unknown setting
                                return -(1 as libc::c_int)
                            }
                        }
                    }
                }
            } else {
                // Invalid URL encoding (not enough data)
                (*tx).flags =
                    ((*tx).flags as libc::c_ulonglong |
                         0x10000 as libc::c_ulonglong) as uint64_t;
                if (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as libc::c_int as
                                           usize].url_encoding_invalid_unwanted
                       as libc::c_uint !=
                       HTP_UNWANTED_IGNORE as libc::c_int as libc::c_uint {
                    (*tx).response_status_expected_number =
                        (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as
                                                libc::c_int as
                                                usize].url_encoding_invalid_unwanted
                            as libc::c_int
                }
                match (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as libc::c_int
                                              as
                                              usize].url_encoding_invalid_handling
                          as libc::c_uint {
                    1 => {
                        current_block_104 = 5650022063725743123;
                        match current_block_104 {
                            10614498797110429124 => {
                                // Cannot decode, because there's not enough data.
                        // Leave the percent character in output.
                        // TODO Configurable handling.
                                rpos = rpos.wrapping_add(1)
                            }
                            5986777620604961003 => {
                                // Leave the percent character in output
                                rpos = rpos.wrapping_add(1)
                            }
                            _ => {
                                // Do not place anything in output; eat
                        // the percent character
                                rpos = rpos.wrapping_add(1);
                                continue ;
                            }
                        }
                    }
                    0 => {
                        current_block_104 = 5986777620604961003;
                        match current_block_104 {
                            10614498797110429124 => {
                                rpos = rpos.wrapping_add(1)
                            }
                            5986777620604961003 => {
                                rpos = rpos.wrapping_add(1)
                            }
                            _ => { rpos = rpos.wrapping_add(1); continue ; }
                        }
                    }
                    2 => {
                        current_block_104 = 10614498797110429124;
                        match current_block_104 {
                            10614498797110429124 => {
                                rpos = rpos.wrapping_add(1)
                            }
                            5986777620604961003 => {
                                rpos = rpos.wrapping_add(1)
                            }
                            _ => { rpos = rpos.wrapping_add(1); continue ; }
                        }
                    }
                    _ => { }
                }
            }
        } else {
            // One non-encoded character
            // Is it a NUL byte?
            if c == 0 as libc::c_int {
                if (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as libc::c_int as
                                           usize].nul_raw_unwanted as
                       libc::c_uint !=
                       HTP_UNWANTED_IGNORE as libc::c_int as libc::c_uint {
                    (*tx).response_status_expected_number =
                        (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as
                                                libc::c_int as
                                                usize].nul_raw_unwanted as
                            libc::c_int
                }
                if (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as libc::c_int as
                                           usize].nul_raw_terminates != 0 {
                    // Terminate path with a raw NUL byte
                    bstr_adjust_len(path, wpos);
                    return 1 as libc::c_int
                }
            }
            rpos = rpos.wrapping_add(1)
        }
        // Note: What if an invalid encoding decodes into a path
                                //       separator? This is theoretical at the moment, because
                                //       the only platform we know doesn't convert separators is
                                //       Apache, who will also respond with 400 if invalid encoding
                                //       is encountered. Thus no check for a separator here.
        // Place the character into output
        // Check for control characters
        if c < 0x20 as libc::c_int {
            if (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as libc::c_int as
                                       usize].control_chars_unwanted as
                   libc::c_uint !=
                   HTP_UNWANTED_IGNORE as libc::c_int as libc::c_uint {
                (*tx).response_status_expected_number =
                    (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as libc::c_int as
                                            usize].control_chars_unwanted as
                        libc::c_int
            }
        }
        // Convert backslashes to forward slashes, if necessary
        if c == '\\' as i32 &&
               (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as libc::c_int as
                                       usize].backslash_convert_slashes != 0 {
            c = '/' as i32
        }
        // Lowercase characters, if necessary
        if (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as libc::c_int as
                                   usize].convert_lowercase != 0 {
            c = tolower(c)
        }
        // If we're compressing separators then we need
        // to track if the previous character was a separator
        if (*cfg).decoder_cfgs[HTP_DECODER_URL_PATH as libc::c_int as
                                   usize].path_separators_compress != 0 {
            if c == '/' as i32 {
                if previous_was_separator == 0 {
                    let fresh3 = wpos;
                    wpos = wpos.wrapping_add(1);
                    *data.offset(fresh3 as isize) = c as libc::c_uchar;
                    previous_was_separator = 1 as libc::c_int
                }
            } else {
                let fresh4 = wpos;
                wpos = wpos.wrapping_add(1);
                *data.offset(fresh4 as isize) = c as libc::c_uchar;
                previous_was_separator = 0 as libc::c_int
            }
        } else {
            let fresh5 = wpos;
            wpos = wpos.wrapping_add(1);
            *data.offset(fresh5 as isize) = c as libc::c_uchar
        }
    }
    bstr_adjust_len(path, wpos);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn htp_tx_urldecode_uri_inplace(mut tx: *mut htp_tx_t,
                                                      mut input: *mut bstr)
 -> htp_status_t {
    let mut flags: uint64_t = 0 as libc::c_int as uint64_t;
    let mut rc: htp_status_t =
        htp_urldecode_inplace_ex((*tx).cfg, HTP_DECODER_URL_PATH, input,
                                 &mut flags,
                                 &mut (*tx).response_status_expected_number);
    if flags as libc::c_ulonglong & 0x10000000 as libc::c_ulonglong != 0 {
        (*tx).flags =
            ((*tx).flags as libc::c_ulonglong | 0x10000 as libc::c_ulonglong)
                as uint64_t
    }
    if flags as libc::c_ulonglong & 0x8000000 as libc::c_ulonglong != 0 {
        (*tx).flags =
            ((*tx).flags as libc::c_ulonglong | 0x4000 as libc::c_ulonglong)
                as uint64_t
    }
    if flags as libc::c_ulonglong & 0x80000000 as libc::c_ulonglong != 0 {
        (*tx).flags =
            ((*tx).flags as libc::c_ulonglong | 0x8000 as libc::c_ulonglong)
                as uint64_t
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn htp_tx_urldecode_params_inplace(mut tx:
                                                             *mut htp_tx_t,
                                                         mut input: *mut bstr)
 -> htp_status_t {
    return htp_urldecode_inplace_ex((*tx).cfg, HTP_DECODER_URLENCODED, input,
                                    &mut (*tx).flags,
                                    &mut (*tx).response_status_expected_number);
}
#[no_mangle]
pub unsafe extern "C" fn htp_urldecode_inplace(mut cfg: *mut htp_cfg_t,
                                               mut ctx: htp_decoder_ctx_t,
                                               mut input: *mut bstr,
                                               mut flags: *mut uint64_t)
 -> htp_status_t {
    let mut expected_status_code: libc::c_int = 0 as libc::c_int;
    return htp_urldecode_inplace_ex(cfg, ctx, input, flags,
                                    &mut expected_status_code);
}
#[no_mangle]
pub unsafe extern "C" fn htp_urldecode_inplace_ex(mut cfg: *mut htp_cfg_t,
                                                  mut ctx: htp_decoder_ctx_t,
                                                  mut input: *mut bstr,
                                                  mut flags: *mut uint64_t,
                                                  mut expected_status_code:
                                                      *mut libc::c_int)
 -> htp_status_t {
    if input.is_null() { return -(1 as libc::c_int) }
    let mut data: *mut libc::c_uchar =
        if (*input).realptr.is_null() {
            (input as
                 *mut libc::c_uchar).offset(::std::mem::size_of::<bstr>() as
                                                libc::c_ulong as isize)
        } else { (*input).realptr };
    if data.is_null() { return -(1 as libc::c_int) }
    let mut len: size_t = (*input).len;
    let mut rpos: size_t = 0 as libc::c_int as size_t;
    let mut wpos: size_t = 0 as libc::c_int as size_t;
    let mut current_block_74: u64;
    while rpos < len && wpos < len {
        let mut c: libc::c_int = *data.offset(rpos as isize) as libc::c_int;
        // Decode encoded characters.
        if c == '%' as i32 {
            // Need at least 2 additional bytes for %HH.
            if rpos.wrapping_add(2 as libc::c_int as libc::c_ulong) < len {
                let mut handled: libc::c_int = 0 as libc::c_int;
                // Decode %uHHHH encoding, but only if allowed in configuration.
                if (*cfg).decoder_cfgs[ctx as usize].u_encoding_decode != 0 {
                    // The next character must be a case-insensitive u.
                    if *data.offset(rpos.wrapping_add(1 as libc::c_int as
                                                          libc::c_ulong) as
                                        isize) as libc::c_int == 'u' as i32 ||
                           *data.offset(rpos.wrapping_add(1 as libc::c_int as
                                                              libc::c_ulong)
                                            as isize) as libc::c_int ==
                               'U' as i32 {
                        handled = 1 as libc::c_int;
                        if (*cfg).decoder_cfgs[ctx as
                                                   usize].u_encoding_unwanted
                               as libc::c_uint !=
                               HTP_UNWANTED_IGNORE as libc::c_int as
                                   libc::c_uint {
                            *expected_status_code =
                                (*cfg).decoder_cfgs[ctx as
                                                        usize].u_encoding_unwanted
                                    as libc::c_int
                        }
                        // Need at least 5 additional bytes for %uHHHH.
                        if rpos.wrapping_add(5 as libc::c_int as
                                                 libc::c_ulong) < len {
                            if *(*__ctype_b_loc()).offset(*data.offset(rpos.wrapping_add(2
                                                                                             as
                                                                                             libc::c_int
                                                                                             as
                                                                                             libc::c_ulong)
                                                                           as
                                                                           isize)
                                                              as libc::c_int
                                                              as isize) as
                                   libc::c_int &
                                   _ISxdigit as libc::c_int as libc::c_ushort
                                       as libc::c_int != 0 &&
                                   *(*__ctype_b_loc()).offset(*data.offset(rpos.wrapping_add(3
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong)
                                                                               as
                                                                               isize)
                                                                  as
                                                                  libc::c_int
                                                                  as isize) as
                                       libc::c_int &
                                       _ISxdigit as libc::c_int as
                                           libc::c_ushort as libc::c_int != 0
                                   &&
                                   *(*__ctype_b_loc()).offset(*data.offset(rpos.wrapping_add(4
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong)
                                                                               as
                                                                               isize)
                                                                  as
                                                                  libc::c_int
                                                                  as isize) as
                                       libc::c_int &
                                       _ISxdigit as libc::c_int as
                                           libc::c_ushort as libc::c_int != 0
                                   &&
                                   *(*__ctype_b_loc()).offset(*data.offset(rpos.wrapping_add(5
                                                                                                 as
                                                                                                 libc::c_int
                                                                                                 as
                                                                                                 libc::c_ulong)
                                                                               as
                                                                               isize)
                                                                  as
                                                                  libc::c_int
                                                                  as isize) as
                                       libc::c_int &
                                       _ISxdigit as libc::c_int as
                                           libc::c_ushort as libc::c_int != 0
                               {
                                // Decode a valid %u encoding.
                                c =
                                    decode_u_encoding_params(cfg, ctx,
                                                             &mut *data.offset(rpos.wrapping_add(2
                                                                                                     as
                                                                                                     libc::c_int
                                                                                                     as
                                                                                                     libc::c_ulong)
                                                                                   as
                                                                                   isize),
                                                             flags);
                                rpos =
                                    (rpos as
                                         libc::c_ulong).wrapping_add(6 as
                                                                         libc::c_int
                                                                         as
                                                                         libc::c_ulong)
                                        as size_t as size_t
                            } else {
                                // Invalid %u encoding (could not find 4 xdigits).
                                *flags =
                                    (*flags as libc::c_ulonglong |
                                         0x10000000 as libc::c_ulonglong) as
                                        uint64_t;
                                if (*cfg).decoder_cfgs[ctx as
                                                           usize].url_encoding_invalid_unwanted
                                       as libc::c_uint !=
                                       HTP_UNWANTED_IGNORE as libc::c_int as
                                           libc::c_uint {
                                    *expected_status_code =
                                        (*cfg).decoder_cfgs[ctx as
                                                                usize].url_encoding_invalid_unwanted
                                            as libc::c_int
                                }
                                match (*cfg).decoder_cfgs[ctx as
                                                              usize].url_encoding_invalid_handling
                                          as libc::c_uint {
                                    1 => {
                                        current_block_74 =
                                            15769233237055051138;
                                        match current_block_74 {
                                            10436515788539709011 => {
                                                // Leave the % in output.
                                                rpos = rpos.wrapping_add(1)
                                            }
                                            15769233237055051138 => {
                                                // Do not place anything in output; consume the %.
                                                rpos = rpos.wrapping_add(1);
                                                continue ;
                                            }
                                            _ => {
                                                // Decode invalid %u encoding.
                                                c =
                                                    decode_u_encoding_params(cfg,
                                                                             ctx,
                                                                             &mut *data.offset(rpos.wrapping_add(2
                                                                                                                     as
                                                                                                                     libc::c_int
                                                                                                                     as
                                                                                                                     libc::c_ulong)
                                                                                                   as
                                                                                                   isize),
                                                                             flags);
                                                rpos =
                                                    (rpos as
                                                         libc::c_ulong).wrapping_add(6
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong)
                                                        as size_t as size_t
                                            }
                                        }
                                    }
                                    0 => {
                                        current_block_74 =
                                            10436515788539709011;
                                        match current_block_74 {
                                            10436515788539709011 => {
                                                rpos = rpos.wrapping_add(1)
                                            }
                                            15769233237055051138 => {
                                                rpos = rpos.wrapping_add(1);
                                                continue ;
                                            }
                                            _ => {
                                                c =
                                                    decode_u_encoding_params(cfg,
                                                                             ctx,
                                                                             &mut *data.offset(rpos.wrapping_add(2
                                                                                                                     as
                                                                                                                     libc::c_int
                                                                                                                     as
                                                                                                                     libc::c_ulong)
                                                                                                   as
                                                                                                   isize),
                                                                             flags);
                                                rpos =
                                                    (rpos as
                                                         libc::c_ulong).wrapping_add(6
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong)
                                                        as size_t as size_t
                                            }
                                        }
                                    }
                                    2 => {
                                        current_block_74 =
                                            16443981440205402410;
                                        match current_block_74 {
                                            10436515788539709011 => {
                                                rpos = rpos.wrapping_add(1)
                                            }
                                            15769233237055051138 => {
                                                rpos = rpos.wrapping_add(1);
                                                continue ;
                                            }
                                            _ => {
                                                c =
                                                    decode_u_encoding_params(cfg,
                                                                             ctx,
                                                                             &mut *data.offset(rpos.wrapping_add(2
                                                                                                                     as
                                                                                                                     libc::c_int
                                                                                                                     as
                                                                                                                     libc::c_ulong)
                                                                                                   as
                                                                                                   isize),
                                                                             flags);
                                                rpos =
                                                    (rpos as
                                                         libc::c_ulong).wrapping_add(6
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong)
                                                        as size_t as size_t
                                            }
                                        }
                                    }
                                    _ => { }
                                }
                            }
                        } else {
                            // Invalid %u encoding; not enough data.
                            *flags =
                                (*flags as libc::c_ulonglong |
                                     0x10000000 as libc::c_ulonglong) as
                                    uint64_t;
                            if (*cfg).decoder_cfgs[ctx as
                                                       usize].url_encoding_invalid_unwanted
                                   as libc::c_uint !=
                                   HTP_UNWANTED_IGNORE as libc::c_int as
                                       libc::c_uint {
                                *expected_status_code =
                                    (*cfg).decoder_cfgs[ctx as
                                                            usize].url_encoding_invalid_unwanted
                                        as libc::c_int
                            }
                            match (*cfg).decoder_cfgs[ctx as
                                                          usize].url_encoding_invalid_handling
                                      as libc::c_uint {
                                1 => {
                                    current_block_74 = 16383797545558020236;
                                    match current_block_74 {
                                        16032006980801283503 => {
                                            // Cannot decode because there's not enough data.
                                    // Leave the % in output.
                                    // TODO Configurable handling of %, u, etc.
                                            rpos = rpos.wrapping_add(1)
                                        }
                                        8223123178938535296 => {
                                            // Leave the % in output.
                                            rpos = rpos.wrapping_add(1)
                                        }
                                        _ => {
                                            // Do not place anything in output; consume the %.
                                            rpos = rpos.wrapping_add(1);
                                            continue ;
                                        }
                                    }
                                }
                                0 => {
                                    current_block_74 = 8223123178938535296;
                                    match current_block_74 {
                                        16032006980801283503 => {
                                            rpos = rpos.wrapping_add(1)
                                        }
                                        8223123178938535296 => {
                                            rpos = rpos.wrapping_add(1)
                                        }
                                        _ => {
                                            rpos = rpos.wrapping_add(1);
                                            continue ;
                                        }
                                    }
                                }
                                2 => {
                                    current_block_74 = 16032006980801283503;
                                    match current_block_74 {
                                        16032006980801283503 => {
                                            rpos = rpos.wrapping_add(1)
                                        }
                                        8223123178938535296 => {
                                            rpos = rpos.wrapping_add(1)
                                        }
                                        _ => {
                                            rpos = rpos.wrapping_add(1);
                                            continue ;
                                        }
                                    }
                                }
                                _ => { }
                            }
                        }
                    }
                }
                // Handle standard URL encoding.
                if handled == 0 {
                    // Need 2 hexadecimal digits.
                    if *(*__ctype_b_loc()).offset(*data.offset(rpos.wrapping_add(1
                                                                                     as
                                                                                     libc::c_int
                                                                                     as
                                                                                     libc::c_ulong)
                                                                   as isize)
                                                      as libc::c_int as isize)
                           as libc::c_int &
                           _ISxdigit as libc::c_int as libc::c_ushort as
                               libc::c_int != 0 &&
                           *(*__ctype_b_loc()).offset(*data.offset(rpos.wrapping_add(2
                                                                                         as
                                                                                         libc::c_int
                                                                                         as
                                                                                         libc::c_ulong)
                                                                       as
                                                                       isize)
                                                          as libc::c_int as
                                                          isize) as
                               libc::c_int &
                               _ISxdigit as libc::c_int as libc::c_ushort as
                                   libc::c_int != 0 {
                        // Decode %HH encoding.
                        c =
                            x2c(&mut *data.offset(rpos.wrapping_add(1 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_ulong)
                                                      as isize)) as
                                libc::c_int;
                        rpos =
                            (rpos as
                                 libc::c_ulong).wrapping_add(3 as libc::c_int
                                                                 as
                                                                 libc::c_ulong)
                                as size_t as size_t
                    } else {
                        // Invalid encoding (enough bytes, but not hexadecimal digits).
                        *flags =
                            (*flags as libc::c_ulonglong |
                                 0x10000000 as libc::c_ulonglong) as uint64_t;
                        if (*cfg).decoder_cfgs[ctx as
                                                   usize].url_encoding_invalid_unwanted
                               as libc::c_uint !=
                               HTP_UNWANTED_IGNORE as libc::c_int as
                                   libc::c_uint {
                            *expected_status_code =
                                (*cfg).decoder_cfgs[ctx as
                                                        usize].url_encoding_invalid_unwanted
                                    as libc::c_int
                        }
                        match (*cfg).decoder_cfgs[ctx as
                                                      usize].url_encoding_invalid_handling
                                  as libc::c_uint {
                            1 => {
                                current_block_74 = 15028968826697170054;
                                match current_block_74 {
                                    7617508444621897972 => {
                                        // Leave the % in output.
                                        rpos = rpos.wrapping_add(1)
                                    }
                                    15028968826697170054 => {
                                        // Do not place anything in output; consume the %.
                                        rpos = rpos.wrapping_add(1);
                                        continue ;
                                    }
                                    _ => {
                                        // Decode.
                                        c =
                                            x2c(&mut *data.offset(rpos.wrapping_add(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_ulong)
                                                                      as
                                                                      isize))
                                                as libc::c_int;
                                        rpos =
                                            (rpos as
                                                 libc::c_ulong).wrapping_add(3
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_ulong)
                                                as size_t as size_t
                                    }
                                }
                            }
                            0 => {
                                current_block_74 = 7617508444621897972;
                                match current_block_74 {
                                    7617508444621897972 => {
                                        rpos = rpos.wrapping_add(1)
                                    }
                                    15028968826697170054 => {
                                        rpos = rpos.wrapping_add(1);
                                        continue ;
                                    }
                                    _ => {
                                        c =
                                            x2c(&mut *data.offset(rpos.wrapping_add(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_ulong)
                                                                      as
                                                                      isize))
                                                as libc::c_int;
                                        rpos =
                                            (rpos as
                                                 libc::c_ulong).wrapping_add(3
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_ulong)
                                                as size_t as size_t
                                    }
                                }
                            }
                            2 => {
                                current_block_74 = 3516197883607697062;
                                match current_block_74 {
                                    7617508444621897972 => {
                                        rpos = rpos.wrapping_add(1)
                                    }
                                    15028968826697170054 => {
                                        rpos = rpos.wrapping_add(1);
                                        continue ;
                                    }
                                    _ => {
                                        c =
                                            x2c(&mut *data.offset(rpos.wrapping_add(1
                                                                                        as
                                                                                        libc::c_int
                                                                                        as
                                                                                        libc::c_ulong)
                                                                      as
                                                                      isize))
                                                as libc::c_int;
                                        rpos =
                                            (rpos as
                                                 libc::c_ulong).wrapping_add(3
                                                                                 as
                                                                                 libc::c_int
                                                                                 as
                                                                                 libc::c_ulong)
                                                as size_t as size_t
                                    }
                                }
                            }
                            _ => { }
                        }
                    }
                }
            } else {
                // Invalid encoding; not enough data (at least 2 bytes required).
                *flags =
                    (*flags as libc::c_ulonglong |
                         0x10000000 as libc::c_ulonglong) as uint64_t;
                if (*cfg).decoder_cfgs[ctx as
                                           usize].url_encoding_invalid_unwanted
                       as libc::c_uint !=
                       HTP_UNWANTED_IGNORE as libc::c_int as libc::c_uint {
                    *expected_status_code =
                        (*cfg).decoder_cfgs[ctx as
                                                usize].url_encoding_invalid_unwanted
                            as libc::c_int
                }
                match (*cfg).decoder_cfgs[ctx as
                                              usize].url_encoding_invalid_handling
                          as libc::c_uint {
                    1 => {
                        current_block_74 = 8697558811166951253;
                        match current_block_74 {
                            13503558473217943653 => {
                                // Cannot decode because there's not enough data.
                        // Leave the % in output.
                        // TODO Configurable handling of %, etc.
                                rpos = rpos.wrapping_add(1)
                            }
                            821486359641935908 => {
                                // Leave the % in output.
                                rpos = rpos.wrapping_add(1)
                            }
                            _ => {
                                // Do not place anything in output; consume the %.
                                rpos = rpos.wrapping_add(1);
                                continue ;
                            }
                        }
                    }
                    0 => {
                        current_block_74 = 821486359641935908;
                        match current_block_74 {
                            13503558473217943653 => {
                                rpos = rpos.wrapping_add(1)
                            }
                            821486359641935908 => {
                                rpos = rpos.wrapping_add(1)
                            }
                            _ => { rpos = rpos.wrapping_add(1); continue ; }
                        }
                    }
                    2 => {
                        current_block_74 = 13503558473217943653;
                        match current_block_74 {
                            13503558473217943653 => {
                                rpos = rpos.wrapping_add(1)
                            }
                            821486359641935908 => {
                                rpos = rpos.wrapping_add(1)
                            }
                            _ => { rpos = rpos.wrapping_add(1); continue ; }
                        }
                    }
                    _ => { }
                }
            }
            // Did we get an encoded NUL byte?
            if c == 0 as libc::c_int {
                if (*cfg).decoder_cfgs[ctx as usize].nul_encoded_unwanted as
                       libc::c_uint !=
                       HTP_UNWANTED_IGNORE as libc::c_int as libc::c_uint {
                    *expected_status_code =
                        (*cfg).decoder_cfgs[ctx as usize].nul_encoded_unwanted
                            as libc::c_int
                }
                *flags =
                    (*flags as libc::c_ulonglong |
                         0x8000000 as libc::c_ulonglong) as uint64_t;
                if (*cfg).decoder_cfgs[ctx as usize].nul_encoded_terminates !=
                       0 {
                    // Terminate the path at the raw NUL byte.
                    bstr_adjust_len(input, wpos);
                    return 1 as libc::c_int
                }
            }
            let fresh6 = wpos;
            wpos = wpos.wrapping_add(1);
            *data.offset(fresh6 as isize) = c as libc::c_uchar
        } else if c == '+' as i32 {
            // Decoding of the plus character is conditional on the configuration.
            if (*cfg).decoder_cfgs[ctx as usize].plusspace_decode != 0 {
                c = 0x20 as libc::c_int
            }
            rpos = rpos.wrapping_add(1);
            let fresh7 = wpos;
            wpos = wpos.wrapping_add(1);
            *data.offset(fresh7 as isize) = c as libc::c_uchar
        } else {
            // One non-encoded byte.
            // Did we get a raw NUL byte?
            if c == 0 as libc::c_int {
                if (*cfg).decoder_cfgs[ctx as usize].nul_raw_unwanted as
                       libc::c_uint !=
                       HTP_UNWANTED_IGNORE as libc::c_int as libc::c_uint {
                    *expected_status_code =
                        (*cfg).decoder_cfgs[ctx as usize].nul_raw_unwanted as
                            libc::c_int
                }
                *flags =
                    (*flags as libc::c_ulonglong |
                         0x80000000 as libc::c_ulonglong) as uint64_t;
                if (*cfg).decoder_cfgs[ctx as usize].nul_raw_terminates != 0 {
                    // Terminate the path at the encoded NUL byte.
                    bstr_adjust_len(input, wpos);
                    return 1 as libc::c_int
                }
            }
            rpos = rpos.wrapping_add(1);
            let fresh8 = wpos;
            wpos = wpos.wrapping_add(1);
            *data.offset(fresh8 as isize) = c as libc::c_uchar
        }
    }
    bstr_adjust_len(input, wpos);
    return 1 as libc::c_int;
}
/* *
 * Normalize a previously-parsed request URI.
 *
 * @param[in] connp
 * @param[in] incomplete
 * @param[in] normalized
 * @return HTP_OK or HTP_ERROR
 */
#[no_mangle]
pub unsafe extern "C" fn htp_normalize_parsed_uri(mut tx: *mut htp_tx_t,
                                                  mut incomplete:
                                                      *mut htp_uri_t,
                                                  mut normalized:
                                                      *mut htp_uri_t)
 -> libc::c_int {
    // Scheme.
    if !(*incomplete).scheme.is_null() {
        // Duplicate and convert to lowercase.
        (*normalized).scheme = bstr_dup_lower((*incomplete).scheme);
        if (*normalized).scheme.is_null() { return -(1 as libc::c_int) }
    }
    // Username.
    if !(*incomplete).username.is_null() {
        (*normalized).username = bstr_dup((*incomplete).username);
        if (*normalized).username.is_null() { return -(1 as libc::c_int) }
        htp_tx_urldecode_uri_inplace(tx, (*normalized).username);
    }
    // Password.
    if !(*incomplete).password.is_null() {
        (*normalized).password = bstr_dup((*incomplete).password);
        if (*normalized).password.is_null() { return -(1 as libc::c_int) }
        htp_tx_urldecode_uri_inplace(tx, (*normalized).password);
    }
    // Hostname.
    if !(*incomplete).hostname.is_null() {
        // We know that incomplete->hostname does not contain
        // port information, so no need to check for it here.
        (*normalized).hostname = bstr_dup((*incomplete).hostname);
        if (*normalized).hostname.is_null() { return -(1 as libc::c_int) }
        htp_tx_urldecode_uri_inplace(tx, (*normalized).hostname);
        htp_normalize_hostname_inplace((*normalized).hostname);
    }
    // Port.
    if !(*incomplete).port.is_null() {
        let mut port_parsed: int64_t =
            htp_parse_positive_integer_whitespace(if (*(*incomplete).port).realptr.is_null()
                                                     {
                                                      ((*incomplete).port as
                                                           *mut libc::c_uchar).offset(::std::mem::size_of::<bstr>()
                                                                                          as
                                                                                          libc::c_ulong
                                                                                          as
                                                                                          isize)
                                                  } else {
                                                      (*(*incomplete).port).realptr
                                                  },
                                                  (*(*incomplete).port).len,
                                                  10 as libc::c_int);
        if port_parsed < 0 as libc::c_int as libc::c_long {
            // Failed to parse the port number.
            (*normalized).port_number = -(1 as libc::c_int);
            (*tx).flags =
                ((*tx).flags as libc::c_ulonglong |
                     0x2000000 as libc::c_ulonglong) as uint64_t
        } else if port_parsed > 0 as libc::c_int as libc::c_long &&
                      port_parsed < 65536 as libc::c_int as libc::c_long {
            // Valid port number.
            (*normalized).port_number = port_parsed as libc::c_int
        } else {
            // Port number out of range.
            (*normalized).port_number = -(1 as libc::c_int);
            (*tx).flags =
                ((*tx).flags as libc::c_ulonglong |
                     0x2000000 as libc::c_ulonglong) as uint64_t
        }
    } else { (*normalized).port_number = -(1 as libc::c_int) }
    // Path.
    if !(*incomplete).path.is_null() {
        // Make a copy of the path, so that we can work on it.
        (*normalized).path = bstr_dup((*incomplete).path);
        if (*normalized).path.is_null() { return -(1 as libc::c_int) }
        // Decode URL-encoded (and %u-encoded) characters, as well as lowercase,
        // compress separators and convert backslashes.
        htp_decode_path_inplace(tx, (*normalized).path);
        // Handle UTF-8 in the path.
        if (*(*tx).cfg).decoder_cfgs[HTP_DECODER_URL_PATH as libc::c_int as
                                         usize].utf8_convert_bestfit != 0 {
            // Decode Unicode characters into a single-byte stream, using best-fit mapping.
            htp_utf8_decode_path_inplace((*tx).cfg, tx, (*normalized).path);
        } else {
            // No decoding, but try to validate the path as a UTF-8 stream.
            htp_utf8_validate_path(tx, (*normalized).path);
        }
        // RFC normalization.
        htp_normalize_uri_path_inplace((*normalized).path);
    }
    // Query string.
    if !(*incomplete).query.is_null() {
        (*normalized).query = bstr_dup((*incomplete).query);
        if (*normalized).query.is_null() { return -(1 as libc::c_int) }
    }
    // Fragment.
    if !(*incomplete).fragment.is_null() {
        (*normalized).fragment = bstr_dup((*incomplete).fragment);
        if (*normalized).fragment.is_null() { return -(1 as libc::c_int) }
        htp_tx_urldecode_uri_inplace(tx, (*normalized).fragment);
    }
    return 1 as libc::c_int;
}
/* *
 * Normalize request hostname. Convert all characters to lowercase and
 * remove trailing dots from the end, if present.
 *
 * @param[in] hostname
 * @return Normalized hostname.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_normalize_hostname_inplace(mut hostname:
                                                            *mut bstr)
 -> *mut bstr {
    if hostname.is_null() { return 0 as *mut bstr }
    bstr_to_lowercase(hostname);
    // Remove dots from the end of the string.    
    while bstr_char_at_end(hostname, 0 as libc::c_int as size_t) == '.' as i32
          {
        bstr_chop(hostname);
    }
    return hostname;
}
/* *
 * Normalize URL path. This function implements the remove dot segments algorithm
 * specified in RFC 3986, section 5.2.4.
 *
 * @param[in] s
 */
#[no_mangle]
pub unsafe extern "C" fn htp_normalize_uri_path_inplace(mut s: *mut bstr) {
    if s.is_null() { return }
    let mut data: *mut libc::c_uchar =
        if (*s).realptr.is_null() {
            (s as
                 *mut libc::c_uchar).offset(::std::mem::size_of::<bstr>() as
                                                libc::c_ulong as isize)
        } else { (*s).realptr };
    if data.is_null() { return }
    let mut len: size_t = (*s).len;
    let mut rpos: size_t = 0 as libc::c_int as size_t;
    let mut wpos: size_t = 0 as libc::c_int as size_t;
    let mut c: libc::c_int = -(1 as libc::c_int);
    while rpos < len && wpos < len {
        if c == -(1 as libc::c_int) {
            let fresh9 = rpos;
            rpos = rpos.wrapping_add(1);
            c = *data.offset(fresh9 as isize) as libc::c_int
        }
        // A. If the input buffer begins with a prefix of "../" or "./",
        //    then remove that prefix from the input buffer; otherwise,
        if c == '.' as i32 {
            if rpos.wrapping_add(1 as libc::c_int as libc::c_ulong) < len &&
                   *data.offset(rpos as isize) as libc::c_int == '.' as i32 &&
                   *data.offset(rpos.wrapping_add(1 as libc::c_int as
                                                      libc::c_ulong) as isize)
                       as libc::c_int == '/' as i32 {
                c = -(1 as libc::c_int);
                rpos =
                    (rpos as
                         libc::c_ulong).wrapping_add(2 as libc::c_int as
                                                         libc::c_ulong) as
                        size_t as size_t;
                continue ;
            } else if rpos < len &&
                          *data.offset(rpos as isize) as libc::c_int ==
                              '/' as i32 {
                c = -(1 as libc::c_int);
                rpos =
                    (rpos as
                         libc::c_ulong).wrapping_add(1 as libc::c_int as
                                                         libc::c_ulong) as
                        size_t as size_t;
                continue ;
            }
        }
        if c == '/' as i32 {
            // B. if the input buffer begins with a prefix of "/./" or "/.",
            //    where "." is a complete path segment, then replace that
            //    prefix with "/" in the input buffer; otherwise,
            if rpos.wrapping_add(1 as libc::c_int as libc::c_ulong) < len &&
                   *data.offset(rpos as isize) as libc::c_int == '.' as i32 &&
                   *data.offset(rpos.wrapping_add(1 as libc::c_int as
                                                      libc::c_ulong) as isize)
                       as libc::c_int == '/' as i32 {
                c = '/' as i32;
                rpos =
                    (rpos as
                         libc::c_ulong).wrapping_add(2 as libc::c_int as
                                                         libc::c_ulong) as
                        size_t as size_t;
                continue ;
            } else if rpos.wrapping_add(1 as libc::c_int as libc::c_ulong) ==
                          len &&
                          *data.offset(rpos as isize) as libc::c_int ==
                              '.' as i32 {
                c = '/' as i32;
                rpos =
                    (rpos as
                         libc::c_ulong).wrapping_add(1 as libc::c_int as
                                                         libc::c_ulong) as
                        size_t as size_t;
                continue ;
            } else if rpos.wrapping_add(2 as libc::c_int as libc::c_ulong) <
                          len &&
                          *data.offset(rpos as isize) as libc::c_int ==
                              '.' as i32 &&
                          *data.offset(rpos.wrapping_add(1 as libc::c_int as
                                                             libc::c_ulong) as
                                           isize) as libc::c_int == '.' as i32
                          &&
                          *data.offset(rpos.wrapping_add(2 as libc::c_int as
                                                             libc::c_ulong) as
                                           isize) as libc::c_int == '/' as i32
             {
                c = '/' as i32;
                rpos =
                    (rpos as
                         libc::c_ulong).wrapping_add(3 as libc::c_int as
                                                         libc::c_ulong) as
                        size_t as size_t;
                // C. if the input buffer begins with a prefix of "/../" or "/..",
            //    where ".." is a complete path segment, then replace that
            //    prefix with "/" in the input buffer and remove the last
            //    segment and its preceding "/" (if any) from the output
            //    buffer; otherwise,
                // Remove the last segment
                while wpos > 0 as libc::c_int as libc::c_ulong &&
                          *data.offset(wpos.wrapping_sub(1 as libc::c_int as
                                                             libc::c_ulong) as
                                           isize) as libc::c_int != '/' as i32
                      {
                    wpos = wpos.wrapping_sub(1)
                }
                if wpos > 0 as libc::c_int as libc::c_ulong {
                    wpos = wpos.wrapping_sub(1)
                }
                continue ;
            } else if rpos.wrapping_add(2 as libc::c_int as libc::c_ulong) ==
                          len &&
                          *data.offset(rpos as isize) as libc::c_int ==
                              '.' as i32 &&
                          *data.offset(rpos.wrapping_add(1 as libc::c_int as
                                                             libc::c_ulong) as
                                           isize) as libc::c_int == '.' as i32
             {
                c = '/' as i32;
                rpos =
                    (rpos as
                         libc::c_ulong).wrapping_add(2 as libc::c_int as
                                                         libc::c_ulong) as
                        size_t as size_t;
                // Remove the last segment
                while wpos > 0 as libc::c_int as libc::c_ulong &&
                          *data.offset(wpos.wrapping_sub(1 as libc::c_int as
                                                             libc::c_ulong) as
                                           isize) as libc::c_int != '/' as i32
                      {
                    wpos = wpos.wrapping_sub(1)
                }
                if wpos > 0 as libc::c_int as libc::c_ulong {
                    wpos = wpos.wrapping_sub(1)
                }
                continue ;
            }
        }
        // D.  if the input buffer consists only of "." or "..", then remove
        // that from the input buffer; otherwise,
        if c == '.' as i32 && rpos == len {
            rpos = rpos.wrapping_add(1)
        } else if c == '.' as i32 &&
                      rpos.wrapping_add(1 as libc::c_int as libc::c_ulong) ==
                          len &&
                      *data.offset(rpos as isize) as libc::c_int == '.' as i32
         {
            rpos =
                (rpos as
                     libc::c_ulong).wrapping_add(2 as libc::c_int as
                                                     libc::c_ulong) as size_t
                    as size_t
        } else {
            // E.  move the first path segment in the input buffer to the end of
        // the output buffer, including the initial "/" character (if
        // any) and any subsequent characters up to, but not including,
        // the next "/" character or the end of the input buffer.
            let fresh10 = wpos;
            wpos = wpos.wrapping_add(1);
            *data.offset(fresh10 as isize) = c as libc::c_uchar;
            while rpos < len &&
                      *data.offset(rpos as isize) as libc::c_int != '/' as i32
                      && wpos < len {
                let fresh11 = rpos;
                rpos = rpos.wrapping_add(1);
                let fresh12 = wpos;
                wpos = wpos.wrapping_add(1);
                *data.offset(fresh12 as isize) =
                    *data.offset(fresh11 as isize)
            }
            c = -(1 as libc::c_int)
        }
    }
    bstr_adjust_len(s, wpos);
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
// 1048576 is 1 Mbyte
//deflate max ratio is about 1000
// Parser states, in the order in which they are
// used as a single transaction is processed.
// Parsing functions
// Private transaction functions
// Utility functions
/* *
 *
 */
#[no_mangle]
pub unsafe extern "C" fn fprint_bstr(mut stream: *mut FILE,
                                     mut name: *const libc::c_char,
                                     mut b: *mut bstr) {
    if b.is_null() {
        fprint_raw_data_ex(stream, name,
                           b"(null)\x00" as *const u8 as *const libc::c_char
                               as *const libc::c_void,
                           0 as libc::c_int as size_t,
                           6 as libc::c_int as size_t);
        return
    }
    fprint_raw_data_ex(stream, name,
                       if (*b).realptr.is_null() {
                           (b as
                                *mut libc::c_uchar).offset(::std::mem::size_of::<bstr>()
                                                               as
                                                               libc::c_ulong
                                                               as isize)
                       } else { (*b).realptr } as *const libc::c_void,
                       0 as libc::c_int as size_t, (*b).len);
}
/* *
 *
 */
#[no_mangle]
pub unsafe extern "C" fn fprint_raw_data(mut stream: *mut FILE,
                                         mut name: *const libc::c_char,
                                         mut data: *const libc::c_void,
                                         mut len: size_t) {
    fprint_raw_data_ex(stream, name, data, 0 as libc::c_int as size_t, len);
}
/* *
 *
 */
#[no_mangle]
pub unsafe extern "C" fn fprint_raw_data_ex(mut stream: *mut FILE,
                                            mut name: *const libc::c_char,
                                            mut _data: *const libc::c_void,
                                            mut offset: size_t,
                                            mut printlen: size_t) {
    let mut data: *const libc::c_uchar = _data as *const libc::c_uchar;
    let mut buf: [libc::c_char; 160] = [0; 160];
    let mut len: size_t = offset.wrapping_add(printlen);
    fprintf(stream,
            b"\n%s: ptr %p offset %u len %u\n\x00" as *const u8 as
                *const libc::c_char, name, data as *mut libc::c_void,
            offset as libc::c_uint, len as libc::c_uint);
    while offset < len {
        let mut i: size_t = 0;
        snprintf(buf.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 160]>() as
                     libc::c_ulong,
                 b"%xlx\x00" as *const u8 as *const libc::c_char,
                 offset as libc::c_uint);
        strlcat(buf.as_mut_ptr(),
                b"  \x00" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 160]>() as
                    libc::c_ulong);
        i = 0 as libc::c_int as size_t;
        while i < 8 as libc::c_int as libc::c_ulong {
            if offset.wrapping_add(i) < len {
                let mut step: [libc::c_char; 4] = [0; 4];
                snprintf(step.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 4]>() as
                             libc::c_ulong,
                         b"%02x \x00" as *const u8 as *const libc::c_char,
                         *data.offset(offset.wrapping_add(i) as isize) as
                             libc::c_int);
                strlcat(buf.as_mut_ptr(), step.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 160]>() as
                            libc::c_ulong);
            } else {
                strlcat(buf.as_mut_ptr(),
                        b"   \x00" as *const u8 as *const libc::c_char,
                        ::std::mem::size_of::<[libc::c_char; 160]>() as
                            libc::c_ulong);
            }
            i = i.wrapping_add(1)
        }
        strlcat(buf.as_mut_ptr(),
                b" \x00" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 160]>() as
                    libc::c_ulong);
        i = 8 as libc::c_int as size_t;
        while i < 16 as libc::c_int as libc::c_ulong {
            if offset.wrapping_add(i) < len {
                let mut step_0: [libc::c_char; 4] = [0; 4];
                snprintf(step_0.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 4]>() as
                             libc::c_ulong,
                         b"%02x \x00" as *const u8 as *const libc::c_char,
                         *data.offset(offset.wrapping_add(i) as isize) as
                             libc::c_int);
                strlcat(buf.as_mut_ptr(), step_0.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 160]>() as
                            libc::c_ulong);
            } else {
                strlcat(buf.as_mut_ptr(),
                        b"   \x00" as *const u8 as *const libc::c_char,
                        ::std::mem::size_of::<[libc::c_char; 160]>() as
                            libc::c_ulong);
            }
            i = i.wrapping_add(1)
        }
        strlcat(buf.as_mut_ptr(),
                b" |\x00" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<[libc::c_char; 160]>() as
                    libc::c_ulong);
        i = 0 as libc::c_int as size_t;
        let mut p: *mut libc::c_char =
            buf.as_mut_ptr().offset(strlen(buf.as_mut_ptr()) as isize);
        while offset.wrapping_add(i) < len &&
                  i < 16 as libc::c_int as libc::c_ulong {
            let mut c: libc::c_int =
                *data.offset(offset.wrapping_add(i) as isize) as libc::c_int;
            if *(*__ctype_b_loc()).offset(c as isize) as libc::c_int &
                   _ISprint as libc::c_int as libc::c_ushort as libc::c_int !=
                   0 {
                let fresh13 = p;
                p = p.offset(1);
                *fresh13 = c as libc::c_char
            } else {
                let fresh14 = p;
                p = p.offset(1);
                *fresh14 = '.' as i32 as libc::c_char
            }
            i = i.wrapping_add(1)
        }
        let fresh15 = p;
        p = p.offset(1);
        *fresh15 = '|' as i32 as libc::c_char;
        let fresh16 = p;
        p = p.offset(1);
        *fresh16 = '\n' as i32 as libc::c_char;
        *p = '\u{0}' as i32 as libc::c_char;
        fprintf(stream, b"%s\x00" as *const u8 as *const libc::c_char,
                buf.as_mut_ptr());
        offset =
            (offset as
                 libc::c_ulong).wrapping_add(16 as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t
    }
    fprintf(stream, b"\n\x00" as *const u8 as *const libc::c_char);
}
/* *
 *
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_in_state_as_string(mut connp:
                                                          *mut htp_connp_t)
 -> *mut libc::c_char {
    if connp.is_null() {
        return b"NULL\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    }
    if (*connp).in_state ==
           Some(htp_connp_REQ_IDLE as
                    unsafe extern "C" fn(_: *mut htp_connp_t) -> htp_status_t)
       {
        return b"REQ_IDLE\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    }
    if (*connp).in_state ==
           Some(htp_connp_REQ_LINE as
                    unsafe extern "C" fn(_: *mut htp_connp_t) -> htp_status_t)
       {
        return b"REQ_LINE\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    }
    if (*connp).in_state ==
           Some(htp_connp_REQ_PROTOCOL as
                    unsafe extern "C" fn(_: *mut htp_connp_t) -> htp_status_t)
       {
        return b"REQ_PROTOCOL\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    }
    if (*connp).in_state ==
           Some(htp_connp_REQ_HEADERS as
                    unsafe extern "C" fn(_: *mut htp_connp_t) -> htp_status_t)
       {
        return b"REQ_HEADERS\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    }
    if (*connp).in_state ==
           Some(htp_connp_REQ_CONNECT_CHECK as
                    unsafe extern "C" fn(_: *mut htp_connp_t) -> htp_status_t)
       {
        return b"REQ_CONNECT_CHECK\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    }
    if (*connp).in_state ==
           Some(htp_connp_REQ_CONNECT_WAIT_RESPONSE as
                    unsafe extern "C" fn(_: *mut htp_connp_t) -> htp_status_t)
       {
        return b"REQ_CONNECT_WAIT_RESPONSE\x00" as *const u8 as
                   *const libc::c_char as *mut libc::c_char
    }
    if (*connp).in_state ==
           Some(htp_connp_REQ_BODY_DETERMINE as
                    unsafe extern "C" fn(_: *mut htp_connp_t) -> htp_status_t)
       {
        return b"REQ_BODY_DETERMINE\x00" as *const u8 as *const libc::c_char
                   as *mut libc::c_char
    }
    if (*connp).in_state ==
           Some(htp_connp_REQ_BODY_IDENTITY as
                    unsafe extern "C" fn(_: *mut htp_connp_t) -> htp_status_t)
       {
        return b"REQ_BODY_IDENTITY\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    }
    if (*connp).in_state ==
           Some(htp_connp_REQ_BODY_CHUNKED_LENGTH as
                    unsafe extern "C" fn(_: *mut htp_connp_t) -> htp_status_t)
       {
        return b"REQ_BODY_CHUNKED_LENGTH\x00" as *const u8 as
                   *const libc::c_char as *mut libc::c_char
    }
    if (*connp).in_state ==
           Some(htp_connp_REQ_BODY_CHUNKED_DATA as
                    unsafe extern "C" fn(_: *mut htp_connp_t) -> htp_status_t)
       {
        return b"REQ_BODY_CHUNKED_DATA\x00" as *const u8 as
                   *const libc::c_char as *mut libc::c_char
    }
    if (*connp).in_state ==
           Some(htp_connp_REQ_BODY_CHUNKED_DATA_END as
                    unsafe extern "C" fn(_: *mut htp_connp_t) -> htp_status_t)
       {
        return b"REQ_BODY_CHUNKED_DATA_END\x00" as *const u8 as
                   *const libc::c_char as *mut libc::c_char
    }
    if (*connp).in_state ==
           Some(htp_connp_REQ_FINALIZE as
                    unsafe extern "C" fn(_: *mut htp_connp_t) -> htp_status_t)
       {
        return b"REQ_FINALIZE\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    }
    if (*connp).in_state ==
           Some(htp_connp_REQ_IGNORE_DATA_AFTER_HTTP_0_9 as
                    unsafe extern "C" fn(_: *mut htp_connp_t) -> htp_status_t)
       {
        return b"REQ_IGNORE_DATA_AFTER_HTTP_0_9\x00" as *const u8 as
                   *const libc::c_char as *mut libc::c_char
    }
    return b"UNKNOWN\x00" as *const u8 as *const libc::c_char as
               *mut libc::c_char;
}
/* *
 *
 */
#[no_mangle]
pub unsafe extern "C" fn htp_connp_out_state_as_string(mut connp:
                                                           *mut htp_connp_t)
 -> *mut libc::c_char {
    if connp.is_null() {
        return b"NULL\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    }
    if (*connp).out_state ==
           Some(htp_connp_RES_IDLE as
                    unsafe extern "C" fn(_: *mut htp_connp_t) -> htp_status_t)
       {
        return b"RES_IDLE\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    }
    if (*connp).out_state ==
           Some(htp_connp_RES_LINE as
                    unsafe extern "C" fn(_: *mut htp_connp_t) -> htp_status_t)
       {
        return b"RES_LINE\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    }
    if (*connp).out_state ==
           Some(htp_connp_RES_HEADERS as
                    unsafe extern "C" fn(_: *mut htp_connp_t) -> htp_status_t)
       {
        return b"RES_HEADERS\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    }
    if (*connp).out_state ==
           Some(htp_connp_RES_BODY_DETERMINE as
                    unsafe extern "C" fn(_: *mut htp_connp_t) -> htp_status_t)
       {
        return b"RES_BODY_DETERMINE\x00" as *const u8 as *const libc::c_char
                   as *mut libc::c_char
    }
    if (*connp).out_state ==
           Some(htp_connp_RES_BODY_IDENTITY_CL_KNOWN as
                    unsafe extern "C" fn(_: *mut htp_connp_t) -> htp_status_t)
       {
        return b"RES_BODY_IDENTITY_CL_KNOWN\x00" as *const u8 as
                   *const libc::c_char as *mut libc::c_char
    }
    if (*connp).out_state ==
           Some(htp_connp_RES_BODY_IDENTITY_STREAM_CLOSE as
                    unsafe extern "C" fn(_: *mut htp_connp_t) -> htp_status_t)
       {
        return b"RES_BODY_IDENTITY_STREAM_CLOSE\x00" as *const u8 as
                   *const libc::c_char as *mut libc::c_char
    }
    if (*connp).out_state ==
           Some(htp_connp_RES_BODY_CHUNKED_LENGTH as
                    unsafe extern "C" fn(_: *mut htp_connp_t) -> htp_status_t)
       {
        return b"RES_BODY_CHUNKED_LENGTH\x00" as *const u8 as
                   *const libc::c_char as *mut libc::c_char
    }
    if (*connp).out_state ==
           Some(htp_connp_RES_BODY_CHUNKED_DATA as
                    unsafe extern "C" fn(_: *mut htp_connp_t) -> htp_status_t)
       {
        return b"RES_BODY_CHUNKED_DATA\x00" as *const u8 as
                   *const libc::c_char as *mut libc::c_char
    }
    if (*connp).out_state ==
           Some(htp_connp_RES_BODY_CHUNKED_DATA_END as
                    unsafe extern "C" fn(_: *mut htp_connp_t) -> htp_status_t)
       {
        return b"RES_BODY_CHUNKED_DATA_END\x00" as *const u8 as
                   *const libc::c_char as *mut libc::c_char
    }
    if (*connp).out_state ==
           Some(htp_connp_RES_FINALIZE as
                    unsafe extern "C" fn(_: *mut htp_connp_t) -> htp_status_t)
       {
        return b"RES_BODY_FINALIZE\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    }
    return b"UNKNOWN\x00" as *const u8 as *const libc::c_char as
               *mut libc::c_char;
}
/* *
 *
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_request_progress_as_string(mut tx:
                                                               *mut htp_tx_t)
 -> *mut libc::c_char {
    if tx.is_null() {
        return b"NULL\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    }
    match (*tx).request_progress as libc::c_uint {
        0 => {
            return b"NOT_STARTED\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        1 => {
            return b"REQ_LINE\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        2 => {
            return b"REQ_HEADERS\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        3 => {
            return b"REQ_BODY\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        4 => {
            return b"REQ_TRAILER\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        5 => {
            return b"COMPLETE\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        _ => { }
    }
    return b"INVALID\x00" as *const u8 as *const libc::c_char as
               *mut libc::c_char;
}
/* *
 *
 */
#[no_mangle]
pub unsafe extern "C" fn htp_tx_response_progress_as_string(mut tx:
                                                                *mut htp_tx_t)
 -> *mut libc::c_char {
    if tx.is_null() {
        return b"NULL\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    }
    match (*tx).response_progress as libc::c_uint {
        0 => {
            return b"NOT_STARTED\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        1 => {
            return b"RES_LINE\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        2 => {
            return b"RES_HEADERS\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        3 => {
            return b"RES_BODY\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        4 => {
            return b"RES_TRAILER\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        5 => {
            return b"COMPLETE\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char
        }
        _ => { }
    }
    return b"INVALID\x00" as *const u8 as *const libc::c_char as
               *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn htp_unparse_uri_noencode(mut uri: *mut htp_uri_t)
 -> *mut bstr {
    if uri.is_null() { return 0 as *mut bstr }
    // On the first pass determine the length of the final string
    let mut len: size_t = 0 as libc::c_int as size_t;
    if !(*uri).scheme.is_null() {
        len =
            (len as libc::c_ulong).wrapping_add((*(*uri).scheme).len) as
                size_t as size_t;
        len =
            (len as
                 libc::c_ulong).wrapping_add(3 as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t
        // "://"
    }
    if !(*uri).username.is_null() || !(*uri).password.is_null() {
        if !(*uri).username.is_null() {
            len =
                (len as libc::c_ulong).wrapping_add((*(*uri).username).len) as
                    size_t as size_t
        }
        // "@"
        len =
            (len as
                 libc::c_ulong).wrapping_add(1 as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t; // ":"
        if !(*uri).password.is_null() {
            len =
                (len as libc::c_ulong).wrapping_add((*(*uri).password).len) as
                    size_t as size_t
        } // ":"
        len =
            (len as
                 libc::c_ulong).wrapping_add(1 as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t
    } // "?"
    if !(*uri).hostname.is_null() {
        len =
            (len as libc::c_ulong).wrapping_add((*(*uri).hostname).len) as
                size_t as size_t
    } // "#"
    if !(*uri).port.is_null() {
        len =
            (len as
                 libc::c_ulong).wrapping_add(1 as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t;
        len =
            (len as libc::c_ulong).wrapping_add((*(*uri).port).len) as size_t
                as size_t
    }
    if !(*uri).path.is_null() {
        len =
            (len as libc::c_ulong).wrapping_add((*(*uri).path).len) as size_t
                as size_t
    }
    if !(*uri).query.is_null() {
        len =
            (len as
                 libc::c_ulong).wrapping_add(1 as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t;
        len =
            (len as libc::c_ulong).wrapping_add((*(*uri).query).len) as size_t
                as size_t
    }
    if !(*uri).fragment.is_null() {
        len =
            (len as
                 libc::c_ulong).wrapping_add(1 as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t;
        len =
            (len as libc::c_ulong).wrapping_add((*(*uri).fragment).len) as
                size_t as size_t
    }
    // On the second pass construct the string
    let mut r: *mut bstr = bstr_alloc(len);
    if r.is_null() { return 0 as *mut bstr }
    if !(*uri).scheme.is_null() {
        bstr_add_noex(r, (*uri).scheme);
        bstr_add_c_noex(r, b"://\x00" as *const u8 as *const libc::c_char);
    }
    if !(*uri).username.is_null() || !(*uri).password.is_null() {
        if !(*uri).username.is_null() { bstr_add_noex(r, (*uri).username); }
        bstr_add_c_noex(r, b":\x00" as *const u8 as *const libc::c_char);
        if !(*uri).password.is_null() { bstr_add_noex(r, (*uri).password); }
        bstr_add_c_noex(r, b"@\x00" as *const u8 as *const libc::c_char);
    }
    if !(*uri).hostname.is_null() { bstr_add_noex(r, (*uri).hostname); }
    if !(*uri).port.is_null() {
        bstr_add_c_noex(r, b":\x00" as *const u8 as *const libc::c_char);
        bstr_add_noex(r, (*uri).port);
    }
    if !(*uri).path.is_null() { bstr_add_noex(r, (*uri).path); }
    if !(*uri).query.is_null() {
        bstr_add_c_noex(r, b"?\x00" as *const u8 as *const libc::c_char);
        bstr_add_noex(r, (*uri).query);
        /*
        bstr *query = bstr_dup(uri->query);
        if (query == NULL) {
            bstr_free(r);
            return NULL;
        }

        htp_uriencoding_normalize_inplace(query);

        bstr_add_c_noex(r, "?");
        bstr_add_noex(r, query);

        bstr_free(query);
        */
    }
    if !(*uri).fragment.is_null() {
        bstr_add_c_noex(r, b"#\x00" as *const u8 as *const libc::c_char);
        bstr_add_noex(r, (*uri).fragment);
    }
    return r;
}
/* *
 * Determine if the information provided on the response line
 * is good enough. Browsers are lax when it comes to response
 * line parsing. In most cases they will only look for the
 * words "http" at the beginning.
 *
 * @param[in] data pointer to bytearray
 * @param[in] len length in bytes of data
 * @return 1 for good enough or 0 for not good enough
 */
#[no_mangle]
pub unsafe extern "C" fn htp_treat_response_line_as_body(mut data:
                                                             *const uint8_t,
                                                         mut len: size_t)
 -> libc::c_int {
    // Browser behavior:
    //      Firefox 3.5.x: (?i)^\s*http
    //      IE: (?i)^\s*http\s*/
    //      Safari: ^HTTP/\d+\.\d+\s+\d{3}
    let mut pos: size_t = 0 as libc::c_int as size_t;
    if data.is_null() { return 1 as libc::c_int }
    while pos < len &&
              (htp_is_space(*data.offset(pos as isize) as libc::c_int) != 0 ||
                   *data.offset(pos as isize) as libc::c_int ==
                       0 as libc::c_int) {
        pos = pos.wrapping_add(1)
    }
    if len < pos.wrapping_add(4 as libc::c_int as libc::c_ulong) {
        return 1 as libc::c_int
    }
    if *data.offset(pos as isize) as libc::c_int != 'H' as i32 &&
           *data.offset(pos as isize) as libc::c_int != 'h' as i32 {
        return 1 as libc::c_int
    }
    if *data.offset(pos.wrapping_add(1 as libc::c_int as libc::c_ulong) as
                        isize) as libc::c_int != 'T' as i32 &&
           *data.offset(pos.wrapping_add(1 as libc::c_int as libc::c_ulong) as
                            isize) as libc::c_int != 't' as i32 {
        return 1 as libc::c_int
    }
    if *data.offset(pos.wrapping_add(2 as libc::c_int as libc::c_ulong) as
                        isize) as libc::c_int != 'T' as i32 &&
           *data.offset(pos.wrapping_add(2 as libc::c_int as libc::c_ulong) as
                            isize) as libc::c_int != 't' as i32 {
        return 1 as libc::c_int
    }
    if *data.offset(pos.wrapping_add(3 as libc::c_int as libc::c_ulong) as
                        isize) as libc::c_int != 'P' as i32 &&
           *data.offset(pos.wrapping_add(3 as libc::c_int as libc::c_ulong) as
                            isize) as libc::c_int != 'p' as i32 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
/* *
 * Run the REQUEST_BODY_DATA hook.
 *
 * @param[in] connp
 * @param[in] d
 */
#[no_mangle]
pub unsafe extern "C" fn htp_req_run_hook_body_data(mut connp:
                                                        *mut htp_connp_t,
                                                    mut d: *mut htp_tx_data_t)
 -> htp_status_t {
    // Do not invoke callbacks with an empty data chunk
    if !(*d).data.is_null() && (*d).len == 0 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int
    }
    // Do not invoke callbacks without a transaction.
    if (*connp).in_tx.is_null() { return 1 as libc::c_int }
    // Run transaction hooks first
    let mut rc: htp_status_t =
        htp_hook_run_all((*(*connp).in_tx).hook_request_body_data,
                         d as *mut libc::c_void);
    if rc != 1 as libc::c_int { return rc }
    // Run configuration hooks second
    rc =
        htp_hook_run_all((*(*connp).cfg).hook_request_body_data,
                         d as *mut libc::c_void);
    if rc != 1 as libc::c_int { return rc }
    // On PUT requests, treat request body as file
    if !(*connp).put_file.is_null() {
        let mut file_data: htp_file_data_t =
            htp_file_data_t{file: 0 as *mut htp_file_t,
                            data: 0 as *const libc::c_uchar,
                            len: 0,};
        file_data.data = (*d).data;
        file_data.len = (*d).len;
        file_data.file = (*connp).put_file;
        (*file_data.file).len =
            ((*file_data.file).len as libc::c_ulong).wrapping_add((*d).len) as
                int64_t as int64_t;
        rc =
            htp_hook_run_all((*(*connp).cfg).hook_request_file_data,
                             &mut file_data as *mut htp_file_data_t as
                                 *mut libc::c_void);
        if rc != 1 as libc::c_int { return rc }
    }
    return 1 as libc::c_int;
}
/* *
 * Run the RESPONSE_BODY_DATA hook.
 *
 * @param[in] connp
 * @param[in] d
 */
#[no_mangle]
pub unsafe extern "C" fn htp_res_run_hook_body_data(mut connp:
                                                        *mut htp_connp_t,
                                                    mut d: *mut htp_tx_data_t)
 -> htp_status_t {
    // Do not invoke callbacks with an empty data chunk.
    if !(*d).data.is_null() && (*d).len == 0 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int
    }
    // Run transaction hooks first
    let mut rc: htp_status_t =
        htp_hook_run_all((*(*connp).out_tx).hook_response_body_data,
                         d as *mut libc::c_void);
    if rc != 1 as libc::c_int { return rc }
    // Run configuration hooks second
    rc =
        htp_hook_run_all((*(*connp).cfg).hook_response_body_data,
                         d as *mut libc::c_void);
    if rc != 1 as libc::c_int { return rc }
    return 1 as libc::c_int;
}
/* *
 * Parses the provided memory region, extracting the double-quoted string.
 *
 * @param[in] data
 * @param[in] len
 * @param[out] out
 * @param[out] endoffset
 * @return HTP_OK on success, HTP_DECLINED if the input is not well formed, and HTP_ERROR on fatal errors.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_extract_quoted_string_as_bstr(mut data:
                                                               *mut libc::c_uchar,
                                                           mut len: size_t,
                                                           mut out:
                                                               *mut *mut bstr,
                                                           mut endoffset:
                                                               *mut size_t)
 -> htp_status_t {
    if data.is_null() || out.is_null() { return -(1 as libc::c_int) }
    if len == 0 as libc::c_int as libc::c_ulong { return 0 as libc::c_int }
    let mut pos: size_t = 0 as libc::c_int as size_t;
    // Check that the first character is a double quote.
    if *data.offset(pos as isize) as libc::c_int != '\"' as i32 {
        return 0 as libc::c_int
    }
    // Step over the double quote.
    pos = pos.wrapping_add(1);
    if pos == len { return 0 as libc::c_int }
    // Calculate the length of the resulting string.
    let mut escaped_chars: size_t = 0 as libc::c_int as size_t;
    while pos < len {
        if *data.offset(pos as isize) as libc::c_int == '\\' as i32 {
            if pos.wrapping_add(1 as libc::c_int as libc::c_ulong) < len {
                escaped_chars = escaped_chars.wrapping_add(1);
                pos =
                    (pos as
                         libc::c_ulong).wrapping_add(2 as libc::c_int as
                                                         libc::c_ulong) as
                        size_t as size_t;
                continue ;
            }
        } else if *data.offset(pos as isize) as libc::c_int == '\"' as i32 {
            break ;
        }
        pos = pos.wrapping_add(1)
    }
    // Have we reached the end of input without seeing the terminating double quote?
    if pos == len { return 0 as libc::c_int }
    // Copy the data and unescape it as necessary.
    let mut outlen: size_t =
        pos.wrapping_sub(1 as libc::c_int as
                             libc::c_ulong).wrapping_sub(escaped_chars);
    *out = bstr_alloc(outlen);
    if (*out).is_null() { return -(1 as libc::c_int) }
    let mut outptr: *mut libc::c_uchar =
        if (**out).realptr.is_null() {
            (*out as
                 *mut libc::c_uchar).offset(::std::mem::size_of::<bstr>() as
                                                libc::c_ulong as isize)
        } else { (**out).realptr };
    let mut outpos: size_t = 0 as libc::c_int as size_t;
    pos = 1 as libc::c_int as size_t;
    while pos < len && outpos < outlen {
        // TODO We are not properly unescaping test here, we're only
        //      handling escaped double quotes.
        if *data.offset(pos as isize) as libc::c_int == '\\' as i32 {
            if pos.wrapping_add(1 as libc::c_int as libc::c_ulong) < len {
                let fresh17 = outpos;
                outpos = outpos.wrapping_add(1);
                *outptr.offset(fresh17 as isize) =
                    *data.offset(pos.wrapping_add(1 as libc::c_int as
                                                      libc::c_ulong) as
                                     isize);
                pos =
                    (pos as
                         libc::c_ulong).wrapping_add(2 as libc::c_int as
                                                         libc::c_ulong) as
                        size_t as size_t;
                continue ;
            }
        } else if *data.offset(pos as isize) as libc::c_int == '\"' as i32 {
            break ;
        }
        let fresh18 = pos;
        pos = pos.wrapping_add(1);
        let fresh19 = outpos;
        outpos = outpos.wrapping_add(1);
        *outptr.offset(fresh19 as isize) = *data.offset(fresh18 as isize)
    }
    bstr_adjust_len(*out, outlen);
    if !endoffset.is_null() { *endoffset = pos }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn htp_parse_ct_header(mut header: *mut bstr,
                                             mut ct: *mut *mut bstr)
 -> htp_status_t {
    if header.is_null() || ct.is_null() { return -(1 as libc::c_int) }
    let mut data: *mut libc::c_uchar =
        if (*header).realptr.is_null() {
            (header as
                 *mut libc::c_uchar).offset(::std::mem::size_of::<bstr>() as
                                                libc::c_ulong as isize)
        } else { (*header).realptr };
    let mut len: size_t = (*header).len;
    // The assumption here is that the header value we receive
    // here has been left-trimmed, which means the starting position
    // is on the media type. On some platforms that may not be the
    // case, and we may need to do the left-trim ourselves.
    // Find the end of the MIME type, using the same approach PHP 5.4.3 uses.
    let mut pos: size_t = 0 as libc::c_int as size_t;
    while pos < len && *data.offset(pos as isize) as libc::c_int != ';' as i32
              && *data.offset(pos as isize) as libc::c_int != ',' as i32 &&
              *data.offset(pos as isize) as libc::c_int != ' ' as i32 {
        pos = pos.wrapping_add(1)
    }
    *ct = bstr_dup_ex(header, 0 as libc::c_int as size_t, pos);
    if (*ct).is_null() { return -(1 as libc::c_int) }
    bstr_to_lowercase(*ct);
    return 1 as libc::c_int;
}
/* *
 * Implements relaxed (not strictly RFC) hostname validation.
 * 
 * @param[in] hostname
 * @return 1 if the supplied hostname is valid; 0 if it is not.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_validate_hostname(mut hostname: *mut bstr)
 -> libc::c_int {
    let mut data: *mut libc::c_uchar =
        if (*hostname).realptr.is_null() {
            (hostname as
                 *mut libc::c_uchar).offset(::std::mem::size_of::<bstr>() as
                                                libc::c_ulong as isize)
        } else { (*hostname).realptr };
    let mut len: size_t = (*hostname).len;
    let mut startpos: size_t = 0 as libc::c_int as size_t;
    let mut pos: size_t = 0 as libc::c_int as size_t;
    if len == 0 as libc::c_int as libc::c_ulong ||
           len > 255 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int
    }
    while pos < len {
        // Validate label characters.
        startpos = pos;
        while pos < len &&
                  *data.offset(pos as isize) as libc::c_int != '.' as i32 {
            let mut c: libc::c_uchar = *data.offset(pos as isize);
            // Exactly one dot expected.
            // According to the RFC, the underscore is not allowed in a label, but
            // we allow it here because we think it's often seen in practice.
            if !(c as libc::c_int >= 'a' as i32 &&
                     c as libc::c_int <= 'z' as i32 ||
                     c as libc::c_int >= 'A' as i32 &&
                         c as libc::c_int <= 'Z' as i32 ||
                     c as libc::c_int >= '0' as i32 &&
                         c as libc::c_int <= '9' as i32 ||
                     c as libc::c_int == '-' as i32 ||
                     c as libc::c_int == '_' as i32) {
                return 0 as libc::c_int
            }
            pos = pos.wrapping_add(1)
        }
        if pos.wrapping_sub(startpos) == 0 as libc::c_int as libc::c_ulong ||
               pos.wrapping_sub(startpos) > 63 as libc::c_int as libc::c_ulong
           {
            return 0 as libc::c_int
        }
        if pos >= len { return 1 as libc::c_int }
        startpos = pos;
        while pos < len &&
                  *data.offset(pos as isize) as libc::c_int == '.' as i32 {
            pos = pos.wrapping_add(1)
        }
        if pos.wrapping_sub(startpos) != 1 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn htp_uri_free(mut uri: *mut htp_uri_t) {
    if uri.is_null() { return }
    bstr_free((*uri).scheme);
    bstr_free((*uri).username);
    bstr_free((*uri).password);
    bstr_free((*uri).hostname);
    bstr_free((*uri).port);
    bstr_free((*uri).path);
    bstr_free((*uri).query);
    bstr_free((*uri).fragment);
    free(uri as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn htp_uri_alloc() -> *mut htp_uri_t {
    let mut u: *mut htp_uri_t =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<htp_uri_t>() as libc::c_ulong) as
            *mut htp_uri_t;
    if u.is_null() { return 0 as *mut htp_uri_t }
    (*u).port_number = -(1 as libc::c_int);
    return u;
}
// Validate label length.
// No more data after label.
// How many dots are there?
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
/* *
 * Performs in-place decoding of the input string, according to the configuration specified
 * by cfg and ctx. On output, various flags (HTP_URLEN_*) might be set.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] input
 * @param[out] flags
 *
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
/* *
 * Performs in-place decoding of the input string, according to the configuration specified
 * by cfg and ctx. On output, various flags (HTP_URLEN_*) might be set. If something in the
 * input would cause a particular server to respond with an error, the appropriate status
 * code will be set.
 *
 * @param[in] cfg
 * @param[in] ctx
 * @param[in] input
 * @param[out] flags
 * @param[out] expected_status_code 0 by default, or status code as necessary
 *
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
/* *
 * Returns the LibHTP version string.
 * 
 * @return LibHTP version, for example "LibHTP v0.5.x".
 */
#[no_mangle]
pub unsafe extern "C" fn htp_get_version() -> *mut libc::c_char {
    return b"LibHTP v0.5.32\x00" as *const u8 as *const libc::c_char as
               *mut libc::c_char;
}
