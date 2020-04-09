use crate::{bstr, bstr_builder, htp_table, htp_transaction, htp_util, Status};
use ::libc;

extern "C" {
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
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

/**
 * This is the main URLENCODED parser structure. It is used to store
 * parser configuration, temporary parsing data, as well as the parameters.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_urlenp_t {
    /** The transaction this parser belongs to. */
    pub tx: *mut htp_transaction::htp_tx_t,
    /** The character used to separate parameters. Defaults to & and should
     *  not be changed without good reason.
     */
    pub argument_separator: libc::c_uchar,
    /** Whether to perform URL-decoding on parameters. */
    pub decode_url_encoding: libc::c_int,
    /** This table contains the list of parameters, indexed by name. */
    pub params: *mut htp_table::htp_table_t,
    // Private fields; these are used during the parsing process only
    pub _state: libc::c_int,
    pub _complete: libc::c_int,
    pub _name: *mut bstr::bstr_t,
    pub _bb: *mut bstr_builder::bstr_builder_t,
}

pub type htp_time_t = libc::timeval;

/* *
 * This method is invoked whenever a piece of data, belonging to a single field (name or value)
 * becomes available. It will either create a new parameter or store the transient information
 * until a parameter can be created.
 *
 * @param[in] urlenp
 * @param[in] data
 * @param[in] startpos
 * @param[in] endpos
 * @param[in] c Should contain -1 if the reason this function is called is because the end of
 *          the current data chunk is reached.
 */
unsafe extern "C" fn htp_urlenp_add_field_piece(
    mut urlenp: *mut htp_urlenp_t,
    mut data: *const libc::c_uchar,
    mut startpos: size_t,
    mut endpos: size_t,
    mut last_char: libc::c_int,
) {
    // Add field if we know it ended (last_char is something other than -1)
    // or if we know that there won't be any more input data (urlenp->_complete is true).
    if last_char != -(1 as libc::c_int) || (*urlenp)._complete != 0 {
        // Prepare the field value, assembling from multiple pieces as necessary.
        let mut field: *mut bstr::bstr_t = 0 as *mut bstr::bstr_t;
        // Did we use the string builder for this field?
        if bstr_builder::bstr_builder_size((*urlenp)._bb) > 0 as libc::c_int as libc::c_ulong {
            // The current field consists of more than once piece, we have to use the string builder.
            // Add current piece to string builder.
            if !data.is_null() && endpos.wrapping_sub(startpos) > 0 as libc::c_int as libc::c_ulong
            {
                bstr_builder::bstr_builder_append_mem(
                    (*urlenp)._bb,
                    data.offset(startpos as isize) as *const libc::c_void,
                    endpos.wrapping_sub(startpos),
                );
            }
            // Generate the field and clear the string builder.
            field = bstr_builder::bstr_builder_to_str((*urlenp)._bb);
            if field.is_null() {
                return;
            }
            bstr_builder::bstr_builder_clear((*urlenp)._bb);
        } else if !data.is_null()
            && endpos.wrapping_sub(startpos) > 0 as libc::c_int as libc::c_ulong
        {
            field = bstr::bstr_dup_mem(
                data.offset(startpos as isize) as *const libc::c_void,
                endpos.wrapping_sub(startpos),
            );
            if field.is_null() {
                return;
            }
        }
        // We only have the current piece to work with, so no need to involve the string builder.
        // Process field as key or value, as appropriate.
        if (*urlenp)._state == 1 as libc::c_int {
            // Key.
            // If there is no more work left to do, then we have a single key. Add it.
            if (*urlenp)._complete != 0 || last_char == (*urlenp).argument_separator as libc::c_int
            {
                // Handling empty pairs is tricky. We don't want to create a pair for
                // an entirely empty input, but in some cases it may be appropriate
                // (e.g., /index.php?&q=2).
                if !field.is_null() || last_char == (*urlenp).argument_separator as libc::c_int {
                    // Add one pair, with an empty value and possibly empty key too.
                    let mut name: *mut bstr::bstr_t = field;
                    if name.is_null() {
                        name = bstr::bstr_dup_c(b"\x00" as *const u8 as *const libc::c_char);
                        if name.is_null() {
                            return;
                        }
                    }
                    let mut value: *mut bstr::bstr =
                        bstr::bstr_dup_c(b"\x00" as *const u8 as *const libc::c_char);
                    if value.is_null() {
                        bstr::bstr_free(name);
                        return;
                    }
                    if (*urlenp).decode_url_encoding != 0 {
                        htp_util::htp_tx_urldecode_params_inplace((*urlenp).tx, name);
                    }
                    htp_table::htp_table_addn((*urlenp).params, name, value as *const libc::c_void);
                    (*urlenp)._name = 0 as *mut bstr::bstr
                }
            } else {
                // This key will possibly be followed by a value, so keep it for later.
                (*urlenp)._name = field
            }
        } else {
            // Value (with a key remembered from before).
            let mut name_0: *mut bstr::bstr_t = (*urlenp)._name;
            (*urlenp)._name = 0 as *mut bstr::bstr_t;
            if name_0.is_null() {
                name_0 = bstr::bstr_dup_c(b"\x00" as *const u8 as *const libc::c_char);
                if name_0.is_null() {
                    bstr::bstr_free(field);
                    return;
                }
            }
            let mut value_0: *mut bstr::bstr_t = field;
            if value_0.is_null() {
                value_0 = bstr::bstr_dup_c(b"\x00" as *const u8 as *const libc::c_char);
                if value_0.is_null() {
                    bstr::bstr_free(name_0);
                    return;
                }
            }
            if (*urlenp).decode_url_encoding != 0 {
                htp_util::htp_tx_urldecode_params_inplace((*urlenp).tx, name_0);
                htp_util::htp_tx_urldecode_params_inplace((*urlenp).tx, value_0);
            }
            htp_table::htp_table_addn((*urlenp).params, name_0, value_0 as *const libc::c_void);
        }
    } else if !data.is_null() && endpos.wrapping_sub(startpos) > 0 as libc::c_int as libc::c_ulong {
        bstr_builder::bstr_builder_append_mem(
            (*urlenp)._bb,
            data.offset(startpos as isize) as *const libc::c_void,
            endpos.wrapping_sub(startpos),
        );
    };
}

