extern "C" {
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn toupper(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn tolower(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn malloc(_: libc::size_t) -> *mut core::ffi::c_void;
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
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::size_t;
}

pub const _ISspace: i32 = 8192;
// Data structures

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct bstr_t {
    /// The length of the string stored in the buffer.
    pub len: usize,

    /// The current size of the buffer. If there is extra room in the
    ///  buffer the string will be able to expand without reallocation.
    pub size: usize,

    /// Optional buffer pointer. If this pointer is NULL the string buffer
    ///  will immediately follow this structure. If the pointer is not NUL,
    ///  it points to the actual buffer used, and there's no data following
    ///  this structure.
    pub realptr: *mut u8,
}

// This function was a macro in libhtp
// #define bstr_len(X) ((*(X)).len)
pub unsafe fn bstr_len(x: *const bstr_t) -> usize {
    (*x).len
}

// This function was a macro in libhtp
// #define bstr_size(X) ((*(X)).size)
pub unsafe fn bstr_size(x: *const bstr_t) -> usize {
    (*x).size
}

// This function was a macro in libhtp
// #define bstr_ptr(X) ( ((*(X)).realptr == NULL) ? ((unsigned char *)(X) + sizeof(bstr_t)) : (unsigned char *)(*(X)).realptr )
pub unsafe fn bstr_ptr(x: *const bstr_t) -> *mut u8 {
    if (*x).realptr.is_null() {
        // bstr optimizes small strings to be 'after this structure'
        (x as *mut u8).offset(std::mem::size_of::<bstr_t>() as isize) as *mut u8
    } else {
        (*x).realptr
    }
}

/// Allocate a zero-length bstring, reserving space for at least size bytes.
///
/// Returns New string instance
pub unsafe fn bstr_alloc(mut len: usize) -> *mut bstr_t {
    let mut b: *mut bstr_t =
        malloc((::std::mem::size_of::<bstr_t>()).wrapping_add(len)) as *mut bstr_t;
    if b.is_null() {
        return 0 as *mut bstr_t;
    }
    (*b).len = 0;
    (*b).size = len;
    (*b).realptr = 0 as *mut u8;
    return b;
}

/// Append as many bytes from the source to destination bstring. The
/// destination storage will not be expanded if there is not enough space in it
/// already to accommodate all of the data.
pub unsafe fn bstr_add_c_noex(mut destination: *mut bstr_t, mut source: *const i8) -> *mut bstr_t {
    return bstr_add_mem_noex(
        destination,
        source as *const core::ffi::c_void,
        strlen(source),
    );
}

/// Append a memory region to destination, growing destination if necessary. If
/// the string is expanded, the pointer will change. You must replace the
/// original destination pointer with the returned one. Destination is not
/// changed on memory allocation failure.
///
/// Returns Updated bstring, or NULL on memory allocation failure.
pub unsafe fn bstr_add_mem(
    mut destination: *mut bstr_t,
    mut data: *const core::ffi::c_void,
    mut len: usize,
) -> *mut bstr_t {
    // Expand the destination if necessary
    if (*destination).size < (*destination).len.wrapping_add(len) {
        destination = bstr_expand(destination, (*destination).len.wrapping_add(len));
        if destination.is_null() {
            return 0 as *mut bstr_t;
        }
    }
    // Add source to destination
    let mut b: *mut bstr_t = destination;
    memcpy(
        (if (*destination).realptr.is_null() {
            (destination as *mut u8).offset(::std::mem::size_of::<bstr_t>() as isize)
        } else {
            (*destination).realptr
        })
        .offset((*b).len as isize) as *mut core::ffi::c_void,
        data,
        len,
    );
    bstr_adjust_len(b, (*b).len.wrapping_add(len));
    return destination;
}

/// Append as many bytes from the source to destination bstring. The
/// destination storage will not be expanded if there is not enough space in it
/// already to accommodate all of the data.
///
/// Returns The destination bstring.
pub unsafe fn bstr_add_mem_noex(
    mut destination: *mut bstr_t,
    mut data: *const core::ffi::c_void,
    mut len: usize,
) -> *mut bstr_t {
    let mut copylen: usize = len;
    // Is there enough room in the destination?
    if (*destination).size < (*destination).len.wrapping_add(copylen) {
        copylen = (*destination).size.wrapping_sub((*destination).len);
        if copylen <= 0 {
            return destination;
        }
    }
    // Copy over the bytes
    let mut b: *mut bstr_t = destination;
    memcpy(
        (if (*destination).realptr.is_null() {
            (destination as *mut u8).offset(::std::mem::size_of::<bstr_t>() as isize)
        } else {
            (*destination).realptr
        })
        .offset((*b).len as isize) as *mut core::ffi::c_void,
        data,
        copylen,
    );
    bstr_adjust_len(b, (*b).len.wrapping_add(copylen));
    return destination;
}

/// Append as many bytes from the source bstring to destination bstring. The
/// destination storage will not be expanded if there is not enough space in it
/// already to accommodate all of the data.
pub unsafe fn bstr_add_noex(
    mut destination: *mut bstr_t,
    mut source: *const bstr_t,
) -> *mut bstr_t {
    return bstr_add_mem_noex(
        destination,
        if (*source).realptr.is_null() {
            (source as *mut u8).offset(::std::mem::size_of::<bstr_t>() as isize)
        } else {
            (*source).realptr
        } as *const core::ffi::c_void,
        (*source).len,
    );
}

/// Adjust bstring length. You will need to use this method whenever
/// you work directly with the string contents, and end up changing
/// its length by direct structure manipulation.
pub unsafe fn bstr_adjust_len(mut b: *mut bstr_t, mut newlen: usize) {
    (*b).len = newlen;
}

/// Adjust bstring size. This does not change the size of the storage behind
/// the bstring, just changes the field that keeps track of how many bytes
/// there are in the storage. You will need to use this function only if
/// you're messing with bstr internals. Use with caution.
pub unsafe fn bstr_adjust_size(mut b: *mut bstr_t, mut newsize: usize) {
    (*b).size = newsize;
}

/// Checks whether bstring begins with NUL-terminated string. Case sensitive.
///
/// Returns 1 if true, otherwise 0.
pub unsafe fn bstr_begins_with_c(mut haystack: *const bstr_t, mut needle: *const i8) -> i32 {
    return bstr_begins_with_mem(haystack, needle as *const core::ffi::c_void, strlen(needle));
}

/// Checks whether bstring begins with NUL-terminated string. Case insensitive.
///
/// Returns 1 if true, otherwise 0.
pub unsafe fn bstr_begins_with_c_nocase(mut haystack: *const bstr_t, mut needle: *const i8) -> i32 {
    return bstr_begins_with_mem_nocase(
        haystack,
        needle as *const core::ffi::c_void,
        strlen(needle),
    );
}

/// Checks whether the bstring begins with the given memory block. Case sensitive.
///
/// Returns 1 if true, otherwise 0.
pub unsafe fn bstr_begins_with_mem(
    mut haystack: *const bstr_t,
    mut _data: *const core::ffi::c_void,
    mut len: usize,
) -> i32 {
    let mut data: *const u8 = _data as *mut u8;
    let mut hdata: *const u8 = if (*haystack).realptr.is_null() {
        (haystack as *mut u8).offset(::std::mem::size_of::<bstr_t>() as isize)
    } else {
        (*haystack).realptr
    };
    let mut hlen: usize = (*haystack).len;
    let mut pos: usize = 0;
    while pos < len && pos < hlen {
        if *hdata.offset(pos as isize) != *data.offset(pos as isize) {
            return 0;
        }
        pos = pos.wrapping_add(1)
    }
    if pos == len {
        return 1;
    } else {
        return 0;
    };
}

/// Checks whether bstring begins with memory block. Case insensitive.
///
/// Returns 1 if true, otherwise 0.
pub unsafe fn bstr_begins_with_mem_nocase(
    mut haystack: *const bstr_t,
    mut _data: *const core::ffi::c_void,
    mut len: usize,
) -> i32 {
    let mut data: *const u8 = _data as *const u8;
    let mut hdata: *const u8 = if (*haystack).realptr.is_null() {
        (haystack as *mut u8).offset(::std::mem::size_of::<bstr_t>() as isize)
    } else {
        (*haystack).realptr
    };
    let mut hlen: usize = (*haystack).len;
    let mut pos: usize = 0;
    while pos < len && pos < hlen {
        if tolower(*hdata.offset(pos as isize) as i32) != tolower(*data.offset(pos as isize) as i32)
        {
            return 0;
        }
        pos = pos.wrapping_add(1)
    }
    if pos == len {
        return 1;
    } else {
        return 0;
    };
}

/// Return the byte at the given position, counting from the end of the string (e.g.,
/// byte at position 0 is the last byte in the string.)
///
/// Returns The byte at the given location, or -1 if the position is out of range.
pub unsafe fn bstr_char_at_end(mut b: *const bstr_t, mut pos: usize) -> i32 {
    let mut data: *mut u8 = if (*b).realptr.is_null() {
        (b as *mut u8).offset(::std::mem::size_of::<bstr_t>() as isize)
    } else {
        (*b).realptr
    };
    let mut len: usize = (*b).len;
    if pos >= len {
        return -1;
    }
    return *data.offset(len.wrapping_sub(1).wrapping_sub(pos) as isize) as i32;
}

/// Remove the last byte from bstring, assuming it contains at least one byte. This
/// function will not reduce the storage that backs the string, only the amount
/// of data used.
pub unsafe fn bstr_chop(mut b: *mut bstr_t) {
    if (*b).len > 0 {
        bstr_adjust_len(b, (*b).len.wrapping_sub(1));
    };
}

/// Return the first position of the provided byte.
///
/// Returns The first position of the byte, or -1 if it could not be found
pub unsafe fn bstr_chr(mut b: *const bstr_t, mut c: i32) -> i32 {
    let mut data: *mut u8 = if (*b).realptr.is_null() {
        (b as *mut u8).offset(::std::mem::size_of::<bstr_t>() as isize)
    } else {
        (*b).realptr
    };
    let mut len: usize = (*b).len;
    let mut i: usize = 0;
    while i < len {
        if *data.offset(i as isize) as i32 == c {
            return i as i32;
        }
        i = i.wrapping_add(1)
    }
    return -1;
}

/// Case-sensitive comparison of two bstrings.
///
/// Returns Zero on string match, 1 if b1 is greater than b2, and -1 if b2 is
///         greater than b1.
#[allow(dead_code)]
pub unsafe fn bstr_cmp(mut b1: *const bstr_t, mut b2: *const bstr_t) -> i32 {
    return bstr_util_cmp_mem(
        if (*b1).realptr.is_null() {
            (b1 as *mut u8).offset(::std::mem::size_of::<bstr_t>() as isize)
        } else {
            (*b1).realptr
        } as *const core::ffi::c_void,
        (*b1).len,
        if (*b2).realptr.is_null() {
            (b2 as *mut u8).offset(::std::mem::size_of::<bstr_t>() as isize)
        } else {
            (*b2).realptr
        } as *const core::ffi::c_void,
        (*b2).len,
    );
}

/// Case-sensitive comparison of a bstring and a NUL-terminated string.
pub unsafe fn bstr_cmp_c(mut b: *const bstr_t, mut c: *const i8) -> i32 {
    return bstr_util_cmp_mem(
        if (*b).realptr.is_null() {
            (b as *mut u8).offset(::std::mem::size_of::<bstr_t>() as isize)
        } else {
            (*b).realptr
        } as *const core::ffi::c_void,
        (*b).len,
        c as *const core::ffi::c_void,
        strlen(c),
    );
}

/// Case-insensitive comparison of a bstring with a NUL-terminated string.
///
/// Returns Zero on string match, 1 if b is greater than cstr, and -1 if cstr is greater than b.
pub unsafe fn bstr_cmp_c_nocase(mut b: *const bstr_t, mut c: *const i8) -> i32 {
    return bstr_util_cmp_mem_nocase(
        if (*b).realptr.is_null() {
            (b as *mut u8).offset(::std::mem::size_of::<bstr_t>() as isize)
        } else {
            (*b).realptr
        } as *const core::ffi::c_void,
        (*b).len,
        c as *const core::ffi::c_void,
        strlen(c),
    );
}

/// Case-insensitive zero-skipping comparison of a bstring with a NUL-terminated string.
///
/// Returns Zero on string match, 1 if b is greater than cstr, and -1 if cstr is greater than b.
pub unsafe fn bstr_cmp_c_nocasenorzero(mut b: *const bstr_t, mut c: *const i8) -> i32 {
    return bstr_util_cmp_mem_nocasenorzero(
        if (*b).realptr.is_null() {
            (b as *mut u8).offset(::std::mem::size_of::<bstr_t>() as isize)
        } else {
            (*b).realptr
        } as *const core::ffi::c_void,
        (*b).len,
        c as *const core::ffi::c_void,
        strlen(c),
    );
}

/// Performs a case-insensitive comparison of a bstring with a memory region.
///
/// Returns Zero ona match, 1 if b is greater than data, and -1 if data is greater than b.
pub unsafe fn bstr_cmp_mem_nocase(
    mut b: *const bstr_t,
    mut data: *const core::ffi::c_void,
    mut len: usize,
) -> i32 {
    return bstr_util_cmp_mem_nocase(
        if (*b).realptr.is_null() {
            (b as *mut u8).offset(::std::mem::size_of::<bstr_t>() as isize)
        } else {
            (*b).realptr
        } as *const core::ffi::c_void,
        (*b).len,
        data,
        len,
    );
}

/// Case-insensitive comparison two bstrings.
///
/// Returns Zero on string match, 1 if b1 is greater than b2, and -1 if b2 is
///         greater than b1.
pub unsafe fn bstr_cmp_nocase(mut b1: *const bstr_t, mut b2: *const bstr_t) -> i32 {
    return bstr_util_cmp_mem_nocase(
        if (*b1).realptr.is_null() {
            (b1 as *mut u8).offset(::std::mem::size_of::<bstr_t>() as isize)
        } else {
            (*b1).realptr
        } as *const core::ffi::c_void,
        (*b1).len,
        if (*b2).realptr.is_null() {
            (b2 as *mut u8).offset(::std::mem::size_of::<bstr_t>() as isize)
        } else {
            (*b2).realptr
        } as *const core::ffi::c_void,
        (*b2).len,
    );
}

// Create a new bstring by copying the provided bstring.
// Returns New bstring, or NULL if memory allocation failed.
pub unsafe fn bstr_dup(mut b: *const bstr_t) -> *mut bstr_t {
    return bstr_dup_ex(b, 0, (*b).len);
}

/// Create a new bstring by copying the provided NUL-terminated string.
///
/// Returns New bstring, or NULL if memory allocation failed.
pub unsafe fn bstr_dup_c(mut cstr: *const i8) -> *mut bstr_t {
    return bstr_dup_mem(cstr as *const core::ffi::c_void, strlen(cstr));
}

// Create a new bstring by copying a part of the provided bstring.
pub unsafe fn bstr_dup_ex(mut b: *const bstr_t, mut offset: usize, mut len: usize) -> *mut bstr_t {
    let mut bnew: *mut bstr_t = bstr_alloc(len);
    if bnew.is_null() {
        return 0 as *mut bstr_t;
    }
    memcpy(
        if (*bnew).realptr.is_null() {
            (bnew as *mut u8).offset(::std::mem::size_of::<bstr_t>() as isize)
        } else {
            (*bnew).realptr
        } as *mut core::ffi::c_void,
        (if (*b).realptr.is_null() {
            (b as *mut u8).offset(::std::mem::size_of::<bstr_t>() as isize)
        } else {
            (*b).realptr
        })
        .offset(offset as isize) as *const core::ffi::c_void,
        len,
    );
    bstr_adjust_len(bnew, len);
    return bnew;
}

/// Create a copy of the provided bstring, then convert it to lowercase.
///
/// Returns New bstring, or NULL if memory allocation failed
pub unsafe fn bstr_dup_lower(mut b: *const bstr_t) -> *mut bstr_t {
    return bstr_to_lowercase(bstr_dup(b));
}

/// Create a new bstring by copying the provided memory region.
///
/// Returns New bstring, or NULL if memory allocation failed
pub unsafe fn bstr_dup_mem(mut data: *const core::ffi::c_void, mut len: usize) -> *mut bstr_t {
    let mut bnew: *mut bstr_t = bstr_alloc(len);
    if bnew.is_null() {
        return 0 as *mut bstr_t;
    }
    memcpy(
        if (*bnew).realptr.is_null() {
            (bnew as *mut u8).offset(::std::mem::size_of::<bstr_t>() as isize)
        } else {
            (*bnew).realptr
        } as *mut core::ffi::c_void,
        data,
        len,
    );
    bstr_adjust_len(bnew, len);
    return bnew;
}

/// Expand internal bstring storage to support at least newsize bytes. The storage
/// is not expanded if the current size is equal or greater to newsize. Because
/// realloc is used underneath, the old pointer to bstring may no longer be valid
/// after this function completes successfully.
///
/// Returns Updated string instance, or NULL if memory allocation failed or if
///         attempt was made to "expand" the bstring to a smaller size.
pub unsafe fn bstr_expand(mut b: *mut bstr_t, mut newsize: usize) -> *mut bstr_t {
    if !(*b).realptr.is_null() {
        // Refuse to expand a wrapped bstring. In the future,
        // we can change this to make a copy of the data, thus
        // leaving the original memory area intact.
        return 0 as *mut bstr_t;
    }
    // Catch attempts to "expand" to a smaller size
    if (*b).size > newsize {
        return 0 as *mut bstr_t;
    }
    let mut bnew: *mut bstr_t = realloc(
        b as *mut core::ffi::c_void,
        (::std::mem::size_of::<bstr_t>()).wrapping_add(newsize),
    ) as *mut bstr_t;
    if bnew.is_null() {
        return 0 as *mut bstr_t;
    }
    bstr_adjust_size(bnew, newsize);
    return bnew;
}

/// Deallocate the supplied bstring instance and set it to NULL. Allows NULL on
/// input.
pub unsafe fn bstr_free(mut b: *mut bstr_t) {
    if b.is_null() {
        return;
    }
    free(b as *mut core::ffi::c_void);
}

/// Find the needle in the haystack, with the needle being a NUL-terminated
/// string.
///
/// Returns Position of the match, or -1 if the needle could not be found.
pub unsafe fn bstr_index_of_c(mut haystack: *const bstr_t, mut needle: *const i8) -> i32 {
    return bstr_index_of_mem(haystack, needle as *const core::ffi::c_void, strlen(needle));
}

/// Find the needle in the haystack, with the needle being a NUL-terminated
/// string. Ignore case differences.
///
/// Returns Position of the match, or -1 if the needle could not be found.
pub unsafe fn bstr_index_of_c_nocase(mut haystack: *const bstr_t, mut needle: *const i8) -> i32 {
    return bstr_index_of_mem_nocase(haystack, needle as *const core::ffi::c_void, strlen(needle));
}

/// Find the needle in the haystack, with the needle being a NUL-terminated
/// string. Ignore case differences. Skip zeroes in haystack
///
/// Returns Position of the match, or -1 if the needle could not be found.
pub unsafe fn bstr_index_of_c_nocasenorzero(
    mut haystack: *const bstr_t,
    mut needle: *const i8,
) -> i32 {
    return bstr_util_mem_index_of_mem_nocasenorzero(
        if (*haystack).realptr.is_null() {
            (haystack as *mut u8).offset(::std::mem::size_of::<bstr_t>() as isize)
        } else {
            (*haystack).realptr
        } as *const core::ffi::c_void,
        (*haystack).len,
        needle as *const core::ffi::c_void,
        strlen(needle),
    );
}

/// Find the needle in the haystack, with the needle being a memory region.
///
/// Returns Position of the match, or -1 if the needle could not be found.
pub unsafe fn bstr_index_of_mem(
    mut haystack: *const bstr_t,
    mut _data2: *const core::ffi::c_void,
    mut len2: usize,
) -> i32 {
    return bstr_util_mem_index_of_mem(
        if (*haystack).realptr.is_null() {
            (haystack as *mut u8).offset(::std::mem::size_of::<bstr_t>() as isize)
        } else {
            (*haystack).realptr
        } as *const core::ffi::c_void,
        (*haystack).len,
        _data2,
        len2,
    );
}

/// Find the needle in the haystack, with the needle being a memory region.
/// Ignore case differences.
///
/// Returns Position of the match, or -1 if the needle could not be found.
pub unsafe fn bstr_index_of_mem_nocase(
    mut haystack: *const bstr_t,
    mut _data2: *const core::ffi::c_void,
    mut len2: usize,
) -> i32 {
    return bstr_util_mem_index_of_mem_nocase(
        if (*haystack).realptr.is_null() {
            (haystack as *mut u8).offset(::std::mem::size_of::<bstr_t>() as isize)
        } else {
            (*haystack).realptr
        } as *const core::ffi::c_void,
        (*haystack).len,
        _data2,
        len2,
    );
}

/// Convert bstring to lowercase. This function converts the supplied string,
/// it does not create a new string.
///
/// Returns The same bstring received on input
pub unsafe fn bstr_to_lowercase(mut b: *mut bstr_t) -> *mut bstr_t {
    if b.is_null() {
        return 0 as *mut bstr_t;
    }
    let mut data: *mut u8 = if (*b).realptr.is_null() {
        (b as *mut u8).offset(::std::mem::size_of::<bstr_t>() as isize)
    } else {
        (*b).realptr
    };
    let mut len: usize = (*b).len;
    let mut i: usize = 0;
    while i < len {
        *data.offset(i as isize) = tolower(*data.offset(i as isize) as i32) as u8;
        i = i.wrapping_add(1)
    }
    return b;
}

/// Case-sensitive comparison of two memory regions.
///
/// Returns Zero if the memory regions are identical, 1 if data1 is greater than
///         data2, and -1 if data2 is greater than data1.
pub unsafe fn bstr_util_cmp_mem(
    mut _data1: *const core::ffi::c_void,
    mut len1: usize,
    mut _data2: *const core::ffi::c_void,
    mut len2: usize,
) -> i32 {
    let mut data1: *const u8 = _data1 as *const u8;
    let mut data2: *const u8 = _data2 as *const u8;
    let mut p1: usize = 0;
    let mut p2: usize = 0;
    while p1 < len1 && p2 < len2 {
        if *data1.offset(p1 as isize) != *data2.offset(p2 as isize) {
            // Difference.
            return if (*data1.offset(p1 as isize)) < *data2.offset(p2 as isize) {
                -1
            } else {
                1
            };
        }
        p1 = p1.wrapping_add(1);
        p2 = p2.wrapping_add(1)
    }
    if p1 == len2 && p2 == len1 {
        // They're identical.
        return 0;
    } else if p1 == len1 {
        return -1;
    } else {
        return 1;
    };
}

/// Case-insensitive comparison of two memory regions.
///
/// Returns Zero if the memory regions are identical, 1 if data1 is greater than
///         data2, and -1 if data2 is greater than data1.
pub unsafe fn bstr_util_cmp_mem_nocase(
    mut _data1: *const core::ffi::c_void,
    mut len1: usize,
    mut _data2: *const core::ffi::c_void,
    mut len2: usize,
) -> i32 {
    let mut data1: *const u8 = _data1 as *const u8;
    let mut data2: *const u8 = _data2 as *const u8;
    let mut p1: usize = 0;
    let mut p2: usize = 0;
    while p1 < len1 && p2 < len2 {
        if tolower(*data1.offset(p1 as isize) as i32) != tolower(*data2.offset(p2 as isize) as i32)
        {
            // One string is shorter.
            // Difference.
            return if tolower(*data1.offset(p1 as isize) as i32)
                < tolower(*data2.offset(p2 as isize) as i32)
            {
                -1
            } else {
                1
            };
        }
        p1 = p1.wrapping_add(1);
        p2 = p2.wrapping_add(1)
    }
    if p1 == len2 && p2 == len1 {
        // They're identical.
        return 0;
    } else if p1 == len1 {
        return -1;
    } else {
        return 1;
    };
}

/// Case-insensitive zero-skipping comparison of two memory regions.
///
/// Returns Zero if the memory regions are identical, 1 if data1 is greater than
///         data2, and -1 if data2 is greater than data1.
///
pub unsafe fn bstr_util_cmp_mem_nocasenorzero(
    mut _data1: *const core::ffi::c_void,
    mut len1: usize,
    mut _data2: *const core::ffi::c_void,
    mut len2: usize,
) -> i32 {
    let mut data1: *const u8 = _data1 as *const u8;
    let mut data2: *const u8 = _data2 as *const u8;
    let mut p1: usize = 0;
    let mut p2: usize = 0;
    while p1 < len1 && p2 < len2 {
        if *data1.offset(p1 as isize) == 0 {
            p1 = p1.wrapping_add(1)
        } else {
            if tolower(*data1.offset(p1 as isize) as i32)
                != tolower(*data2.offset(p2 as isize) as i32)
            {
                // One string is shorter.
                // Difference.
                return if tolower(*data1.offset(p1 as isize) as i32)
                    < tolower(*data2.offset(p2 as isize) as i32)
                {
                    -1
                } else {
                    1
                };
            }
            p1 = p1.wrapping_add(1);
            p2 = p2.wrapping_add(1)
        }
    }
    while p1 < len1 && *data1.offset(p1 as isize) == 0 {
        p1 = p1.wrapping_add(1)
    }
    if p1 == len1 && p2 == len2 {
        // They're identical.
        return 0;
    } else if p1 == len1 {
        return -1;
    } else {
        return 1;
    };
}

/// Convert contents of a memory region to a positive integer.
///
/// If the conversion was successful, this function returns the
/// number. When the conversion fails, -1 will be returned when not
/// one valid digit was found, and -2 will be returned if an overflow
/// occurred.
pub unsafe fn bstr_util_mem_to_pint(
    mut _data: *const core::ffi::c_void,
    mut len: usize,
    mut base: i32,
    mut lastlen: *mut usize,
) -> i64 {
    let mut data: *const u8 = _data as *mut u8;
    let mut rval: i64 = 0;
    let mut tflag: i64 = 0;
    let mut i: usize = 0;
    *lastlen = i;
    while i < len {
        let mut d: i32 = *data.offset(i as isize) as i32;
        *lastlen = i;
        // One string is shorter.
        // Convert character to digit.
        if d >= '0' as i32 && d <= '9' as i32 {
            d -= '0' as i32
        } else if d >= 'a' as i32 && d <= 'z' as i32 {
            d -= 'a' as i32 - 10
        } else if d >= 'A' as i32 && d <= 'Z' as i32 {
            d -= 'A' as i32 - 10
        } else {
            d = -1
        }
        // Check that the digit makes sense with the base we are using.
        if d == -1 || d >= base {
            if tflag != 0 {
                // Return what we have so far; lastlen points
                // to the first non-digit position.
                return rval;
            } else {
                // We didn't see a single digit.
                return -1;
            }
        }
        if tflag != 0 {
            if ((9223372036854775807 as i64 - d as i64) / base as i64) < rval {
                // Overflow
                return -2;
            }
            rval *= base as i64;
            rval += d as i64
        } else {
            rval = d as i64;
            tflag = 1
        }
        i = i.wrapping_add(1)
    }
    *lastlen = i.wrapping_add(1);
    return rval;
}

/// Searches a memory block for the given NUL-terminated string. Case insensitive.
///
/// Returns Index of the first location of the needle on success, or -1 if the needle was not found.
pub unsafe fn bstr_util_mem_index_of_c_nocase(
    mut _data1: *const core::ffi::c_void,
    mut len1: usize,
    mut cstr: *const i8,
) -> i32 {
    return bstr_util_mem_index_of_mem_nocase(
        _data1,
        len1,
        cstr as *const core::ffi::c_void,
        strlen(cstr),
    );
}

/// Searches the haystack memory block for the needle memory block. Case sensitive.
///
/// Returns Index of the first location of the needle on success, or -1 if the needle was not found.
pub unsafe fn bstr_util_mem_index_of_mem(
    mut _data1: *const core::ffi::c_void,
    mut len1: usize,
    mut _data2: *const core::ffi::c_void,
    mut len2: usize,
) -> i32 {
    let mut data1: *const u8 = _data1 as *mut u8;
    let mut data2: *const u8 = _data2 as *mut u8;
    let mut i: usize = 0;
    let mut j: usize = 0;
    // If we ever want to optimize this function, the following link
    // might be useful: http://en.wikipedia.org/wiki/Knuth-Morris-Pratt_algorithm
    while i < len1 {
        let mut k: usize = i;
        j = 0;
        while j < len2 && k < len1 {
            if *data1.offset(k as isize) != *data2.offset(j as isize) {
                break;
            }
            j = j.wrapping_add(1);
            k = k.wrapping_add(1)
        }
        if j == len2 {
            return i as i32;
        }
        i = i.wrapping_add(1)
    }
    return -1;
}

/// Searches the haystack memory block for the needle memory block. Case sensitive.
///
/// Returns Index of the first location of the needle on success, or -1 if the needle was not found.
pub unsafe fn bstr_util_mem_index_of_mem_nocase(
    mut _data1: *const core::ffi::c_void,
    mut len1: usize,
    mut _data2: *const core::ffi::c_void,
    mut len2: usize,
) -> i32 {
    let mut data1: *const u8 = _data1 as *mut u8;
    let mut data2: *const u8 = _data2 as *mut u8;
    let mut i: usize = 0;
    let mut j: usize = 0;
    // If we ever want to optimize this function, the following link
    // might be useful: http://en.wikipedia.org/wiki/Knuth-Morris-Pratt_algorithm
    while i < len1 {
        let mut k: usize = i;
        j = 0;
        while j < len2 && k < len1 {
            if toupper(*data1.offset(k as isize) as i32)
                != toupper(*data2.offset(j as isize) as i32)
            {
                break;
            }
            j = j.wrapping_add(1);
            k = k.wrapping_add(1)
        }
        if j == len2 {
            return i as i32;
        }
        i = i.wrapping_add(1)
    }
    return -1;
}

/// Searches the haystack memory block for the needle memory block. Case sensitive. Skips zeroes in data1
///
/// Returns Index of the first location of the needle on success, or -1 if the needle was not found.
pub unsafe fn bstr_util_mem_index_of_mem_nocasenorzero(
    mut _data1: *const core::ffi::c_void,
    mut len1: usize,
    mut _data2: *const core::ffi::c_void,
    mut len2: usize,
) -> i32 {
    let mut data1: *const u8 = _data1 as *mut u8;
    let mut data2: *const u8 = _data2 as *mut u8;
    let mut i: usize = 0;
    let mut j: usize = 0;
    // If we ever want to optimize this function, the following link
    // might be useful: http://en.wikipedia.org/wiki/Knuth-Morris-Pratt_algorithm
    while i < len1 {
        let mut k: usize = i;
        if !(*data1.offset(i as isize) == 0) {
            j = 0;
            while j < len2 && k < len1 {
                if *data1.offset(k as isize) == 0 {
                    j = j.wrapping_sub(1)
                } else if toupper(*data1.offset(k as isize) as i32)
                    != toupper(*data2.offset(j as isize) as i32)
                {
                    break;
                }
                j = j.wrapping_add(1);
                k = k.wrapping_add(1)
            }
            if j == len2 {
                return i as i32;
            }
        }
        // skip leading zeroes to avoid quadratic complexity
        i = i.wrapping_add(1)
    }
    return -1;
}

/// Removes whitespace from the beginning and the end of a memory region. The data
/// itself is not modified; this function only adjusts the provided pointers.
pub unsafe fn bstr_util_mem_trim(mut data: *mut *mut u8, mut len: *mut usize) {
    if data.is_null() || len.is_null() {
        return;
    }
    let mut d: *mut u8 = *data;
    let mut l: usize = *len;
    // Ignore whitespace at the beginning.
    let mut pos: usize = 0;
    while pos < l
        && *(*__ctype_b_loc()).offset(*d.offset(pos as isize) as isize) as i32 & _ISspace != 0
    {
        pos = pos.wrapping_add(1)
    }
    d = d.offset(pos as isize);
    l = (l).wrapping_sub(pos);
    // Ignore whitespace at the end.
    while l > 0
        && *(*__ctype_b_loc()).offset(*d.offset(l.wrapping_sub(1) as isize) as isize) as i32
            & _ISspace
            != 0
    {
        l = l.wrapping_sub(1)
    }
    *data = d;
    *len = l;
}

/// Take the provided memory region, allocate a new memory buffer, and construct
/// a NUL-terminated string, replacing each NUL byte with "\0" (two bytes). The
/// caller is responsible to keep track of the allocated memory area and free
/// it once it is no longer needed.
///
/// Returns The newly created NUL-terminated string, or NULL in case of memory
///         allocation failure.
pub unsafe fn bstr_util_memdup_to_c(
    mut _data: *const core::ffi::c_void,
    mut len: usize,
) -> *mut i8 {
    let mut data: *const u8 = _data as *mut u8;
    // Count how many NUL bytes we have in the string.
    let mut i: usize = 0;
    let mut nulls: usize = 0;
    while i < len {
        if *data.offset(i as isize) == '\u{0}' as u8 {
            nulls = nulls.wrapping_add(1)
        }
        i = i.wrapping_add(1)
    }
    // Now copy the string into a NUL-terminated buffer.
    let mut r: *mut i8 = 0 as *mut i8;
    let mut d: *mut i8 = 0 as *mut i8;
    d = malloc(len.wrapping_add(nulls).wrapping_add(1)) as *mut i8;
    r = d;
    if d.is_null() {
        return 0 as *mut i8;
    }
    loop {
        let fresh0 = len;
        len = len.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        if *data == '\u{0}' as u8 {
            data = data.offset(1);
            let fresh1 = d;
            d = d.offset(1);
            *fresh1 = '\\' as i8;
            let fresh2 = d;
            d = d.offset(1);
            *fresh2 = '0' as i8
        } else {
            let fresh3 = data;
            data = data.offset(1);
            let fresh4 = d;
            d = d.offset(1);
            *fresh4 = *fresh3 as i8
        }
    }
    *d = '\u{0}' as i8;
    return r;
}

/// Create a new NUL-terminated string out of the provided bstring. If NUL bytes
/// are contained in the bstring, each will be replaced with "\0" (two characters).
/// The caller is responsible to keep track of the allocated memory area and free
/// it once it is no longer needed.
pub unsafe fn bstr_util_strdup_to_c(mut b: *const bstr_t) -> *mut i8 {
    if b.is_null() {
        return 0 as *mut i8;
    }
    return bstr_util_memdup_to_c(
        if (*b).realptr.is_null() {
            (b as *mut u8).offset(::std::mem::size_of::<bstr_t>() as isize)
        } else {
            (*b).realptr
        } as *const core::ffi::c_void,
        (*b).len,
    );
}

/// Create a new bstring from the provided memory buffer without
/// copying the data. The caller must ensure that the buffer remains
/// valid for as long as the bstring is used.
///
/// Returns New bstring, or NULL on memory allocation failure.
pub unsafe fn bstr_wrap_mem(mut data: *const core::ffi::c_void, mut len: usize) -> *mut bstr_t {
    let mut b: *mut bstr_t = malloc(::std::mem::size_of::<bstr_t>()) as *mut bstr_t;
    if b.is_null() {
        return 0 as *mut bstr_t;
    }
    (*b).len = len;
    (*b).size = (*b).len;
    (*b).realptr = data as *mut u8;
    return b;
}
