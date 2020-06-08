use crate::{bstr, htp_list, Status};

extern "C" {
    #[no_mangle]
    fn calloc(_: libc::size_t, _: libc::size_t) -> *mut core::ffi::c_void;
    #[no_mangle]
    fn free(__ptr: *mut core::ffi::c_void);
}

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
    bb: *mut bstr_builder_t,
    data: *const core::ffi::c_void,
    len: usize,
) -> Status {
    let b: *const bstr::bstr_t = bstr::bstr_dup_mem(data, len);
    if b.is_null() {
        return Status::ERROR;
    }
    htp_list::htp_list_array_push((*bb).pieces, b as *mut core::ffi::c_void)
}

/// Clears this string builder, destroying all existing pieces. You may
/// want to clear a builder once you've either read all the pieces and
/// done something with them, or after you've converted the builder into
/// a single string.
pub unsafe fn bstr_builder_clear(bb: *const bstr_builder_t) {
    // Do nothing if the list is empty
    if htp_list::htp_list_array_size((*bb).pieces) == 0 {
        return;
    }
    let mut i: usize = 0;
    let n: usize = htp_list::htp_list_array_size((*bb).pieces);
    while i < n {
        let b: *mut bstr::bstr_t =
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
    let mut bb: *mut bstr_builder_t =
        calloc(1, ::std::mem::size_of::<bstr_builder_t>()) as *mut bstr_builder_t;
    if bb.is_null() {
        return 0 as *mut bstr_builder_t;
    }
    (*bb).pieces = htp_list::htp_list_array_create(16);
    if (*bb).pieces.is_null() {
        free(bb as *mut core::ffi::c_void);
        return 0 as *mut bstr_builder_t;
    }
    bb
}

/// Destroys an existing string builder, also destroying all
/// the pieces stored within.
pub unsafe fn bstr_builder_destroy(bb: *const bstr_builder_t) {
    if bb.is_null() {
        return;
    }
    // Destroy any pieces we might have
    let mut i: usize = 0;
    let n: usize = htp_list::htp_list_array_size((*bb).pieces);
    while i < n {
        let b: *mut bstr::bstr_t =
            htp_list::htp_list_array_get((*bb).pieces, i) as *mut bstr::bstr_t;
        bstr::bstr_free(b);
        i = i.wrapping_add(1)
    }
    htp_list::htp_list_array_destroy((*bb).pieces);
    free(bb as *mut core::ffi::c_void);
}

/// Returns the size (the number of pieces) currently in a string builder.
///
/// Returns size
pub unsafe fn bstr_builder_size(bb: *const bstr_builder_t) -> usize {
    htp_list::htp_list_array_size((*bb).pieces)
}

/// Creates a single string out of all the pieces held in a
/// string builder. This method will not destroy any of the pieces.
///
/// Returns New string, or NULL on error.
pub unsafe fn bstr_builder_to_str(bb: *const bstr_builder_t) -> *mut bstr::bstr_t {
    let mut len: usize = 0;
    // Determine the size of the string
    let mut i: usize = 0;
    let n: usize = htp_list::htp_list_array_size((*bb).pieces);
    while i < n {
        let b: *const bstr::bstr_t =
            htp_list::htp_list_array_get((*bb).pieces, i) as *mut bstr::bstr_t;
        len = (len).wrapping_add(bstr::bstr_len(b));
        i = i.wrapping_add(1)
    }
    // Allocate string
    let bnew: *mut bstr::bstr_t = bstr::bstr_alloc(len);
    if bnew.is_null() {
        return 0 as *mut bstr::bstr_t;
    }
    // Determine the size of the string
    let mut i_0: usize = 0;
    let n_0: usize = htp_list::htp_list_array_size((*bb).pieces);
    while i_0 < n_0 {
        let b_0: *mut bstr::bstr_t =
            htp_list::htp_list_array_get((*bb).pieces, i_0) as *mut bstr::bstr_t;
        bstr::bstr_add_noex(bnew, b_0);
        i_0 = i_0.wrapping_add(1)
    }
    bnew
}
