use ::libc;
extern "C" {
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn toupper(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn tolower(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type __int64_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
// Data structures
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bstr_t {
    pub len: size_t,
    pub size: size_t,
    pub realptr: *mut libc::c_uchar,
}
pub type size_t = libc::c_ulong;
pub type bstr = bstr_t;
pub type int64_t = __int64_t;
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
pub unsafe extern "C" fn bstr_alloc(mut len: size_t) -> *mut bstr {
    let mut b: *mut bstr =
        malloc((::std::mem::size_of::<bstr>() as libc::c_ulong).wrapping_add(len)) as *mut bstr;
    if b.is_null() {
        return 0 as *mut bstr;
    }
    (*b).len = 0 as libc::c_int as size_t;
    (*b).size = len;
    (*b).realptr = 0 as *mut libc::c_uchar;
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn bstr_add(
    mut destination: *mut bstr,
    mut source: *const bstr,
) -> *mut bstr {
    return bstr_add_mem(
        destination,
        if (*source).realptr.is_null() {
            (source as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
        } else {
            (*source).realptr
        } as *const libc::c_void,
        (*source).len,
    );
}
#[no_mangle]
pub unsafe extern "C" fn bstr_add_c(
    mut bdestination: *mut bstr,
    mut csource: *const libc::c_char,
) -> *mut bstr {
    return bstr_add_mem(
        bdestination,
        csource as *const libc::c_void,
        strlen(csource),
    );
}
#[no_mangle]
pub unsafe extern "C" fn bstr_add_c_noex(
    mut destination: *mut bstr,
    mut source: *const libc::c_char,
) -> *mut bstr {
    return bstr_add_mem_noex(destination, source as *const libc::c_void, strlen(source));
}
#[no_mangle]
pub unsafe extern "C" fn bstr_add_mem(
    mut destination: *mut bstr,
    mut data: *const libc::c_void,
    mut len: size_t,
) -> *mut bstr {
    // Expand the destination if necessary
    if (*destination).size < (*destination).len.wrapping_add(len) {
        destination = bstr_expand(destination, (*destination).len.wrapping_add(len));
        if destination.is_null() {
            return 0 as *mut bstr;
        }
    }
    // Add source to destination
    let mut b: *mut bstr = destination;
    memcpy(
        (if (*destination).realptr.is_null() {
            (destination as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
        } else {
            (*destination).realptr
        })
        .offset((*b).len as isize) as *mut libc::c_void,
        data,
        len,
    );
    bstr_adjust_len(b, (*b).len.wrapping_add(len));
    return destination;
}
#[no_mangle]
pub unsafe extern "C" fn bstr_add_mem_noex(
    mut destination: *mut bstr,
    mut data: *const libc::c_void,
    mut len: size_t,
) -> *mut bstr {
    let mut copylen: size_t = len;
    // Is there enough room in the destination?
    if (*destination).size < (*destination).len.wrapping_add(copylen) {
        copylen = (*destination).size.wrapping_sub((*destination).len);
        if copylen <= 0 as libc::c_int as libc::c_ulong {
            return destination;
        }
    }
    // Copy over the bytes
    let mut b: *mut bstr = destination;
    memcpy(
        (if (*destination).realptr.is_null() {
            (destination as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
        } else {
            (*destination).realptr
        })
        .offset((*b).len as isize) as *mut libc::c_void,
        data,
        copylen,
    );
    bstr_adjust_len(b, (*b).len.wrapping_add(copylen));
    return destination;
}
#[no_mangle]
pub unsafe extern "C" fn bstr_add_noex(
    mut destination: *mut bstr,
    mut source: *const bstr,
) -> *mut bstr {
    return bstr_add_mem_noex(
        destination,
        if (*source).realptr.is_null() {
            (source as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
        } else {
            (*source).realptr
        } as *const libc::c_void,
        (*source).len,
    );
}
#[no_mangle]
pub unsafe extern "C" fn bstr_adjust_len(mut b: *mut bstr, mut newlen: size_t) {
    (*b).len = newlen;
}
#[no_mangle]
pub unsafe extern "C" fn bstr_adjust_realptr(mut b: *mut bstr, mut newrealptr: *mut libc::c_void) {
    (*b).realptr = newrealptr as *mut libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn bstr_adjust_size(mut b: *mut bstr, mut newsize: size_t) {
    (*b).size = newsize;
}
#[no_mangle]
pub unsafe extern "C" fn bstr_begins_with(
    mut haystack: *const bstr,
    mut needle: *const bstr,
) -> libc::c_int {
    return bstr_begins_with_mem(
        haystack,
        if (*needle).realptr.is_null() {
            (needle as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
        } else {
            (*needle).realptr
        } as *const libc::c_void,
        (*needle).len,
    );
}
#[no_mangle]
pub unsafe extern "C" fn bstr_begins_with_c(
    mut haystack: *const bstr,
    mut needle: *const libc::c_char,
) -> libc::c_int {
    return bstr_begins_with_mem(haystack, needle as *const libc::c_void, strlen(needle));
}
#[no_mangle]
pub unsafe extern "C" fn bstr_begins_with_c_nocase(
    mut haystack: *const bstr,
    mut needle: *const libc::c_char,
) -> libc::c_int {
    return bstr_begins_with_mem_nocase(haystack, needle as *const libc::c_void, strlen(needle));
}
#[no_mangle]
pub unsafe extern "C" fn bstr_begins_with_nocase(
    mut haystack: *const bstr,
    mut needle: *const bstr,
) -> libc::c_int {
    return bstr_begins_with_mem_nocase(
        haystack,
        if (*needle).realptr.is_null() {
            (needle as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
        } else {
            (*needle).realptr
        } as *const libc::c_void,
        (*needle).len,
    );
}
#[no_mangle]
pub unsafe extern "C" fn bstr_begins_with_mem(
    mut haystack: *const bstr,
    mut _data: *const libc::c_void,
    mut len: size_t,
) -> libc::c_int {
    let mut data: *const libc::c_uchar = _data as *mut libc::c_uchar;
    let mut hdata: *const libc::c_uchar = if (*haystack).realptr.is_null() {
        (haystack as *mut libc::c_uchar)
            .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
    } else {
        (*haystack).realptr
    };
    let mut hlen: size_t = (*haystack).len;
    let mut pos: size_t = 0 as libc::c_int as size_t;
    while pos < len && pos < hlen {
        if *hdata.offset(pos as isize) as libc::c_int != *data.offset(pos as isize) as libc::c_int {
            return 0 as libc::c_int;
        }
        pos = pos.wrapping_add(1)
    }
    if pos == len {
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn bstr_begins_with_mem_nocase(
    mut haystack: *const bstr,
    mut _data: *const libc::c_void,
    mut len: size_t,
) -> libc::c_int {
    let mut data: *const libc::c_uchar = _data as *const libc::c_uchar;
    let mut hdata: *const libc::c_uchar = if (*haystack).realptr.is_null() {
        (haystack as *mut libc::c_uchar)
            .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
    } else {
        (*haystack).realptr
    };
    let mut hlen: size_t = (*haystack).len;
    let mut pos: size_t = 0 as libc::c_int as size_t;
    while pos < len && pos < hlen {
        if tolower(*hdata.offset(pos as isize) as libc::c_int)
            != tolower(*data.offset(pos as isize) as libc::c_int)
        {
            return 0 as libc::c_int;
        }
        pos = pos.wrapping_add(1)
    }
    if pos == len {
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn bstr_char_at(mut b: *const bstr, mut pos: size_t) -> libc::c_int {
    let mut data: *mut libc::c_uchar = if (*b).realptr.is_null() {
        (b as *mut libc::c_uchar).offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
    } else {
        (*b).realptr
    };
    let mut len: size_t = (*b).len;
    if pos >= len {
        return -(1 as libc::c_int);
    }
    return *data.offset(pos as isize) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bstr_char_at_end(mut b: *const bstr, mut pos: size_t) -> libc::c_int {
    let mut data: *mut libc::c_uchar = if (*b).realptr.is_null() {
        (b as *mut libc::c_uchar).offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
    } else {
        (*b).realptr
    };
    let mut len: size_t = (*b).len;
    if pos >= len {
        return -(1 as libc::c_int);
    }
    return *data.offset(
        len.wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_sub(pos) as isize,
    ) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bstr_chop(mut b: *mut bstr) {
    if (*b).len > 0 as libc::c_int as libc::c_ulong {
        bstr_adjust_len(b, (*b).len.wrapping_sub(1 as libc::c_int as libc::c_ulong));
    };
}
#[no_mangle]
pub unsafe extern "C" fn bstr_chr(mut b: *const bstr, mut c: libc::c_int) -> libc::c_int {
    let mut data: *mut libc::c_uchar = if (*b).realptr.is_null() {
        (b as *mut libc::c_uchar).offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
    } else {
        (*b).realptr
    };
    let mut len: size_t = (*b).len;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < len {
        if *data.offset(i as isize) as libc::c_int == c {
            return i as libc::c_int;
        }
        i = i.wrapping_add(1)
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn bstr_cmp(mut b1: *const bstr, mut b2: *const bstr) -> libc::c_int {
    return bstr_util_cmp_mem(
        if (*b1).realptr.is_null() {
            (b1 as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
        } else {
            (*b1).realptr
        } as *const libc::c_void,
        (*b1).len,
        if (*b2).realptr.is_null() {
            (b2 as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
        } else {
            (*b2).realptr
        } as *const libc::c_void,
        (*b2).len,
    );
}
#[no_mangle]
pub unsafe extern "C" fn bstr_cmp_c(mut b: *const bstr, mut c: *const libc::c_char) -> libc::c_int {
    return bstr_util_cmp_mem(
        if (*b).realptr.is_null() {
            (b as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
        } else {
            (*b).realptr
        } as *const libc::c_void,
        (*b).len,
        c as *const libc::c_void,
        strlen(c),
    );
}
#[no_mangle]
pub unsafe extern "C" fn bstr_cmp_c_nocase(
    mut b: *const bstr,
    mut c: *const libc::c_char,
) -> libc::c_int {
    return bstr_util_cmp_mem_nocase(
        if (*b).realptr.is_null() {
            (b as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
        } else {
            (*b).realptr
        } as *const libc::c_void,
        (*b).len,
        c as *const libc::c_void,
        strlen(c),
    );
}
#[no_mangle]
pub unsafe extern "C" fn bstr_cmp_c_nocasenorzero(
    mut b: *const bstr,
    mut c: *const libc::c_char,
) -> libc::c_int {
    return bstr_util_cmp_mem_nocasenorzero(
        if (*b).realptr.is_null() {
            (b as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
        } else {
            (*b).realptr
        } as *const libc::c_void,
        (*b).len,
        c as *const libc::c_void,
        strlen(c),
    );
}
#[no_mangle]
pub unsafe extern "C" fn bstr_cmp_mem(
    mut b: *const bstr,
    mut data: *const libc::c_void,
    mut len: size_t,
) -> libc::c_int {
    return bstr_util_cmp_mem(
        if (*b).realptr.is_null() {
            (b as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
        } else {
            (*b).realptr
        } as *const libc::c_void,
        (*b).len,
        data,
        len,
    );
}
#[no_mangle]
pub unsafe extern "C" fn bstr_cmp_mem_nocase(
    mut b: *const bstr,
    mut data: *const libc::c_void,
    mut len: size_t,
) -> libc::c_int {
    return bstr_util_cmp_mem_nocase(
        if (*b).realptr.is_null() {
            (b as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
        } else {
            (*b).realptr
        } as *const libc::c_void,
        (*b).len,
        data,
        len,
    );
}
#[no_mangle]
pub unsafe extern "C" fn bstr_cmp_nocase(mut b1: *const bstr, mut b2: *const bstr) -> libc::c_int {
    return bstr_util_cmp_mem_nocase(
        if (*b1).realptr.is_null() {
            (b1 as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
        } else {
            (*b1).realptr
        } as *const libc::c_void,
        (*b1).len,
        if (*b2).realptr.is_null() {
            (b2 as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
        } else {
            (*b2).realptr
        } as *const libc::c_void,
        (*b2).len,
    );
}
#[no_mangle]
pub unsafe extern "C" fn bstr_dup(mut b: *const bstr) -> *mut bstr {
    return bstr_dup_ex(b, 0 as libc::c_int as size_t, (*b).len);
}
#[no_mangle]
pub unsafe extern "C" fn bstr_dup_c(mut cstr: *const libc::c_char) -> *mut bstr {
    return bstr_dup_mem(cstr as *const libc::c_void, strlen(cstr));
}
#[no_mangle]
pub unsafe extern "C" fn bstr_dup_ex(
    mut b: *const bstr,
    mut offset: size_t,
    mut len: size_t,
) -> *mut bstr {
    let mut bnew: *mut bstr = bstr_alloc(len);
    if bnew.is_null() {
        return 0 as *mut bstr;
    }
    memcpy(
        if (*bnew).realptr.is_null() {
            (bnew as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
        } else {
            (*bnew).realptr
        } as *mut libc::c_void,
        (if (*b).realptr.is_null() {
            (b as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
        } else {
            (*b).realptr
        })
        .offset(offset as isize) as *const libc::c_void,
        len,
    );
    bstr_adjust_len(bnew, len);
    return bnew;
}
#[no_mangle]
pub unsafe extern "C" fn bstr_dup_lower(mut b: *const bstr) -> *mut bstr {
    return bstr_to_lowercase(bstr_dup(b));
}
#[no_mangle]
pub unsafe extern "C" fn bstr_dup_mem(mut data: *const libc::c_void, mut len: size_t) -> *mut bstr {
    let mut bnew: *mut bstr = bstr_alloc(len);
    if bnew.is_null() {
        return 0 as *mut bstr;
    }
    memcpy(
        if (*bnew).realptr.is_null() {
            (bnew as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
        } else {
            (*bnew).realptr
        } as *mut libc::c_void,
        data,
        len,
    );
    bstr_adjust_len(bnew, len);
    return bnew;
}
#[no_mangle]
pub unsafe extern "C" fn bstr_expand(mut b: *mut bstr, mut newsize: size_t) -> *mut bstr {
    if !(*b).realptr.is_null() {
        // Refuse to expand a wrapped bstring. In the future,
        // we can change this to make a copy of the data, thus
        // leaving the original memory area intact.
        return 0 as *mut bstr;
    }
    // Catch attempts to "expand" to a smaller size
    if (*b).size > newsize {
        return 0 as *mut bstr;
    }
    let mut bnew: *mut bstr = realloc(
        b as *mut libc::c_void,
        (::std::mem::size_of::<bstr>() as libc::c_ulong).wrapping_add(newsize),
    ) as *mut bstr;
    if bnew.is_null() {
        return 0 as *mut bstr;
    }
    bstr_adjust_size(bnew, newsize);
    return bnew;
}
#[no_mangle]
pub unsafe extern "C" fn bstr_free(mut b: *mut bstr) {
    if b.is_null() {
        return;
    }
    free(b as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn bstr_index_of(
    mut haystack: *const bstr,
    mut needle: *const bstr,
) -> libc::c_int {
    return bstr_index_of_mem(
        haystack,
        if (*needle).realptr.is_null() {
            (needle as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
        } else {
            (*needle).realptr
        } as *const libc::c_void,
        (*needle).len,
    );
}
#[no_mangle]
pub unsafe extern "C" fn bstr_index_of_c(
    mut haystack: *const bstr,
    mut needle: *const libc::c_char,
) -> libc::c_int {
    return bstr_index_of_mem(haystack, needle as *const libc::c_void, strlen(needle));
}
#[no_mangle]
pub unsafe extern "C" fn bstr_index_of_c_nocase(
    mut haystack: *const bstr,
    mut needle: *const libc::c_char,
) -> libc::c_int {
    return bstr_index_of_mem_nocase(haystack, needle as *const libc::c_void, strlen(needle));
}
#[no_mangle]
pub unsafe extern "C" fn bstr_index_of_c_nocasenorzero(
    mut haystack: *const bstr,
    mut needle: *const libc::c_char,
) -> libc::c_int {
    return bstr_util_mem_index_of_mem_nocasenorzero(
        if (*haystack).realptr.is_null() {
            (haystack as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
        } else {
            (*haystack).realptr
        } as *const libc::c_void,
        (*haystack).len,
        needle as *const libc::c_void,
        strlen(needle),
    );
}
#[no_mangle]
pub unsafe extern "C" fn bstr_index_of_mem(
    mut haystack: *const bstr,
    mut _data2: *const libc::c_void,
    mut len2: size_t,
) -> libc::c_int {
    return bstr_util_mem_index_of_mem(
        if (*haystack).realptr.is_null() {
            (haystack as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
        } else {
            (*haystack).realptr
        } as *const libc::c_void,
        (*haystack).len,
        _data2,
        len2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn bstr_index_of_mem_nocase(
    mut haystack: *const bstr,
    mut _data2: *const libc::c_void,
    mut len2: size_t,
) -> libc::c_int {
    return bstr_util_mem_index_of_mem_nocase(
        if (*haystack).realptr.is_null() {
            (haystack as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
        } else {
            (*haystack).realptr
        } as *const libc::c_void,
        (*haystack).len,
        _data2,
        len2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn bstr_index_of_nocase(
    mut haystack: *const bstr,
    mut needle: *const bstr,
) -> libc::c_int {
    return bstr_index_of_mem_nocase(
        haystack,
        if (*needle).realptr.is_null() {
            (needle as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
        } else {
            (*needle).realptr
        } as *const libc::c_void,
        (*needle).len,
    );
}
#[no_mangle]
pub unsafe extern "C" fn bstr_rchr(mut b: *const bstr, mut c: libc::c_int) -> libc::c_int {
    let mut data: *const libc::c_uchar = if (*b).realptr.is_null() {
        (b as *mut libc::c_uchar).offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
    } else {
        (*b).realptr
    };
    let mut len: size_t = (*b).len;
    let mut i: size_t = len;
    while i > 0 as libc::c_int as libc::c_ulong {
        if *data.offset(i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize) as libc::c_int
            == c
        {
            return i.wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
        }
        i = i.wrapping_sub(1)
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn bstr_to_lowercase(mut b: *mut bstr) -> *mut bstr {
    if b.is_null() {
        return 0 as *mut bstr;
    }
    let mut data: *mut libc::c_uchar = if (*b).realptr.is_null() {
        (b as *mut libc::c_uchar).offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
    } else {
        (*b).realptr
    };
    let mut len: size_t = (*b).len;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < len {
        *data.offset(i as isize) =
            tolower(*data.offset(i as isize) as libc::c_int) as libc::c_uchar;
        i = i.wrapping_add(1)
    }
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn bstr_util_cmp_mem(
    mut _data1: *const libc::c_void,
    mut len1: size_t,
    mut _data2: *const libc::c_void,
    mut len2: size_t,
) -> libc::c_int {
    let mut data1: *const libc::c_uchar = _data1 as *const libc::c_uchar;
    let mut data2: *const libc::c_uchar = _data2 as *const libc::c_uchar;
    let mut p1: size_t = 0 as libc::c_int as size_t;
    let mut p2: size_t = 0 as libc::c_int as size_t;
    while p1 < len1 && p2 < len2 {
        if *data1.offset(p1 as isize) as libc::c_int != *data2.offset(p2 as isize) as libc::c_int {
            // Difference.
            return if (*data1.offset(p1 as isize) as libc::c_int)
                < *data2.offset(p2 as isize) as libc::c_int
            {
                -(1 as libc::c_int)
            } else {
                1 as libc::c_int
            };
        }
        p1 = p1.wrapping_add(1);
        p2 = p2.wrapping_add(1)
    }
    if p1 == len2 && p2 == len1 {
        // They're identical.
        return 0 as libc::c_int;
    } else if p1 == len1 {
        return -(1 as libc::c_int);
    } else {
        return 1 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn bstr_util_cmp_mem_nocase(
    mut _data1: *const libc::c_void,
    mut len1: size_t,
    mut _data2: *const libc::c_void,
    mut len2: size_t,
) -> libc::c_int {
    let mut data1: *const libc::c_uchar = _data1 as *const libc::c_uchar;
    let mut data2: *const libc::c_uchar = _data2 as *const libc::c_uchar;
    let mut p1: size_t = 0 as libc::c_int as size_t;
    let mut p2: size_t = 0 as libc::c_int as size_t;
    while p1 < len1 && p2 < len2 {
        if tolower(*data1.offset(p1 as isize) as libc::c_int)
            != tolower(*data2.offset(p2 as isize) as libc::c_int)
        {
            // One string is shorter.
            // Difference.
            return if tolower(*data1.offset(p1 as isize) as libc::c_int)
                < tolower(*data2.offset(p2 as isize) as libc::c_int)
            {
                -(1 as libc::c_int)
            } else {
                1 as libc::c_int
            };
        }
        p1 = p1.wrapping_add(1);
        p2 = p2.wrapping_add(1)
    }
    if p1 == len2 && p2 == len1 {
        // They're identical.
        return 0 as libc::c_int;
    } else if p1 == len1 {
        return -(1 as libc::c_int);
    } else {
        return 1 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn bstr_util_cmp_mem_nocasenorzero(
    mut _data1: *const libc::c_void,
    mut len1: size_t,
    mut _data2: *const libc::c_void,
    mut len2: size_t,
) -> libc::c_int {
    let mut data1: *const libc::c_uchar = _data1 as *const libc::c_uchar;
    let mut data2: *const libc::c_uchar = _data2 as *const libc::c_uchar;
    let mut p1: size_t = 0 as libc::c_int as size_t;
    let mut p2: size_t = 0 as libc::c_int as size_t;
    while p1 < len1 && p2 < len2 {
        if *data1.offset(p1 as isize) as libc::c_int == 0 as libc::c_int {
            p1 = p1.wrapping_add(1)
        } else {
            if tolower(*data1.offset(p1 as isize) as libc::c_int)
                != tolower(*data2.offset(p2 as isize) as libc::c_int)
            {
                // One string is shorter.
                // Difference.
                return if tolower(*data1.offset(p1 as isize) as libc::c_int)
                    < tolower(*data2.offset(p2 as isize) as libc::c_int)
                {
                    -(1 as libc::c_int)
                } else {
                    1 as libc::c_int
                };
            }
            p1 = p1.wrapping_add(1);
            p2 = p2.wrapping_add(1)
        }
    }
    while p1 < len1 && *data1.offset(p1 as isize) as libc::c_int == 0 as libc::c_int {
        p1 = p1.wrapping_add(1)
    }
    if p1 == len1 && p2 == len2 {
        // They're identical.
        return 0 as libc::c_int;
    } else if p1 == len1 {
        return -(1 as libc::c_int);
    } else {
        return 1 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn bstr_util_mem_to_pint(
    mut _data: *const libc::c_void,
    mut len: size_t,
    mut base: libc::c_int,
    mut lastlen: *mut size_t,
) -> int64_t {
    let mut data: *const libc::c_uchar = _data as *mut libc::c_uchar;
    let mut rval: int64_t = 0 as libc::c_int as int64_t;
    let mut tflag: int64_t = 0 as libc::c_int as int64_t;
    let mut i: size_t = 0 as libc::c_int as size_t;
    *lastlen = i;
    i = 0 as libc::c_int as size_t;
    while i < len {
        let mut d: libc::c_int = *data.offset(i as isize) as libc::c_int;
        *lastlen = i;
        // One string is shorter.
        // Convert character to digit.
        if d >= '0' as i32 && d <= '9' as i32 {
            d -= '0' as i32
        } else if d >= 'a' as i32 && d <= 'z' as i32 {
            d -= 'a' as i32 - 10 as libc::c_int
        } else if d >= 'A' as i32 && d <= 'Z' as i32 {
            d -= 'A' as i32 - 10 as libc::c_int
        } else {
            d = -(1 as libc::c_int)
        }
        // Check that the digit makes sense with the base we are using.
        if d == -(1 as libc::c_int) || d >= base {
            if tflag != 0 {
                // Return what we have so far; lastlen points
                // to the first non-digit position.
                return rval;
            } else {
                // We didn't see a single digit.
                return -(1 as libc::c_int) as int64_t;
            }
        }
        if tflag != 0 {
            if ((9223372036854775807 as libc::c_long - d as libc::c_long) / base as libc::c_long)
                < rval
            {
                // Overflow
                return -(2 as libc::c_int) as int64_t;
            }
            rval *= base as libc::c_long;
            rval += d as libc::c_long
        } else {
            rval = d as int64_t;
            tflag = 1 as libc::c_int as int64_t
        }
        i = i.wrapping_add(1)
    }
    *lastlen = i.wrapping_add(1 as libc::c_int as libc::c_ulong);
    return rval;
}
#[no_mangle]
pub unsafe extern "C" fn bstr_util_mem_index_of_c(
    mut _data1: *const libc::c_void,
    mut len1: size_t,
    mut cstr: *const libc::c_char,
) -> libc::c_int {
    return bstr_util_mem_index_of_mem(_data1, len1, cstr as *const libc::c_void, strlen(cstr));
}
#[no_mangle]
pub unsafe extern "C" fn bstr_util_mem_index_of_c_nocase(
    mut _data1: *const libc::c_void,
    mut len1: size_t,
    mut cstr: *const libc::c_char,
) -> libc::c_int {
    return bstr_util_mem_index_of_mem_nocase(
        _data1,
        len1,
        cstr as *const libc::c_void,
        strlen(cstr),
    );
}
#[no_mangle]
pub unsafe extern "C" fn bstr_util_mem_index_of_mem(
    mut _data1: *const libc::c_void,
    mut len1: size_t,
    mut _data2: *const libc::c_void,
    mut len2: size_t,
) -> libc::c_int {
    let mut data1: *const libc::c_uchar = _data1 as *mut libc::c_uchar;
    let mut data2: *const libc::c_uchar = _data2 as *mut libc::c_uchar;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    // If we ever want to optimize this function, the following link
    // might be useful: http://en.wikipedia.org/wiki/Knuth-Morris-Pratt_algorithm
    i = 0 as libc::c_int as size_t;
    while i < len1 {
        let mut k: size_t = i;
        j = 0 as libc::c_int as size_t;
        while j < len2 && k < len1 {
            if *data1.offset(k as isize) as libc::c_int != *data2.offset(j as isize) as libc::c_int
            {
                break;
            }
            j = j.wrapping_add(1);
            k = k.wrapping_add(1)
        }
        if j == len2 {
            return i as libc::c_int;
        }
        i = i.wrapping_add(1)
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn bstr_util_mem_index_of_mem_nocase(
    mut _data1: *const libc::c_void,
    mut len1: size_t,
    mut _data2: *const libc::c_void,
    mut len2: size_t,
) -> libc::c_int {
    let mut data1: *const libc::c_uchar = _data1 as *mut libc::c_uchar;
    let mut data2: *const libc::c_uchar = _data2 as *mut libc::c_uchar;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    // If we ever want to optimize this function, the following link
    // might be useful: http://en.wikipedia.org/wiki/Knuth-Morris-Pratt_algorithm
    i = 0 as libc::c_int as size_t;
    while i < len1 {
        let mut k: size_t = i;
        j = 0 as libc::c_int as size_t;
        while j < len2 && k < len1 {
            if toupper(*data1.offset(k as isize) as libc::c_int)
                != toupper(*data2.offset(j as isize) as libc::c_int)
            {
                break;
            }
            j = j.wrapping_add(1);
            k = k.wrapping_add(1)
        }
        if j == len2 {
            return i as libc::c_int;
        }
        i = i.wrapping_add(1)
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn bstr_util_mem_index_of_mem_nocasenorzero(
    mut _data1: *const libc::c_void,
    mut len1: size_t,
    mut _data2: *const libc::c_void,
    mut len2: size_t,
) -> libc::c_int {
    let mut data1: *const libc::c_uchar = _data1 as *mut libc::c_uchar;
    let mut data2: *const libc::c_uchar = _data2 as *mut libc::c_uchar;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    // If we ever want to optimize this function, the following link
    // might be useful: http://en.wikipedia.org/wiki/Knuth-Morris-Pratt_algorithm
    i = 0 as libc::c_int as size_t;
    while i < len1 {
        let mut k: size_t = i;
        if !(*data1.offset(i as isize) as libc::c_int == 0 as libc::c_int) {
            j = 0 as libc::c_int as size_t;
            while j < len2 && k < len1 {
                if *data1.offset(k as isize) as libc::c_int == 0 as libc::c_int {
                    j = j.wrapping_sub(1)
                } else if toupper(*data1.offset(k as isize) as libc::c_int)
                    != toupper(*data2.offset(j as isize) as libc::c_int)
                {
                    break;
                }
                j = j.wrapping_add(1);
                k = k.wrapping_add(1)
            }
            if j == len2 {
                return i as libc::c_int;
            }
        }
        // skip leading zeroes to avoid quadratic complexity
        i = i.wrapping_add(1)
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn bstr_util_mem_trim(
    mut data: *mut *mut libc::c_uchar,
    mut len: *mut size_t,
) {
    if data.is_null() || len.is_null() {
        return;
    }
    let mut d: *mut libc::c_uchar = *data;
    let mut l: size_t = *len;
    // Ignore whitespace at the beginning.
    let mut pos: size_t = 0 as libc::c_int as size_t;
    while pos < l
        && *(*__ctype_b_loc()).offset(*d.offset(pos as isize) as libc::c_int as isize)
            as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        pos = pos.wrapping_add(1)
    }
    d = d.offset(pos as isize);
    l = (l as libc::c_ulong).wrapping_sub(pos) as size_t as size_t;
    // Ignore whitespace at the end.
    while l > 0 as libc::c_int as libc::c_ulong
        && *(*__ctype_b_loc()).offset(
            *d.offset(l.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize) as libc::c_int
                as isize,
        ) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        l = l.wrapping_sub(1)
    }
    *data = d;
    *len = l;
}
#[no_mangle]
pub unsafe extern "C" fn bstr_util_memdup_to_c(
    mut _data: *const libc::c_void,
    mut len: size_t,
) -> *mut libc::c_char {
    let mut data: *const libc::c_uchar = _data as *mut libc::c_uchar;
    // Count how many NUL bytes we have in the string.
    let mut i: size_t = 0;
    let mut nulls: size_t = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < len {
        if *data.offset(i as isize) as libc::c_int == '\u{0}' as i32 {
            nulls = nulls.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    // Now copy the string into a NUL-terminated buffer.
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
    d = malloc(
        len.wrapping_add(nulls)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    r = d;
    if d.is_null() {
        return 0 as *mut libc::c_char;
    }
    loop {
        let fresh0 = len;
        len = len.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        if *data as libc::c_int == '\u{0}' as i32 {
            data = data.offset(1);
            let fresh1 = d;
            d = d.offset(1);
            *fresh1 = '\\' as i32 as libc::c_char;
            let fresh2 = d;
            d = d.offset(1);
            *fresh2 = '0' as i32 as libc::c_char
        } else {
            let fresh3 = data;
            data = data.offset(1);
            let fresh4 = d;
            d = d.offset(1);
            *fresh4 = *fresh3 as libc::c_char
        }
    }
    *d = '\u{0}' as i32 as libc::c_char;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn bstr_util_strdup_to_c(mut b: *const bstr) -> *mut libc::c_char {
    if b.is_null() {
        return 0 as *mut libc::c_char;
    }
    return bstr_util_memdup_to_c(
        if (*b).realptr.is_null() {
            (b as *mut libc::c_uchar)
                .offset(::std::mem::size_of::<bstr>() as libc::c_ulong as isize)
        } else {
            (*b).realptr
        } as *const libc::c_void,
        (*b).len,
    );
}
#[no_mangle]
pub unsafe extern "C" fn bstr_wrap_c(mut cstr: *const libc::c_char) -> *mut bstr {
    return bstr_wrap_mem(
        cstr as *mut libc::c_uchar as *const libc::c_void,
        strlen(cstr),
    );
}
// Defines
// Functions
/* *
 * Append source bstring to destination bstring, growing destination if
 * necessary. If the destination bstring is expanded, the pointer will change.
 * You must replace the original destination pointer with the returned one.
 * Destination is not changed on memory allocation failure.
 *
 * @param[in] bdestination
 * @param[in] bsource
 * @return Updated bstring, or NULL on memory allocation failure.
 */
/* *
 * Append a NUL-terminated source to destination, growing destination if
 * necessary. If the string is expanded, the pointer will change. You must
 * replace the original destination pointer with the returned one. Destination
 * is not changed on memory allocation failure.
 *
 * @param[in] b
 * @param[in] cstr
 * @return Updated bstring, or NULL on memory allocation failure.
 */
/* *
 * Append as many bytes from the source to destination bstring. The
 * destination storage will not be expanded if there is not enough space in it
 * already to accommodate all of the data.
 *
 * @param[in] b
 * @param[in] cstr
 * @return The destination bstring.
 */
/* *
 * Append a memory region to destination, growing destination if necessary. If
 * the string is expanded, the pointer will change. You must replace the
 * original destination pointer with the returned one. Destination is not
 * changed on memory allocation failure.
 *
 * @param[in] b
 * @param[in] data
 * @param[in] len
 * @return Updated bstring, or NULL on memory allocation failure.
 */
/* *
 * Append as many bytes from the source to destination bstring. The
 * destination storage will not be expanded if there is not enough space in it
 * already to accommodate all of the data.
 *
 * @param[in] b
 * @param[in] data
 * @param[in] len
 * @return The destination bstring.
 */
/* *
 * Append as many bytes from the source bstring to destination bstring. The
 * destination storage will not be expanded if there is not enough space in it
 * already to accommodate all of the data.
 *
 * @param[in] bdestination
 * @param[in] bsource
 * @return The destination bstring.
 */
/* *
 * Adjust bstring length. You will need to use this method whenever
 * you work directly with the string contents, and end up changing
 * its length by direct structure manipulation.
 *
 * @param[in] b
 * @param[in] newlen
 */
/* *
 * Change the external pointer used by bstring. You will need to use this
 * function only if you're messing with bstr internals. Use with caution.
 *
 * @param[in] b
 * @param[in] newrealptr
 */
/* *
 * Adjust bstring size. This does not change the size of the storage behind
 * the bstring, just changes the field that keeps track of how many bytes
 * there are in the storage. You will need to use this function only if
 * you're messing with bstr internals. Use with caution.
 *
 * @param[in] b
 * @param[in] newsize
 */
/* *
 * Allocate a zero-length bstring, reserving space for at least size bytes.
 *
 * @param[in] size
 * @return New string instance
 */
/* *
 * Checks whether bstring begins with another bstring. Case sensitive.
 *
 * @param[in] bhaystack
 * @param[in] bneedle
 * @return 1 if true, otherwise 0.
 */
/* *
 * Checks whether bstring begins with NUL-terminated string. Case sensitive.
 *
 * @param[in] bhaystack
 * @param[in] cneedle
 * @return 1 if true, otherwise 0.
 */
/* *
 * Checks whether bstring begins with NUL-terminated string. Case insensitive.
 *
 * @param[in] bhaystack
 * @param[in] cneedle
 * @return 1 if true, otherwise 0.
 */
/* *
 * Checks whether the bstring begins with the given memory block. Case sensitive.
 *
 * @param[in] bhaystack
 * @param[in] data
 * @param[in] len
 * @return 1 if true, otherwise 0.
 */
/* *
 * Checks whether bstring begins with memory block. Case insensitive.
 *
 * @param[in] bhaystack
 * @param[in] data
 * @param[in] len
 * @return 1 if true, otherwise 0.
 */
/* *
 * Checks whether bstring begins with another bstring. Case insensitive.
 *
 * @param[in] bhaystack
 * @param[in] cneedle
 * @return 1 if true, otherwise 0.
 */
/* *
 * Return the byte at the given position.
 *
 * @param[in] b
 * @param[in] pos
 * @return The byte at the given location, or -1 if the position is out of range.
 */
/* *
 * Return the byte at the given position, counting from the end of the string (e.g.,
 * byte at position 0 is the last byte in the string.)
 *
 * @param[in] b
 * @param[in] pos
 * @return The byte at the given location, or -1 if the position is out of range.
 */
/* *
 * Remove the last byte from bstring, assuming it contains at least one byte. This
 * function will not reduce the storage that backs the string, only the amount
 * of data used.
 *
 * @param[in] b
 */
/* *
 * Return the first position of the provided byte.
 *
 * @param[in] b
 * @param[in] c
 * @return The first position of the byte, or -1 if it could not be found
 */
/* *
 * Case-sensitive comparison of two bstrings.
 *
 * @param[in] b1
 * @param[in] b2
 * @return Zero on string match, 1 if b1 is greater than b2, and -1 if b2 is
 *         greater than b1.
 */
/* *
 * Case-sensitive comparison of a bstring and a NUL-terminated string.
 *
 * @param[in] b
 * @param[in] cstr
 * @return Zero on string match, 1 if b is greater than cstr, and -1 if cstr is
 *         greater than b.
 */
/* *
 * Case-insensitive comparison of a bstring with a NUL-terminated string.
 *
 * @param[in] b
 * @param[in] cstr
 * @return Zero on string match, 1 if b is greater than cstr, and -1 if cstr is greater than b.
 */
/* *
 * Case-insensitive zero-skipping comparison of a bstring with a NUL-terminated string.
 *
 * @param[in] b
 * @param[in] cstr
 * @return Zero on string match, 1 if b is greater than cstr, and -1 if cstr is greater than b.
 */
/* *
 * Performs a case-sensitive comparison of a bstring with a memory region.
 *
 * @param[in] b
 * @param[in] data
 * @param[in] len
 * @return Zero ona match, 1 if b is greater than data, and -1 if data is greater than b.
 */
/* *
 * Performs a case-insensitive comparison of a bstring with a memory region.
 *
 * @param[in] b
 * @param[in] data
 * @param[in] len
 * @return Zero ona match, 1 if b is greater than data, and -1 if data is greater than b.
 */
/* *
 * Case-insensitive comparison two bstrings.
 *
 * @param[in] b1
 * @param[in] b2
 * @return Zero on string match, 1 if b1 is greater than b2, and -1 if b2 is
 *         greater than b1.
 */
/* *
 * Case-insensitive and zero skipping comparison two bstrings.
 *
 * @param[in] b1
 * @param[in] b2
 * @return Zero on string match, 1 if b1 is greater than b2, and -1 if b2 is
 *         greater than b1.
 */
/* *
 * Create a new bstring by copying the provided bstring.
 *
 * @param[in] b
 * @return New bstring, or NULL if memory allocation failed.
 */
/* *
 * Create a new bstring by copying the provided NUL-terminated string.
 *
 * @param[in] cstr
 * @return New bstring, or NULL if memory allocation failed.
 */
/* *
 * Create a new bstring by copying a part of the provided bstring.
 *
 * @param[in] b
 * @param[in] offset
 * @param[in] len
 * @return New bstring, or NULL if memory allocation failed.
 */
/* *
 * Create a copy of the provided bstring, then convert it to lowercase.
 *
 * @param[in] b
 * @return New bstring, or NULL if memory allocation failed
 */
/* *
 * Create a new bstring by copying the provided memory region.
 *
 * @param[in] data
 * @param[in] len
 * @return New bstring, or NULL if memory allocation failed
 */
/* *
 * Expand internal bstring storage to support at least newsize bytes. The storage
 * is not expanded if the current size is equal or greater to newsize. Because
 * realloc is used underneath, the old pointer to bstring may no longer be valid
 * after this function completes successfully.
 *
 * @param[in] b
 * @param[in] newsize
 * @return Updated string instance, or NULL if memory allocation failed or if
 *         attempt was made to "expand" the bstring to a smaller size.
 */
/* *
 * Deallocate the supplied bstring instance and set it to NULL. Allows NULL on
 * input.
 *
 * @param[in] b
 */
/* *
 * Find the needle in the haystack.
 *
 * @param[in] bhaystack
 * @param[in] bneedle
 * @return Position of the match, or -1 if the needle could not be found.
 */
/* *
 * Find the needle in the haystack, ignoring case differences.
 *
 * @param[in] bhaystack
 * @param[in] bneedle
 * @return Position of the match, or -1 if the needle could not be found.
 */
/* *
 * Find the needle in the haystack, with the needle being a NUL-terminated
 * string.
 *
 * @param[in] bhaystack
 * @param[in] cneedle
 * @return Position of the match, or -1 if the needle could not be found.
 */
/* *
 * Find the needle in the haystack, with the needle being a NUL-terminated
 * string. Ignore case differences.
 *
 * @param[in] bhaystack
 * @param[in] cneedle
 * @return Position of the match, or -1 if the needle could not be found.
 */
/* *
 * Find the needle in the haystack, with the needle being a NUL-terminated
 * string. Ignore case differences. Skip zeroes in haystack
 *
 * @param[in] bhaystack
 * @param[in] cneedle
 * @return Position of the match, or -1 if the needle could not be found.
 */
/* *
 * Find the needle in the haystack, with the needle being a memory region.
 *
 * @param[in] bhaystack
 * @param[in] data
 * @param[in] len
 * @return Position of the match, or -1 if the needle could not be found.
 */
/* *
 * Find the needle in the haystack, with the needle being a memory region.
 * Ignore case differences.
 *
 * @param[in] bhaystack
 * @param[in] data
 * @param[in] len
 * @return Position of the match, or -1 if the needle could not be found.
 */
/* *
 * Return the last position of a character (byte).
 *
 * @param[in] b
 * @param[in] c
 * @return The last position of the character, or -1 if it could not be found.
 */
/* *
 * Convert bstring to lowercase. This function converts the supplied string,
 * it does not create a new string.
 *
 * @param[in] b
 * @return The same bstring received on input
 */
/* *
 * Case-sensitive comparison of two memory regions.
 *
 * @param[in] data1
 * @param[in] len1
 * @param[in] data2
 * @param[in] len2
 * @return Zero if the memory regions are identical, 1 if data1 is greater than
 *         data2, and -1 if data2 is greater than data1.
 */
/* *
 * Case-insensitive comparison of two memory regions.
 *
 * @param[in] data1
 * @param[in] len1
 * @param[in] data2
 * @param[in] len2
 * @return Zero if the memory regions are identical, 1 if data1 is greater than
 *         data2, and -1 if data2 is greater than data1.
 */
/* *
 * Case-insensitive zero-skipping comparison of two memory regions.
 *
 * @param[in] data1
 * @param[in] len1
 * @param[in] data2
 * @param[in] len2
 * @return Zero if the memory regions are identical, 1 if data1 is greater than
 *         data2, and -1 if data2 is greater than data1.
 */
/* *
 * Convert contents of a memory region to a positive integer.
 *
 * @param[in] data
 * @param[in] len
 * @param[in] base The desired number base.
 * @param[in] lastlen Points to the first unused byte in the region
 * @return If the conversion was successful, this function returns the
 *         number. When the conversion fails, -1 will be returned when not
 *         one valid digit was found, and -2 will be returned if an overflow
 *         occurred.
 */
/* *
 * Searches a memory block for the given NUL-terminated string. Case sensitive.
 *
 * @param[in] data
 * @param[in] len
 * @param[in] cstr
 * @return Index of the first location of the needle on success, or -1 if the needle was not found.
 */
/* *
 * Searches a memory block for the given NUL-terminated string. Case insensitive.
 *
 * @param[in] data
 * @param[in] len
 * @param[in] cstr
 * @return Index of the first location of the needle on success, or -1 if the needle was not found.
 */
/* *
 * Searches the haystack memory block for the needle memory block. Case sensitive.
 *
 * @param data1
 * @param len1
 * @param data2
 * @param len2
 * @return Index of the first location of the needle on success, or -1 if the needle was not found.
 */
/* *
 * Searches the haystack memory block for the needle memory block. Case sensitive.
 *
 * @param data1
 * @param len1
 * @param data2
 * @param len2
 * @return Index of the first location of the needle on success, or -1 if the needle was not found.
 */
/* *
 * Searches the haystack memory block for the needle memory block. Case sensitive. Skips zeroes in data1
 *
 * @param data1
 * @param len1
 * @param data2
 * @param len2
 * @return Index of the first location of the needle on success, or -1 if the needle was not found.
 */
/* *
 * Removes whitespace from the beginning and the end of a memory region. The data
 * itself is not modified; this function only adjusts the provided pointers.
 *
 * @param[in,out] data
 * @param[in,out] len
 */
/* *
 * Take the provided memory region, allocate a new memory buffer, and construct
 * a NUL-terminated string, replacing each NUL byte with "\0" (two bytes). The
 * caller is responsible to keep track of the allocated memory area and free
 * it once it is no longer needed.
 *
 * @param[in] data
 * @param[in] len
 * @return The newly created NUL-terminated string, or NULL in case of memory
 *         allocation failure.
 */
/* *
 * Create a new NUL-terminated string out of the provided bstring. If NUL bytes
 * are contained in the bstring, each will be replaced with "\0" (two characters).
 * The caller is responsible to keep track of the allocated memory area and free
 * it once it is no longer needed.
 *
 * @param[in] b
 * @return The newly created NUL-terminated string, or NULL in case of memory
 *         allocation failure.
 */
/* *
 * Create a new bstring from the provided NUL-terminated string and without
 * copying the data. The caller must ensure that the input string continues
 * to point to a valid memory location for as long as the bstring is used.
 *
 * @param[in] cstr
 * @return New bstring, or NULL on memory allocation failure.
 */
/* *
 * Create a new bstring from the provided memory buffer without
 * copying the data. The caller must ensure that the buffer remains
 * valid for as long as the bstring is used.
 *
 * @param[in] data
 * @param[in] len
 * @return New bstring, or NULL on memory allocation failure.
 */
#[no_mangle]
pub unsafe extern "C" fn bstr_wrap_mem(
    mut data: *const libc::c_void,
    mut len: size_t,
) -> *mut bstr {
    let mut b: *mut bstr = malloc(::std::mem::size_of::<bstr>() as libc::c_ulong) as *mut bstr;
    if b.is_null() {
        return 0 as *mut bstr;
    }
    (*b).len = len;
    (*b).size = (*b).len;
    (*b).realptr = data as *mut libc::c_uchar;
    return b;
}
