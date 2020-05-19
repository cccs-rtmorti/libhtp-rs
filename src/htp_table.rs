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
#[derive(Copy, Clone, PartialEq, Debug)]
enum htp_table_alloc_t {
    /// This is the default value, used only until the first element is added.
    HTP_TABLE_KEYS_ALLOC_UKNOWN,
    /// Keys are copied.
    HTP_TABLE_KEYS_COPIED,
    /// Keys are adopted and freed when the table is destroyed.
    HTP_TABLE_KEYS_ADOPTED,
    /// Keys are only referenced; the caller is still responsible for freeing them after the table is destroyed.
    HTP_TABLE_KEYS_REFERENCED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htp_table_t {
    /// Table key and value pairs are stored in this list; name first, then value.
    pub list: htp_list::htp_list_array_t,
    /// Key management strategy. Initially set to HTP_TABLE_KEYS_ALLOC_UKNOWN. The
    /// actual strategy is determined by the first allocation.
    alloc_type: htp_table_alloc_t,
}

unsafe fn _htp_table_add(
    mut table: *mut htp_table_t,
    mut key: *const bstr::bstr_t,
    mut element: *const libc::c_void,
) -> Status {
    // Add key.
    if htp_list::htp_list_array_push(&mut (*table).list, key as *mut libc::c_void) != Status::OK {
        return Status::ERROR;
    }
    // Add element.
    if htp_list::htp_list_array_push(&mut (*table).list, element as *mut libc::c_void) != Status::OK
    {
        htp_list::htp_list_array_pop(&mut (*table).list);
        return Status::ERROR;
    }
    return Status::OK;
}

/// Add a new element to the table. The key will be copied, and the copy
/// managed by the table. The table keeps a pointer to the element. It is the
/// callers responsibility to ensure the pointer remains valid.
///
/// Returns HTP_OK on success, HTP_ERROR on failure.
pub unsafe fn htp_table_add(
    mut table: *mut htp_table_t,
    mut key: *const bstr::bstr_t,
    mut element: *const libc::c_void,
) -> Status {
    if table.is_null() || key.is_null() {
        return Status::ERROR;
    }
    // Keep track of how keys are allocated, and
    // ensure that all invocations are consistent.
    if (*table).alloc_type == htp_table_alloc_t::HTP_TABLE_KEYS_ALLOC_UKNOWN {
        (*table).alloc_type = htp_table_alloc_t::HTP_TABLE_KEYS_COPIED
    } else if (*table).alloc_type != htp_table_alloc_t::HTP_TABLE_KEYS_COPIED {
        return Status::ERROR;
    }
    let mut dupkey: *mut bstr::bstr = bstr::bstr_dup(key);
    if dupkey.is_null() {
        return Status::ERROR;
    }
    if _htp_table_add(table, dupkey, element) != Status::OK {
        bstr::bstr_free(dupkey);
        return Status::ERROR;
    }
    return Status::OK;
}

/// Add a new element to the table. The key provided will be adopted and managed
/// by the table. You should not keep a copy of the pointer to the key unless you're
/// certain that the table will live longer that the copy. The table keeps a pointer
/// to the element. It is the callers responsibility to ensure the pointer remains
/// valid.
///
/// Returns HTP_OK on success, HTP_ERROR on failure.
pub unsafe fn htp_table_addn(
    mut table: *mut htp_table_t,
    mut key: *const bstr::bstr_t,
    mut element: *const libc::c_void,
) -> Status {
    if table.is_null() || key.is_null() {
        return Status::ERROR;
    }
    // Keep track of how keys are allocated, and
    // ensure that all invocations are consistent.
    if (*table).alloc_type == htp_table_alloc_t::HTP_TABLE_KEYS_ALLOC_UKNOWN {
        (*table).alloc_type = htp_table_alloc_t::HTP_TABLE_KEYS_ADOPTED
    } else if (*table).alloc_type != htp_table_alloc_t::HTP_TABLE_KEYS_ADOPTED {
        return Status::ERROR;
    }
    return _htp_table_add(table, key, element);
}

/// Add a new element to the table. The key provided will be only referenced and the
/// caller remains responsible to keep it alive until after the table is destroyed. The
/// table keeps a pointer to the element. It is the callers responsibility to ensure
/// the pointer remains valid.
///
/// Returns HTP_OK on success, HTP_ERROR on failure.
pub unsafe fn htp_table_addk(
    mut table: *mut htp_table_t,
    mut key: *const bstr::bstr_t,
    mut element: *const libc::c_void,
) -> Status {
    if table.is_null() || key.is_null() {
        return Status::ERROR;
    }
    // Keep track of how keys are allocated, and
    // ensure that all invocations are consistent.
    if (*table).alloc_type == htp_table_alloc_t::HTP_TABLE_KEYS_ALLOC_UKNOWN {
        (*table).alloc_type = htp_table_alloc_t::HTP_TABLE_KEYS_REFERENCED
    } else if (*table).alloc_type != htp_table_alloc_t::HTP_TABLE_KEYS_REFERENCED {
        return Status::ERROR;
    }
    return _htp_table_add(table, key, element);
}

/// Remove all elements from the table. This function handles keys
/// according to the active allocation strategy. If the elements need freeing,
/// you need to free them before invoking this function.
pub unsafe fn htp_table_clear(mut table: *mut htp_table_t) {
    if table.is_null() {
        return;
    }
    // Free the table keys, but only if we're managing them.
    if (*table).alloc_type == htp_table_alloc_t::HTP_TABLE_KEYS_COPIED
        || (*table).alloc_type == htp_table_alloc_t::HTP_TABLE_KEYS_ADOPTED
    {
        let mut key: *mut bstr::bstr_t = 0 as *mut bstr::bstr_t;
        let mut i: size_t = 0 as libc::c_int as size_t;
        let mut n: size_t = htp_list::htp_list_array_size(&mut (*table).list);
        while i < n {
            key = htp_list::htp_list_array_get(&mut (*table).list, i) as *mut bstr::bstr;
            bstr::bstr_free(key);
            i = (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                as size_t
        }
    }
    htp_list::htp_list_array_clear(&mut (*table).list);
}

/// Create a new table structure. The table will grow automatically as needed,
/// but you are required to provide a starting size.
///
/// size: The starting size.
///
/// Returns Newly created table instance, or NULL on failure.
pub unsafe fn htp_table_create(mut size: size_t) -> *mut htp_table_t {
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
    if htp_list::htp_list_array_init(
        &mut (*table).list,
        size.wrapping_mul(2 as libc::c_int as libc::c_ulong),
    ) == Status::ERROR
    {
        free(table as *mut libc::c_void);
        return 0 as *mut htp_table_t;
    }
    return table;
}

/// Destroy a table. This function handles the keys according to the active
/// allocation strategy. If the elements need freeing, you need to free them
/// before invoking this function. After the table has been destroyed,
/// the pointer is set to NULL.
pub unsafe fn htp_table_destroy(mut table: *mut htp_table_t) {
    if table.is_null() {
        return;
    }
    htp_table_clear(table);
    htp_list::htp_list_array_release(&mut (*table).list);
    free(table as *mut libc::c_void);
}

/// Destroy the given table, but don't free the keys. even if they are managed by
/// the table. Use this method when the responsibility for the keys has been transferred
/// elsewhere. After the table has been destroyed, the pointer is set to NULL.
pub unsafe fn htp_table_destroy_ex(mut table: *mut htp_table_t) {
    if table.is_null() {
        return;
    }
    // Change allocation strategy in order to
    // prevent the keys from being freed.
    (*table).alloc_type = htp_table_alloc_t::HTP_TABLE_KEYS_REFERENCED;
    htp_table_destroy(table);
}

/// Retrieve the first element that matches the given bstr::bstr_t key.
///
/// Returns Matched element, or NULL if no elements match the key.
pub unsafe fn htp_table_get(
    mut table: *const htp_table_t,
    mut key: *const bstr::bstr_t,
) -> *mut libc::c_void {
    if table.is_null() || key.is_null() {
        return 0 as *mut libc::c_void;
    }
    // Iterate through the list, comparing
    // keys with the parameter, return data if found.
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut n: size_t = htp_list::htp_list_array_size(&(*table).list);
    while i < n {
        let mut key_candidate: *mut bstr::bstr =
            htp_list::htp_list_array_get(&(*table).list, i) as *mut bstr::bstr;
        let mut element: *mut libc::c_void = htp_list::htp_list_array_get(
            &(*table).list,
            i.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        if bstr::bstr_cmp_nocase(key_candidate, key) == 0 as libc::c_int {
            return element;
        }
        i = (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t
    }
    return 0 as *mut libc::c_void;
}

/// Retrieve the first element that matches the given NUL-terminated key.
pub unsafe fn htp_table_get_c(
    mut table: *const htp_table_t,
    mut ckey: *const libc::c_char,
) -> *mut libc::c_void {
    if table.is_null() || ckey.is_null() {
        return 0 as *mut libc::c_void;
    }
    // Iterate through the list, comparing
    // keys with the parameter, return data if found.
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut n: size_t = htp_list::htp_list_array_size(&(*table).list);
    while i < n {
        let mut key_candidate: *mut bstr::bstr =
            htp_list::htp_list_array_get(&(*table).list, i) as *mut bstr::bstr;
        let mut element: *mut libc::c_void = htp_list::htp_list_array_get(
            &(*table).list,
            i.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        if bstr::bstr_cmp_c_nocasenorzero(key_candidate, ckey) == 0 as libc::c_int {
            return element;
        }
        i = (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t
    }
    return 0 as *mut libc::c_void;
}

/// Retrieve key and element at the given index.
pub unsafe fn htp_table_get_index(
    mut table: *const htp_table_t,
    mut idx: size_t,
    mut key: *mut *mut bstr::bstr_t,
) -> *mut libc::c_void {
    if table.is_null() {
        return 0 as *mut libc::c_void;
    }
    if idx >= htp_list::htp_list_array_size(&(*table).list) {
        return 0 as *mut libc::c_void;
    }
    if !key.is_null() {
        *key = htp_list::htp_list_array_get(
            &(*table).list,
            idx.wrapping_mul(2 as libc::c_int as libc::c_ulong),
        ) as *mut bstr::bstr_t
    }
    return htp_list::htp_list_array_get(
        &(*table).list,
        idx.wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
}

/// Retrieve table key defined by the provided pointer and length.
///
/// Returns Matched element, or NULL if no elements match the key.
pub unsafe fn htp_table_get_mem(
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
    let mut n: size_t = htp_list::htp_list_array_size(&(*table).list);
    while i < n {
        let mut key_candidate: *mut bstr::bstr =
            htp_list::htp_list_array_get(&(*table).list, i) as *mut bstr::bstr;
        let mut element: *mut libc::c_void = htp_list::htp_list_array_get(
            &(*table).list,
            i.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        if bstr::bstr_cmp_mem_nocase(key_candidate, key, key_len) == 0 as libc::c_int {
            return element;
        }
        i = (i as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t as size_t
    }
    return 0 as *mut libc::c_void;
}

/// Return the size of the table.
pub unsafe fn htp_table_size(mut table: *const htp_table_t) -> size_t {
    if table.is_null() {
        return 0 as libc::c_int as size_t;
    }
    return htp_list::htp_list_array_size(&(*table).list)
        .wrapping_div(2 as libc::c_int as libc::c_ulong);
}
