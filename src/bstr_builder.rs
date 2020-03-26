use ::libc;
extern "C" {
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn htp_list_array_create(size: size_t) -> *mut crate::src::htp_list::htp_list_array_t;
    #[no_mangle]
    fn htp_list_array_clear(l: *mut crate::src::htp_list::htp_list_array_t);
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
    fn bstr_dup_c(cstr: *const libc::c_char) -> *mut bstr;
    #[no_mangle]
    fn bstr_dup_mem(data: *const libc::c_void, len: size_t) -> *mut bstr;
    #[no_mangle]
    fn bstr_free(b: *mut bstr);
    #[no_mangle]
    fn bstr_alloc(size: size_t) -> *mut bstr;
    #[no_mangle]
    fn bstr_add_noex(bdestination: *mut bstr, bsource: *const bstr) -> *mut bstr;
}

pub type size_t = libc::c_ulong;
pub type bstr = crate::src::bstr::bstr_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bstr_builder_t {
    pub pieces: *mut crate::src::htp_list::htp_list_array_t,
}

pub type htp_status_t = libc::c_int;

/**
 * Adds one new string to the builder. This function will adopt the
 * string and destroy it when the builder itself is destroyed.
 *
 * @param[in] bb
 * @param[in] b
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn bstr_builder_appendn(
    mut bb: *mut bstr_builder_t,
    mut b: *mut bstr,
) -> htp_status_t {
    return htp_list_array_push((*bb).pieces, b as *mut libc::c_void);
}

/**
 * Adds one new piece, in the form of a NUL-terminated string, to
 * the builder. This function will make a copy of the provided string.
 *
 * @param[in] bb
 * @param[in] cstr
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn bstr_builder_append_c(
    mut bb: *mut bstr_builder_t,
    mut cstr: *const libc::c_char,
) -> htp_status_t {
    let mut b: *mut bstr = bstr_dup_c(cstr);
    if b.is_null() {
        return -(1 as libc::c_int);
    }
    return htp_list_array_push((*bb).pieces, b as *mut libc::c_void);
}

/**
 * Adds one new piece, defined with the supplied pointer and
 * length, to the builder. This function will make a copy of the
 * provided data region.
 *
 * @param[in] bb
 * @param[in] data
 * @param[in] len
 * @return @return HTP_OK on success, HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn bstr_builder_append_mem(
    mut bb: *mut bstr_builder_t,
    mut data: *const libc::c_void,
    mut len: size_t,
) -> htp_status_t {
    let mut b: *mut bstr = bstr_dup_mem(data, len);
    if b.is_null() {
        return -(1 as libc::c_int);
    }
    return htp_list_array_push((*bb).pieces, b as *mut libc::c_void);
}

/**
 * Clears this string builder, destroying all existing pieces. You may
 * want to clear a builder once you've either read all the pieces and
 * done something with them, or after you've converted the builder into
 * a single string.
 *
 * @param[in] bb
 */
#[no_mangle]
pub unsafe extern "C" fn bstr_builder_clear(mut bb: *mut bstr_builder_t) {
    // Do nothing if the list is empty
    if htp_list_array_size((*bb).pieces) == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut n: size_t = htp_list_array_size((*bb).pieces);
    while i < n {
        let mut b: *mut bstr = htp_list_array_get((*bb).pieces, i) as *mut bstr;
        bstr_free(b);
        i = i.wrapping_add(1)
    }
    htp_list_array_clear((*bb).pieces);
}

/**
 * Creates a new string builder.
 *
 * @return New string builder, or NULL on error.
 */
#[no_mangle]
pub unsafe extern "C" fn bstr_builder_create() -> *mut bstr_builder_t {
    let mut bb: *mut bstr_builder_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<bstr_builder_t>() as libc::c_ulong,
    ) as *mut bstr_builder_t;
    if bb.is_null() {
        return 0 as *mut bstr_builder_t;
    }
    (*bb).pieces = htp_list_array_create(16 as libc::c_int as size_t);
    if (*bb).pieces.is_null() {
        free(bb as *mut libc::c_void);
        return 0 as *mut bstr_builder_t;
    }
    return bb;
}

/**
 * Destroys an existing string builder, also destroying all
 * the pieces stored within.
 *
 * @param[in] bb
 */
#[no_mangle]
pub unsafe extern "C" fn bstr_builder_destroy(mut bb: *mut bstr_builder_t) {
    if bb.is_null() {
        return;
    }
    // Destroy any pieces we might have
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut n: size_t = htp_list_array_size((*bb).pieces);
    while i < n {
        let mut b: *mut bstr = htp_list_array_get((*bb).pieces, i) as *mut bstr;
        bstr_free(b);
        i = i.wrapping_add(1)
    }
    htp_list_array_destroy((*bb).pieces);
    free(bb as *mut libc::c_void);
}

/**
 * Returns the size (the number of pieces) currently in a string builder.
 *
 * @param[in] bb
 * @return size
 */
#[no_mangle]
pub unsafe extern "C" fn bstr_builder_size(mut bb: *const bstr_builder_t) -> size_t {
    return htp_list_array_size((*bb).pieces);
}

/**
 * Creates a single string out of all the pieces held in a
 * string builder. This method will not destroy any of the pieces.
 *
 * @param[in] bb
 * @return New string, or NULL on error.
 */
#[no_mangle]
pub unsafe extern "C" fn bstr_builder_to_str(mut bb: *const bstr_builder_t) -> *mut bstr {
    let mut len: size_t = 0 as libc::c_int as size_t;
    // Determine the size of the string
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut n: size_t = htp_list_array_size((*bb).pieces);
    while i < n {
        let mut b: *mut bstr = htp_list_array_get((*bb).pieces, i) as *mut bstr;
        len = (len as libc::c_ulong).wrapping_add((*b).len) as size_t as size_t;
        i = i.wrapping_add(1)
    }
    // Allocate string
    let mut bnew: *mut bstr = bstr_alloc(len);
    if bnew.is_null() {
        return 0 as *mut bstr;
    }
    // Determine the size of the string
    let mut i_0: size_t = 0 as libc::c_int as size_t;
    let mut n_0: size_t = htp_list_array_size((*bb).pieces);
    while i_0 < n_0 {
        let mut b_0: *mut bstr = htp_list_array_get((*bb).pieces, i_0) as *mut bstr;
        bstr_add_noex(bnew, b_0);
        i_0 = i_0.wrapping_add(1)
    }
    return bnew;
}
