use crate::Status;
use ::libc;
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_list_array_t {
    pub first: size_t,
    pub last: size_t,
    pub max_size: size_t,
    pub current_size: size_t,
    pub elements: *mut *mut libc::c_void,
}

// Array-backed list

/**
 * Initialize an array-backed list.
 *
 * @param[in] l
 * @param[in] size
 * @return HTP_OK or HTP_ERROR if allocation failed
 */
#[no_mangle]
pub unsafe extern "C" fn htp_list_array_init(
    mut l: *mut htp_list_array_t,
    mut size: size_t,
) -> Status {
    // Allocate the initial batch of elements.
    (*l).elements =
        malloc(size.wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong))
            as *mut *mut libc::c_void;
    if (*l).elements.is_null() {
        return Status::ERROR;
    }
    // Initialize the structure.
    (*l).first = 0 as libc::c_int as size_t;
    (*l).last = 0 as libc::c_int as size_t;
    (*l).current_size = 0 as libc::c_int as size_t;
    (*l).max_size = size;
    return Status::OK;
}

/**
 * Create new array-backed list.
 *
 * @param[in] size
 * @return Newly created list.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_list_array_create(mut size: size_t) -> *mut htp_list_array_t {
    // It makes no sense to create a zero-size list.
    if size == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut htp_list_array_t;
    }
    // Allocate the list structure.
    let mut l: *mut htp_list_array_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<htp_list_array_t>() as libc::c_ulong,
    ) as *mut htp_list_array_t;
    if l.is_null() {
        return 0 as *mut htp_list_array_t;
    }
    if htp_list_array_init(l, size) == Status::ERROR {
        free(l as *mut libc::c_void);
        return 0 as *mut htp_list_array_t;
    }
    return l;
}

/**
 * Remove all elements from the list. It is the responsibility of the caller
 * to iterate over list elements and deallocate them if necessary, prior to
 * invoking this function.
 *
 * @param[in] l
 */
#[no_mangle]
pub unsafe extern "C" fn htp_list_array_clear(mut l: *mut htp_list_array_t) {
    if l.is_null() {
        return;
    }
    // Continue using already allocated memory; just reset the fields.
    (*l).first = 0 as libc::c_int as size_t;
    (*l).last = 0 as libc::c_int as size_t;
    (*l).current_size = 0 as libc::c_int as size_t;
}

/**
 * Free the memory occupied by this list. This function assumes
 * the elements held by the list were freed beforehand.
 *
 * @param[in] l
 */
#[no_mangle]
pub unsafe extern "C" fn htp_list_array_destroy(mut l: *mut htp_list_array_t) {
    if l.is_null() {
        return;
    }
    free((*l).elements as *mut libc::c_void);
    free(l as *mut libc::c_void);
}

/**
 * Free the memory occupied by this list, except itself.
 * This function assumes the elements held by the list
 * were freed beforehand.
 *
 * @param[in] l
 */
#[no_mangle]
pub unsafe extern "C" fn htp_list_array_release(mut l: *mut htp_list_array_t) {
    if l.is_null() {
        return;
    }
    free((*l).elements as *mut libc::c_void);
}

/**
 * Find the element at the given index.
 *
 * @param[in] l
 * @param[in] idx
 * @return the desired element, or NULL if the list is too small, or
 *         if the element at that position carries a NULL
 */
#[no_mangle]
pub unsafe extern "C" fn htp_list_array_get(
    mut l: *const htp_list_array_t,
    mut idx: size_t,
) -> *mut libc::c_void {
    if l.is_null() {
        return 0 as *mut libc::c_void;
    }
    if idx.wrapping_add(1 as libc::c_int as libc::c_ulong) > (*l).current_size {
        return 0 as *mut libc::c_void;
    }
    if (*l).first.wrapping_add(idx) < (*l).max_size {
        return *(*l).elements.offset((*l).first.wrapping_add(idx) as isize);
    } else {
        return *(*l)
            .elements
            .offset(idx.wrapping_sub((*l).max_size.wrapping_sub((*l).first)) as isize);
    };
}