/* *
 * Creates a new URLENCODED parser.
 *
 * @return New parser, or NULL on memory allocation failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_urlenp_create(
    mut tx: *mut htp_transaction::htp_tx_t,
) -> *mut htp_urlenp_t {
    let mut urlenp: *mut htp_urlenp_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<htp_urlenp_t>() as libc::c_ulong,
    ) as *mut htp_urlenp_t;
    if urlenp.is_null() {
        return 0 as *mut htp_urlenp_t;
    }
    (*urlenp).tx = tx;
    (*urlenp).params = htp_table::htp_table_create(32 as libc::c_int as size_t);
    if (*urlenp).params.is_null() {
        free(urlenp as *mut libc::c_void);
        return 0 as *mut htp_urlenp_t;
    }
    (*urlenp)._bb = bstr_builder::bstr_builder_create();
    if (*urlenp)._bb.is_null() {
        htp_table::htp_table_destroy((*urlenp).params);
        free(urlenp as *mut libc::c_void);
        return 0 as *mut htp_urlenp_t;
    }
    (*urlenp).argument_separator = '&' as i32 as libc::c_uchar;
    (*urlenp).decode_url_encoding = 1 as libc::c_int;
    (*urlenp)._state = 1 as libc::c_int;
    return urlenp;
}

/* *
 * Destroys an existing URLENCODED parser.
 *
 * @param[in] urlenp
 */
#[no_mangle]
pub unsafe extern "C" fn htp_urlenp_destroy(mut urlenp: *mut htp_urlenp_t) {
    if urlenp.is_null() {
        return;
    }
    if !(*urlenp)._name.is_null() {
        bstr::bstr_free((*urlenp)._name);
    }
    bstr_builder::bstr_builder_destroy((*urlenp)._bb);
    if !(*urlenp).params.is_null() {
        // Destroy parameters.
        let mut i: size_t = 0 as libc::c_int as size_t;
        let mut n: size_t = htp_table::htp_table_size((*urlenp).params);
        while i < n {
            let mut b: *mut bstr::bstr =
                htp_table::htp_table_get_index((*urlenp).params, i, 0 as *mut *mut bstr::bstr)
                    as *mut bstr::bstr;
            // Parameter name will be freed by the table code.
            bstr::bstr_free(b);
            i = i.wrapping_add(1)
        }
        htp_table::htp_table_destroy((*urlenp).params);
    }
    free(urlenp as *mut libc::c_void);
}

