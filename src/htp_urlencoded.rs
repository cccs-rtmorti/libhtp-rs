use crate::{bstr, bstr_builder, htp_table, htp_transaction, htp_util, Status};

extern "C" {
    #[no_mangle]
    fn calloc(_: libc::size_t, _: libc::size_t) -> *mut core::ffi::c_void;
    #[no_mangle]
    fn free(__ptr: *mut core::ffi::c_void);
}

/// This is the main URLENCODED parser structure. It is used to store
/// parser configuration, temporary parsing data, as well as the parameters.
#[derive(Clone)]
pub struct htp_urlenp_t {
    /// The transaction this parser belongs to.
    pub tx: *mut htp_transaction::htp_tx_t,
    /// The character used to separate parameters. Defaults to & and should
    /// not be changed without good reason.
    pub argument_separator: u8,
    /// Whether to perform URL-decoding on parameters.
    pub decode_url_encoding: i32,
    /// This table contains the list of parameters, indexed by name.
    pub params: htp_table::htp_table_t<bstr::bstr_t>,
    // Private fields; these are used during the parsing process only
    pub _state: i32,
    pub _complete: i32,
    pub _name: *mut bstr::bstr_t,
    pub _bb: *mut bstr_builder::bstr_builder_t,
}

/// This method is invoked whenever a piece of data, belonging to a single field (name or value)
/// becomes available. It will either create a new parameter or store the transient information
/// until a parameter can be created.
///
/// last_char: Should contain -1 if the reason this function is called is because the end of
///            the current data chunk is reached.
unsafe fn htp_urlenp_add_field_piece(
    mut urlenp: *mut htp_urlenp_t,
    data: *const u8,
    startpos: usize,
    endpos: usize,
    last_char: i32,
) {
    // Add field if we know it ended (last_char is something other than -1)
    // or if we know that there won't be any more input data (urlenp->_complete is true).
    if last_char != -1 || (*urlenp)._complete != 0 {
        // Prepare the field value, assembling from multiple pieces as necessary.
        let mut field: *mut bstr::bstr_t = 0 as *mut bstr::bstr_t;
        // Did we use the string builder for this field?
        if bstr_builder::bstr_builder_size((*urlenp)._bb) > 0 {
            // The current field consists of more than once piece, we have to use the string builder.
            // Add current piece to string builder.
            if !data.is_null() && endpos.wrapping_sub(startpos) > 0 {
                bstr_builder::bstr_builder_append_mem(
                    (*urlenp)._bb,
                    data.offset(startpos as isize) as *const core::ffi::c_void,
                    endpos.wrapping_sub(startpos),
                );
            }
            // Generate the field and clear the string builder.
            field = bstr_builder::bstr_builder_to_str((*urlenp)._bb);
            if field.is_null() {
                return;
            }
            bstr_builder::bstr_builder_clear((*urlenp)._bb);
        } else if !data.is_null() && endpos.wrapping_sub(startpos) > 0 {
            field = bstr::bstr_dup_mem(
                data.offset(startpos as isize) as *const core::ffi::c_void,
                endpos.wrapping_sub(startpos),
            );
            if field.is_null() {
                return;
            }
        }
        // We only have the current piece to work with, so no need to involve the string builder.
        // Process field as key or value, as appropriate.
        if (*urlenp)._state == 1 {
            // Key.
            // If there is no more work left to do, then we have a single key. Add it.
            if (*urlenp)._complete != 0 || last_char == (*urlenp).argument_separator as i32 {
                // Handling empty pairs is tricky. We don't want to create a pair for
                // an entirely empty input, but in some cases it may be appropriate
                // (e.g., /index.php?&q=2).
                if !field.is_null() || last_char == (*urlenp).argument_separator as i32 {
                    // Add one pair, with an empty value and possibly empty key too.
                    let mut name = if !field.is_null() {
                        (*field).clone()
                    } else {
                        bstr::bstr_t::new()
                    };
                    let value = bstr::bstr_t::new();
                    if (*urlenp).decode_url_encoding != 0 {
                        // Ignore result
                        let _ = htp_util::htp_tx_urldecode_params_inplace(
                            &mut *(*urlenp).tx,
                            &mut name,
                        );
                    }
                    (*urlenp).params.add(name, value);
                    (*urlenp)._name = 0 as *mut bstr::bstr_t
                }
            } else {
                // This key will possibly be followed by a value, so keep it for later.
                (*urlenp)._name = field
            }
        } else {
            // Value (with a key remembered from before).
            let mut name_0 = if !(*urlenp)._name.is_null() {
                (*(*urlenp)._name).clone()
            } else {
                bstr::bstr_t::new()
            };
            let mut value_0 = if !field.is_null() {
                (*field).clone()
            } else {
                bstr::bstr_t::new()
            };
            if (*urlenp).decode_url_encoding != 0 {
                // Ignore results.
                let _ = htp_util::htp_tx_urldecode_params_inplace(&mut *(*urlenp).tx, &mut name_0);
                let _ = htp_util::htp_tx_urldecode_params_inplace(&mut *(*urlenp).tx, &mut value_0);
            }
            (*urlenp).params.add(name_0, value_0);
        }
    } else if !data.is_null() && endpos.wrapping_sub(startpos) > 0 {
        bstr_builder::bstr_builder_append_mem(
            (*urlenp)._bb,
            data.offset(startpos as isize) as *const core::ffi::c_void,
            endpos.wrapping_sub(startpos),
        );
    };
}

