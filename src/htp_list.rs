use crate::Status;

extern "C" {
    #[no_mangle]
    fn malloc(_: libc::size_t) -> *mut core::ffi::c_void;
    #[no_mangle]
    fn calloc(_: libc::size_t, _: libc::size_t) -> *mut core::ffi::c_void;
    #[no_mangle]
    fn realloc(_: *mut core::ffi::c_void, _: libc::size_t) -> *mut core::ffi::c_void;
    #[no_mangle]
    fn free(__ptr: *mut core::ffi::c_void);
    #[no_mangle]
    fn memcpy(
        _: *mut core::ffi::c_void,
        _: *const core::ffi::c_void,
        _: libc::size_t,
    ) -> *mut core::ffi::c_void;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_list_array_t {
    pub first: usize,
    pub last: usize,
    pub max_size: usize,
    pub current_size: usize,
    pub elements: *mut *mut core::ffi::c_void,
}

// Array-backed list

/// Initialize an array-backed list.
///
/// Returns HTP_OK or HTP_ERROR if allocation failed
pub unsafe fn htp_list_array_init(mut l: *mut htp_list_array_t, size: usize) -> Status {
    // Allocate the initial batch of elements.
    (*l).elements = malloc(size.wrapping_mul(::std::mem::size_of::<*mut core::ffi::c_void>()))
        as *mut *mut core::ffi::c_void;
    if (*l).elements.is_null() {
        return Status::ERROR;
    }
    // Initialize the structure.
    (*l).first = 0;
    (*l).last = 0;
    (*l).current_size = 0;
    (*l).max_size = size;
    Status::OK
}

/// Create new array-backed list.
///
/// Returns Newly created list.
pub unsafe fn htp_list_array_create(size: usize) -> *mut htp_list_array_t {
    // It makes no sense to create a zero-size list.
    if size == 0 {
        return 0 as *mut htp_list_array_t;
    }
    // Allocate the list structure.
    let l: *mut htp_list_array_t =
        calloc(1, ::std::mem::size_of::<htp_list_array_t>()) as *mut htp_list_array_t;
    if l.is_null() {
        return 0 as *mut htp_list_array_t;
    }
    if htp_list_array_init(l, size) == Status::ERROR {
        free(l as *mut core::ffi::c_void);
        return 0 as *mut htp_list_array_t;
    }
    l
}

/// Remove all elements from the list. It is the responsibility of the caller
/// to iterate over list elements and deallocate them if necessary, prior to
/// invoking this function.
pub unsafe fn htp_list_array_clear(mut l: *mut htp_list_array_t) {
    if l.is_null() {
        return;
    }
    // Continue using already allocated memory; just reset the fields.
    (*l).first = 0;
    (*l).last = 0;
    (*l).current_size = 0;
}

/// Free the memory occupied by this list. This function assumes
/// the elements held by the list were freed beforehand.
pub unsafe fn htp_list_array_destroy(l: *mut htp_list_array_t) {
    if l.is_null() {
        return;
    }
    free((*l).elements as *mut core::ffi::c_void);
    free(l as *mut core::ffi::c_void);
}

/// Free the memory occupied by this list, except itself.
/// This function assumes the elements held by the list
/// were freed beforehand.
pub unsafe fn htp_list_array_release(l: *mut htp_list_array_t) {
    if l.is_null() {
        return;
    }
    free((*l).elements as *mut core::ffi::c_void);
}

/// Find the element at the given index.
///
/// Returns the desired element, or NULL if the list is too small, or
///         if the element at that position carries a NULL
pub unsafe fn htp_list_array_get(l: *const htp_list_array_t, idx: usize) -> *mut core::ffi::c_void {
    if l.is_null() {
        return 0 as *mut core::ffi::c_void;
    }
    if idx.wrapping_add(1) > (*l).current_size {
        return 0 as *mut core::ffi::c_void;
    }
    if (*l).first.wrapping_add(idx) < (*l).max_size {
        return *(*l).elements.offset((*l).first.wrapping_add(idx) as isize);
    }
    *(*l)
        .elements
        .offset(idx.wrapping_sub((*l).max_size.wrapping_sub((*l).first)) as isize)
}

/// Remove one element from the end of the list.
///
/// Returns The removed element, or NULL if the list is empty.
pub unsafe fn htp_list_array_pop(mut l: *mut htp_list_array_t) -> *mut core::ffi::c_void {
    if l.is_null() {
        return 0 as *mut core::ffi::c_void;
    }
    let mut r: *const core::ffi::c_void = 0 as *const core::ffi::c_void;
    if (*l).current_size == 0 {
        return 0 as *mut core::ffi::c_void;
    }
    let mut pos: usize = (*l).first.wrapping_add((*l).current_size).wrapping_sub(1);
    if pos > (*l).max_size.wrapping_sub(1) {
        pos = (pos).wrapping_sub((*l).max_size)
    }
    r = *(*l).elements.offset(pos as isize);
    (*l).last = pos;
    (*l).current_size = (*l).current_size.wrapping_sub(1);
    r as *mut core::ffi::c_void
}

/// Add new element to the end of the list, expanding the list as necessary.
///
/// Returns HTP_OK on success or HTP_ERROR on failure.
pub unsafe fn htp_list_array_push(
    mut l: *mut htp_list_array_t,
    e: *mut core::ffi::c_void,
) -> Status {
    if l.is_null() {
        return Status::ERROR;
    }
    // Check whether we're full
    if (*l).current_size >= (*l).max_size {
        let new_size: usize = (*l).max_size.wrapping_mul(2);
        let mut newblock: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
        if (*l).first == 0 {
            // The simple case of expansion is when the first
            // element in the list resides in the first slot. In
            // that case we just add some new space to the end,
            // adjust the max_size and that's that.
            newblock = realloc(
                (*l).elements as *mut core::ffi::c_void,
                new_size.wrapping_mul(::std::mem::size_of::<*mut core::ffi::c_void>()),
            );
            if newblock.is_null() {
                return Status::ERROR;
            }
        } else {
            // When the first element is not in the first
            // memory slot, we need to rearrange the order
            // of the elements in order to expand the storage area.
            // coverity[suspicious_sizeof]
            newblock =
                malloc(new_size.wrapping_mul(::std::mem::size_of::<*mut core::ffi::c_void>()));
            if newblock.is_null() {
                return Status::ERROR;
            }
            // Copy the beginning of the list to the beginning of the new memory block
            // coverity[suspicious_sizeof]
            memcpy(
                newblock,
                ((*l).elements as *mut i8).offset(
                    ((*l).first).wrapping_mul(::std::mem::size_of::<*mut core::ffi::c_void>())
                        as isize,
                ) as *mut core::ffi::c_void,
                ((*l).max_size)
                    .wrapping_sub((*l).first)
                    .wrapping_mul(::std::mem::size_of::<*mut core::ffi::c_void>()),
            );
            // Append the second part of the list to the end
            memcpy(
                (newblock as *mut i8).offset(
                    ((*l).max_size)
                        .wrapping_sub((*l).first)
                        .wrapping_mul(::std::mem::size_of::<*mut core::ffi::c_void>())
                        as isize,
                ) as *mut core::ffi::c_void,
                (*l).elements as *mut core::ffi::c_void,
                ((*l).first).wrapping_mul(::std::mem::size_of::<*mut core::ffi::c_void>()),
            );
            free((*l).elements as *mut core::ffi::c_void);
        }
        (*l).first = 0;
        (*l).last = (*l).current_size;
        (*l).max_size = new_size;
        (*l).elements = newblock as *mut *mut core::ffi::c_void
    }
    *(*l).elements.offset((*l).last as isize) = e;
    (*l).current_size = (*l).current_size.wrapping_add(1);
    (*l).last = (*l).last.wrapping_add(1);
    if (*l).last == (*l).max_size {
        (*l).last = 0
    }
    Status::OK
}

/// Replace the element at the given index with the provided element.
///
/// Returns HTP_OK if an element with the given index was replaced; HTP_ERROR
///         if the desired index does not exist.
pub unsafe fn htp_list_array_replace(
    l: *mut htp_list_array_t,
    idx: usize,
    e: *mut core::ffi::c_void,
) -> Status {
    if l.is_null() {
        return Status::ERROR;
    }
    if idx.wrapping_add(1) > (*l).current_size {
        return Status::DECLINED;
    }
    *(*l)
        .elements
        .offset((*l).first.wrapping_add(idx).wrapping_rem((*l).max_size) as isize) = e;
    Status::OK
}

/// Returns the size of the list.
///
/// Returns List size.
pub unsafe fn htp_list_array_size(l: *const htp_list_array_t) -> usize {
    if l.is_null() {
        return (-1 as i32) as usize;
    }
    (*l).current_size
}
