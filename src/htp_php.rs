use ::libc;
extern "C" {
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn bstr_dup(b: *const bstr) -> *mut bstr;
    #[no_mangle]
    fn bstr_dup_mem(data: *const libc::c_void, len: size_t) -> *mut bstr;
    #[no_mangle]
    fn bstr_free(b: *mut bstr);
}
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
pub type htp_status_t = libc::c_int;
pub type bstr = crate::src::bstr::bstr_t;

/* *
 * This is a proof-of-concept processor that processes parameter names in
 * a way _similar_ to PHP. Whitespace at the beginning is removed, and the
 * remaining whitespace characters are converted to underscores. Proper
 * research of PHP's behavior is needed before we can claim to be emulating it.
 *
 * @param[in,out] p
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_php_parameter_processor(
    mut p: *mut crate::src::htp_transaction::htp_param_t,
) -> htp_status_t {
    if p.is_null() {
        return -(1 as libc::c_int);
    }
    // Name transformation
    let mut new_name: *mut bstr = 0 as *mut bstr;
    // Ignore whitespace characters at the beginning of parameter name.
    let mut data: *mut libc::c_uchar = if (*(*p).name).realptr.is_null() {
        ((*p).name as *mut libc::c_uchar)
            .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
    } else {
        (*(*p).name).realptr
    };
    let mut len: size_t = (*(*p).name).len;
    let mut pos: size_t = 0 as libc::c_int as size_t;
    // Advance over any whitespace characters at the beginning of the name.
    while pos < len
        && *(*__ctype_b_loc()).offset(*data.offset(pos as isize) as libc::c_int as isize)
            as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        pos = pos.wrapping_add(1)
    }
    // Have we seen any whitespace?
    if pos > 0 as libc::c_int as libc::c_ulong {
        // Make a copy of the name, starting with
        // the first non-whitespace character.
        new_name = bstr_dup_mem(
            data.offset(pos as isize) as *const libc::c_void,
            len.wrapping_sub(pos),
        );
        if new_name.is_null() {
            return -(1 as libc::c_int);
        }
    }
    // Replace remaining whitespace characters with underscores.
    let mut offset: size_t = pos;
    pos = 0 as libc::c_int as size_t;
    // Advance to the end of name or to the first whitespace character.
    while offset.wrapping_add(pos) < len
        && *(*__ctype_b_loc()).offset(*data.offset(pos as isize) as libc::c_int as isize)
            as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            == 0
    {
        pos = pos.wrapping_add(1)
    }
    // Are we at the end of the name?
    if offset.wrapping_add(pos) < len {
        // Seen whitespace within the string.
        // Make a copy of the name if needed (which would be the case
        // with a parameter that does not have any whitespace in front).
        if new_name.is_null() {
            new_name = bstr_dup((*p).name);
            if new_name.is_null() {
                return -(1 as libc::c_int);
            }
        }
        // Change the pointers to the new name and ditch the offset.
        data = if (*new_name).realptr.is_null() {
            (new_name as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
        } else {
            (*new_name).realptr
        };
        len = (*new_name).len;
        // Replace any whitespace characters in the copy with underscores.
        while pos < len {
            if *(*__ctype_b_loc()).offset(*data.offset(pos as isize) as libc::c_int as isize)
                as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
            {
                *data.offset(pos as isize) = '_' as i32 as libc::c_uchar
            }
            pos = pos.wrapping_add(1)
        }
    }
    // If we made any changes, free the old parameter name and put the new one in.
    if !new_name.is_null() {
        bstr_free((*p).name);
        (*p).name = new_name
    }
    return 1 as libc::c_int;
}
