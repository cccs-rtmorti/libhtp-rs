use crate::{bstr, list, Status};

#[derive(Clone)]
pub struct bstr_builder_t {
    pub pieces: list::List<*mut bstr::bstr_t>,
}

impl bstr_builder_t {
    fn new() -> Self {
        Self {
            pieces: list::List::with_capacity(16),
        }
    }
}

impl Drop for bstr_builder_t {
    fn drop(&mut self) {
        for each in &self.pieces {
            unsafe { bstr::bstr_free(*each) };
        }
    }
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
    let b: *mut bstr::bstr_t = bstr::bstr_dup_mem(data, len);
    if b.is_null() {
        return Status::ERROR;
    }
    (*bb).pieces.push(b);
    Status::OK
}

/// Clears this string builder, destroying all existing pieces. You may
/// want to clear a builder once you've either read all the pieces and
/// done something with them, or after you've converted the builder into
/// a single string.
pub unsafe fn bstr_builder_clear(bb: *mut bstr_builder_t) {
    // Do nothing if the list is empty
    if (*bb).pieces.len() == 0 {
        return;
    }
    for each in &(*bb).pieces {
        bstr::bstr_free(*each);
    }
    (*bb).pieces.clear();
}

/// Creates a new string builder.
///
/// Returns New string builder, or NULL on error.
pub unsafe fn bstr_builder_create() -> *mut bstr_builder_t {
    Box::into_raw(Box::new(bstr_builder_t::new()))
}

/// Destroys an existing string builder, also destroying all
/// the pieces stored within.
pub unsafe fn bstr_builder_destroy(bb: *mut bstr_builder_t) {
    if bb.is_null() {
        return;
    }
    Box::from_raw(bb);
}

/// Returns the size (the number of pieces) currently in a string builder.
///
/// Returns size
pub unsafe fn bstr_builder_size(bb: *const bstr_builder_t) -> usize {
    (*bb).pieces.len()
}

/// Creates a single string out of all the pieces held in a
/// string builder. This method will not destroy any of the pieces.
///
/// Returns New string, or NULL on error.
pub unsafe fn bstr_builder_to_str(bb: *mut bstr_builder_t) -> *mut bstr::bstr_t {
    let mut len: usize = 0;
    // Determine the size of the string
    for each in &(*bb).pieces {
        len = len.wrapping_add(bstr::bstr_len(*each));
    }
    // Allocate string
    let bnew: *mut bstr::bstr_t = bstr::bstr_alloc(len);
    if bnew.is_null() {
        return 0 as *mut bstr::bstr_t;
    }
    // Determine the size of the string
    for each in &(*bb).pieces {
        bstr::bstr_add_noex(bnew, *each);
    }
    bnew
}