/**
 * Remove one element from the end of the list.
 *
 * @param[in] l
 * @return The removed element, or NULL if the list is empty.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_list_array_pop(mut l: *mut htp_list_array_t) -> *mut libc::c_void {
    if l.is_null() {
        return 0 as *mut libc::c_void;
    }
    let mut r: *const libc::c_void = 0 as *const libc::c_void;
    if (*l).current_size == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_void;
    }
    let mut pos: size_t = (*l)
        .first
        .wrapping_add((*l).current_size)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    if pos
        > (*l)
            .max_size
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        pos = (pos as libc::c_ulong).wrapping_sub((*l).max_size) as size_t as size_t
    }
    r = *(*l).elements.offset(pos as isize);
    (*l).last = pos;
    (*l).current_size = (*l).current_size.wrapping_sub(1);
    return r as *mut libc::c_void;
}

/**
 * Add new element to the end of the list, expanding the list as necessary.
 *
 * @param[in] l
 * @param[in] e
 * @return HTP_OK on success or HTP_ERROR on failure.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn htp_list_array_push(
    mut l: *mut htp_list_array_t,
    mut e: *mut libc::c_void,
) -> Status {
    if l.is_null() {
        return Status::ERROR;
    }
    // Check whether we're full
    if (*l).current_size >= (*l).max_size {
        let mut new_size: size_t = (*l)
            .max_size
            .wrapping_mul(2 as libc::c_int as libc::c_ulong);
        let mut newblock: *mut libc::c_void = 0 as *mut libc::c_void;
        if (*l).first == 0 as libc::c_int as libc::c_ulong {
            // The simple case of expansion is when the first
            // element in the list resides in the first slot. In
            // that case we just add some new space to the end,
            // adjust the max_size and that's that.
            newblock = realloc(
                (*l).elements as *mut libc::c_void,
                new_size.wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong),
            );
            if newblock.is_null() {
                return Status::ERROR;
            }
        } else {
            // When the first element is not in the first
            // memory slot, we need to rearrange the order
            // of the elements in order to expand the storage area.
            /* coverity[suspicious_sizeof] */
            newblock = malloc(
                new_size.wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong),
            );
            if newblock.is_null() {
                return Status::ERROR;
            }
            // Copy the beginning of the list to the beginning of the new memory block
            /* coverity[suspicious_sizeof] */
            memcpy(
                newblock,
                ((*l).elements as *mut libc::c_char).offset(
                    (*l).first
                        .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                        as isize,
                ) as *mut libc::c_void,
                (*l).max_size
                    .wrapping_sub((*l).first)
                    .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong),
            );
            // Append the second part of the list to the end
            memcpy(
                (newblock as *mut libc::c_char).offset(
                    (*l).max_size
                        .wrapping_sub((*l).first)
                        .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                        as isize,
                ) as *mut libc::c_void,
                (*l).elements as *mut libc::c_void,
                (*l).first
                    .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong),
            );
            free((*l).elements as *mut libc::c_void);
        }
        (*l).first = 0 as libc::c_int as size_t;
        (*l).last = (*l).current_size;
        (*l).max_size = new_size;
        (*l).elements = newblock as *mut *mut libc::c_void
    }
    let ref mut fresh0 = *(*l).elements.offset((*l).last as isize);
    *fresh0 = e;
    (*l).current_size = (*l).current_size.wrapping_add(1);
    (*l).last = (*l).last.wrapping_add(1);
    if (*l).last == (*l).max_size {
        (*l).last = 0 as libc::c_int as size_t
    }
    return Status::OK;
}

/**
 * Replace the element at the given index with the provided element.
 *
 * @param[in] l
 * @param[in] idx
 * @param[in] e
 *
 * @return HTP_OK if an element with the given index was replaced; HTP_ERROR
 *         if the desired index does not exist.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_list_array_replace(
    mut l: *mut htp_list_array_t,
    mut idx: size_t,
    mut e: *mut libc::c_void,
) -> Status {
    if l.is_null() {
        return Status::ERROR;
    }
    if idx.wrapping_add(1 as libc::c_int as libc::c_ulong) > (*l).current_size {
        return Status::DECLINED;
    }
    let ref mut fresh1 = *(*l)
        .elements
        .offset((*l).first.wrapping_add(idx).wrapping_rem((*l).max_size) as isize);
    *fresh1 = e;
    return Status::OK;
}

/**
 * Returns the size of the list.
 *
 * @param[in] l
 * @return List size.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_list_array_size(mut l: *const htp_list_array_t) -> size_t {
    if l.is_null() {
        return -(1 as libc::c_int) as size_t;
    }
    return (*l).current_size;
}
/* *
 * Remove one element from the beginning of the list.
 *
 * @param[in] l
 * @return The removed element, or NULL if the list is empty.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_list_array_shift(mut l: *mut htp_list_array_t) -> *mut libc::c_void {
    if l.is_null() {
        return 0 as *mut libc::c_void;
    }
    let mut r: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*l).current_size == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_void;
    }
    r = *(*l).elements.offset((*l).first as isize);
    (*l).first = (*l).first.wrapping_add(1);
    if (*l).first == (*l).max_size {
        (*l).first = 0 as libc::c_int as size_t
    }
    (*l).current_size = (*l).current_size.wrapping_sub(1);
    return r;
}
