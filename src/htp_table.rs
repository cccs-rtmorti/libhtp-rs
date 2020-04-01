use ::libc;
extern "C" {
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn htp_list_array_init(
        l: *mut crate::src::htp_list::htp_list_array_t,
        size: size_t,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_list_array_clear(l: *mut crate::src::htp_list::htp_list_array_t);
    #[no_mangle]
    fn htp_list_array_release(l: *mut crate::src::htp_list::htp_list_array_t);
    #[no_mangle]
    fn htp_list_array_get(
        l: *const crate::src::htp_list::htp_list_array_t,
        idx: size_t,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn htp_list_array_pop(l: *mut crate::src::htp_list::htp_list_array_t) -> *mut libc::c_void;
    #[no_mangle]
    fn htp_list_array_push(
        l: *mut crate::src::htp_list::htp_list_array_t,
        e: *mut libc::c_void,
    ) -> htp_status_t;
    #[no_mangle]
    fn htp_list_array_size(l: *const crate::src::htp_list::htp_list_array_t) -> size_t;
    #[no_mangle]
    fn bstr_cmp_c_nocasenorzero(b: *const bstr, cstr: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn bstr_cmp_mem_nocase(b: *const bstr, data: *const libc::c_void, len: size_t) -> libc::c_int;
    #[no_mangle]
    fn bstr_cmp_nocase(b1: *const bstr, b2: *const bstr) -> libc::c_int;
    #[no_mangle]
    fn bstr_dup(b: *const bstr) -> *mut bstr;
    #[no_mangle]
    fn bstr_free(b: *mut bstr);
}
pub type size_t = libc::c_ulong;
pub type htp_status_t = libc::c_int;

pub type bstr = crate::src::bstr::bstr_t;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
enum htp_table_alloc_t {
    /** This is the default value, used only until the first element is added. */
    HTP_TABLE_KEYS_ALLOC_UKNOWN,
    /** Keys are copied.*/
    HTP_TABLE_KEYS_COPIED,
    /** Keys are adopted and freed when the table is destroyed. */
    HTP_TABLE_KEYS_ADOPTED,
    /** Keys are only referenced; the caller is still responsible for freeing them after the table is destroyed. */
    HTP_TABLE_KEYS_REFERENCED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_table_t {
    /** Table key and value pairs are stored in this list; name first, then value. */
    pub list: crate::src::htp_list::htp_list_array_t,
    /**
     * Key management strategy. Initially set to HTP_TABLE_KEYS_ALLOC_UKNOWN. The
     * actual strategy is determined by the first allocation.
     */
    alloc_type: htp_table_alloc_t,
}

unsafe extern "C" fn _htp_table_add(
    mut table: *mut htp_table_t,
    mut key: *const bstr,
    mut element: *const libc::c_void,
) -> htp_status_t {
    // Add key.
    if htp_list_array_push(&mut (*table).list, key as *mut libc::c_void) != 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    // Add element.
    if htp_list_array_push(&mut (*table).list, element as *mut libc::c_void) != 1 as libc::c_int {
        htp_list_array_pop(&mut (*table).list);
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}

/**
 * Add a new element to the table. The key will be copied, and the copy
 * managed by the table. The table keeps a pointer to the element. It is the
 * callers responsibility to ensure the pointer remains valid.
 *
 * @param[in] table
 * @param[in] key
 * @param[in] element
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_table_add(
    mut table: *mut htp_table_t,
    mut key: *const bstr,
    mut element: *const libc::c_void,
) -> htp_status_t {
    if table.is_null() || key.is_null() {
        return -(1 as libc::c_int);
    }
    // Keep track of how keys are allocated, and
    // ensure that all invocations are consistent.
    if (*table).alloc_type == htp_table_alloc_t::HTP_TABLE_KEYS_ALLOC_UKNOWN {
        (*table).alloc_type = htp_table_alloc_t::HTP_TABLE_KEYS_COPIED
    } else if (*table).alloc_type != htp_table_alloc_t::HTP_TABLE_KEYS_COPIED {
        return -(1 as libc::c_int);
    }
    let mut dupkey: *mut bstr = bstr_dup(key);
    if dupkey.is_null() {
        return -(1 as libc::c_int);
    }
    if _htp_table_add(table, dupkey, element) != 1 as libc::c_int {
        bstr_free(dupkey);
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}

/**
 * Add a new element to the table. The key provided will be adopted and managed
 * by the table. You should not keep a copy of the pointer to the key unless you're
 * certain that the table will live longer that the copy. The table keeps a pointer
 * to the element. It is the callers responsibility to ensure the pointer remains
 * valid.
 *
 * @param[in] table
 * @param[in] key
 * @param[in] element
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_table_addn(
    mut table: *mut htp_table_t,
    mut key: *const bstr,
    mut element: *const libc::c_void,
) -> htp_status_t {
    if table.is_null() || key.is_null() {
        return -(1 as libc::c_int);
    }
    // Keep track of how keys are allocated, and
    // ensure that all invocations are consistent.
    if (*table).alloc_type == htp_table_alloc_t::HTP_TABLE_KEYS_ALLOC_UKNOWN {
        (*table).alloc_type = htp_table_alloc_t::HTP_TABLE_KEYS_ADOPTED
    } else if (*table).alloc_type != htp_table_alloc_t::HTP_TABLE_KEYS_ADOPTED {
        return -(1 as libc::c_int);
    }
    return _htp_table_add(table, key, element);
}

/**
 * Add a new element to the table. The key provided will be only referenced and the
 * caller remains responsible to keep it alive until after the table is destroyed. The
 * table keeps a pointer to the element. It is the callers responsibility to ensure
 * the pointer remains valid.
 *
 * @param[in] table
 * @param[in] key
 * @param[in] element
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_table_addk(
    mut table: *mut htp_table_t,
    mut key: *const bstr,
    mut element: *const libc::c_void,
) -> htp_status_t {
    if table.is_null() || key.is_null() {
        return -(1 as libc::c_int);
    }
    // Keep track of how keys are allocated, and
    // ensure that all invocations are consistent.
    if (*table).alloc_type == htp_table_alloc_t::HTP_TABLE_KEYS_ALLOC_UKNOWN {
        (*table).alloc_type = htp_table_alloc_t::HTP_TABLE_KEYS_REFERENCED
    } else if (*table).alloc_type != htp_table_alloc_t::HTP_TABLE_KEYS_REFERENCED {
        return -(1 as libc::c_int);
    }
    return _htp_table_add(table, key, element);
}

/**
 * Remove all elements from the table. This function handles keys
 * according to the active allocation strategy. If the elements need freeing,
 * you need to free them before invoking this function.
 *
 * @param[in] table
 */
#[no_mangle]
pub unsafe extern "C" fn htp_table_clear(mut table: *mut htp_table_t) {
    if table.is_null() {
        return;
    }
    // Free the table keys, but only if we're managing them.
    if (*table).alloc_type == htp_table_alloc_t::HTP_TABLE_KEYS_COPIED
        || (*table).alloc_type == htp_table_alloc_t::HTP_TABLE_KEYS_ADOPTED
    {
        let mut key: *mut bstr = 0 as *mut bstr;
        let mut i: size_t = 0 as libc::c_int as size_t;
        let mut n: size_t = htp_list_array_size(&mut (*table).list);
        while i < n {
            key = htp_list_array_get(&mut (*table).list, i) as *mut bstr;
            bstr_free(key);
            i = (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                as size_t
        }
    }
    htp_list_array_clear(&mut (*table).list);
}

/**
 * Remove all elements from the table without freeing any of the keys, even
 * if the table is using an allocation strategy where keys belong to it. This
 * function is useful if all the keys have been adopted by some other structure.
 *
 * @param[in] table
 */
#[no_mangle]
pub unsafe extern "C" fn htp_table_clear_ex(mut table: *mut htp_table_t) {
    if table.is_null() {
        return;
    }
    // This function does not free table keys.
    htp_list_array_clear(&mut (*table).list);
}

/**
 * Create a new table structure. The table will grow automatically as needed,
 * but you are required to provide a starting size.
 *
 * @param[in] size The starting size.
 * @return Newly created table instance, or NULL on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_table_create(mut size: size_t) -> *mut htp_table_t {
    if size == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut htp_table_t;
    }
    let mut table: *mut htp_table_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<htp_table_t>() as libc::c_ulong,
    ) as *mut htp_table_t;
    if table.is_null() {
        return 0 as *mut htp_table_t;
    }
    (*table).alloc_type = htp_table_alloc_t::HTP_TABLE_KEYS_ALLOC_UKNOWN;
    // Use a list behind the scenes.
    if htp_list_array_init(
        &mut (*table).list,
        size.wrapping_mul(2 as libc::c_int as libc::c_ulong),
    ) == -(1 as libc::c_int)
    {
        free(table as *mut libc::c_void);
        return 0 as *mut htp_table_t;
    }
    return table;
}

/**
 * Destroy a table. This function handles the keys according to the active
 * allocation strategy. If the elements need freeing, you need to free them
 * before invoking this function. After the table has been destroyed,
 * the pointer is set to NULL.
 *
 * @param[in]   table
 */
#[no_mangle]
pub unsafe extern "C" fn htp_table_destroy(mut table: *mut htp_table_t) {
    if table.is_null() {
        return;
    }
    htp_table_clear(table);
    htp_list_array_release(&mut (*table).list);
    free(table as *mut libc::c_void);
}

/**
 * Destroy the given table, but don't free the keys. even if they are managed by
 * the table. Use this method when the responsibility for the keys has been transferred
 * elsewhere. After the table has been destroyed, the pointer is set to NULL.
 *
 * @param[in] table
 */
#[no_mangle]
pub unsafe extern "C" fn htp_table_destroy_ex(mut table: *mut htp_table_t) {
    if table.is_null() {
        return;
    }
    // Change allocation strategy in order to
    // prevent the keys from being freed.
    (*table).alloc_type = htp_table_alloc_t::HTP_TABLE_KEYS_REFERENCED;
    htp_table_destroy(table);
}

/**
 * Retrieve the first element that matches the given bstr key.
 *
 * @param[in] table
 * @param[in] key
 * @return Matched element, or NULL if no elements match the key.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_table_get(
    mut table: *const htp_table_t,
    mut key: *const bstr,
) -> *mut libc::c_void {
    if table.is_null() || key.is_null() {
        return 0 as *mut libc::c_void;
    }
    // Iterate through the list, comparing
    // keys with the parameter, return data if found.
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut n: size_t = htp_list_array_size(&(*table).list);
    while i < n {
        let mut key_candidate: *mut bstr = htp_list_array_get(&(*table).list, i) as *mut bstr;
        let mut element: *mut libc::c_void = htp_list_array_get(
            &(*table).list,
            i.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        if bstr_cmp_nocase(key_candidate, key) == 0 as libc::c_int {
            return element;
        }
        i = (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t
    }
    return 0 as *mut libc::c_void;
}

/**
 * Retrieve the first element that matches the given NUL-terminated key.
 *
 * @param[in] table
 * @param[in] ckey
 * @return Matched element, or NULL if no elements match the key.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_table_get_c(
    mut table: *const htp_table_t,
    mut ckey: *const libc::c_char,
) -> *mut libc::c_void {
    if table.is_null() || ckey.is_null() {
        return 0 as *mut libc::c_void;
    }
    // Iterate through the list, comparing
    // keys with the parameter, return data if found.
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut n: size_t = htp_list_array_size(&(*table).list);
    while i < n {
        let mut key_candidate: *mut bstr = htp_list_array_get(&(*table).list, i) as *mut bstr;
        let mut element: *mut libc::c_void = htp_list_array_get(
            &(*table).list,
            i.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        if bstr_cmp_c_nocasenorzero(key_candidate, ckey) == 0 as libc::c_int {
            return element;
        }
        i = (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t
    }
    return 0 as *mut libc::c_void;
}

/**
 * Retrieve key and element at the given index.
 *
 * @param[in] table
 * @param[in] idx
 * @param[in,out] key Pointer in which the key will be returned. Can be NULL.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_table_get_index(
    mut table: *const htp_table_t,
    mut idx: size_t,
    mut key: *mut *mut bstr,
) -> *mut libc::c_void {
    if table.is_null() {
        return 0 as *mut libc::c_void;
    }
    if idx >= htp_list_array_size(&(*table).list) {
        return 0 as *mut libc::c_void;
    }
    if !key.is_null() {
        *key = htp_list_array_get(
            &(*table).list,
            idx.wrapping_mul(2 as libc::c_int as libc::c_ulong),
        ) as *mut bstr
    }
    return htp_list_array_get(
        &(*table).list,
        idx.wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
}

/**
 * Retrieve table key defined by the provided pointer and length.
 *
 * @param[in] table
 * @param[in] key
 * @param[in] key_len
 * @return Matched element, or NULL if no elements match the key.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_table_get_mem(
    mut table: *const htp_table_t,
    mut key: *const libc::c_void,
    mut key_len: size_t,
) -> *mut libc::c_void {
    if table.is_null() || key == 0 as *mut libc::c_void {
        return 0 as *mut libc::c_void;
    }
    // Iterate through the list, comparing
    // keys with the parameter, return data if found.
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut n: size_t = htp_list_array_size(&(*table).list);
    while i < n {
        let mut key_candidate: *mut bstr = htp_list_array_get(&(*table).list, i) as *mut bstr;
        let mut element: *mut libc::c_void = htp_list_array_get(
            &(*table).list,
            i.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        if bstr_cmp_mem_nocase(key_candidate, key, key_len) == 0 as libc::c_int {
            return element;
        }
        i = (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t
    }
    return 0 as *mut libc::c_void;
}

/* *
 * Return the size of the table.
 *
 * @param[in] table
 * @return table size
 */
#[no_mangle]
pub unsafe extern "C" fn htp_table_size(mut table: *const htp_table_t) -> size_t {
    if table.is_null() {
        return 0 as libc::c_int as size_t;
    }
    return htp_list_array_size(&(*table).list).wrapping_div(2 as libc::c_int as libc::c_ulong);
}
