use ::libc;
extern "C" {
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    /* *
 * Initialize an array-backed list.
 *
 * @param[in] l
 * @param[in] size
 * @return HTP_OK or HTP_ERROR if allocation failed
 */
    #[no_mangle]
    fn htp_list_array_init(l: *mut htp_list_array_t, size: size_t)
     -> htp_status_t;
    /* *
 * Remove all elements from the list. It is the responsibility of the caller
 * to iterate over list elements and deallocate them if necessary, prior to
 * invoking this function.
 *
 * @param[in] l
 */
    #[no_mangle]
    fn htp_list_array_clear(l: *mut htp_list_array_t);
    /* *
 * Free the memory occupied by this list, except itself.
 * This function assumes the elements held by the list
 * were freed beforehand.
 *
 * @param[in] l
 */
    #[no_mangle]
    fn htp_list_array_release(l: *mut htp_list_array_t);
    /* *
 * Find the element at the given index.
 *
 * @param[in] l
 * @param[in] idx
 * @return the desired element, or NULL if the list is too small, or
 *         if the element at that position carries a NULL
 */
    #[no_mangle]
    fn htp_list_array_get(l: *const htp_list_array_t, idx: size_t)
     -> *mut libc::c_void;
    /* *
 * Remove one element from the end of the list.
 *
 * @param[in] l
 * @return The removed element, or NULL if the list is empty.
 */
    #[no_mangle]
    fn htp_list_array_pop(l: *mut htp_list_array_t) -> *mut libc::c_void;
    /* *
 * Add new element to the end of the list, expanding the list as necessary.
 *
 * @param[in] l
 * @param[in] e
 * @return HTP_OK on success or HTP_ERROR on failure.
 *
 */
    #[no_mangle]
    fn htp_list_array_push(l: *mut htp_list_array_t, e: *mut libc::c_void)
     -> htp_status_t;
    /* *
 * Returns the size of the list.
 *
 * @param[in] l
 * @return List size.
 */
    #[no_mangle]
    fn htp_list_array_size(l: *const htp_list_array_t) -> size_t;
    /* *
 * Case-insensitive zero-skipping comparison of a bstring with a NUL-terminated string.
 *
 * @param[in] b
 * @param[in] cstr
 * @return Zero on string match, 1 if b is greater than cstr, and -1 if cstr is greater than b.
 */
    #[no_mangle]
    fn bstr_cmp_c_nocasenorzero(b: *const bstr, cstr: *const libc::c_char)
     -> libc::c_int;
    /* *
 * Performs a case-insensitive comparison of a bstring with a memory region.
 *
 * @param[in] b
 * @param[in] data
 * @param[in] len
 * @return Zero ona match, 1 if b is greater than data, and -1 if data is greater than b.
 */
    #[no_mangle]
    fn bstr_cmp_mem_nocase(b: *const bstr, data: *const libc::c_void,
                           len: size_t) -> libc::c_int;
    /* *
 * Case-insensitive comparison two bstrings.
 *
 * @param[in] b1
 * @param[in] b2
 * @return Zero on string match, 1 if b1 is greater than b2, and -1 if b2 is
 *         greater than b1.
 */
    #[no_mangle]
    fn bstr_cmp_nocase(b1: *const bstr, b2: *const bstr) -> libc::c_int;
    /* *
 * Create a new bstring by copying the provided bstring.
 *
 * @param[in] b
 * @return New bstring, or NULL if memory allocation failed.
 */
    #[no_mangle]
    fn bstr_dup(b: *const bstr) -> *mut bstr;
    /* *
 * Deallocate the supplied bstring instance and set it to NULL. Allows NULL on
 * input.
 *
 * @param[in] b
 */
    #[no_mangle]
    fn bstr_free(b: *mut bstr);
}
pub type size_t = libc::c_ulong;
/* **************************************************************************
 * Copyright (c) 2009-2010 Open Information Security Foundation
 * Copyright (c) 2010-2013 Qualys, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.

 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.

 * - Neither the name of the Qualys, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 ***************************************************************************/
