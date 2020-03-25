use ::libc;
extern "C" {
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    // Functions
    /* *
     * Create new array-backed list.
     *
     * @param[in] size
     * @return Newly created list.
     */
    #[no_mangle]
    fn htp_list_array_create(size: size_t) -> *mut htp_list_array_t;
    /* *
     * Free the memory occupied by this list. This function assumes
     * the elements held by the list were freed beforehand.
     *
     * @param[in] l
     */
    #[no_mangle]
    fn htp_list_array_destroy(l: *mut htp_list_array_t);
    /* *
     * Find the element at the given index.
     *
     * @param[in] l
     * @param[in] idx
     * @return the desired element, or NULL if the list is too small, or
     *         if the element at that position carries a NULL
     */
    #[no_mangle]
    fn htp_list_array_get(l: *const htp_list_array_t, idx: size_t) -> *mut libc::c_void;
    /* *
     * Add new element to the end of the list, expanding the list as necessary.
     *
     * @param[in] l
     * @param[in] e
     * @return HTP_OK on success or HTP_ERROR on failure.
     *
     */
    #[no_mangle]
    fn htp_list_array_push(l: *mut htp_list_array_t, e: *mut libc::c_void) -> htp_status_t;
    /* *
     * Returns the size of the list.
     *
     * @param[in] l
     * @return List size.
     */
    #[no_mangle]
    fn htp_list_array_size(l: *const htp_list_array_t) -> size_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_hook_t {
    pub callbacks: *mut htp_list_array_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_list_array_t {
    pub first: size_t,
    pub last: size_t,
    pub max_size: size_t,
    pub current_size: size_t,
    pub elements: *mut *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct htp_callback_t {
    pub fn_0: htp_callback_fn_t,
}
pub type htp_callback_fn_t = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>;
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
#[no_mangle]
pub unsafe extern "C" fn htp_hook_copy(mut hook: *const htp_hook_t) -> *mut htp_hook_t {
    if hook.is_null() {
        return 0 as *mut htp_hook_t;
    }
    let mut copy: *mut htp_hook_t = htp_hook_create();
    if copy.is_null() {
        return 0 as *mut htp_hook_t;
    }
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut n: size_t = htp_list_array_size((*hook).callbacks);
    while i < n {
        let mut callback: *mut htp_callback_t =
            htp_list_array_get((*hook).callbacks, i) as *mut htp_callback_t;
        if htp_hook_register(&mut copy, (*callback).fn_0) != 1 as libc::c_int {
            htp_hook_destroy(copy);
            return 0 as *mut htp_hook_t;
        }
        i = i.wrapping_add(1)
    }
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn htp_hook_create() -> *mut htp_hook_t {
    let mut hook: *mut htp_hook_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<htp_hook_t>() as libc::c_ulong,
    ) as *mut htp_hook_t;
    if hook.is_null() {
        return 0 as *mut htp_hook_t;
    }
    (*hook).callbacks = htp_list_array_create(4 as libc::c_int as size_t);
    if (*hook).callbacks.is_null() {
        free(hook as *mut libc::c_void);
        return 0 as *mut htp_hook_t;
    }
    return hook;
}
#[no_mangle]
pub unsafe extern "C" fn htp_hook_destroy(mut hook: *mut htp_hook_t) {
    if hook.is_null() {
        return;
    }
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut n: size_t = htp_list_array_size((*hook).callbacks);
    while i < n {
        free(htp_list_array_get((*hook).callbacks, i) as *mut htp_callback_t as *mut libc::c_void);
        i = i.wrapping_add(1)
    }
    htp_list_array_destroy((*hook).callbacks);
    free(hook as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn htp_hook_register(
    mut hook: *mut *mut htp_hook_t,
    callback_fn: htp_callback_fn_t,
) -> htp_status_t {
    if hook.is_null() {
        return -(1 as libc::c_int);
    }
    let mut callback: *mut htp_callback_t = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<htp_callback_t>() as libc::c_ulong,
    ) as *mut htp_callback_t;
    if callback.is_null() {
        return -(1 as libc::c_int);
    }
    (*callback).fn_0 = callback_fn;
    // Create a new hook if one does not exist
    let mut hook_created: libc::c_int = 0 as libc::c_int;
    if (*hook).is_null() {
        hook_created = 1 as libc::c_int;
        *hook = htp_hook_create();
        if (*hook).is_null() {
            free(callback as *mut libc::c_void);
            return -(1 as libc::c_int);
        }
    }
    // Add callback
    if htp_list_array_push((**hook).callbacks, callback as *mut libc::c_void) != 1 as libc::c_int {
        if hook_created != 0 {
            free(*hook as *mut libc::c_void);
        }
        free(callback as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn htp_hook_run_all(
    mut hook: *mut htp_hook_t,
    mut user_data: *mut libc::c_void,
) -> htp_status_t {
    if hook.is_null() {
        return 1 as libc::c_int;
    }
    // Loop through the registered callbacks, giving each a chance to run.
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut n: size_t = htp_list_array_size((*hook).callbacks);
    while i < n {
        let mut callback: *mut htp_callback_t =
            htp_list_array_get((*hook).callbacks, i) as *mut htp_callback_t;
        let mut rc: htp_status_t = (*callback).fn_0.expect("non-null function pointer")(user_data);
        // A hook can return HTP_OK to say that it did some work,
        // or HTP_DECLINED to say that it did no work. Anything else
        // is treated as an error.
        if rc != 1 as libc::c_int && rc != 0 as libc::c_int {
            return rc;
        }
        i = i.wrapping_add(1)
    }
    return 1 as libc::c_int;
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
 * Creates a copy of the provided hook. The hook is allowed to be NULL,
 * in which case this function simply returns a NULL.
 *
 * @param[in] hook
 * @return A copy of the hook, or NULL (if the provided hook was NULL
 *         or, if it wasn't, if there was a memory allocation problem while
 *         constructing a copy).
 */
/* *
 * Creates a new hook.
 *
 * @return New htp_hook_t structure on success, NULL on failure.
 */
/* *
 * Destroys an existing hook. It is all right to send a NULL
 * to this method because it will simply return straight away.
 *
 * @param[in] hook
 */
/* *
 * Registers a new callback with the hook.
 *
 * @param[in] hook
 * @param[in] callback_fn
 * @return HTP_OK on success, HTP_ERROR on memory allocation error.
 */
/* *
 * Runs all the callbacks associated with a given hook. Only stops if
 * one of the callbacks returns an error (HTP_ERROR) or stop (HTP_STOP).
 *
 * @param[in] hook
 * @param[in] user_data
 * @return HTP_OK if at least one hook ran successfully, HTP_STOP if there was
 *         no error but processing should stop, and HTP_ERROR or any other value
 *         less than zero on error.
 */
/* *
 * Run callbacks one by one until one of them accepts to service the hook.
 *
 * @param[in] hook
 * @param[in] user_data
 * @return HTP_OK if a hook was found to process the callback, HTP_DECLINED if
 *         no hook could be found, HTP_STOP if a hook signalled the processing
 *         to stop, and HTP_ERROR or any other value less than zero on error.
 */
#[no_mangle]
pub unsafe extern "C" fn htp_hook_run_one(
    mut hook: *mut htp_hook_t,
    mut user_data: *mut libc::c_void,
) -> htp_status_t {
    if hook.is_null() {
        return 0 as libc::c_int;
    }
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut n: size_t = htp_list_array_size((*hook).callbacks);
    while i < n {
        let mut callback: *mut htp_callback_t =
            htp_list_array_get((*hook).callbacks, i) as *mut htp_callback_t;
        let mut rc: htp_status_t = (*callback).fn_0.expect("non-null function pointer")(user_data);
        // A hook can return HTP_DECLINED to say that it did no work,
        // and we'll ignore that. If we see HTP_OK or anything else,
        // we stop processing (because it was either a successful
        // handling or an error).
        if rc != 0 as libc::c_int {
            // Return HTP_OK or an error.
            return rc;
        }
        i = i.wrapping_add(1)
    }
    // No hook wanted to process the callback.
    return 0 as libc::c_int;
}