/// Creates a new URLENCODED parser.
///
/// Returns New parser, or NULL on memory allocation failure.
pub unsafe fn htp_urlenp_create(tx: *mut htp_transaction::htp_tx_t) -> *mut htp_urlenp_t {
    let urlenp: *mut htp_urlenp_t =
        calloc(1, ::std::mem::size_of::<htp_urlenp_t>()) as *mut htp_urlenp_t;
    if urlenp.is_null() {
        return 0 as *mut htp_urlenp_t;
    }
    (*urlenp).tx = tx;
    (*urlenp).params = htp_table::htp_table_t::with_capacity(32);
    (*urlenp)._bb = bstr_builder::bstr_builder_create();
    if (*urlenp)._bb.is_null() {
        free(urlenp as *mut core::ffi::c_void);
        return 0 as *mut htp_urlenp_t;
    }
    (*urlenp).argument_separator = '&' as u8;
    (*urlenp).decode_url_encoding = 1;
    (*urlenp)._state = 1;
    urlenp
}

/// Destroys an existing URLENCODED parser.
pub unsafe fn htp_urlenp_destroy(urlenp: *mut htp_urlenp_t) {
    if urlenp.is_null() {
        return;
    }
    if !(*urlenp)._name.is_null() {
        bstr::bstr_free((*urlenp)._name);
    }
    bstr_builder::bstr_builder_destroy((*urlenp)._bb);
    (*urlenp).params.elements.clear();
    free(urlenp as *mut core::ffi::c_void);
}

/// Finalizes parsing, forcing the parser to convert any outstanding
/// data into parameters. This method should be invoked at the end
/// of a parsing operation that used htp_urlenp_parse_partial().
///
/// Returns Success indication
pub unsafe fn htp_urlenp_finalize(mut urlenp: *mut htp_urlenp_t) -> Status {
    (*urlenp)._complete = 1;
    htp_urlenp_parse_partial(urlenp, 0 as *const core::ffi::c_void, 0)
}

/// Parses the provided data chunk under the assumption
/// that it contains all the data that will be parsed. When this
/// method is used for parsing the finalization method should not
/// be invoked.
pub unsafe fn htp_urlenp_parse_complete(
    urlenp: *mut htp_urlenp_t,
    data: *const core::ffi::c_void,
    len: usize,
) -> Status {
    htp_urlenp_parse_partial(urlenp, data, len);
    htp_urlenp_finalize(urlenp)
}

/// Parses the provided data chunk, keeping state to allow streaming parsing, i.e., the
/// parsing where only partial information is available at any one time. The method
/// htp_urlenp_finalize() must be invoked at the end to finalize parsing.
pub unsafe fn htp_urlenp_parse_partial(
    mut urlenp: *mut htp_urlenp_t,
    _data: *const core::ffi::c_void,
    mut len: usize,
) -> Status {
    let data: *mut u8 = _data as *mut u8;
    let mut startpos: usize = 0;
    let mut pos: usize = 0;
    let mut c: i32 = 0;
    if data.is_null() {
        len = 0
    }
    loop {
        // Get the next character, or use -1 to indicate end of input.
        if pos < len {
            c = *data.offset(pos as isize) as i32
        } else {
            c = -1
        }
        match (*urlenp)._state {
            1 => {
                // Look for =, argument separator, or end of input.
                if c == '=' as i32 || c == (*urlenp).argument_separator as i32 || c == -1 {
                    // Data from startpos to pos.
                    htp_urlenp_add_field_piece(urlenp, data, startpos, pos, c);
                    // If it's not the end of input, then it must be the end of this field.
                    if c != -1 {
                        // Next state.
                        startpos = pos.wrapping_add(1);
                        if c == (*urlenp).argument_separator as i32 {
                            (*urlenp)._state = 1
                        } else {
                            (*urlenp)._state = 2
                        }
                    }
                }
                pos = pos.wrapping_add(1)
            }
            2 => {
                // Look for argument separator or end of input.
                if c == (*urlenp).argument_separator as i32 || c == -1 {
                    // Data from startpos to pos.
                    htp_urlenp_add_field_piece(urlenp, data, startpos, pos, c);
                    // If it's not the end of input, then it must be the end of this field.
                    if c != -1 {
                        // Next state.
                        startpos = pos.wrapping_add(1);
                        (*urlenp)._state = 1
                    }
                }
                pos = pos.wrapping_add(1)
            }
            _ => {
                // Invalid state.
                return Status::ERROR;
            }
        }
        if c == -1 {
            break;
        }
    }
    Status::OK
}