/* *
 * @file
 * @author Ivan Ristic <ivanr@webkreator.com>
 */
pub type htp_status_t = libc::c_int;
/* **************************************************************************
 * Copyright (c) 2009-2010 Open Information Security Foundation
 * Copyright (c) 2010-2013 Qualys, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.

 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.

 * - Neither the name of the Qualys, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 ***************************************************************************/
/* *
 * @file
 * @author Ivan Ristic <ivanr@webkreator.com>
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_list_array_t {
    pub first: size_t,
    pub last: size_t,
    pub max_size: size_t,
    pub current_size: size_t,
    pub elements: *mut *mut libc::c_void,
}
/* **************************************************************************
 * Copyright (c) 2009-2010 Open Information Security Foundation
 * Copyright (c) 2010-2013 Qualys, Inc.
 * All rights reserved.
 * 
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 * 
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.

 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.

 * - Neither the name of the Qualys, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 * 
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 ***************************************************************************/
/* *
 * @file
 * @author Ivan Ristic <ivanr@webkreator.com>
 */
pub type bstr = bstr_t;
// Data structures
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bstr_t {
    pub len: size_t,
    pub size: size_t,
    pub realptr: *mut libc::c_uchar,
}
/* **************************************************************************
 * Copyright (c) 2009-2010 Open Information Security Foundation
 * Copyright (c) 2010-2013 Qualys, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.

 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.

 * - Neither the name of the Qualys, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 ***************************************************************************/
/* *
 * @file
 * @author Ivan Ristic <ivanr@webkreator.com>
 */
/* * This is the default value, used only until the first element is added. */
/* * Keys are copied.*/
/* * Keys are adopted and freed when the table is destroyed. */
/* * Keys are only referenced; the caller is still responsible for freeing them after the table is destroyed. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_table_t {
    pub list: htp_list_array_t,
    pub alloc_type: htp_table_alloc_t,
}
pub type htp_table_alloc_t = libc::c_uint;
pub const HTP_TABLE_KEYS_REFERENCED: htp_table_alloc_t = 3;
pub const HTP_TABLE_KEYS_ADOPTED: htp_table_alloc_t = 2;
pub const HTP_TABLE_KEYS_COPIED: htp_table_alloc_t = 1;
pub const HTP_TABLE_KEYS_ALLOC_UKNOWN: htp_table_alloc_t = 0;
/* **************************************************************************
 * Copyright (c) 2009-2010 Open Information Security Foundation
 * Copyright (c) 2010-2013 Qualys, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.

 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.

 * - Neither the name of the Qualys, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 ***************************************************************************/
/* *
 * @file
 * @author Ivan Ristic <ivanr@webkreator.com>
 */
