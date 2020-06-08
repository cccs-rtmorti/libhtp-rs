use bstr::{BString, ByteSlice, B};
use core::cmp::Ordering;
use std::boxed::Box;
use std::ffi::CStr;
use std::ops::{Deref, DerefMut};
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::size_t) -> *mut core::ffi::c_void;
}

#[derive(Clone, Debug)]
pub struct bstr_t {
    // Wrap a BString under the hood. We want to be able to
    // implement behaviours on top of this if needed, so we wrap
    // it instead of exposing it directly in our public API.
    s: BString,
}

impl bstr_t {
    /// Make a new owned bstr_t
    pub fn new() -> Self {
        bstr_t {
            s: BString::from(Vec::new()),
        }
    }

    /// Make a new owned bstr_t with given capacity
    pub fn with_capacity(len: usize) -> Self {
        bstr_t {
            s: BString::from(Vec::with_capacity(len)),
        }
    }

    /// Compare this bstr with the given slice
    pub fn cmp<B: AsRef<[u8]>>(&self, other: B) -> Ordering {
        self.as_slice().cmp(other.as_ref())
    }

    /// Compare bstr with the given slice, ingnoring ascii case.
    pub fn cmp_nocase<B: AsRef<[u8]>>(&self, other: B) -> Ordering {
        let lefts = &self.as_slice();
        let rights = &other.as_ref();
        let left = LowercaseIterator::new(lefts);
        let right = LowercaseIterator::new(rights);
        left.cmp(right)
    }

    /// Case insensitive comparison between self and other, ignoring any zeros in self
    pub fn cmp_nocase_nozero<B: AsRef<[u8]>>(&self, other: B) -> Ordering {
        let lefts = &self.as_slice();
        let rights = &other.as_ref();
        let left = LowercaseNoZeroIterator::new(lefts);
        let right = LowercaseIterator::new(rights);
        left.cmp(right)
    }

    /// Extend this bstr with the given slice
    pub fn add<B: AsRef<[u8]>>(&mut self, other: B) {
        self.extend_from_slice(other.as_ref())
    }

    /// Extend the bstr as much as possible without growing
    pub fn add_noex<B: AsRef<[u8]>>(&mut self, other: B) {
        let len = std::cmp::min(self.capacity() - self.len(), other.as_ref().len());
        self.add(&other.as_ref()[..len]);
    }

    /// Return true if this bstr starts with other
    pub fn starts_with<B: AsRef<[u8]>>(&self, other: B) -> bool {
        self.as_slice().starts_with(other.as_ref())
    }

    /// Return true if this bstr starts with other, ignoring ascii case
    pub fn starts_with_nocase<B: AsRef<[u8]>>(&self, other: B) -> bool {
        if self.len() < other.as_ref().len() {
            return false;
        }
        let len: usize = std::cmp::min(self.len(), other.as_ref().len());
        self.as_slice()[..len].eq_ignore_ascii_case(&other.as_ref()[..len])
    }

    /// Find the index of the given slice
    pub fn index_of<B: AsRef<[u8]>>(&self, other: B) -> Option<usize> {
        self.find(other.as_ref())
    }

    /// Find the index of the given slice ignoring ascii case
    pub fn index_of_nocase<B: AsRef<[u8]>>(&self, other: B) -> Option<usize> {
        let src = &self.as_slice()[..];
        let mut haystack = LowercaseIterator::new(&src);
        let needle = other.as_ref().to_ascii_lowercase();
        haystack.index_of(&needle)
    }

    /// Find the index of the given slice ignoring ascii case and any zeros in self
    pub fn index_of_nocase_nozero<B: AsRef<[u8]>>(&self, other: B) -> Option<usize> {
        let src = &self.as_slice()[..];
        let mut haystack = LowercaseNoZeroIterator::new(&src);
        let needle = other.as_ref().to_ascii_lowercase();
        haystack.index_of(&needle)
    }
}

// Trait Implementations for bstr_t

/// Let callers access BString functions
impl Deref for bstr_t {
    type Target = BString;

    fn deref(&self) -> &Self::Target {
        &self.s
    }
}

/// Let callers access mutable BString functions
impl DerefMut for bstr_t {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.s
    }
}

/// Construct a new bstr_t from anything that satisfies AsRef<[u8]>
impl<T: AsRef<[u8]>> From<T> for bstr_t {
    fn from(src: T) -> Self {
        bstr_t {
            s: BString::from(src.as_ref()),
        }
    }
}