/* *
 * Finalizes parsing, forcing the parser to convert any outstanding
 * data into parameters. This method should be invoked at the end
 * of a parsing operation that used htp_urlenp_parse_partial().
 *
 * @param[in] urlenp
 * @return Success indication
 */
#[no_mangle]
pub unsafe extern "C" fn htp_urlenp_finalize(mut urlenp: *mut htp_urlenp_t) -> Status {
    (*urlenp)._complete = 1 as libc::c_int;
    return htp_urlenp_parse_partial(urlenp, 0 as *const libc::c_void, 0 as libc::c_int as size_t);
}

/* *
 * Parses the provided data chunk under the assumption
 * that it contains all the data that will be parsed. When this
 * method is used for parsing the finalization method should not
 * be invoked.
 *
 * @param[in] urlenp
 * @param[in] data
 * @param[in] len
 * @return
 */
#[no_mangle]
pub unsafe extern "C" fn htp_urlenp_parse_complete(
    mut urlenp: *mut htp_urlenp_t,
    mut data: *const libc::c_void,
    mut len: size_t,
) -> Status {
    htp_urlenp_parse_partial(urlenp, data, len);
    return htp_urlenp_finalize(urlenp);
}

/* *
 * Parses the provided data chunk, keeping state to allow streaming parsing, i.e., the
 * parsing where only partial information is available at any one time. The method
 * htp_urlenp_finalize() must be invoked at the end to finalize parsing.
 *
 * @param[in] urlenp
 * @param[in] _data
 * @param[in] len
 * @return
 */
#[no_mangle]
pub unsafe extern "C" fn htp_urlenp_parse_partial(
    mut urlenp: *mut htp_urlenp_t,
    mut _data: *const libc::c_void,
    mut len: size_t,
) -> Status {
    let mut data: *mut libc::c_uchar = _data as *mut libc::c_uchar;
    let mut startpos: size_t = 0 as libc::c_int as size_t;
    let mut pos: size_t = 0 as libc::c_int as size_t;
    let mut c: libc::c_int = 0;
    if data.is_null() {
        len = 0 as libc::c_int as size_t
    }
    loop {
        // Get the next character, or use -1 to indicate end of input.
        if pos < len {
            c = *data.offset(pos as isize) as libc::c_int
        } else {
            c = -(1 as libc::c_int)
        }
        match (*urlenp)._state {
            1 => {
                // Look for =, argument separator, or end of input.
                if c == '=' as i32
                    || c == (*urlenp).argument_separator as libc::c_int
                    || c == -(1 as libc::c_int)
                {
                    // Data from startpos to pos.
                    htp_urlenp_add_field_piece(urlenp, data, startpos, pos, c);
                    // If it's not the end of input, then it must be the end of this field.
                    if c != -(1 as libc::c_int) {
                        // Next state.
                        startpos = pos.wrapping_add(1 as libc::c_int as libc::c_ulong);
                        if c == (*urlenp).argument_separator as libc::c_int {
                            (*urlenp)._state = 1 as libc::c_int
                        } else {
                            (*urlenp)._state = 2 as libc::c_int
                        }
                    }
                }
                pos = pos.wrapping_add(1)
            }
            2 => {
                // Look for argument separator or end of input.
                if c == (*urlenp).argument_separator as libc::c_int || c == -(1 as libc::c_int) {
                    // Data from startpos to pos.
                    htp_urlenp_add_field_piece(urlenp, data, startpos, pos, c);
                    // If it's not the end of input, then it must be the end of this field.
                    if c != -(1 as libc::c_int) {
                        // Next state.
                        startpos = pos.wrapping_add(1 as libc::c_int as libc::c_ulong);
                        (*urlenp)._state = 1 as libc::c_int
                    }
                }
                pos = pos.wrapping_add(1)
            }
            _ => {
                // Invalid state.
                return Status::ERROR;
            }
        }
        if !(c != -(1 as libc::c_int)) {
            break;
        }
    }
    return Status::OK;
}