unsafe extern "C" fn _htp_table_add(mut table: *mut htp_table_t,
                                    mut key: *const bstr,
                                    mut element: *const libc::c_void)
 -> htp_status_t {
    // Add key.
    if htp_list_array_push(&mut (*table).list, key as *mut libc::c_void) !=
           1 as libc::c_int {
        return -(1 as libc::c_int)
    }
    // Add element.
    if htp_list_array_push(&mut (*table).list, element as *mut libc::c_void)
           != 1 as libc::c_int {
        htp_list_array_pop(&mut (*table).list);
        return -(1 as libc::c_int)
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn htp_table_add(mut table: *mut htp_table_t,
                                       mut key: *const bstr,
                                       mut element: *const libc::c_void)
 -> htp_status_t {
    if table.is_null() || key.is_null() { return -(1 as libc::c_int) }
    // Keep track of how keys are allocated, and
    // ensure that all invocations are consistent.
    if (*table).alloc_type as libc::c_uint ==
           HTP_TABLE_KEYS_ALLOC_UKNOWN as libc::c_int as libc::c_uint {
        (*table).alloc_type = HTP_TABLE_KEYS_COPIED
    } else if (*table).alloc_type as libc::c_uint !=
                  HTP_TABLE_KEYS_COPIED as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int)
    }
    let mut dupkey: *mut bstr = bstr_dup(key);
    if dupkey.is_null() { return -(1 as libc::c_int) }
    if _htp_table_add(table, dupkey, element) != 1 as libc::c_int {
        bstr_free(dupkey);
        return -(1 as libc::c_int)
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn htp_table_addn(mut table: *mut htp_table_t,
                                        mut key: *const bstr,
                                        mut element: *const libc::c_void)
 -> htp_status_t {
    if table.is_null() || key.is_null() { return -(1 as libc::c_int) }
    // Keep track of how keys are allocated, and
    // ensure that all invocations are consistent.
    if (*table).alloc_type as libc::c_uint ==
           HTP_TABLE_KEYS_ALLOC_UKNOWN as libc::c_int as libc::c_uint {
        (*table).alloc_type = HTP_TABLE_KEYS_ADOPTED
    } else if (*table).alloc_type as libc::c_uint !=
                  HTP_TABLE_KEYS_ADOPTED as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int)
    }
    return _htp_table_add(table, key, element);
}
#[no_mangle]
pub unsafe extern "C" fn htp_table_addk(mut table: *mut htp_table_t,
                                        mut key: *const bstr,
                                        mut element: *const libc::c_void)
 -> htp_status_t {
    if table.is_null() || key.is_null() { return -(1 as libc::c_int) }
    // Keep track of how keys are allocated, and
    // ensure that all invocations are consistent.
    if (*table).alloc_type as libc::c_uint ==
           HTP_TABLE_KEYS_ALLOC_UKNOWN as libc::c_int as libc::c_uint {
        (*table).alloc_type = HTP_TABLE_KEYS_REFERENCED
    } else if (*table).alloc_type as libc::c_uint !=
                  HTP_TABLE_KEYS_REFERENCED as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int)
    }
    return _htp_table_add(table, key, element);
}
#[no_mangle]
pub unsafe extern "C" fn htp_table_clear(mut table: *mut htp_table_t) {
    if table.is_null() { return }
    // Free the table keys, but only if we're managing them.
    if (*table).alloc_type as libc::c_uint ==
           HTP_TABLE_KEYS_COPIED as libc::c_int as libc::c_uint ||
           (*table).alloc_type as libc::c_uint ==
               HTP_TABLE_KEYS_ADOPTED as libc::c_int as libc::c_uint {
        let mut key: *mut bstr = 0 as *mut bstr;
        let mut i: size_t = 0 as libc::c_int as size_t;
        let mut n: size_t = htp_list_array_size(&mut (*table).list);
        while i < n {
            key = htp_list_array_get(&mut (*table).list, i) as *mut bstr;
            bstr_free(key);
            i =
                (i as
                     libc::c_ulong).wrapping_add(2 as libc::c_int as
                                                     libc::c_ulong) as size_t
                    as size_t
        }
    }
    htp_list_array_clear(&mut (*table).list);
}
#[no_mangle]
pub unsafe extern "C" fn htp_table_clear_ex(mut table: *mut htp_table_t) {
    if table.is_null() { return }
    // This function does not free table keys.
    htp_list_array_clear(&mut (*table).list);
}
#[no_mangle]
pub unsafe extern "C" fn htp_table_create(mut size: size_t)
 -> *mut htp_table_t {
    if size == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut htp_table_t
    }
    let mut table: *mut htp_table_t =
        calloc(1 as libc::c_int as libc::c_ulong,
               ::std::mem::size_of::<htp_table_t>() as libc::c_ulong) as
            *mut htp_table_t;
    if table.is_null() { return 0 as *mut htp_table_t }
    (*table).alloc_type = HTP_TABLE_KEYS_ALLOC_UKNOWN;
    // Use a list behind the scenes.
    if htp_list_array_init(&mut (*table).list,
                           size.wrapping_mul(2 as libc::c_int as
                                                 libc::c_ulong)) ==
           -(1 as libc::c_int) {
        free(table as *mut libc::c_void);
        return 0 as *mut htp_table_t
    }
    return table;
}
#[no_mangle]
pub unsafe extern "C" fn htp_table_destroy(mut table: *mut htp_table_t) {
    if table.is_null() { return }
    htp_table_clear(table);
    htp_list_array_release(&mut (*table).list);
    free(table as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn htp_table_destroy_ex(mut table: *mut htp_table_t) {
    if table.is_null() { return }
    // Change allocation strategy in order to
    // prevent the keys from being freed.
    (*table).alloc_type = HTP_TABLE_KEYS_REFERENCED;
    htp_table_destroy(table);
}
#[no_mangle]
pub unsafe extern "C" fn htp_table_get(mut table: *const htp_table_t,
                                       mut key: *const bstr)
 -> *mut libc::c_void {
    if table.is_null() || key.is_null() { return 0 as *mut libc::c_void }
    // Iterate through the list, comparing
    // keys with the parameter, return data if found.    
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut n: size_t = htp_list_array_size(&(*table).list);
    while i < n {
        let mut key_candidate: *mut bstr =
            htp_list_array_get(&(*table).list, i) as *mut bstr;
        let mut element: *mut libc::c_void =
            htp_list_array_get(&(*table).list,
                               i.wrapping_add(1 as libc::c_int as
                                                  libc::c_ulong));
        if bstr_cmp_nocase(key_candidate, key) == 0 as libc::c_int {
            return element
        }
        i =
            (i as
                 libc::c_ulong).wrapping_add(2 as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn htp_table_get_c(mut table: *const htp_table_t,
                                         mut ckey: *const libc::c_char)
 -> *mut libc::c_void {
    if table.is_null() || ckey.is_null() { return 0 as *mut libc::c_void }
    // Iterate through the list, comparing
    // keys with the parameter, return data if found.    
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut n: size_t = htp_list_array_size(&(*table).list);
    while i < n {
        let mut key_candidate: *mut bstr =
            htp_list_array_get(&(*table).list, i) as *mut bstr;
        let mut element: *mut libc::c_void =
            htp_list_array_get(&(*table).list,
                               i.wrapping_add(1 as libc::c_int as
                                                  libc::c_ulong));
        if bstr_cmp_c_nocasenorzero(key_candidate, ckey) == 0 as libc::c_int {
            return element
        }
        i =
            (i as
                 libc::c_ulong).wrapping_add(2 as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn htp_table_get_index(mut table: *const htp_table_t,
                                             mut idx: size_t,
                                             mut key: *mut *mut bstr)
 -> *mut libc::c_void {
    if table.is_null() { return 0 as *mut libc::c_void }
    if idx >= htp_list_array_size(&(*table).list) {
        return 0 as *mut libc::c_void
    }
    if !key.is_null() {
        *key =
            htp_list_array_get(&(*table).list,
                               idx.wrapping_mul(2 as libc::c_int as
                                                    libc::c_ulong)) as
                *mut bstr
    }
    return htp_list_array_get(&(*table).list,
                              idx.wrapping_mul(2 as libc::c_int as
                                                   libc::c_ulong).wrapping_add(1
                                                                                   as
                                                                                   libc::c_int
                                                                                   as
                                                                                   libc::c_ulong));
}
#[no_mangle]
pub unsafe extern "C" fn htp_table_get_mem(mut table: *const htp_table_t,
                                           mut key: *const libc::c_void,
                                           mut key_len: size_t)
 -> *mut libc::c_void {
    if table.is_null() || key == 0 as *mut libc::c_void {
        return 0 as *mut libc::c_void
    }
    // Iterate through the list, comparing
    // keys with the parameter, return data if found.
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut n: size_t = htp_list_array_size(&(*table).list);
    while i < n {
        let mut key_candidate: *mut bstr =
            htp_list_array_get(&(*table).list, i) as *mut bstr;
        let mut element: *mut libc::c_void =
            htp_list_array_get(&(*table).list,
                               i.wrapping_add(1 as libc::c_int as
                                                  libc::c_ulong));
        if bstr_cmp_mem_nocase(key_candidate, key, key_len) ==
               0 as libc::c_int {
            return element
        }
        i =
            (i as
                 libc::c_ulong).wrapping_add(2 as libc::c_int as
                                                 libc::c_ulong) as size_t as
                size_t
    }
    return 0 as *mut libc::c_void;
}
/* **************************************************************************
 * Copyright (c) 2009-2010 Open Information Security Foundation
 * Copyright (c) 2010-2013 Qualys, Inc.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 * - Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.

 * - Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in the
 *   documentation and/or other materials provided with the distribution.

 * - Neither the name of the Qualys, Inc. nor the names of its
 *   contributors may be used to endorse or promote products derived from
 *   this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 ***************************************************************************/
/* *
 * @file
 * @author Ivan Ristic <ivanr@webkreator.com>
 */
/* *
 * Add a new element to the table. The key will be copied, and the copy
 * managed by the table. The table keeps a pointer to the element. It is the
 * callers responsibility to ensure the pointer remains valid.
 *
 * @param[in] table
 * @param[in] key
 * @param[in] element
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
/* *
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
/* *
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
/* *
 * Remove all elements from the table. This function handles keys
 * according to the active allocation strategy. If the elements need freeing,
 * you need to free them before invoking this function.
 *
 * @param[in] table
 */
/* *
 * Remove all elements from the table without freeing any of the keys, even
 * if the table is using an allocation strategy where keys belong to it. This
 * function is useful if all the keys have been adopted by some other structure.
 *
 * @param[in] table
 */
/* *
 * Create a new table structure. The table will grow automatically as needed,
 * but you are required to provide a starting size.
 *
 * @param[in] size The starting size.
 * @return Newly created table instance, or NULL on failure.
 */
/* *
 * Destroy a table. This function handles the keys according to the active
 * allocation strategy. If the elements need freeing, you need to free them
 * before invoking this function. After the table has been destroyed,
 * the pointer is set to NULL.
 *
 * @param[in]   table
 */
/* *
 * Destroy the given table, but don't free the keys. even if they are managed by
 * the table. Use this method when the responsibility for the keys has been transferred
 * elsewhere. After the table has been destroyed, the pointer is set to NULL.
 *
 * @param[in] table
 */
/* *
 * Retrieve the first element that matches the given bstr key.
 *
 * @param[in] table
 * @param[in] key
 * @return Matched element, or NULL if no elements match the key.
 */
/* *
 * Retrieve the first element that matches the given NUL-terminated key.
 *
 * @param[in] table
 * @param[in] ckey
 * @return Matched element, or NULL if no elements match the key.
 */
/* *
 * Retrieve key and element at the given index.
 *
 * @param[in] table
 * @param[in] idx
 * @param[in,out] key Pointer in which the key will be returned. Can be NULL.
 * @return HTP_OK on success, HTP_ERROR on failure.
 */
/* *
 * Retrieve table key defined by the provided pointer and length.
 *
 * @param[in] table
 * @param[in] key
 * @param[in] key_len
 * @return Matched element, or NULL if no elements match the key.
 */
/* *
 * Return the size of the table.
 *
 * @param[in] table
 * @return table size
 */
#[no_mangle]
pub unsafe extern "C" fn htp_table_size(mut table: *const htp_table_t)
 -> size_t {
    if table.is_null() { return 0 as libc::c_int as size_t }
    return htp_list_array_size(&(*table).list).wrapping_div(2 as libc::c_int
                                                                as
                                                                libc::c_ulong);
}