/// A trait that lets us find the byte index of slices in a generic way.
///
/// This layer of abstraction is motivated by the need to find needle in
/// haystack when we want to perform case sensitive, case insensitive, and
/// case insensitive + zero skipping. All of these algorithms are identical
/// except we compare the needle bytes with the src bytes in different ways,
/// and in the case of zero skipping we want to pretend that zero bytes in
/// the haystack do not exist. So we define iterators for each of lowercase
/// and lowercase + zero skipping, and then implement this trait for both of
/// those, and then define the search function in terms of this trait.
trait SubIterator: Iterator<Item = u8> {
    /// Return a new iterator of the same type starting at the current byte index
    fn subiter(&self) -> Self;
    /// Return the current byte index into the iterator
    fn index(&self) -> usize;
    /// Find the given needle in self and return the byte index
    fn index_of(&mut self, needle: impl AsRef<[u8]>) -> Option<usize>;
}

/// Find the byte index of the given slice in the source.
///
/// Someday an enterprising soul can implement this function inside SubIterator
/// directly (where it arguably belongs), but this involves handling dyn Self,
/// and implementing it this way lets monomorphization emit concrete
/// implementations for each of the two types we actually have.
fn index_of<T: SubIterator, S: AsRef<[u8]>>(haystack: &mut T, needle: &S) -> Option<usize> {
    let first = needle.as_ref().first()?;
    let mut s = haystack.next();
    while s.is_some() {
        if s.unwrap() == *first {
            let mut test = haystack.subiter();
            let mut equal = false;
            for cmp_byte in needle.as_ref().as_bytes() {
                let b = test.next();
                equal = b.is_some() && b.unwrap() == *cmp_byte;
                if !equal {
                    break;
                }
            }
            if equal {
                return Some(haystack.index());
            }
        }
        s = haystack.next();
    }
    None
}

/// A convenience iterator for anything that satisfies AsRef<[u8]>
/// that yields lowercase ascii bytes and skips null bytes
struct LowercaseNoZeroIterator<'a, T: AsRef<[u8]>> {
    src: &'a T,
    idx: usize,
    first: bool,
}

impl<'a, T: AsRef<[u8]>> LowercaseNoZeroIterator<'a, T> {
    fn new(src: &'a T) -> Self {
        LowercaseNoZeroIterator {
            src,
            idx: 0,
            first: true,
        }
    }
}

impl<T: AsRef<[u8]>> Iterator for LowercaseNoZeroIterator<'_, T> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.first {
                self.first = false;
            } else {
                self.idx += 1;
            }
            let next = if let Some(c) = self.src.as_ref().get(self.idx) {
                Some(c.to_ascii_lowercase())
            } else {
                None
            };
            if next != Some(0) {
                break next;
            }
        }
    }
}

impl<'a, T: AsRef<[u8]>> SubIterator for LowercaseNoZeroIterator<'_, T> {
    fn subiter(&self) -> Self {
        LowercaseNoZeroIterator {
            src: &self.src,
            idx: self.idx,
            first: true,
        }
    }

    fn index(&self) -> usize {
        self.idx
    }

    fn index_of(&mut self, needle: impl AsRef<[u8]>) -> Option<usize> {
        index_of(self, &needle)
    }
}

/// A convenience iterator for anything that satisfies AsRef<[u8]>
/// that yields lowercase ascii bytes
struct LowercaseIterator<'a, T: AsRef<[u8]>> {
    src: &'a T,
    idx: usize,
    first: bool,
}

impl<'a, T: AsRef<[u8]>> LowercaseIterator<'a, T> {
    fn new(src: &'a T) -> Self {
        LowercaseIterator {
            src,
            idx: 0,
            first: true,
        }
    }
}

impl<T: AsRef<[u8]>> Iterator for LowercaseIterator<'_, T> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
        } else {
            self.idx += 1;
        }
        if let Some(c) = self.src.as_ref().get(self.idx) {
            Some(c.to_ascii_lowercase())
        } else {
            None
        }
    }
}

impl<'a, T: AsRef<[u8]>> SubIterator for LowercaseIterator<'_, T> {
    fn subiter(&self) -> Self {
        LowercaseIterator {
            src: &self.src,
            idx: self.idx,
            first: true,
        }
    }

    fn index(&self) -> usize {
        self.idx
    }

    fn index_of(&mut self, needle: impl AsRef<[u8]>) -> Option<usize> {
        index_of(self, &needle)
    }
}

