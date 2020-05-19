use crate::{bstr, htp_list, Status};
use ::libc;

extern "C" {
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
}

pub type size_t = libc::c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bstr_builder_t {
    pub pieces: *mut htp_list::htp_list_array_t,
}

/// Adds one new piece, defined with the supplied pointer and
/// length, to the builder. This function will make a copy of the
/// provided data region.
///
/// Returns HTP_OK on success, HTP_ERROR on failure.
pub unsafe fn bstr_builder_append_mem(
    mut bb: *mut bstr_builder_t,
    mut data: *const libc::c_void,
    mut len: size_t,
) -> Status {
    let mut b: *mut bstr::bstr_t = bstr::bstr_dup_mem(data, len);
    if b.is_null() {
        return Status::ERROR;
    }
    return htp_list::htp_list_array_push((*bb).pieces, b as *mut libc::c_void);
}

/// Clears this string builder, destroying all existing pieces. You may
/// want to clear a builder once you've either read all the pieces and
/// done something with them, or after you've converted the builder into
/// a single string.
pub unsafe fn bstr_builder_clear(mut bb: *mut bstr_builder_t) {
    // Do nothing if the list is empty
    if htp_list::htp_list_array_size((*bb).pieces) == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut n: size_t = htp_list::htp_list_array_size((*bb).pieces);
    while i < n {
        let mut b: *mut bstr::bstr_t =
            htp_list::htp_list_array_get((*bb).pieces, i) as *mut bstr::bstr_t;
        bstr::bstr_free(b);
        i = i.wrapping_add(1)
    }
    htp_list::htp_list_array_clear((*bb).pieces);
}

/// Creates a new string builder.
///
/// Returns New string builder, or NULL on error.
pub unsafe fn bstr_builder_create() -> *mut bstr_builder_t {
    let mut bb: *mut bstr_builder_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<bstr_builder_t>() as libc::c_ulong,
    ) as *mut bstr_builder_t;
    if bb.is_null() {
        return 0 as *mut bstr_builder_t;
    }
    (*bb).pieces = htp_list::htp_list_array_create(16 as libc::c_int as size_t);
    if (*bb).pieces.is_null() {
        free(bb as *mut libc::c_void);
        return 0 as *mut bstr_builder_t;
    }
    return bb;
}

/// Destroys an existing string builder, also destroying all
/// the pieces stored within.
pub unsafe fn bstr_builder_destroy(mut bb: *mut bstr_builder_t) {
    if bb.is_null() {
        return;
    }
    // Destroy any pieces we might have
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut n: size_t = htp_list::htp_list_array_size((*bb).pieces);
    while i < n {
        let mut b: *mut bstr::bstr_t =
            htp_list::htp_list_array_get((*bb).pieces, i) as *mut bstr::bstr_t;
        bstr::bstr_free(b);
        i = i.wrapping_add(1)
    }
    htp_list::htp_list_array_destroy((*bb).pieces);
    free(bb as *mut libc::c_void);
}

/// Returns the size (the number of pieces) currently in a string builder.
///
/// Returns size
pub unsafe fn bstr_builder_size(mut bb: *const bstr_builder_t) -> size_t {
    return htp_list::htp_list_array_size((*bb).pieces);
}

/// Creates a single string out of all the pieces held in a
/// string builder. This method will not destroy any of the pieces.
///
/// Returns New string, or NULL on error.
pub unsafe fn bstr_builder_to_str(mut bb: *const bstr_builder_t) -> *mut bstr::bstr_t {
    let mut len: size_t = 0 as libc::c_int as size_t;
    // Determine the size of the string
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut n: size_t = htp_list::htp_list_array_size((*bb).pieces);
    while i < n {
        let mut b: *mut bstr::bstr_t =
            htp_list::htp_list_array_get((*bb).pieces, i) as *mut bstr::bstr_t;
        len = (len as libc::c_ulong).wrapping_add((*b).len) as size_t as size_t;
        i = i.wrapping_add(1)
    }
    // Allocate string
    let mut bnew: *mut bstr::bstr_t = bstr::bstr_alloc(len);
    if bnew.is_null() {
        return 0 as *mut bstr::bstr_t;
    }
    // Determine the size of the string
    let mut i_0: size_t = 0 as libc::c_int as size_t;
    let mut n_0: size_t = htp_list::htp_list_array_size((*bb).pieces);
    while i_0 < n_0 {
        let mut b_0: *mut bstr::bstr_t =
            htp_list::htp_list_array_get((*bb).pieces, i_0) as *mut bstr::bstr_t;
        bstr::bstr_add_noex(bnew, b_0);
        i_0 = i_0.wrapping_add(1)
    }
    return bnew;
}
