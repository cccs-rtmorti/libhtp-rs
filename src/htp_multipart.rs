use ::libc;
extern "C" {
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    #[no_mangle]
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn umask(__mask: __mode_t) -> __mode_t;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn memchr(_: *const libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> *mut libc::c_char;
    #[no_mangle]
    fn strncat(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> *mut libc::c_char;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn htp_list_array_create(size: size_t) -> *mut crate::src::htp_list::htp_list_array_t;
    #[no_mangle]
    fn htp_list_array_destroy(l: *mut crate::src::htp_list::htp_list_array_t);
    #[no_mangle]
    fn htp_list_array_get(
        l: *const crate::src::htp_list::htp_list_array_t,
        idx: size_t,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn htp_list_array_push(
        l: *mut crate::src::htp_list::htp_list_array_t,
        e: *mut libc::c_void,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_list_array_size(l: *const crate::src::htp_list::htp_list_array_t) -> size_t;
    #[no_mangle]
    fn bstr_builder_append_mem(
        bb: *mut crate::src::bstr_builder::bstr_builder_t,
        data: *const libc::c_void,
        len: size_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn bstr_builder_clear(bb: *mut crate::src::bstr_builder::bstr_builder_t);
    #[no_mangle]
    fn bstr_builder_create() -> *mut crate::src::bstr_builder::bstr_builder_t;
    #[no_mangle]
    fn bstr_builder_destroy(bb: *mut crate::src::bstr_builder::bstr_builder_t);
    #[no_mangle]
    fn bstr_builder_size(bb: *const crate::src::bstr_builder::bstr_builder_t) -> size_t;
    #[no_mangle]
    fn bstr_builder_to_str(bb: *const crate::src::bstr_builder::bstr_builder_t) -> *mut bstr;
    #[no_mangle]
    fn bstr_add_mem(b: *mut bstr, data: *const libc::c_void, len: size_t) -> *mut bstr;
    #[no_mangle]
    fn bstr_add_mem_noex(b: *mut bstr, data: *const libc::c_void, len: size_t) -> *mut bstr;
    #[no_mangle]
    fn bstr_add_noex(bdestination: *mut bstr, bsource: *const bstr) -> *mut bstr;
    #[no_mangle]
    fn bstr_adjust_len(b: *mut bstr, newlen: size_t);
    #[no_mangle]
    fn bstr_begins_with_c(bhaystack: *const bstr, cneedle: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn bstr_cmp_c_nocase(b: *const bstr, cstr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn bstr_dup_mem(data: *const libc::c_void, len: size_t) -> *mut bstr;
    #[no_mangle]
    fn bstr_expand(b: *mut bstr, newsize: size_t) -> *mut bstr;
    #[no_mangle]
    fn bstr_free(b: *mut bstr);
    #[no_mangle]
    fn bstr_index_of_c(bhaystack: *const bstr, cneedle: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn bstr_index_of_c_nocase(bhaystack: *const bstr, cneedle: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn bstr_util_mem_index_of_c_nocase(
        data: *const libc::c_void,
        len: size_t,
        cstr: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn htp_hook_run_all(
        hook: *mut crate::src::htp_hooks::htp_hook_t,
        user_data: *mut libc::c_void,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_table_add(
        table: *mut crate::src::htp_table::htp_table_t,
        key: *const bstr,
        element: *const libc::c_void,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_table_create(size: size_t) -> *mut crate::src::htp_table::htp_table_t;
    #[no_mangle]
    fn htp_table_destroy(table: *mut crate::src::htp_table::htp_table_t);
    #[no_mangle]
    fn htp_table_get(
        table: *const crate::src::htp_table::htp_table_t,
        key: *const bstr,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn htp_table_get_c(
        table: *const crate::src::htp_table::htp_table_t,
        ckey: *const libc::c_char,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn htp_table_get_index(
        table: *const crate::src::htp_table::htp_table_t,
        idx: size_t,
        key: *mut *mut bstr,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn htp_table_size(table: *const crate::src::htp_table::htp_table_t) -> size_t;
    #[no_mangle]
    fn htp_is_token(c: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn htp_is_lws(c: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn htp_is_space(c: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn htp_parse_ct_header(header: *mut bstr, ct: *mut *mut bstr) -> htp_status_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type uint64_t = __uint64_t;
pub type ssize_t = __ssize_t;
pub type mode_t = __mode_t;

pub type htp_status_t = libc::c_int;
pub type bstr = crate::src::bstr::bstr_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_mpartp_t {
    pub multipart: htp_multipart_t,
    pub cfg: *mut crate::src::htp_config::htp_cfg_t,
    pub extract_files: libc::c_int,
    pub extract_limit: libc::c_int,
    pub extract_dir: *mut libc::c_char,
    pub file_count: libc::c_int,

    // Parsing callbacks
    pub handle_data: Option<
        unsafe extern "C" fn(
            _: *mut htp_mpartp_t,
            _: *const libc::c_uchar,
            _: size_t,
            _: libc::c_int,
        ) -> libc::c_int,
    >,
    pub handle_boundary: Option<unsafe extern "C" fn(_: *mut htp_mpartp_t) -> libc::c_int>,
    // Internal parsing fields; move into a private structure
    /**
     * Parser state; one of MULTIPART_STATE_* constants.
     */
    parser_state: htp_multipart_state_t,
    /**
     * Keeps track of the current position in the boundary matching progress.
     * When this field reaches boundary_len, we have a boundary match.
     */
    pub boundary_match_pos: size_t,
    /**
     * Pointer to the part that is currently being processed.
     */
    pub current_part: *mut htp_multipart_part_t,
    /**
     * This parser consists of two layers: the outer layer is charged with
     * finding parts, and the internal layer handles part data. There is an
     * interesting interaction between the two parsers. Because the
     * outer layer is seeing every line (it has to, in order to test for
     * boundaries), it also effectively also splits input into lines. The
     * inner parser deals with two areas: first is the headers, which are
     * line based, followed by binary data. When parsing headers, the inner
     * parser can reuse the lines identified by the outer parser. In this
     * variable we keep the current parsing mode of the part, which helps
     * us process input data more efficiently. The possible values are
     * MULTIPART_MODE_LINE and MULTIPART_MODE_DATA.
     */
    current_part_mode: htp_part_mode_t,
    /**
     * Used for buffering when a potential boundary is fragmented
     * across many input data buffers. On a match, the data stored here is
     * discarded. When there is no match, the buffer is processed as data
     * (belonging to the currently active part).
     */
    pub boundary_pieces: *mut crate::src::bstr_builder::bstr_builder_t,
    pub part_header_pieces: *mut crate::src::bstr_builder::bstr_builder_t,
    pub pending_header_line: *mut bstr,
    /**
     * Stores text part pieces until the entire part is seen, at which
     * point the pieces are assembled into a single buffer, and the
     * builder cleared.
     */
    pub part_data_pieces: *mut crate::src::bstr_builder::bstr_builder_t,
    /**
     * The offset of the current boundary candidate, relative to the most
     * recent data chunk (first unprocessed chunk of data).
     */
    pub boundary_candidate_pos: size_t,
    /**
     * When we encounter a CR as the last byte in a buffer, we don't know
     * if the byte is part of a CRLF combination. If it is, then the CR
     * might be a part of a boundary. But if it is not, it's current
     * part's data. Because we know how to handle everything before the
     * CR, we do, and we use this flag to indicate that a CR byte is
     * effectively being buffered. This is probably a case of premature
     * optimization, but I am going to leave it in for now.
     */
    pub cr_aside: libc::c_int,
    /**
     * When set, indicates that this parser no longer owns names and
     * values of MULTIPART_PART_TEXT parts. It is used to avoid data
     * duplication when the parser is used by LibHTP internally.
     */
    pub gave_up_data: libc::c_int,
}

/**
 * Holds information related to a part.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_multipart_part_t {
    /** Pointer to the parser. */
    pub parser: *mut htp_mpartp_t,
    /** Part type; see the MULTIPART_PART_* constants. */
    pub type_0: htp_multipart_type_t,
    /** Raw part length (i.e., headers and data). */
    pub len: size_t,
    /** Part name, from the Content-Disposition header. Can be NULL. */
    pub name: *mut bstr,
    /**
     * Part value; the contents depends on the type of the part:
     * 1) NULL for files; 2) contains complete part contents for
     * preamble and epilogue parts (they have no headers), and
     * 3) data only (headers excluded) for text and unknown parts.
     */
    pub value: *mut bstr,
    /** Part content type, from the Content-Type header. Can be NULL. */
    pub content_type: *mut bstr,
    /** Part headers (htp_header_t instances), using header name as the key. */
    pub headers: *mut crate::src::htp_table::htp_table_t,
    /** File data, available only for MULTIPART_PART_FILE parts. */
    pub file: *mut crate::src::htp_util::htp_file_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
enum htp_part_mode_t {
    /** When in line mode, the parser is handling part headers. */
    MODE_LINE,
    /** When in data mode, the parser is consuming part data. */
    MODE_DATA,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
enum htp_multipart_state_t {
    /** Initial state, after the parser has been created but before the boundary initialized. */
    STATE_INIT,
    /** Processing data, waiting for a new line (which might indicate a new boundary). */
    STATE_DATA,
    /** Testing a potential boundary. */
    STATE_BOUNDARY,
    /** Checking the first byte after a boundary. */
    STATE_BOUNDARY_IS_LAST1,
    /** Checking the second byte after a boundary. */
    STATE_BOUNDARY_IS_LAST2,
    /** Consuming linear whitespace after a boundary. */
    STATE_BOUNDARY_EAT_LWS,
    /** Used after a CR byte is detected in htp_multipart_state_t::STATE_BOUNDARY_EAT_LWS. */
    STATE_BOUNDARY_EAT_LWS_CR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum htp_multipart_type_t {
    /** Unknown part. */
    MULTIPART_PART_UNKNOWN,
    /** Text (parameter) part. */
    MULTIPART_PART_TEXT,
    /** File part. */
    MULTIPART_PART_FILE,
    /** Free-text part before the first boundary. */
    MULTIPART_PART_PREAMBLE,
    /** Free-text part after the last boundary. */
    MULTIPART_PART_EPILOGUE,
}

/**
 * Holds information related to a multipart body.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_multipart_t {
    /** Multipart boundary. */
    pub boundary: *mut libc::c_char,
    /** Boundary length. */
    pub boundary_len: size_t,
    /** How many boundaries were there? */
    pub boundary_count: libc::c_int,
    /** List of parts, in the order in which they appeared in the body. */
    pub parts: *mut crate::src::htp_list::htp_list_array_t,
    /** Parsing flags. */
    pub flags: uint64_t,
}

pub type htp_time_t = libc::timeval;
/* *
 * Determines the type of a Content-Disposition parameter.
 *
 * @param[in] data
 * @param[in] startpos
 * @param[in] pos
 * @return CD_PARAM_OTHER, CD_PARAM_NAME or CD_PARAM_FILENAME.
 */
unsafe extern "C" fn htp_mpartp_cd_param_type(
    mut data: *mut libc::c_uchar,
    mut startpos: size_t,
    mut endpos: size_t,
) -> libc::c_int {
    if endpos.wrapping_sub(startpos) == 4 as libc::c_int as libc::c_ulong {
        if memcmp(
            data.offset(startpos as isize) as *const libc::c_void,
            b"name\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            return 1 as libc::c_int;
        }
    } else if endpos.wrapping_sub(startpos) == 8 as libc::c_int as libc::c_ulong {
        if memcmp(
            data.offset(startpos as isize) as *const libc::c_void,
            b"filename\x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            8 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            return 2 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}

/**
 * Returns the multipart structure created by the parser.
 *
 * @param[in] parser
 * @return The main multipart structure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_mpartp_get_multipart(
    mut parser: *mut htp_mpartp_t,
) -> *mut htp_multipart_t {
    return &mut (*parser).multipart;
}

/* *
 * Decodes a C-D header value. This is impossible to do correctly without a
 * parsing personality because most browsers are broken:
 *  - Firefox encodes " as \", and \ is not encoded.
 *  - Chrome encodes " as %22.
 *  - IE encodes " as \", and \ is not encoded.
 *  - Opera encodes " as \" and \ as \\.
 * @param[in] b
 */
unsafe extern "C" fn htp_mpart_decode_quoted_cd_value_inplace(mut b: *mut bstr) {
    let mut s: *mut libc::c_uchar = if (*b).realptr.is_null() {
        (b as *mut libc::c_uchar).offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
    } else {
        (*b).realptr
    };
    let mut d: *mut libc::c_uchar = if (*b).realptr.is_null() {
        (b as *mut libc::c_uchar).offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
    } else {
        (*b).realptr
    };
    let mut len: size_t = (*b).len;
    let mut pos: size_t = 0 as libc::c_int as size_t;
    while pos < len {
        // Ignore \ when before \ or ".
        if *s as libc::c_int == '\\' as i32
            && pos.wrapping_add(1 as libc::c_int as libc::c_ulong) < len
            && (*s.offset(1 as libc::c_int as isize) as libc::c_int == '\"' as i32
                || *s.offset(1 as libc::c_int as isize) as libc::c_int == '\\' as i32)
        {
            s = s.offset(1);
            pos = pos.wrapping_add(1)
        }
        let fresh0 = s;
        s = s.offset(1);
        let fresh1 = d;
        d = d.offset(1);
        *fresh1 = *fresh0;
        pos = pos.wrapping_add(1)
    }
    bstr_adjust_len(
        b,
        len.wrapping_sub(s.wrapping_offset_from(d) as libc::c_long as libc::c_ulong),
    );
}

/* *
 * Parses the Content-Disposition part header.
 *
 * @param[in] part
 * @return HTP_OK on success (header found and parsed), HTP_DECLINED if there is no C-D header or if
 *         it could not be processed, and HTP_ERROR on fatal error.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_mpart_part_parse_c_d(
    mut part: *mut htp_multipart_part_t,
) -> htp_status_t {
    // Find the C-D header.
    let mut h: *mut crate::src::htp_transaction::htp_header_t = htp_table_get_c(
        (*part).headers,
        b"content-disposition\x00" as *const u8 as *const libc::c_char,
    )
        as *mut crate::src::htp_transaction::htp_header_t;
    if h.is_null() {
        (*(*part).parser).multipart.flags |= 0x2000 as libc::c_int as libc::c_ulong;
        return 0 as libc::c_int;
    }
    // Require "form-data" at the beginning of the header.
    if bstr_index_of_c(
        (*h).value,
        b"form-data\x00" as *const u8 as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        (*(*part).parser).multipart.flags |= 0x100000 as libc::c_int as libc::c_ulong;
        return 0 as libc::c_int;
    }
    // The parsing starts here.
    let mut data: *mut libc::c_uchar = if (*(*h).value).realptr.is_null() {
        ((*h).value as *mut libc::c_uchar)
            .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
    } else {
        (*(*h).value).realptr
    }; // Start after "form-data"
    let mut len: size_t = (*(*h).value).len;
    let mut pos: size_t = 9 as libc::c_int as size_t;
    // Main parameter parsing loop (once per parameter).
    while pos < len {
        // Ignore whitespace.
        while pos < len
            && *(*__ctype_b_loc()).offset(*data.offset(pos as isize) as libc::c_int as isize)
                as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            pos = pos.wrapping_add(1)
        }
        if pos == len {
            (*(*part).parser).multipart.flags |= 0x100000 as libc::c_int as libc::c_ulong;
            return 0 as libc::c_int;
        }
        // Continue to parse the next parameter, if any.
        if *data.offset(pos as isize) as libc::c_int != ';' as i32 {
            (*(*part).parser).multipart.flags |= 0x100000 as libc::c_int as libc::c_ulong;
            return 0 as libc::c_int;
        }
        pos = pos.wrapping_add(1);
        while pos < len
            && *(*__ctype_b_loc()).offset(*data.offset(pos as isize) as libc::c_int as isize)
                as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        // Expecting a semicolon.
        // Go over the whitespace before parameter name.
        {
            pos = pos.wrapping_add(1)
        }
        if pos == len {
            (*(*part).parser).multipart.flags |= 0x100000 as libc::c_int as libc::c_ulong;
            return 0 as libc::c_int;
        }
        let mut start: size_t = pos;
        while pos < len
            && (*(*__ctype_b_loc()).offset(*data.offset(pos as isize) as libc::c_int as isize)
                as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                == 0
                && *data.offset(pos as isize) as libc::c_int != '=' as i32)
        // Found the starting position of the parameter name.
        // Look for the ending position.
        {
            pos = pos.wrapping_add(1)
        }
        if pos == len {
            (*(*part).parser).multipart.flags |= 0x100000 as libc::c_int as libc::c_ulong;
            return 0 as libc::c_int;
        }
        let mut param_type: libc::c_int = htp_mpartp_cd_param_type(data, start, pos);
        while pos < len
            && *(*__ctype_b_loc()).offset(*data.offset(pos as isize) as libc::c_int as isize)
                as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        // Ending position is in "pos" now.
        // Determine parameter type ("name", "filename", or other).
        // Ignore whitespace after parameter name, if any.
        {
            pos = pos.wrapping_add(1)
        }
        if pos == len {
            (*(*part).parser).multipart.flags |= 0x100000 as libc::c_int as libc::c_ulong;
            return 0 as libc::c_int;
        }
        if *data.offset(pos as isize) as libc::c_int != '=' as i32 {
            (*(*part).parser).multipart.flags |= 0x100000 as libc::c_int as libc::c_ulong;
            return 0 as libc::c_int;
        }
        pos = pos.wrapping_add(1);
        while pos < len
            && *(*__ctype_b_loc()).offset(*data.offset(pos as isize) as libc::c_int as isize)
                as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        // Equals.
        // Go over the whitespace before the parameter value.
        {
            pos = pos.wrapping_add(1)
        }
        if pos == len {
            (*(*part).parser).multipart.flags |= 0x100000 as libc::c_int as libc::c_ulong;
            return 0 as libc::c_int;
        }
        if *data.offset(pos as isize) as libc::c_int != '\"' as i32 {
            // Expecting a double quote.
            // Bare string or non-standard quoting, which we don't like.
            (*(*part).parser).multipart.flags |= 0x100000 as libc::c_int as libc::c_ulong; // Over the double quote.
            return 0 as libc::c_int;
        }
        pos = pos.wrapping_add(1);
        start = pos;
        while pos < len && *data.offset(pos as isize) as libc::c_int != '\"' as i32
        // We have the starting position of the value.
        // Find the end of the value.
        {
            // Check for escaping.
            if *data.offset(pos as isize) as libc::c_int == '\\' as i32 {
                if pos.wrapping_add(1 as libc::c_int as libc::c_ulong) >= len {
                    // A backslash as the last character in the C-D header.
                    (*(*part).parser).multipart.flags |= 0x100000 as libc::c_int as libc::c_ulong;
                    return 0 as libc::c_int;
                }
                // Allow " and \ to be escaped.
                if *data.offset(pos.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int
                    == '\"' as i32
                    || *data.offset(pos.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize)
                        as libc::c_int
                        == '\\' as i32
                {
                    // Go over the quoted character.
                    pos = pos.wrapping_add(1)
                }
            }
            pos = pos.wrapping_add(1)
        }
        if pos == len {
            (*(*part).parser).multipart.flags |= 0x100000 as libc::c_int as libc::c_ulong;
            return 0 as libc::c_int;
        }
        if *data.offset(pos as isize) as libc::c_int != '\"' as i32 {
            (*(*part).parser).multipart.flags |= 0x100000 as libc::c_int as libc::c_ulong;
            return 0 as libc::c_int;
        }
        pos = pos.wrapping_add(1);
        match param_type {
            1 => {
                // If we've reached the end of the string that means the
                // value was not terminated properly (the second double quote is missing).
                // Expecting the terminating double quote.
                // Over the terminating double quote.
                // Finally, process the parameter value.
                // Check that we have not seen the name parameter already.
                if !(*part).name.is_null() {
                    (*(*part).parser).multipart.flags |= 0x40000 as libc::c_int as libc::c_ulong;
                    return 0 as libc::c_int;
                }
                (*part).name = bstr_dup_mem(
                    data.offset(start as isize) as *const libc::c_void,
                    pos.wrapping_sub(start)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                if (*part).name.is_null() {
                    return -(1 as libc::c_int);
                }
                htp_mpart_decode_quoted_cd_value_inplace((*part).name);
            }
            2 => {
                // Check that we have not seen the filename parameter already.
                if !(*part).file.is_null() {
                    (*(*part).parser).multipart.flags |= 0x40000 as libc::c_int as libc::c_ulong;
                    return 0 as libc::c_int;
                }
                (*part).file = calloc(
                    1 as libc::c_int as libc::c_ulong,
                    ::std::mem::size_of::<crate::src::htp_util::htp_file_t>() as libc::c_ulong,
                ) as *mut crate::src::htp_util::htp_file_t;
                if (*part).file.is_null() {
                    return -(1 as libc::c_int);
                }
                (*(*part).file).fd = -(1 as libc::c_int);
                (*(*part).file).source =
                    crate::src::htp_util::htp_file_source_t::HTP_FILE_MULTIPART;
                (*(*part).file).filename = bstr_dup_mem(
                    data.offset(start as isize) as *const libc::c_void,
                    pos.wrapping_sub(start)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                if (*(*part).file).filename.is_null() {
                    free((*part).file as *mut libc::c_void);
                    return -(1 as libc::c_int);
                }
                htp_mpart_decode_quoted_cd_value_inplace((*(*part).file).filename);
            }
            _ => {
                // Unknown parameter.
                (*(*part).parser).multipart.flags |= 0x80000 as libc::c_int as libc::c_ulong;
                return 0 as libc::c_int;
            }
        }
    }
    return 1 as libc::c_int;
}

/* *
 * Parses the Content-Type part header, if present.
 *
 * @param[in] part
 * @return HTP_OK on success, HTP_DECLINED if the C-T header is not present, and HTP_ERROR on failure.
 */
unsafe extern "C" fn htp_mpart_part_parse_c_t(mut part: *mut htp_multipart_part_t) -> htp_status_t {
    let mut h: *mut crate::src::htp_transaction::htp_header_t = htp_table_get_c(
        (*part).headers,
        b"content-type\x00" as *const u8 as *const libc::c_char,
    )
        as *mut crate::src::htp_transaction::htp_header_t;
    if h.is_null() {
        return 0 as libc::c_int;
    }
    return htp_parse_ct_header((*h).value, &mut (*part).content_type);
}

/* *
 * Processes part headers.
 *
 * @param[in] part
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_mpart_part_process_headers(
    mut part: *mut htp_multipart_part_t,
) -> htp_status_t {
    if htp_mpart_part_parse_c_d(part) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if htp_mpart_part_parse_c_t(part) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}

/* *
 * Parses one part header.
 *
 * @param[in] part
 * @param[in] data
 * @param[in] len
 * @return HTP_OK on success, HTP_DECLINED on parsing error, HTP_ERROR on fatal error.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_mpartp_parse_header(
    mut part: *mut htp_multipart_part_t,
    mut data: *const libc::c_uchar,
    mut len: size_t,
) -> htp_status_t {
    let mut name_start: size_t = 0;
    let mut name_end: size_t = 0;
    let mut value_start: size_t = 0;
    let mut value_end: size_t = 0;
    // We do not allow NUL bytes here.
    if !memchr(data as *const libc::c_void, '\u{0}' as i32, len).is_null() {
        (*(*part).parser).multipart.flags |= 0x400000 as libc::c_int as libc::c_ulong;
        return 0 as libc::c_int;
    }
    name_start = 0 as libc::c_int as size_t;
    // Look for the starting position of the name first.
    let mut colon_pos: size_t = 0 as libc::c_int as size_t;
    while colon_pos < len && htp_is_space(*data.offset(colon_pos as isize) as libc::c_int) != 0 {
        colon_pos = colon_pos.wrapping_add(1)
    }
    if colon_pos != 0 as libc::c_int as libc::c_ulong {
        // Whitespace before header name.
        (*(*part).parser).multipart.flags |= 0x10000 as libc::c_int as libc::c_ulong;
        return 0 as libc::c_int;
    }
    // Now look for the colon.
    while colon_pos < len && *data.offset(colon_pos as isize) as libc::c_int != ':' as i32 {
        colon_pos = colon_pos.wrapping_add(1)
    }
    if colon_pos == len {
        // Missing colon.
        (*(*part).parser).multipart.flags |= 0x10000 as libc::c_int as libc::c_ulong;
        return 0 as libc::c_int;
    }
    if colon_pos == 0 as libc::c_int as libc::c_ulong {
        // Empty header name.
        (*(*part).parser).multipart.flags |= 0x10000 as libc::c_int as libc::c_ulong;
        return 0 as libc::c_int;
    }
    name_end = colon_pos;
    // Ignore LWS after header name.
    let mut prev: size_t = name_end;
    if prev > name_start
        && htp_is_lws(
            *data.offset(prev.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                as libc::c_int,
        ) != 0
    {
        prev = prev.wrapping_sub(1);
        name_end = name_end.wrapping_sub(1);
        // LWS after field name. Not allowing for now.
        (*(*part).parser).multipart.flags |= 0x10000 as libc::c_int as libc::c_ulong;
        return 0 as libc::c_int;
    }
    // Header value.
    value_start = colon_pos.wrapping_add(1 as libc::c_int as libc::c_ulong);
    // Ignore LWS before value.
    while value_start < len && htp_is_lws(*data.offset(value_start as isize) as libc::c_int) != 0 {
        value_start = value_start.wrapping_add(1)
    }
    if value_start == len {
        // No header value.
        (*(*part).parser).multipart.flags |= 0x10000 as libc::c_int as libc::c_ulong;
        return 0 as libc::c_int;
    }
    // Assume the value is at the end.
    value_end = len;
    // Check that the header name is a token.
    let mut i: size_t = name_start;
    while i < name_end {
        if htp_is_token(*data.offset(i as isize) as libc::c_int) == 0 {
            (*(*part).parser).multipart.flags |= 0x10000 as libc::c_int as libc::c_ulong;
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1)
    }
    // Now extract the name and the value.
    let mut h: *mut crate::src::htp_transaction::htp_header_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<crate::src::htp_transaction::htp_header_t>() as libc::c_ulong,
    )
        as *mut crate::src::htp_transaction::htp_header_t;
    if h.is_null() {
        return -(1 as libc::c_int);
    }
    (*h).name = bstr_dup_mem(
        data.offset(name_start as isize) as *const libc::c_void,
        name_end.wrapping_sub(name_start),
    );
    if (*h).name.is_null() {
        free(h as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    (*h).value = bstr_dup_mem(
        data.offset(value_start as isize) as *const libc::c_void,
        value_end.wrapping_sub(value_start),
    );
    if (*h).value.is_null() {
        bstr_free((*h).name);
        free(h as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    if bstr_cmp_c_nocase(
        (*h).name,
        b"content-disposition\x00" as *const u8 as *const libc::c_char,
    ) != 0 as libc::c_int
        && bstr_cmp_c_nocase(
            (*h).name,
            b"content-type\x00" as *const u8 as *const libc::c_char,
        ) != 0 as libc::c_int
    {
        (*(*part).parser).multipart.flags |= 0x8000 as libc::c_int as libc::c_ulong
    }
    // Check if the header already exists.
    let mut h_existing: *mut crate::src::htp_transaction::htp_header_t =
        htp_table_get((*part).headers, (*h).name) as *mut crate::src::htp_transaction::htp_header_t;
    if !h_existing.is_null() {
        // Add to the existing header.
        let mut new_value: *mut bstr = bstr_expand(
            (*h_existing).value,
            (*(*h_existing).value)
                .len
                .wrapping_add(2 as libc::c_int as libc::c_ulong)
                .wrapping_add((*(*h).value).len),
        );
        if new_value.is_null() {
            bstr_free((*h).name);
            bstr_free((*h).value);
            free(h as *mut libc::c_void);
            return -(1 as libc::c_int);
        }
        (*h_existing).value = new_value;
        bstr_add_mem_noex(
            (*h_existing).value,
            b", \x00" as *const u8 as *const libc::c_char as *const libc::c_void,
            2 as libc::c_int as size_t,
        );
        bstr_add_noex((*h_existing).value, (*h).value);
        // The header is no longer needed.
        bstr_free((*h).name);
        bstr_free((*h).value);
        free(h as *mut libc::c_void);
        // Keep track of same-name headers.
        (*h_existing).flags |= 0x4000 as libc::c_int as libc::c_ulong;
        (*(*part).parser).multipart.flags |= 0x4000 as libc::c_int as libc::c_ulong
    } else if htp_table_add((*part).headers, (*h).name, h as *const libc::c_void)
        != 1 as libc::c_int
    {
        bstr_free((*h).value);
        bstr_free((*h).name);
        free(h as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}

/* *
 * Creates a new Multipart part.
 *
 * @param[in] parser
 * @return New part instance, or NULL on memory allocation failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_mpart_part_create(
    mut parser: *mut htp_mpartp_t,
) -> *mut htp_multipart_part_t {
    let mut part: *mut htp_multipart_part_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<htp_multipart_part_t>() as libc::c_ulong,
    ) as *mut htp_multipart_part_t;
    if part.is_null() {
        return 0 as *mut htp_multipart_part_t;
    }
    (*part).headers = htp_table_create(4 as libc::c_int as size_t);
    if (*part).headers.is_null() {
        free(part as *mut libc::c_void);
        return 0 as *mut htp_multipart_part_t;
    }
    (*part).parser = parser;
    bstr_builder_clear((*parser).part_data_pieces);
    bstr_builder_clear((*parser).part_header_pieces);
    return part;
}

/* *
 * Destroys a part.
 *
 * @param[in] part
 * @param[in] gave_up_data
 */
#[no_mangle]
pub unsafe extern "C" fn htp_mpart_part_destroy(
    mut part: *mut htp_multipart_part_t,
    mut gave_up_data: libc::c_int,
) {
    if part.is_null() {
        return;
    }
    if !(*part).file.is_null() {
        bstr_free((*(*part).file).filename);
        if !(*(*part).file).tmpname.is_null() {
            unlink((*(*part).file).tmpname);
            free((*(*part).file).tmpname as *mut libc::c_void);
        }
        free((*part).file as *mut libc::c_void);
        (*part).file = 0 as *mut crate::src::htp_util::htp_file_t
    }
    if gave_up_data == 0 || (*part).type_0 != htp_multipart_type_t::MULTIPART_PART_TEXT {
        bstr_free((*part).name);
        bstr_free((*part).value);
    }
    bstr_free((*part).content_type);
    if !(*part).headers.is_null() {
        let mut h: *mut crate::src::htp_transaction::htp_header_t =
            0 as *mut crate::src::htp_transaction::htp_header_t;
        let mut i: size_t = 0 as libc::c_int as size_t;
        let mut n: size_t = htp_table_size((*part).headers);
        while i < n {
            h = htp_table_get_index((*part).headers, i, 0 as *mut *mut bstr)
                as *mut crate::src::htp_transaction::htp_header_t;
            bstr_free((*h).name);
            bstr_free((*h).value);
            free(h as *mut libc::c_void);
            i = i.wrapping_add(1)
        }
        htp_table_destroy((*part).headers);
    }
    free(part as *mut libc::c_void);
}

/* *
 * Finalizes part processing.
 *
 * @param[in] part
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_mpart_part_finalize_data(
    mut part: *mut htp_multipart_part_t,
) -> htp_status_t {
    // Determine if this part is the epilogue.
    if (*(*part).parser).multipart.flags & 0x40 as libc::c_int as libc::c_ulong != 0 {
        if (*part).type_0 == htp_multipart_type_t::MULTIPART_PART_UNKNOWN {
            // Assume that the unknown part after the last boundary is the epilogue.
            (*(*(*part).parser).current_part).type_0 =
                htp_multipart_type_t::MULTIPART_PART_EPILOGUE;
            // But if we've already seen a part we thought was the epilogue,
            // raise HTP_MULTIPART_PART_UNKNOWN. Multiple epilogues are not allowed.
            if (*(*part).parser).multipart.flags & 0x20 as libc::c_int as libc::c_ulong != 0 {
                (*(*part).parser).multipart.flags |= 0x2000 as libc::c_int as libc::c_ulong
            }
            (*(*part).parser).multipart.flags |= 0x20 as libc::c_int as libc::c_ulong
        } else {
            (*(*part).parser).multipart.flags |= 0x80 as libc::c_int as libc::c_ulong
        }
    }
    // Sanity checks.
    // Have we seen complete part headers? If we have not, that means that the part ended prematurely.
    if (*(*(*part).parser).current_part).type_0 != htp_multipart_type_t::MULTIPART_PART_EPILOGUE
        && (*(*part).parser).current_part_mode != htp_part_mode_t::MODE_DATA
    {
        (*(*part).parser).multipart.flags |= 0x200000 as libc::c_int as libc::c_ulong
    }
    // Have we been able to determine the part type? If not, this means
    // that the part did not contain the C-D header.
    if (*part).type_0 == htp_multipart_type_t::MULTIPART_PART_UNKNOWN {
        (*(*part).parser).multipart.flags |= 0x2000 as libc::c_int as libc::c_ulong
    }
    // Finalize part value.
    if (*part).type_0 == htp_multipart_type_t::MULTIPART_PART_FILE {
        // Notify callbacks about the end of the file.
        htp_mpartp_run_request_file_data_hook(
            part,
            0 as *const libc::c_uchar,
            0 as libc::c_int as size_t,
        );
        // If we are storing the file to disk, close the file descriptor.
        if (*(*part).file).fd != -(1 as libc::c_int) {
            close((*(*part).file).fd);
        }
    } else if bstr_builder_size((*(*part).parser).part_data_pieces)
        > 0 as libc::c_int as libc::c_ulong
    {
        (*part).value = bstr_builder_to_str((*(*part).parser).part_data_pieces);
        bstr_builder_clear((*(*part).parser).part_data_pieces);
    }
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn htp_mpartp_run_request_file_data_hook(
    mut part: *mut htp_multipart_part_t,
    mut data: *const libc::c_uchar,
    mut len: size_t,
) -> htp_status_t {
    if (*(*part).parser).cfg.is_null() {
        return 1 as libc::c_int;
    }
    // Combine value pieces into a single buffer.
    // Keep track of the file length.
    (*(*part).file).len =
        ((*(*part).file).len as libc::c_ulong).wrapping_add(len) as int64_t as int64_t;
    // Package data for the callbacks.
    let mut file_data: crate::src::htp_util::htp_file_data_t =
        crate::src::htp_util::htp_file_data_t {
            file: 0 as *mut crate::src::htp_util::htp_file_t,
            data: 0 as *const libc::c_uchar,
            len: 0,
        };
    file_data.file = (*part).file;
    file_data.data = data;
    file_data.len = len;
    // Send data to callbacks
    let mut rc: htp_status_t = htp_hook_run_all(
        (*(*(*part).parser).cfg).hook_request_file_data,
        &mut file_data as *mut crate::src::htp_util::htp_file_data_t as *mut libc::c_void,
    );
    if rc != 1 as libc::c_int {
        return rc;
    }
    return 1 as libc::c_int;
}

/* *
 * Handles part data.
 *
 * @param[in] part
 * @param[in] data
 * @param[in] len
 * @param[in] is_line
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_mpart_part_handle_data(
    mut part: *mut htp_multipart_part_t,
    mut data: *const libc::c_uchar,
    mut len: size_t,
    mut is_line: libc::c_int,
) -> htp_status_t {
    // Keep track of raw part length.
    (*part).len = ((*part).len as libc::c_ulong).wrapping_add(len) as size_t as size_t;
    // If we're processing a part that came after the last boundary, then we're not sure if it
    // is the epilogue part or some other part (in case of evasion attempt). For that reason we
    // will keep all its data in the part_data_pieces structure. If it ends up not being the
    // epilogue, this structure will be cleared.
    if (*(*part).parser).multipart.flags & 0x40 as libc::c_int as libc::c_ulong != 0
        && (*part).type_0 == htp_multipart_type_t::MULTIPART_PART_UNKNOWN
    {
        bstr_builder_append_mem(
            (*(*part).parser).part_data_pieces,
            data as *const libc::c_void,
            len,
        );
    }
    if (*(*part).parser).current_part_mode == htp_part_mode_t::MODE_LINE {
        // Line mode.
        if is_line != 0 {
            // End of the line.
            let mut line: *mut bstr = 0 as *mut bstr;
            // If this line came to us in pieces, combine them now into a single buffer.
            if bstr_builder_size((*(*part).parser).part_header_pieces)
                > 0 as libc::c_int as libc::c_ulong
            {
                bstr_builder_append_mem(
                    (*(*part).parser).part_header_pieces,
                    data as *const libc::c_void,
                    len,
                );
                line = bstr_builder_to_str((*(*part).parser).part_header_pieces);
                if line.is_null() {
                    return -(1 as libc::c_int);
                }
                bstr_builder_clear((*(*part).parser).part_header_pieces);
                data = if (*line).realptr.is_null() {
                    (line as *mut libc::c_uchar)
                        .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
                } else {
                    (*line).realptr
                };
                len = (*line).len
            }
            // Ignore the line endings.
            if len > 1 as libc::c_int as libc::c_ulong {
                if *data.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int
                    == '\n' as i32
                {
                    len = len.wrapping_sub(1)
                }
                if *data.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int
                    == '\r' as i32
                {
                    len = len.wrapping_sub(1)
                }
            } else if len > 0 as libc::c_int as libc::c_ulong {
                if *data.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                    as libc::c_int
                    == '\n' as i32
                {
                    len = len.wrapping_sub(1)
                }
            }
            // Is it an empty line?
            if len == 0 as libc::c_int as libc::c_ulong {
                // Empty line; process headers and switch to data mode.
                // Process the pending header, if any.
                if !(*(*part).parser).pending_header_line.is_null() {
                    if htp_mpartp_parse_header(
                        part,
                        if (*(*(*part).parser).pending_header_line).realptr.is_null() {
                            ((*(*part).parser).pending_header_line as *mut libc::c_uchar)
                                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
                        } else {
                            (*(*(*part).parser).pending_header_line).realptr
                        },
                        (*(*(*part).parser).pending_header_line).len,
                    ) == -(1 as libc::c_int)
                    {
                        bstr_free(line);
                        return -(1 as libc::c_int);
                    }
                    bstr_free((*(*part).parser).pending_header_line);
                    (*(*part).parser).pending_header_line = 0 as *mut bstr
                }
                if htp_mpart_part_process_headers(part) == -(1 as libc::c_int) {
                    bstr_free(line);
                    return -(1 as libc::c_int);
                }
                (*(*part).parser).current_part_mode = htp_part_mode_t::MODE_DATA;
                bstr_builder_clear((*(*part).parser).part_header_pieces);
                if !(*part).file.is_null() {
                    // Changing part type because we have a filename.
                    (*part).type_0 = htp_multipart_type_t::MULTIPART_PART_FILE;
                    if (*(*part).parser).extract_files != 0
                        && (*(*part).parser).file_count < (*(*part).parser).extract_limit
                    {
                        let mut buf: [libc::c_char; 255] = [0; 255];
                        strncpy(
                            buf.as_mut_ptr(),
                            (*(*part).parser).extract_dir,
                            254 as libc::c_int as libc::c_ulong,
                        );
                        strncat(
                            buf.as_mut_ptr(),
                            b"/libhtp-multipart-file-XXXXXX\x00" as *const u8
                                as *const libc::c_char,
                            (254 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(strlen(buf.as_mut_ptr())),
                        );
                        (*(*part).file).tmpname = strdup(buf.as_mut_ptr());
                        if (*(*part).file).tmpname.is_null() {
                            bstr_free(line);
                            return -(1 as libc::c_int);
                        }
                        let mut previous_mask: mode_t = umask(
                            (0o100 as libc::c_int
                                | (0o400 as libc::c_int
                                    | 0o200 as libc::c_int
                                    | 0o100 as libc::c_int)
                                    >> 3 as libc::c_int
                                | (0o400 as libc::c_int
                                    | 0o200 as libc::c_int
                                    | 0o100 as libc::c_int)
                                    >> 3 as libc::c_int
                                    >> 3 as libc::c_int) as __mode_t,
                        );
                        (*(*part).file).fd = mkstemp((*(*part).file).tmpname);
                        umask(previous_mask);
                        if (*(*part).file).fd < 0 as libc::c_int {
                            bstr_free(line);
                            return -(1 as libc::c_int);
                        }
                        (*(*part).parser).file_count += 1
                    }
                } else if !(*part).name.is_null() {
                    // Changing part type because we have a name.
                    (*part).type_0 = htp_multipart_type_t::MULTIPART_PART_TEXT;
                    bstr_builder_clear((*(*part).parser).part_data_pieces);
                }
            } else if (*(*part).parser).pending_header_line.is_null() {
                if !line.is_null() {
                    (*(*part).parser).pending_header_line = line;
                    line = 0 as *mut bstr
                } else {
                    (*(*part).parser).pending_header_line =
                        bstr_dup_mem(data as *const libc::c_void, len);
                    if (*(*part).parser).pending_header_line.is_null() {
                        return -(1 as libc::c_int);
                    }
                }
            } else if *(*__ctype_b_loc())
                .offset(*data.offset(0 as libc::c_int as isize) as libc::c_int as isize)
                as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                // Not an empty line.
                // Is there a pending header?
                // Is this a folded line?
                // Folding; add to the existing line.
                (*(*part).parser).multipart.flags |= 0x1000 as libc::c_int as libc::c_ulong;
                (*(*part).parser).pending_header_line = bstr_add_mem(
                    (*(*part).parser).pending_header_line,
                    data as *const libc::c_void,
                    len,
                );
                if (*(*part).parser).pending_header_line.is_null() {
                    bstr_free(line);
                    return -(1 as libc::c_int);
                }
            } else {
                // Process the pending header line.
                if htp_mpartp_parse_header(
                    part,
                    if (*(*(*part).parser).pending_header_line).realptr.is_null() {
                        ((*(*part).parser).pending_header_line as *mut libc::c_uchar)
                            .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
                    } else {
                        (*(*(*part).parser).pending_header_line).realptr
                    },
                    (*(*(*part).parser).pending_header_line).len,
                ) == -(1 as libc::c_int)
                {
                    bstr_free(line);
                    return -(1 as libc::c_int);
                }
                bstr_free((*(*part).parser).pending_header_line);
                if !line.is_null() {
                    (*(*part).parser).pending_header_line = line;
                    line = 0 as *mut bstr
                } else {
                    (*(*part).parser).pending_header_line =
                        bstr_dup_mem(data as *const libc::c_void, len);
                    if (*(*part).parser).pending_header_line.is_null() {
                        return -(1 as libc::c_int);
                    }
                }
            }
            bstr_free(line);
            line = 0 as *mut bstr
        } else {
            // Not end of line; keep the data chunk for later.
            bstr_builder_append_mem(
                (*(*part).parser).part_header_pieces,
                data as *const libc::c_void,
                len,
            );
        }
    } else {
        // Data mode; keep the data chunk for later (but not if it is a file).
        match (*part).type_0 as libc::c_uint {
            4 | 3 | 1 | 0 => {
                // Make a copy of the data in RAM.
                bstr_builder_append_mem(
                    (*(*part).parser).part_data_pieces,
                    data as *const libc::c_void,
                    len,
                );
            }
            2 => {
                // Invoke file data callbacks.
                htp_mpartp_run_request_file_data_hook(part, data, len);
                // Optionally, store the data in a file.
                if (*(*part).file).fd != -(1 as libc::c_int) {
                    if write((*(*part).file).fd, data as *const libc::c_void, len)
                        < 0 as libc::c_int as libc::c_long
                    {
                        return -(1 as libc::c_int);
                    }
                }
            }
            _ => {
                // Internal error.
                return -(1 as libc::c_int);
            }
        }
    }
    return 1 as libc::c_int;
}

/* *
 * Handles data, creating new parts as necessary.
 *
 * @param[in] mpartp
 * @param[in] data
 * @param[in] len
 * @param[in] is_line
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
unsafe extern "C" fn htp_mpartp_handle_data(
    mut parser: *mut htp_mpartp_t,
    mut data: *const libc::c_uchar,
    mut len: size_t,
    mut is_line: libc::c_int,
) -> htp_status_t {
    if len == 0 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int;
    }
    // Do we have a part already?
    if (*parser).current_part.is_null() {
        // Create a new part.
        (*parser).current_part = htp_mpart_part_create(parser);
        if (*parser).current_part.is_null() {
            return -(1 as libc::c_int);
        }
        if (*parser).multipart.boundary_count == 0 as libc::c_int {
            // We haven't seen a boundary yet, so this must be the preamble part.
            (*(*parser).current_part).type_0 = htp_multipart_type_t::MULTIPART_PART_PREAMBLE;
            (*parser).multipart.flags |= 0x10 as libc::c_int as libc::c_ulong;
            (*parser).current_part_mode = htp_part_mode_t::MODE_DATA
        } else {
            // Part after preamble.
            (*parser).current_part_mode = htp_part_mode_t::MODE_LINE
        }
        // Add part to the list.
        htp_list_array_push(
            (*parser).multipart.parts,
            (*parser).current_part as *mut libc::c_void,
        );
    }
    // Send data to the part.
    return htp_mpart_part_handle_data((*parser).current_part, data, len, is_line);
}

/* *
 * Handles a boundary event, which means that it will finalize a part if one exists.
 *
 * @param[in] mpartp
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
unsafe extern "C" fn htp_mpartp_handle_boundary(mut parser: *mut htp_mpartp_t) -> htp_status_t {
    if !(*parser).current_part.is_null() {
        if htp_mpart_part_finalize_data((*parser).current_part) != 1 as libc::c_int {
            return -(1 as libc::c_int);
        }
        // We're done with this part
        (*parser).current_part = 0 as *mut htp_multipart_part_t;
        // Revert to line mode
        (*parser).current_part_mode = htp_part_mode_t::MODE_LINE
    }
    return 1 as libc::c_int;
}

unsafe extern "C" fn htp_mpartp_init_boundary(
    mut parser: *mut htp_mpartp_t,
    mut data: *mut libc::c_uchar,
    mut len: size_t,
) -> htp_status_t {
    if parser.is_null() || data.is_null() {
        return -(1 as libc::c_int);
    }
    // Copy the boundary and convert it to lowercase.
    (*parser).multipart.boundary_len = len.wrapping_add(4 as libc::c_int as libc::c_ulong);
    (*parser).multipart.boundary = malloc(
        (*parser)
            .multipart
            .boundary_len
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if (*parser).multipart.boundary.is_null() {
        return -(1 as libc::c_int);
    }
    *(*parser)
        .multipart
        .boundary
        .offset(0 as libc::c_int as isize) = '\r' as i32 as libc::c_char;
    *(*parser)
        .multipart
        .boundary
        .offset(1 as libc::c_int as isize) = '\n' as i32 as libc::c_char;
    *(*parser)
        .multipart
        .boundary
        .offset(2 as libc::c_int as isize) = '-' as i32 as libc::c_char;
    *(*parser)
        .multipart
        .boundary
        .offset(3 as libc::c_int as isize) = '-' as i32 as libc::c_char;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < len {
        *(*parser)
            .multipart
            .boundary
            .offset(i.wrapping_add(4 as libc::c_int as libc::c_ulong) as isize) =
            *data.offset(i as isize) as libc::c_char;
        i = i.wrapping_add(1)
    }
    *(*parser)
        .multipart
        .boundary
        .offset((*parser).multipart.boundary_len as isize) = '\u{0}' as i32 as libc::c_char;
    // We're starting in boundary-matching mode. The first boundary can appear without the
    // CRLF, and our starting state expects that. If we encounter non-boundary data, the
    // state will switch to data mode. Then, if the data is CRLF or LF, we will go back
    // to boundary matching. Thus, we handle all the possibilities.
    (*parser).parser_state = htp_multipart_state_t::STATE_BOUNDARY;
    (*parser).boundary_match_pos = 2 as libc::c_int as size_t;
    return 1 as libc::c_int;
}

/**
 * Creates a new multipart/form-data parser. On a successful invocation,
 * the ownership of the boundary parameter is transferred to the parser.
 *
 * @param[in] cfg
 * @param[in] boundary
 * @param[in] flags
 * @return New parser instance, or NULL on memory allocation failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_mpartp_create(
    mut cfg: *mut crate::src::htp_config::htp_cfg_t,
    mut boundary: *mut bstr,
    mut flags: uint64_t,
) -> *mut htp_mpartp_t {
    if cfg.is_null() || boundary.is_null() {
        return 0 as *mut htp_mpartp_t;
    }
    let mut parser: *mut htp_mpartp_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<htp_mpartp_t>() as libc::c_ulong,
    ) as *mut htp_mpartp_t;
    if parser.is_null() {
        return 0 as *mut htp_mpartp_t;
    }
    (*parser).cfg = cfg;
    (*parser).boundary_pieces = bstr_builder_create();
    if (*parser).boundary_pieces.is_null() {
        htp_mpartp_destroy(parser);
        return 0 as *mut htp_mpartp_t;
    }
    (*parser).part_data_pieces = bstr_builder_create();
    if (*parser).part_data_pieces.is_null() {
        htp_mpartp_destroy(parser);
        return 0 as *mut htp_mpartp_t;
    }
    (*parser).part_header_pieces = bstr_builder_create();
    if (*parser).part_header_pieces.is_null() {
        htp_mpartp_destroy(parser);
        return 0 as *mut htp_mpartp_t;
    }
    (*parser).multipart.parts = htp_list_array_create(64 as libc::c_int as size_t);
    if (*parser).multipart.parts.is_null() {
        htp_mpartp_destroy(parser);
        return 0 as *mut htp_mpartp_t;
    }
    (*parser).multipart.flags = flags;
    (*parser).parser_state = htp_multipart_state_t::STATE_INIT;
    (*parser).extract_files = (*cfg).extract_request_files;
    (*parser).extract_dir = (*cfg).tmpdir;
    if (*cfg).extract_request_files_limit >= 0 as libc::c_int {
        (*parser).extract_limit = (*cfg).extract_request_files_limit
    } else {
        (*parser).extract_limit = 16 as libc::c_int
    }
    (*parser).handle_data = Some(
        htp_mpartp_handle_data
            as unsafe extern "C" fn(
                _: *mut htp_mpartp_t,
                _: *const libc::c_uchar,
                _: size_t,
                _: libc::c_int,
            ) -> htp_status_t,
    );
    (*parser).handle_boundary = Some(
        htp_mpartp_handle_boundary as unsafe extern "C" fn(_: *mut htp_mpartp_t) -> htp_status_t,
    );
    // Initialize the boundary.
    let mut rc: htp_status_t = htp_mpartp_init_boundary(
        parser,
        if (*boundary).realptr.is_null() {
            (boundary as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
        } else {
            (*boundary).realptr
        },
        (*boundary).len,
    );
    if rc != 1 as libc::c_int {
        htp_mpartp_destroy(parser);
        return 0 as *mut htp_mpartp_t;
    }
    // On success, the ownership of the boundary parameter
    // is transferred to us. We made a copy, and so we
    // don't need it any more.
    bstr_free(boundary);
    return parser;
}

/**
 * Destroys the provided parser.
 *
 * @param[in] parser
 */
#[no_mangle]
pub unsafe extern "C" fn htp_mpartp_destroy(mut parser: *mut htp_mpartp_t) {
    if parser.is_null() {
        return;
    }
    if !(*parser).multipart.boundary.is_null() {
        free((*parser).multipart.boundary as *mut libc::c_void);
    }
    bstr_builder_destroy((*parser).boundary_pieces);
    bstr_builder_destroy((*parser).part_header_pieces);
    bstr_free((*parser).pending_header_line);
    bstr_builder_destroy((*parser).part_data_pieces);
    // Free the parts.
    if !(*parser).multipart.parts.is_null() {
        let mut i: size_t = 0 as libc::c_int as size_t;
        let mut n: size_t = htp_list_array_size((*parser).multipart.parts);
        while i < n {
            let mut part: *mut htp_multipart_part_t =
                htp_list_array_get((*parser).multipart.parts, i) as *mut htp_multipart_part_t;
            htp_mpart_part_destroy(part, (*parser).gave_up_data);
            i = i.wrapping_add(1)
        }
        htp_list_array_destroy((*parser).multipart.parts);
    }
    free(parser as *mut libc::c_void);
}

/* *
 * Processes set-aside data.
 *
 * @param[in] mpartp
 * @param[in] data
 * @param[in] pos
 * @param[in] startpos
 * @param[in] return_pos
 * @param[in] matched
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
unsafe extern "C" fn htp_martp_process_aside(
    mut parser: *mut htp_mpartp_t,
    mut matched: libc::c_int,
) -> htp_status_t {
    // The stored data pieces can contain up to one line. If we're in data mode and there
    // was no boundary match, things are straightforward -- we process everything as data.
    // If there was a match, we need to take care to not send the line ending as data, nor
    // anything that follows (because it's going to be a part of the boundary). Similarly,
    // when we are in line mode, we need to split the first data chunk, processing the first
    // part as line and the second part as data.
    // Do we need to do any chunk splitting?
    if matched != 0 || (*parser).current_part_mode == htp_part_mode_t::MODE_LINE {
        // Line mode or boundary match
        // Process the CR byte, if set aside.
        if matched == 0 && (*parser).cr_aside != 0 {
            // Treat as part data, when there is not a match.
            (*parser).handle_data.expect("non-null function pointer")(
                parser,
                &*::std::mem::transmute::<&[u8; 2], &[libc::c_char; 2]>(b"\r\x00")
                    as *const [libc::c_char; 2] as *mut libc::c_uchar,
                1 as libc::c_int as size_t,
                0 as libc::c_int,
            );
            (*parser).cr_aside = 0 as libc::c_int
        } else {
            // Treat as boundary, when there is a match.
            (*parser).cr_aside = 0 as libc::c_int
        }
        // We know that we went to match a boundary because
        // we saw a new line. Now we have to find that line and
        // process it. It's either going to be in the current chunk,
        // or in the first stored chunk.
        if bstr_builder_size((*parser).boundary_pieces) > 0 as libc::c_int as libc::c_ulong {
            let mut first: libc::c_int = 1 as libc::c_int;
            let mut i: size_t = 0 as libc::c_int as size_t;
            let mut n: size_t = htp_list_array_size((*(*parser).boundary_pieces).pieces);
            while i < n {
                let mut b: *mut bstr =
                    htp_list_array_get((*(*parser).boundary_pieces).pieces, i) as *mut bstr;
                if first != 0 {
                    first = 0 as libc::c_int;
                    // Split the first chunk.
                    if matched == 0 {
                        // In line mode, we are OK with line endings.
                        (*parser).handle_data.expect("non-null function pointer")(
                            parser,
                            if (*b).realptr.is_null() {
                                (b as *mut libc::c_uchar)
                                    .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
                            } else {
                                (*b).realptr
                            },
                            (*parser).boundary_candidate_pos,
                            1 as libc::c_int,
                        );
                    } else {
                        // But if there was a match, the line ending belongs to the boundary.
                        let mut dx: *mut libc::c_uchar = if (*b).realptr.is_null() {
                            (b as *mut libc::c_uchar)
                                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
                        } else {
                            (*b).realptr
                        };
                        let mut lx: size_t = (*parser).boundary_candidate_pos;
                        // Remove LF or CRLF.
                        if lx > 0 as libc::c_int as libc::c_ulong
                            && *dx
                                .offset(lx.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
                                as libc::c_int
                                == '\n' as i32
                        {
                            lx = lx.wrapping_sub(1);
                            // Remove CR.
                            if lx > 0 as libc::c_int as libc::c_ulong
                                && *dx.offset(
                                    lx.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize
                                ) as libc::c_int
                                    == '\r' as i32
                            {
                                lx = lx.wrapping_sub(1)
                            }
                        }
                        (*parser).handle_data.expect("non-null function pointer")(
                            parser,
                            dx,
                            lx,
                            0 as libc::c_int,
                        );
                    }
                    // The second part of the split chunks belongs to the boundary
                    // when matched, data otherwise.
                    if matched == 0 {
                        (*parser).handle_data.expect("non-null function pointer")(
                            parser,
                            (if (*b).realptr.is_null() {
                                (b as *mut libc::c_uchar)
                                    .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
                            } else {
                                (*b).realptr
                            })
                            .offset((*parser).boundary_candidate_pos as isize),
                            (*b).len.wrapping_sub((*parser).boundary_candidate_pos),
                            0 as libc::c_int,
                        );
                    }
                } else if matched == 0 {
                    (*parser).handle_data.expect("non-null function pointer")(
                        parser,
                        if (*b).realptr.is_null() {
                            (b as *mut libc::c_uchar)
                                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
                        } else {
                            (*b).realptr
                        },
                        (*b).len,
                        0 as libc::c_int,
                    );
                }
                i = i.wrapping_add(1)
            }
            bstr_builder_clear((*parser).boundary_pieces);
        }
    } else {
        // Do not send data if there was a boundary match. The stored
        // data belongs to the boundary.
        // Data mode and no match.
        // In data mode, we process the lone CR byte as data.
        if (*parser).cr_aside != 0 {
            (*parser).handle_data.expect("non-null function pointer")(
                parser,
                &*::std::mem::transmute::<&[u8; 2], &[libc::c_char; 2]>(b"\r\x00")
                    as *const [libc::c_char; 2] as *const libc::c_uchar,
                1 as libc::c_int as size_t,
                0 as libc::c_int,
            );
            (*parser).cr_aside = 0 as libc::c_int
        }
        // We then process any pieces that we might have stored, also as data.
        if bstr_builder_size((*parser).boundary_pieces) > 0 as libc::c_int as libc::c_ulong {
            let mut i_0: size_t = 0 as libc::c_int as size_t;
            let mut n_0: size_t = htp_list_array_size((*(*parser).boundary_pieces).pieces);
            while i_0 < n_0 {
                let mut b_0: *mut bstr =
                    htp_list_array_get((*(*parser).boundary_pieces).pieces, i_0) as *mut bstr;
                (*parser).handle_data.expect("non-null function pointer")(
                    parser,
                    if (*b_0).realptr.is_null() {
                        (b_0 as *mut libc::c_uchar)
                            .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
                    } else {
                        (*b_0).realptr
                    },
                    (*b_0).len,
                    0 as libc::c_int,
                );
                i_0 = i_0.wrapping_add(1)
            }
            bstr_builder_clear((*parser).boundary_pieces);
        }
    }
    return 1 as libc::c_int;
}

/**
 * Finalize parsing.
 *
 * @param[in] parser
 * @returns HTP_OK on success, HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_mpartp_finalize(mut parser: *mut htp_mpartp_t) -> htp_status_t {
    if !(*parser).current_part.is_null() {
        // Process buffered data, if any.
        htp_martp_process_aside(parser, 0 as libc::c_int);
        // Finalize the last part.
        if htp_mpart_part_finalize_data((*parser).current_part) != 1 as libc::c_int {
            return -(1 as libc::c_int);
        }
        // It is OK to end abruptly in the epilogue part, but not in any other.
        if (*(*parser).current_part).type_0 != htp_multipart_type_t::MULTIPART_PART_EPILOGUE {
            (*parser).multipart.flags |= 0x100 as libc::c_int as libc::c_ulong
        }
    }
    bstr_builder_clear((*parser).boundary_pieces);
    return 1 as libc::c_int;
}
/* *
 * Parses a chunk of multipart/form-data data. This function should be called
 * as many times as necessary until all data has been consumed.
 *
 * @param[in] parser
 * @param[in] data
 * @param[in] len
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_mpartp_parse(
    mut parser: *mut htp_mpartp_t,
    mut _data: *const libc::c_void,
    mut len: size_t,
) -> htp_status_t {
    let mut data: *mut libc::c_uchar = _data as *mut libc::c_uchar;
    // The current position in the entire input buffer.
    let mut pos: size_t = 0 as libc::c_int as size_t;
    // The position of the first unprocessed byte of data. We split the
    // input buffer into smaller chunks, according to their purpose. Once
    // an entire such smaller chunk is processed, we move to the next
    // and update startpos.
    let mut startpos: size_t = 0 as libc::c_int as size_t;
    // The position of the (possible) boundary. We investigate for possible
    // boundaries whenever we encounter CRLF or just LF. If we don't find a
    // boundary we need to go back, and this is what data_return_pos helps with.
    let mut data_return_pos: size_t = 0 as libc::c_int as size_t;
    // While there's data in the input buffer.
    while pos < len {
        'c_11171: loop {
            match (*parser).parser_state as libc::c_uint {
                0 => {
                    // Incomplete initialization.
                    return -(1 as libc::c_int);
                }
                1 => {
                    // Handle part data.
                    // While there's data in the input buffer.
                    while pos < len {
                        // Check for a CRLF-terminated line.
                        if *data.offset(pos as isize) as libc::c_int == '\r' as i32 {
                            // We have a CR byte.
                            // Is this CR the last byte in the input buffer?
                            if pos.wrapping_add(1 as libc::c_int as
                                                        libc::c_ulong) == len
                                   {
                                    // We have CR as the last byte in input. We are going to process
                            // what we have in the buffer as data, except for the CR byte,
                            // which we're going to leave for later. If it happens that a
                            // CR is followed by a LF and then a boundary, the CR is going
                            // to be discarded.
                                    pos =
                                        pos.wrapping_add(1); // Advance over CR.
                                    (*parser).cr_aside = 1 as libc::c_int
                                } else if *data.offset(pos.wrapping_add(1 as
                                                                            libc::c_int
                                                                            as
                                                                            libc::c_ulong)
                                                           as isize) as
                                              libc::c_int == '\n' as i32 {
                                    // We have CR and at least one more byte in the buffer, so we
                            // are able to test for the LF byte too.
                                    pos =
                                        (pos as
                                             libc::c_ulong).wrapping_add(2 as
                                                                             libc::c_int
                                                                             as
                                                                             libc::c_ulong)
                                            as size_t as
                                            size_t; // Advance over CR and LF.
                                    (*parser).multipart.flags |=
                                        0x2 as libc::c_int as libc::c_ulong;
                                    // Prepare to switch to boundary testing.
                                    data_return_pos =
                                        pos; // After LF; position of the first dash.
                                    (*parser).boundary_candidate_pos =
                                        pos.wrapping_sub(startpos);
                                    (*parser).boundary_match_pos =
                                        2 as libc::c_int as size_t;
                                    (*parser).parser_state = htp_multipart_state_t::STATE_BOUNDARY;
                                    continue 'c_11171 ;
                                } else {
                                    // This is not a new line; advance over the
                                // byte and clear the CR set-aside flag.
                                    pos = pos.wrapping_add(1);
                                    (*parser).cr_aside = 0 as libc::c_int
                                }
                        } else if *data.offset(pos as isize) as libc::c_int == '\n' as i32 {
                            // Check for a LF-terminated line.
                            pos = pos.wrapping_add(1); // Advance over LF.
                                                       // Did we have a CR in the previous input chunk?
                            if (*parser).cr_aside == 0 as libc::c_int {
                                (*parser).multipart.flags |= 0x1 as libc::c_int as libc::c_ulong
                            } else {
                                (*parser).multipart.flags |= 0x2 as libc::c_int as libc::c_ulong
                            }
                            // Prepare to switch to boundary testing.
                            data_return_pos = pos; // After LF; position of the first dash.
                            (*parser).boundary_candidate_pos = pos.wrapping_sub(startpos);
                            (*parser).boundary_match_pos = 2 as libc::c_int as size_t;
                            (*parser).parser_state = htp_multipart_state_t::STATE_BOUNDARY;
                            continue 'c_11171;
                        } else {
                            // Take one byte from input
                            pos = pos.wrapping_add(1);
                            // Earlier we might have set aside a CR byte not knowing if the next
                            // byte is a LF. Now we know that it is not, and so we can release the CR.
                            if (*parser).cr_aside != 0 {
                                (*parser).handle_data.expect("non-null function pointer")(
                                    parser,
                                    &*::std::mem::transmute::<&[u8; 2], &[libc::c_char; 2]>(
                                        b"\r\x00",
                                    )
                                        as *const [libc::c_char; 2]
                                        as *mut libc::c_uchar,
                                    1 as libc::c_int as size_t,
                                    0 as libc::c_int,
                                );
                                (*parser).cr_aside = 0 as libc::c_int
                            }
                        }
                    }
                    // No more data in the input buffer; process the data chunk.
                    (*parser).handle_data.expect("non-null function pointer")(
                        parser,
                        data.offset(startpos as isize),
                        pos.wrapping_sub(startpos)
                            .wrapping_sub((*parser).cr_aside as libc::c_ulong),
                        0 as libc::c_int,
                    );
                    break;
                }
                2 => {
                    // Handle a possible boundary.
                    while pos < len {
                        // Check if the bytes match.
                        if !(*data.offset(pos as isize) as libc::c_int
                            == *(*parser)
                                .multipart
                                .boundary
                                .offset((*parser).boundary_match_pos as isize)
                                as libc::c_int)
                        {
                            // Boundary mismatch.
                            // Process stored (buffered) data.
                            htp_martp_process_aside(parser, 0 as libc::c_int);
                            // Return back where data parsing left off.
                            if (*parser).current_part_mode == htp_part_mode_t::MODE_LINE {
                                // In line mode, we process the line.
                                (*parser).handle_data.expect("non-null function pointer")(
                                    parser,
                                    data.offset(startpos as isize),
                                    data_return_pos.wrapping_sub(startpos),
                                    1 as libc::c_int,
                                );
                                startpos = data_return_pos
                            } else {
                                // In data mode, we go back where we left off.
                                pos = data_return_pos
                            }
                            (*parser).parser_state = htp_multipart_state_t::STATE_DATA;
                            continue 'c_11171;
                        } else {
                            // Consume one matched boundary byte
                            pos = pos.wrapping_add(1);
                            (*parser).boundary_match_pos =
                                (*parser).boundary_match_pos.wrapping_add(1);
                            // Have we seen all boundary bytes?
                            if !((*parser).boundary_match_pos == (*parser).multipart.boundary_len) {
                                continue;
                            }
                            // Boundary match!
                            // Process stored (buffered) data.
                            htp_martp_process_aside(parser, 1 as libc::c_int);
                            // Process data prior to the boundary in the current input buffer.
                            // Because we know this is the last chunk before boundary, we can
                            // remove the line endings.
                            let mut dlen: size_t = data_return_pos.wrapping_sub(startpos);
                            if dlen > 0 as libc::c_int as libc::c_ulong
                                && *data.offset(
                                    startpos
                                        .wrapping_add(dlen)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        as isize,
                                ) as libc::c_int
                                    == '\n' as i32
                            {
                                dlen = dlen.wrapping_sub(1)
                            }
                            if dlen > 0 as libc::c_int as libc::c_ulong
                                && *data.offset(
                                    startpos
                                        .wrapping_add(dlen)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                        as isize,
                                ) as libc::c_int
                                    == '\r' as i32
                            {
                                dlen = dlen.wrapping_sub(1)
                            }
                            (*parser).handle_data.expect("non-null function pointer")(
                                parser,
                                data.offset(startpos as isize),
                                dlen,
                                1 as libc::c_int,
                            );
                            // Keep track of how many boundaries we've seen.
                            (*parser).multipart.boundary_count += 1;
                            if (*parser).multipart.flags & 0x40 as libc::c_int as libc::c_ulong != 0
                            {
                                (*parser).multipart.flags |= 0x80 as libc::c_int as libc::c_ulong
                            }
                            // Run boundary match.
                            (*parser)
                                .handle_boundary
                                .expect("non-null function pointer")(
                                parser
                            );
                            // We now need to check if this is the last boundary in the payload
                            (*parser).parser_state = htp_multipart_state_t::STATE_BOUNDARY_IS_LAST2;
                            continue 'c_11171;
                        }
                    }
                    // No more data in the input buffer; store (buffer) the unprocessed
                    // part for later, for after we find out if this is a boundary.
                    bstr_builder_append_mem(
                        (*parser).boundary_pieces,
                        data.offset(startpos as isize) as *const libc::c_void,
                        len.wrapping_sub(startpos),
                    );
                    break;
                }
                4 => {
                    // Examine the first byte after the last boundary character. If it is
                    // a dash, then we maybe processing the last boundary in the payload. If
                    // it is not, move to eat all bytes until the end of the line.
                    if *data.offset(pos as isize) as libc::c_int == '-' as i32 {
                        // Found one dash, now go to check the next position.
                        pos = pos.wrapping_add(1);
                        (*parser).parser_state = htp_multipart_state_t::STATE_BOUNDARY_IS_LAST1
                    } else {
                        // This is not the last boundary. Change state but
                        // do not advance the position, allowing the next
                        // state to process the byte.
                        (*parser).parser_state = htp_multipart_state_t::STATE_BOUNDARY_EAT_LWS
                    }
                    break;
                }
                3 => {
                    // Examine the byte after the first dash; expected to be another dash.
                    // If not, eat all bytes until the end of the line.
                    if *data.offset(pos as isize) as libc::c_int == '-' as i32 {
                        // This is indeed the last boundary in the payload.
                        pos = pos.wrapping_add(1);
                        (*parser).multipart.flags |= 0x40 as libc::c_int as libc::c_ulong;
                        (*parser).parser_state = htp_multipart_state_t::STATE_BOUNDARY_EAT_LWS
                    } else {
                        // The second character is not a dash, and so this is not
                        // the final boundary. Raise the flag for the first dash,
                        // and change state to consume the rest of the boundary line.
                        (*parser).multipart.flags |= 0x8 as libc::c_int as libc::c_ulong;
                        (*parser).parser_state = htp_multipart_state_t::STATE_BOUNDARY_EAT_LWS
                    }
                    break;
                }
                5 => {
                    if *data.offset(pos as isize) as libc::c_int == '\r' as i32 {
                        // CR byte, which could indicate a CRLF line ending.
                        pos = pos.wrapping_add(1);
                        (*parser).parser_state = htp_multipart_state_t::STATE_BOUNDARY_EAT_LWS_CR
                    } else if *data.offset(pos as isize) as libc::c_int == '\n' as i32 {
                        // LF line ending; we're done with boundary processing; data bytes follow.
                        pos = pos.wrapping_add(1);
                        startpos = pos;
                        (*parser).multipart.flags |= 0x1 as libc::c_int as libc::c_ulong;
                        (*parser).parser_state = htp_multipart_state_t::STATE_DATA
                    } else if htp_is_lws(*data.offset(pos as isize) as libc::c_int) != 0 {
                        // Linear white space is allowed here.
                        (*parser).multipart.flags |= 0x4 as libc::c_int as libc::c_ulong;
                        pos = pos.wrapping_add(1)
                    } else {
                        // Unexpected byte; consume, but remain in the same state.
                        (*parser).multipart.flags |= 0x8 as libc::c_int as libc::c_ulong;
                        pos = pos.wrapping_add(1)
                    }
                    break;
                }
                6 => {
                    if *data.offset(pos as isize) as libc::c_int == '\n' as i32 {
                        // CRLF line ending; we're done with boundary processing; data bytes follow.
                        pos = pos.wrapping_add(1);
                        startpos = pos;
                        (*parser).multipart.flags |= 0x2 as libc::c_int as libc::c_ulong;
                        (*parser).parser_state = htp_multipart_state_t::STATE_DATA
                    } else {
                        // Not a line ending; start again, but do not process this byte.
                        (*parser).multipart.flags |= 0x8 as libc::c_int as libc::c_ulong;
                        (*parser).parser_state = htp_multipart_state_t::STATE_BOUNDARY_EAT_LWS
                    }
                    break;
                }
                _ => {
                    break;
                    // switch
                }
            }
        }
    }
    return 1 as libc::c_int;
}

unsafe extern "C" fn htp_mpartp_validate_boundary(
    mut boundary: *mut bstr,
    mut flags: *mut uint64_t,
) {
    /*

    RFC 1341:

    The only mandatory parameter for the multipart  Content-Type
    is  the  boundary  parameter,  which  consists  of  1  to 70
    characters from a set of characters known to be very  robust
    through  email  gateways,  and  NOT ending with white space.
    (If a boundary appears to end with white  space,  the  white
    space  must be presumed to have been added by a gateway, and
    should  be  deleted.)   It  is  formally  specified  by  the
    following BNF:

    boundary := 0*69<bchars> bcharsnospace

    bchars := bcharsnospace / " "

    bcharsnospace :=    DIGIT / ALPHA / "'" / "(" / ")" / "+" / "_"
                          / "," / "-" / "." / "/" / ":" / "=" / "?"
     */
    /*
     Chrome: Content-Type: multipart/form-data; boundary=----WebKitFormBoundaryT4AfwQCOgIxNVwlD
    Firefox: Content-Type: multipart/form-data; boundary=---------------------------21071316483088
       MSIE: Content-Type: multipart/form-data; boundary=---------------------------7dd13e11c0452
      Opera: Content-Type: multipart/form-data; boundary=----------2JL5oh7QWEDwyBllIRc7fh
     Safari: Content-Type: multipart/form-data; boundary=----WebKitFormBoundaryre6zL3b0BelnTY5S
     */
    let mut data: *mut libc::c_uchar = if (*boundary).realptr.is_null() {
        (boundary as *mut libc::c_uchar)
            .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
    } else {
        (*boundary).realptr
    };
    let mut len: size_t = (*boundary).len;
    // The RFC allows up to 70 characters. In real life,
    // boundaries tend to be shorter.
    if len == 0 as libc::c_int as libc::c_ulong || len > 70 as libc::c_int as libc::c_ulong {
        *flags |= 0x200 as libc::c_int as libc::c_ulong
    }
    // Check boundary characters. This check is stricter than the
    // RFC, which seems to allow many separator characters.
    let mut pos: size_t = 0 as libc::c_int as size_t;
    while pos < len {
        if !(*data.offset(pos as isize) as libc::c_int >= '0' as i32
            && *data.offset(pos as isize) as libc::c_int <= '9' as i32
            || *data.offset(pos as isize) as libc::c_int >= 'a' as i32
                && *data.offset(pos as isize) as libc::c_int <= 'z' as i32
            || *data.offset(pos as isize) as libc::c_int >= 'A' as i32
                && *data.offset(pos as isize) as libc::c_int <= 'Z' as i32
            || *data.offset(pos as isize) as libc::c_int == '-' as i32)
        {
            match *data.offset(pos as isize) as libc::c_int {
                39 | 40 | 41 | 43 | 95 | 44 | 46 | 47 | 58 | 61 | 63 => {
                    // These characters are allowed by the RFC, but not common.
                    *flags |= 0x400 as libc::c_int as libc::c_ulong
                }
                _ => {
                    // Invalid character.
                    *flags |= 0x200 as libc::c_int as libc::c_ulong
                }
            }
        }
        pos = pos.wrapping_add(1)
    }
}

unsafe extern "C" fn htp_mpartp_validate_content_type(
    mut content_type: *mut bstr,
    mut flags: *mut uint64_t,
) {
    let mut data: *mut libc::c_uchar = if (*content_type).realptr.is_null() {
        (content_type as *mut libc::c_uchar)
            .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
    } else {
        (*content_type).realptr
    };
    let mut len: size_t = (*content_type).len;
    let mut counter: size_t = 0 as libc::c_int as size_t;
    while len > 0 as libc::c_int as libc::c_ulong {
        let mut i: libc::c_int = bstr_util_mem_index_of_c_nocase(
            data as *const libc::c_void,
            len,
            b"boundary\x00" as *const u8 as *const libc::c_char,
        );
        if i == -(1 as libc::c_int) {
            break;
        }
        data = data.offset(i as isize);
        len = len.wrapping_sub(i as libc::c_ulong);
        // In order to work around the fact that WebKit actually uses
        // the word "boundary" in their boundary, we also require one
        // equals character the follow the words.
        // "multipart/form-data; boundary=----WebKitFormBoundaryT4AfwQCOgIxNVwlD"
        if memchr(data as *const libc::c_void, '=' as i32, len).is_null() {
            break;
        }
        counter = counter.wrapping_add(1);
        // Check for case variations.
        let mut j: size_t = 0 as libc::c_int as size_t;
        while j < 8 as libc::c_int as libc::c_ulong {
            if !(*data as libc::c_int >= 'a' as i32 && *data as libc::c_int <= 'z' as i32) {
                *flags |= 0x200 as libc::c_int as libc::c_ulong
            }
            data = data.offset(1);
            len = len.wrapping_sub(1);
            j = j.wrapping_add(1)
        }
    }
    // How many boundaries have we seen?
    if counter > 1 as libc::c_int as libc::c_ulong {
        *flags |= 0x200 as libc::c_int as libc::c_ulong
    };
}

/* *
 * Looks for boundary in the supplied Content-Type request header. The extracted
 * boundary will be allocated on the heap.
 *
 * @param[in] content_type
 * @param[out] boundary
 * @param[out] multipart_flags Multipart flags, which are not compatible from general LibHTP flags.
 * @return HTP_OK on success (boundary found), HTP_DECLINED if boundary was not found,
 *         and HTP_ERROR on failure. Flags may be set on HTP_OK and HTP_DECLINED. For
 *         example, if a boundary could not be extracted but there is indication that
 *         one is present, HTP_MULTIPART_HBOUNDARY_INVALID will be set.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_mpartp_find_boundary(
    mut content_type: *mut bstr,
    mut boundary: *mut *mut bstr,
    mut flags: *mut uint64_t,
) -> htp_status_t {
    if content_type.is_null() || boundary.is_null() || flags.is_null() {
        return -(1 as libc::c_int);
    }
    // Our approach is to ignore the MIME type and instead just look for
    // the boundary. This approach is more reliable in the face of various
    // evasion techniques that focus on submitting invalid MIME types.
    // Reset flags.
    *flags = 0 as libc::c_int as uint64_t;
    // Look for the boundary, case insensitive.
    let mut i: libc::c_int = bstr_index_of_c_nocase(
        content_type,
        b"boundary\x00" as *const u8 as *const libc::c_char,
    );
    if i == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    let mut data: *mut libc::c_uchar = (if (*content_type).realptr.is_null() {
        (content_type as *mut libc::c_uchar)
            .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
    } else {
        (*content_type).realptr
    })
    .offset(i as isize)
    .offset(8 as libc::c_int as isize);
    let mut len: size_t = (*content_type)
        .len
        .wrapping_sub(i as libc::c_ulong)
        .wrapping_sub(8 as libc::c_int as libc::c_ulong);
    // Look for the boundary value.
    let mut pos: size_t = 0 as libc::c_int as size_t;
    while pos < len && *data.offset(pos as isize) as libc::c_int != '=' as i32 {
        if htp_is_space(*data.offset(pos as isize) as libc::c_int) != 0 {
            // It is unusual to see whitespace before the equals sign.
            *flags |= 0x400 as libc::c_int as libc::c_ulong
        } else {
            // But seeing a non-whitespace character may indicate evasion.
            *flags |= 0x200 as libc::c_int as libc::c_ulong
        }
        pos = pos.wrapping_add(1)
    }
    if pos >= len {
        // No equals sign in the header.
        *flags |= 0x200 as libc::c_int as libc::c_ulong;
        return 0 as libc::c_int;
    }
    // Go over the '=' character.
    pos = pos.wrapping_add(1);
    // Ignore any whitespace after the equals sign.
    while pos < len && htp_is_space(*data.offset(pos as isize) as libc::c_int) != 0 {
        if htp_is_space(*data.offset(pos as isize) as libc::c_int) != 0 {
            // It is unusual to see whitespace after
            // the equals sign.
            *flags |= 0x400 as libc::c_int as libc::c_ulong
        }
        pos = pos.wrapping_add(1)
    }
    if pos >= len {
        // No value after the equals sign.
        *flags |= 0x200 as libc::c_int as libc::c_ulong;
        return 0 as libc::c_int;
    }
    if *data.offset(pos as isize) as libc::c_int == '\"' as i32 {
        // Quoted boundary.
        // Possibly not very unusual, but let's see.
        *flags |= 0x400 as libc::c_int as libc::c_ulong;
        // Over the double quote.
        pos = pos.wrapping_add(1); // Over the double quote.
        let mut startpos: size_t = pos; // Starting position of the boundary.
        while pos < len && *data.offset(pos as isize) as libc::c_int != '\"' as i32
        // Look for the terminating double quote.
        {
            pos = pos.wrapping_add(1)
        }
        if pos >= len {
            // Ran out of space without seeing
            // the terminating double quote.
            *flags |= 0x200 as libc::c_int as libc::c_ulong;
            // Include the starting double quote in the boundary.
            startpos = startpos.wrapping_sub(1)
        }
        *boundary = bstr_dup_mem(
            data.offset(startpos as isize) as *const libc::c_void,
            pos.wrapping_sub(startpos),
        );
        if (*boundary).is_null() {
            return -(1 as libc::c_int);
        }
        pos = pos.wrapping_add(1)
    } else {
        // Boundary not quoted.
        let mut startpos_0: size_t = pos;
        // Find the end of the boundary. For the time being, we replicate
        // the behavior of PHP 5.4.x. This may result with a boundary that's
        // closer to what would be accepted in real life. Our subsequent
        // checks of boundary characters will catch irregularities.
        while pos < len
            && *data.offset(pos as isize) as libc::c_int != ',' as i32
            && *data.offset(pos as isize) as libc::c_int != ';' as i32
            && htp_is_space(*data.offset(pos as isize) as libc::c_int) == 0
        {
            pos = pos.wrapping_add(1)
        }
        *boundary = bstr_dup_mem(
            data.offset(startpos_0 as isize) as *const libc::c_void,
            pos.wrapping_sub(startpos_0),
        );
        if (*boundary).is_null() {
            return -(1 as libc::c_int);
        }
    }
    // Check for a zero-length boundary.
    if (**boundary).len == 0 as libc::c_int as libc::c_ulong {
        *flags |= 0x200 as libc::c_int as libc::c_ulong;
        bstr_free(*boundary);
        *boundary = 0 as *mut bstr;
        return 0 as libc::c_int;
    }
    // Allow only whitespace characters after the boundary.
    let mut seen_space: libc::c_int = 0 as libc::c_int;
    let mut seen_non_space: libc::c_int = 0 as libc::c_int;
    while pos < len {
        if htp_is_space(*data.offset(pos as isize) as libc::c_int) == 0 {
            seen_non_space = 1 as libc::c_int
        } else {
            seen_space = 1 as libc::c_int
        }
        pos = pos.wrapping_add(1)
    }
    // Raise INVALID if we see any non-space characters,
    // but raise UNUSUAL if we see _only_ space characters.
    if seen_non_space != 0 {
        *flags |= 0x200 as libc::c_int as libc::c_ulong
    } else if seen_space != 0 {
        *flags |= 0x400 as libc::c_int as libc::c_ulong
    }
    // Validate boundary characters.
    htp_mpartp_validate_boundary(*boundary, flags);
    // Correlate with the MIME type. This might be a tad too
    // sensitive because it may catch non-browser access with sloppy
    // implementations, but let's go with it for now.
    if bstr_begins_with_c(
        content_type,
        b"multipart/form-data;\x00" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        *flags |= 0x200 as libc::c_int as libc::c_ulong
    }
    htp_mpartp_validate_content_type(content_type, flags);
    return 1 as libc::c_int;
}