/// A convenience macro to turn Ordering into an integer mapping
/// Ordering::Less => -1,
/// Ordering::Equal => 0,
/// Ordering::Greater => 1,
macro_rules! ordering2int {
    ( $ord:expr ) => {
        match $ord {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    };
}

// C-Style interface

/// Return the length of the string
pub unsafe fn bstr_len(x: *const bstr_t) -> usize {
    (*x).len()
}

/// Return the capacity of the string
pub unsafe fn bstr_size(x: *const bstr_t) -> usize {
    (*x).capacity()
}

/// Return a pointer to the underlying vector
pub unsafe fn bstr_ptr(x: *const bstr_t) -> *mut u8 {
    (*x).as_ptr() as *mut u8
}

/// Allocate a zero-length bstring, reserving space for at least size bytes.
///
/// Returns New string instance
pub unsafe fn bstr_alloc(len: usize) -> *mut bstr_t {
    let b = bstr_t {
        s: BString::from(Vec::with_capacity(len)),
    };
    let boxed = Box::new(b);
    Box::into_raw(boxed)
}

/// Deallocate the supplied bstring instance. Allows NULL on input.
pub unsafe fn bstr_free(b: *mut bstr_t) {
    if !b.is_null() {
        // b will be dropped when this box goes out of scope
        Box::from_raw(b);
    }
}

/// Append source bstring to destination bstring, growing destination if
/// necessary. If the destination bstring is expanded, the pointer will change.
/// You must replace the original destination pointer with the returned one.
///
/// Returns Updated bstring
pub unsafe fn bstr_add(destination: *mut bstr_t, source: *const bstr_t) -> *mut bstr_t {
    nullcheck!(destination, source);

    (*destination).add((*source).as_slice());
    destination
}

/// Append a NUL-terminated source to destination, growing destination if
/// necessary. If the string is expanded, the pointer will change. You must
/// replace the original destination pointer with the returned one.
///
/// Returns Updated bstring
pub unsafe fn bstr_add_c(destination: *mut bstr_t, csource: *const i8) -> *mut bstr_t {
    nullcheck!(destination, csource);

    let cs = CStr::from_ptr(csource);
    (*destination).add(cs.to_bytes());
    destination
}

/// Append as many bytes from the source to destination bstring. The
/// destination storage will not be expanded if there is not enough space in it
/// already to accommodate all of the data.
pub unsafe fn bstr_add_c_noex(destination: *mut bstr_t, source: *const i8) -> *mut bstr_t {
    nullcheck!(destination, source);

    let cs = CStr::from_ptr(source);
    (*destination).add_noex(&cs.to_bytes());
    destination
}

/// Append a memory region to destination, growing destination if necessary. If
/// the string is expanded, the pointer will change. You must replace the
/// original destination pointer with the returned one.
///
/// Returns Updated bstring
pub unsafe fn bstr_add_mem(
    destination: *mut bstr_t,
    data: *const core::ffi::c_void,
    len: usize,
) -> *mut bstr_t {
    nullcheck!(destination, data);

    let s = std::slice::from_raw_parts(data as *const u8, len);
    (*destination).add(s);
    destination
}

/// Append as many bytes from the source to destination bstring. The
/// destination storage will not be expanded if there is not enough space in it
/// already to accommodate all of the data.
///
/// Returns The destination bstring.
pub unsafe fn bstr_add_mem_noex(
    destination: *mut bstr_t,
    data: *const core::ffi::c_void,
    len: usize,
) -> *mut bstr_t {
    nullcheck!(destination, data);

    let s = std::slice::from_raw_parts(data as *const u8, len);
    (*destination).add_noex(s);
    destination
}

/// Append as many bytes from the source bstring to destination bstring. The
/// destination storage will not be expanded if there is not enough space in it
/// already to accommodate all of the data.
pub unsafe fn bstr_add_noex(destination: *mut bstr_t, source: *const bstr_t) -> *mut bstr_t {
    nullcheck!(destination, source);

    (*destination).add_noex((*source).as_slice());
    destination
}

/// Adjust bstring length. You will need to use this method whenever
/// you work directly with the string contents, and end up changing
/// its length by direct structure manipulation.
pub unsafe fn bstr_adjust_len(b: *mut bstr_t, newlen: usize) {
    nullcheck!(b);
    // FIXME: This is wildly unsafe. This function should not exist. It only
    // exists because some callers grab the mut pointer and mess with the
    // bstr contents. We should find all these callers and give them nice
    // APIs to do the things they want safely, and then they won't care about
    // adjusting the length, and then this function can die.
    (*b).set_len(newlen)
}

/// Adjust bstring size. This does not change the size of the storage behind
/// the bstring, just changes the field that keeps track of how many bytes
/// there are in the storage. You will need to use this function only if
/// you're messing with bstr_t internals. Use with caution.
pub unsafe fn bstr_adjust_size(mut _b: *mut bstr_t, mut _newsize: usize) {
    // FIXME: This really shouldn't exist. What it wants to do
    // doesn't map to the universe where the underlying datatype
    // knows how big it is, so this is a no-op.
}

/// Checks whether bstring begins with another bstring. Case sensitive.
///
/// Returns 1 if true, otherwise 0.
pub unsafe fn bstr_begins_with(haystack: *const bstr_t, needle: *const bstr_t) -> i32 {
    nullcheck!(haystack, needle);
    (*haystack).starts_with((*needle).as_slice()) as i32
}

/// Checks whether bstring begins with NUL-terminated string. Case sensitive.
///
/// Returns 1 if true, otherwise 0.
pub unsafe fn bstr_begins_with_c(haystack: *const bstr_t, needle: *const i8) -> i32 {
    nullcheck!(haystack, needle);

    let cs = CStr::from_ptr(needle);
    (*haystack).starts_with(cs.to_bytes()) as i32
}

/// Checks whether bstring begins with NUL-terminated string. Case insensitive.
///
/// Returns 1 if true, otherwise 0.
pub unsafe fn bstr_begins_with_c_nocase(haystack: *const bstr_t, needle: *const i8) -> i32 {
    nullcheck!(haystack, needle);

    let cs = CStr::from_ptr(needle);
    (*haystack).starts_with_nocase(cs.to_bytes()) as i32
}

/// Checks whether bstring begins with another bstring. Case insensitive.
///
/// Returns 1 if true, otherwise 0.
pub unsafe fn bstr_begins_with_nocase(haystack: *const bstr_t, needle: *const bstr_t) -> i32 {
    nullcheck!(haystack, needle);
    (*haystack).starts_with_nocase((*needle).as_slice()) as i32
}

/// Checks whether the bstring begins with the given memory block. Case sensitive.
///
/// Returns 1 if true, otherwise 0.
pub unsafe fn bstr_begins_with_mem(
    haystack: *const bstr_t,
    data: *const core::ffi::c_void,
    len: usize,
) -> i32 {
    nullcheck!(haystack, data);

    let s = std::slice::from_raw_parts(data as *const u8, len);
    (*haystack).starts_with(s) as i32
}

/// Checks whether bstring begins with memory block. Case insensitive.
///
/// Returns 1 if true, otherwise 0.
pub unsafe fn bstr_begins_with_mem_nocase(
    haystack: *const bstr_t,
    data: *const core::ffi::c_void,
    len: usize,
) -> i32 {
    nullcheck!(haystack, data);

    let s = std::slice::from_raw_parts(data as *const u8, len);
    (*haystack).starts_with_nocase(s) as i32
}

/// Return the byte at the given position.
///
/// Returns The byte at the given location, or -1 if the position is out of range.
pub unsafe fn bstr_char_at(b: *const bstr_t, pos: usize) -> i32 {
    nullcheck!(b);

    match (*b).get(pos) {
        Some(c) => *c as i32,
        None => -1,
    }
}

/// Return the byte at the given position, counting from the end of the string (e.g.,
/// byte at position 0 is the last byte in the string.)
///
/// Returns The byte at the given location, or -1 if the position is out of range.
pub unsafe fn bstr_char_at_end(b: *const bstr_t, pos: usize) -> i32 {
    nullcheck!(b);

    if let Some(idx) = bstr_len(b).checked_sub(pos + 1) {
        if let Some(c) = (*b).get(idx) {
            return *c as i32;
        }
    }
    -1
}

/// Remove the last byte from bstring, assuming it contains at least one byte. This
/// function will not reduce the storage that backs the string, only the amount
/// of data used.
pub unsafe fn bstr_chop(b: *mut bstr_t) {
    nullcheck!(b);

    (*b).pop();
}

/// Return the first position of the provided byte.
///
/// Returns The first position of the byte, or -1 if it could not be found
pub unsafe fn bstr_chr(b: *const bstr_t, c: i32) -> i32 {
    nullcheck!(b);

    if let Some(idx) = (*b).find_byte(c as u8) {
        return idx as i32;
    }
    -1
}

/// Case-sensitive comparison of two bstrings.
///
/// Returns Zero on string match, 1 if b1 is greater than b2, and -1 if b2 is
///         greater than b1.
pub unsafe fn bstr_cmp(b1: *const bstr_t, b2: *const bstr_t) -> i32 {
    nullcheck!(b1, b2);

    ordering2int!((*b1).cmp(&(*b2).as_slice()))
}

/// Case-sensitive comparison of a bstring and a NUL-terminated string.
pub unsafe fn bstr_cmp_c(b: *const bstr_t, c: *const i8) -> i32 {
    nullcheck!(b, c);

    let cs = CStr::from_ptr(c);
    ordering2int!((*b).cmp(cs.to_bytes()))
}

/// Case-insensitive comparison of a bstring with a NUL-terminated string.
///
/// Returns Zero on string match, 1 if b is greater than cstr, and -1 if cstr is greater than b.
pub unsafe fn bstr_cmp_c_nocase(b: *const bstr_t, c: *const i8) -> i32 {
    nullcheck!(b, c);

    let cs = CStr::from_ptr(c);
    ordering2int!((*b).cmp_nocase(cs.to_bytes()))
}

/// Case-insensitive zero-skipping comparison of a bstring with a NUL-terminated string.
///
/// Returns Zero on string match, 1 if b is greater than cstr, and -1 if cstr is greater than b.
pub unsafe fn bstr_cmp_c_nocasenorzero(b: *const bstr_t, c: *const i8) -> i32 {
    nullcheck!(b, c);

    let cs = CStr::from_ptr(c);
    ordering2int!((*b).cmp_nocase_nozero(cs.to_bytes()))
}

/// Case-insensitive comparison two bstrings.
///
/// Returns Zero on string match, 1 if b1 is greater than b2, and -1 if b2 is
///         greater than b1.
pub unsafe fn bstr_cmp_nocase(b1: *const bstr_t, b2: *const bstr_t) -> i32 {
    nullcheck!(b1, b2);

    ordering2int!((*b1).cmp_nocase((*b2).as_slice()))
}

/// Create a new bstring by copying the provided bstring.
///
/// Returns New bstring, or NULL if memory allocation failed.
pub unsafe fn bstr_dup(b: *const bstr_t) -> *mut bstr_t {
    nullcheck!(b);

    // Normally all of these would just be b.clone(), but because
    // the memory management here is still C-like in that this function
    // just returns a pointer, it's more sane to keep running everything
    // through bstr_alloc() to get the heap / Box stuff right, and then
    // we can transition callers to using clone() instead of these
    // functions, then we can delete bstr_alloc(), and be normal.
    let new = bstr_alloc((*b).len());
    (*new).add((*b).as_slice());
    new
}

/// Create a new bstring by copying the provided NUL-terminated string.
///
/// Returns New bstring, or NULL if memory allocation failed.
pub unsafe fn bstr_dup_c(cstr: *const i8) -> *mut bstr_t {
    nullcheck!(cstr);

    let cs = CStr::from_ptr(cstr).to_bytes();
    let new = bstr_alloc(cs.len());
    (*new).add(cs);
    new
}

/// Create a new bstring by copying a part of the provided bstring.
pub unsafe fn bstr_dup_ex(b: *const bstr_t, offset: usize, len: usize) -> *mut bstr_t {
    nullcheck!(b);

    let start = offset;
    let end = offset + len;
    if end > (*b).len() {
        return std::ptr::null_mut();
    }

    let new = bstr_alloc(len);
    (*new).add(&(*b).as_slice()[start..end]);
    new
}

/// Create a copy of the provided bstring, then convert it to lowercase.
///
/// Returns New bstring, or NULL if memory allocation failed
pub unsafe fn bstr_dup_lower(b: *const bstr_t) -> *mut bstr_t {
    nullcheck!(b);

    let new = bstr_alloc((*b).len());
    (*new).add(&(*b).as_slice());
    (*new).make_ascii_lowercase();
    new
}

/// Create a new bstring by copying the provided memory region.
///
/// Returns New bstring, or NULL if memory allocation failed
pub unsafe fn bstr_dup_mem(data: *const core::ffi::c_void, len: usize) -> *mut bstr_t {
    let new = bstr_alloc(len);

    if !data.is_null() {
        let s = std::slice::from_raw_parts(data as *const u8, len);
        (*new).add(s);
    }
    new
}

/// Create a new bstring by copying the provided bstring.
///
/// Returns New bstring, or NULL if memory allocation failed.
pub unsafe fn bstr_clone(b: &bstr_t) -> *mut bstr_t {
    let new = bstr_alloc(b.len());
    (*new).add(b.as_slice());
    new
}

/// Expand internal bstring storage to support at least newsize bytes. The storage
/// is not expanded if the current size is equal or greater to newsize. Because
/// realloc is used underneath, the old pointer to bstring may no longer be valid
/// after this function completes successfully.
///
/// Returns Updated string instance, or NULL if memory allocation failed or if
///         attempt was made to "expand" the bstring to a smaller size.
pub unsafe fn bstr_expand(b: *mut bstr_t, newsize: usize) -> *mut bstr_t {
    if b.is_null() {
        return std::ptr::null_mut();
    }

    let newsize = newsize;
    if newsize <= (*b).capacity() {
        return std::ptr::null_mut();
    }

    let additional = newsize - (*b).len();
    (*b).reserve(additional);
    b
}

/// Find the needle in the haystack.
///
/// Returns Position of the match, or -1 if the needle could not be found.
pub unsafe fn bstr_index_of(haystack: *const bstr_t, needle: *const bstr_t) -> i32 {
    nullcheck!(haystack, needle);

    match (*haystack).find((*needle).as_slice()) {
        Some(idx) => idx as i32,
        None => -1,
    }
}

/// Find the needle in the haystack, with the needle being a NUL-terminated
/// string.
///
/// Returns Position of the match, or -1 if the needle could not be found.
pub unsafe fn bstr_index_of_c(haystack: *const bstr_t, needle: *const i8) -> i32 {
    nullcheck!(haystack, needle);

    let cs = CStr::from_ptr(needle);
    match (*haystack).find(cs.to_bytes()) {
        Some(idx) => idx as i32,
        None => -1,
    }
}

/// Find the needle in the haystack, with the needle being a NUL-terminated
/// string. Ignore case differences.
///
/// Returns Position of the match, or -1 if the needle could not be found.
pub unsafe fn bstr_index_of_c_nocase(haystack: *const bstr_t, needle: *const i8) -> i32 {
    nullcheck!(haystack, needle);

    let cs = CStr::from_ptr(needle);
    match (*haystack).index_of_nocase(cs.to_bytes()) {
        Some(idx) => idx as i32,
        None => -1,
    }
}

/// Find the needle in the haystack, with the needle being a NUL-terminated
/// string. Ignore case differences. Skip zeroes in haystack
///
/// Returns Position of the match, or -1 if the needle could not be found.
pub unsafe fn bstr_index_of_c_nocasenorzero(haystack: *const bstr_t, needle: *const i8) -> i32 {
    nullcheck!(haystack, needle);

    let cs = CStr::from_ptr(needle);
    match (*haystack).index_of_nocase_nozero(cs.to_bytes()) {
        Some(idx) => idx as i32,
        None => -1,
    }
}

/// Find the needle in the haystack, with the needle being a memory region.
///
/// Returns Position of the match, or -1 if the needle could not be found.
pub unsafe fn bstr_index_of_mem(
    haystack: *const bstr_t,
    data: *const core::ffi::c_void,
    len: usize,
) -> i32 {
    nullcheck!(haystack, data);

    let s = std::slice::from_raw_parts(data as *const u8, len);
    match (*haystack).index_of(s) {
        Some(idx) => idx as i32,
        None => -1,
    }
}

/// Find the needle in the haystack, with the needle being a memory region.
/// Ignore case differences.
///
/// Returns Position of the match, or -1 if the needle could not be found.
pub unsafe fn bstr_index_of_mem_nocase(
    haystack: *const bstr_t,
    data: *const core::ffi::c_void,
    len: usize,
) -> i32 {
    nullcheck!(haystack, data);

    let s = std::slice::from_raw_parts(data as *const u8, len);
    match (*haystack).index_of_nocase(s) {
        Some(idx) => idx as i32,
        None => -1,
    }
}

/// Find the needle in the haystack, ignoring case differences.
///
/// Returns Position of the match, or -1 if the needle could not be found.
pub unsafe fn bstr_index_of_nocase(haystack: *const bstr_t, needle: *const bstr_t) -> i32 {
    nullcheck!(haystack, needle);

    match (*haystack).index_of_nocase((*needle).as_slice()) {
        Some(idx) => idx as i32,
        None => -1,
    }
}

/// Return the last position of a character (byte).
///
/// Returns The last position of the character, or -1 if it could not be found.
pub unsafe fn bstr_rchr(b: *const bstr_t, c: i32) -> i32 {
    nullcheck!(b);

    match (*b).rfind_byte(c as u8) {
        Some(idx) => idx as i32,
        None => -1,
    }
}

/// Convert bstring to lowercase. This function converts the supplied string,
/// it does not create a new string.
///
/// Returns The same bstring received on input
pub unsafe fn bstr_to_lowercase(b: *mut bstr_t) -> *mut bstr_t {
    if b.is_null() {
        return std::ptr::null_mut();
    }

    (*b).make_ascii_lowercase();
    b
}

/// Case-sensitive comparison of two memory regions.
///
/// Returns Zero if the memory regions are identical, 1 if data1 is greater than
///         data2, and -1 if data2 is greater than data1.
pub unsafe fn bstr_util_cmp_mem(
    data1: *const core::ffi::c_void,
    len1: usize,
    data2: *const core::ffi::c_void,
    len2: usize,
) -> i32 {
    nullcheck!(data1, data2);

    let left = &std::slice::from_raw_parts(data1 as *const u8, len1);
    let right = &std::slice::from_raw_parts(data2 as *const u8, len2);
    ordering2int!(left.cmp(right))
}

/// Convert contents of a memory region to a positive integer.
///
/// If the conversion was successful, this function returns the
/// number. When the conversion fails, -1 will be returned when not
/// one valid digit was found, and -2 will be returned if an overflow
/// occurred.
pub unsafe fn bstr_util_mem_to_pint(
    data: *const core::ffi::c_void,
    len: usize,
    base: i32,
    lastlen: *mut usize,
) -> i64 {
    nullcheck!(data);
    // sanity check radix is in the convertable range
    // and will fit inside a u8
    if base < 2 || base > 36 {
        return -1;
    }

    // initialize out param
    *lastlen = 0;
    let mut rval: i64 = 0;

    // Make an open range [first, last) for the range of digits
    // and range of characters appropriate for this base
    let upper = base as u8;
    let search = if base <= 10 {
        (('0' as u8, '0' as u8 + upper), (255, 0), (255, 0))
    } else {
        (
            ('0' as u8, '9' as u8),
            ('a' as u8, 'a' as u8 + upper - 10),
            ('A' as u8, 'A' as u8 + upper - 10),
        )
    };

    let src = std::slice::from_raw_parts(data as *const u8, len);
    for b in src {
        match if (search.0).0 <= *b && *b < (search.0).1 {
            Some(*b - (search.0).0)
        } else if (search.1).0 <= *b && *b < (search.1).1 {
            Some(10 + *b - (search.1).0)
        } else if (search.2).0 <= *b && *b < (search.2).1 {
            Some(10 + *b - (search.2).0)
        } else {
            None
        } {
            None => return if *lastlen == 0 { -1 } else { rval },
            Some(d) => {
                *lastlen += 1;
                match rval.checked_mul(base as i64) {
                    None => return -2,
                    Some(new) => match new.checked_add(d as i64) {
                        None => return -2,
                        Some(new) => rval = new,
                    },
                }
            }
        }
    }
    *lastlen += 1;
    rval
}

/// Searches a memory block for the given NUL-terminated string. Case insensitive.
///
/// Returns Index of the first location of the needle on success, or -1 if the needle was not found.
pub unsafe fn bstr_util_mem_index_of_c_nocase(
    data: *const core::ffi::c_void,
    len: usize,
    cstr: *const i8,
) -> i32 {
    nullcheck!(data, cstr);

    let src_slice = std::slice::from_raw_parts(data as *const u8, len);
    let mut haystack = LowercaseIterator::new(&src_slice);
    let needle = CStr::from_ptr(cstr).to_bytes().to_ascii_lowercase();

    match haystack.index_of(needle) {
        Some(idx) => idx as i32,
        None => -1,
    }
}

/// Searches the haystack memory block for the needle memory block. Case sensitive.
///
/// Returns Index of the first location of the needle on success, or -1 if the needle was not found.
pub unsafe fn bstr_util_mem_index_of_mem(
    data1: *const core::ffi::c_void,
    len1: usize,
    data2: *const core::ffi::c_void,
    len2: usize,
) -> i32 {
    nullcheck!(data1, data2);

    let haystack = std::slice::from_raw_parts(data1 as *const u8, len1);
    let needle = std::slice::from_raw_parts(data2 as *const u8, len2);

    match haystack.find(needle) {
        Some(idx) => idx as i32,
        None => -1,
    }
}

/// Removes whitespace from the beginning and the end of a memory region. The data
/// itself is not modified; this function only adjusts the provided pointers.
pub unsafe fn bstr_util_mem_trim(data: *mut *mut u8, len: *mut usize) {
    if data.is_null() || len.is_null() || (*data).is_null() {
        return;
    }
    let src = std::slice::from_raw_parts(*data, *len);
    let bstr = B(src);
    let trimmed = bstr.trim();
    *data = trimmed.as_ptr() as *mut u8;
    *len = trimmed.len();
}

/// Create a new NUL-terminated string out of the provided bstring. If NUL bytes
/// are contained in the bstring, each will be replaced with "\0" (two characters).
/// The caller is responsible to keep track of the allocated memory area and free
/// it once it is no longer needed.
pub unsafe fn bstr_util_strdup_to_c(b: *const bstr_t) -> *mut i8 {
    if b.is_null() {
        return std::ptr::null_mut();
    }
    let src = std::slice::from_raw_parts(bstr_ptr(b), bstr_len(b));

    // Since the memory returned here is just a char* and the caller will
    // free() it we have to use malloc() here.
    // So we allocate enough space for doubled NULL bytes plus the trailing NULL.
    let mut null_count = 1;
    for byte in src {
        if *byte == 0 {
            null_count += 1;
        }
    }
    let newlen = bstr_len(b) + null_count;
    let mem = libc::malloc(newlen) as *mut i8;
    let dst: &mut [i8] = std::slice::from_raw_parts_mut(mem, newlen);
    let mut dst_idx = 0;
    for byte in src {
        if *byte == 0 {
            dst[dst_idx] = '\\' as i8;
            dst_idx += 1;
            dst[dst_idx] = '0' as i8;
            dst_idx += 1;
        } else {
            dst[dst_idx] = *byte as i8;
            dst_idx += 1;
        }
    }
    dst[dst_idx] = 0;

    mem
}

// Tests

#[test]
fn Compare() {
    let b = bstr_t::from("ABCDefgh");
    // direct equality
    assert_eq!(Ordering::Equal, b.cmp("ABCDefgh"));
    // case sensitive
    assert_ne!(Ordering::Equal, b.cmp("abcdefgh"));
    // src shorter than dst
    assert_eq!(Ordering::Less, b.cmp("ABCDefghi"));
    // src longer than dst
    assert_eq!(Ordering::Greater, b.cmp("ABCDefg"));
    // case less
    assert_eq!(Ordering::Less, b.cmp("abcdefgh"));
    // case greater
    assert_eq!(Ordering::Greater, b.cmp("ABCDEFGH"));
}

#[test]
fn CompareNocase() {
    let b = bstr_t::from("ABCDefgh");
    assert_eq!(Ordering::Equal, b.cmp_nocase("ABCDefgh"));
    assert_eq!(Ordering::Equal, b.cmp_nocase("abcdefgh"));
    assert_eq!(Ordering::Equal, b.cmp_nocase("ABCDEFGH"));
    assert_eq!(Ordering::Less, b.cmp_nocase("ABCDefghi"));
    assert_eq!(Ordering::Greater, b.cmp_nocase("ABCDefg"));
}

#[test]
fn CompareNocaseNozero() {
    // nocase_nozero only applies to the source string. The caller
    // is not expected to pass in a search string with nulls in it.
    let b = bstr_t::from("A\x00B\x00\x00C\x00Defg\x00h");
    assert_eq!(Ordering::Equal, b.cmp_nocase_nozero("ABCDefgh"));
    assert_eq!(Ordering::Equal, b.cmp_nocase_nozero("abcdefgh"));
    assert_eq!(Ordering::Equal, b.cmp_nocase_nozero("ABCDEFGH"));
    assert_eq!(Ordering::Less, b.cmp_nocase_nozero("ABCDefghi"));
    assert_eq!(Ordering::Greater, b.cmp_nocase_nozero("ABCDefg"));
}

#[test]
fn Add() {
    let mut b = bstr_t::from("ABCD");
    b.add("efgh");
    assert_eq!(Ordering::Equal, b.cmp("ABCDefgh"));
}

#[test]
fn AddNoEx() {
    let mut b = bstr_t::from("ABCD");
    b.add_noex("efghijklmnopqrstuvwxyz");
    assert_eq!(4, b.len());

    let mut c = bstr_t::with_capacity(10);
    c.add_noex("ABCD");
    assert_eq!(4, c.len());
    c.add_noex("efghijklmnopqrstuvwxyz");
    assert_eq!(10, c.len());
    assert_eq!(Ordering::Equal, c.cmp("ABCDefghij"))
}

#[test]
fn StartsWith() {
    let b = bstr_t::from("ABCD");
    assert!(b.starts_with("AB"));
}

#[test]
fn StartsWithNocase() {
    let b = bstr_t::from("ABCD");
    assert!(b.starts_with_nocase("Ab"));
}

#[test]
fn IndexOf() {
    let b = bstr_t::from("ABCDefgh");
    assert_eq!(Some(4), b.index_of("e"));
    assert_eq!(Some(0), b.index_of("A"));
    assert_eq!(Some(7), b.index_of("h"));
    assert_eq!(Some(3), b.index_of("De"));
    assert_eq!(None, b.index_of("z"));
    assert_eq!(None, b.index_of("a"));
    assert_eq!(None, b.index_of("hi"));
}

#[test]
fn IndexOfNocase() {
    let b = bstr_t::from("ABCDefgh");
    assert_eq!(Some(4), b.index_of_nocase("E"));
    assert_eq!(Some(0), b.index_of_nocase("a"));
    assert_eq!(Some(0), b.index_of_nocase("A"));
    assert_eq!(Some(7), b.index_of_nocase("H"));
    assert_eq!(Some(3), b.index_of_nocase("dE"));
    assert_eq!(None, b.index_of_nocase("z"));
    assert_eq!(None, b.index_of_nocase("Hi"));
}

#[test]
fn IndexOfNocaseNozero() {
    let b = bstr_t::from("A\x00B\x00\x00C\x00Defg\x00h");
    assert_eq!(Some(8), b.index_of_nocase_nozero("E"));
    assert_eq!(Some(0), b.index_of_nocase_nozero("a"));
    assert_eq!(Some(0), b.index_of_nocase_nozero("A"));
    assert_eq!(Some(12), b.index_of_nocase_nozero("H"));
    assert_eq!(Some(7), b.index_of_nocase_nozero("dE"));
    assert_eq!(Some(2), b.index_of_nocase_nozero("bc"));
    assert_eq!(None, b.index_of_nocase_nozero("z"));
    assert_eq!(None, b.index_of_nocase_nozero("Hi"));
    assert_eq!(None, b.index_of_nocase_nozero("ghi"));
}
