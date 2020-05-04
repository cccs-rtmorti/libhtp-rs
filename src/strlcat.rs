use ::libc;
extern "C" {
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;

// Copyright (c) 1998 Todd C. Miller <Todd.Miller@courtesan.com>
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in the
//    documentation and/or other materials provided with the distribution.
// 3. The name of the author may not be used to endorse or promote products
//    derived from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY
// AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL
// THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS;
// OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
// OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF
// ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

/// Appends src to string dst of size siz (unlike strncat, siz is the
/// full size of dst, not space left).  At most siz-1 characters
/// will be copied.  Always NUL terminates (unless siz <= strlen(dst)).
/// Returns strlen(initial dst) + strlen(src); if retval >= siz,
/// truncation occurred.
#[no_mangle]
pub unsafe extern "C" fn strlcat(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut siz: size_t,
) -> size_t {
    let mut d: *mut libc::c_char = dst;
    let mut s: *const libc::c_char = src;
    let mut n: size_t = siz;
    let mut dlen: size_t = 0;
    loop
    // Find the end of dst and adjust bytes left but don't go past end
    {
        let fresh0 = n;
        n = n.wrapping_sub(1);
        if !(fresh0 != 0 as libc::c_int as libc::c_ulong && *d as libc::c_int != '\u{0}' as i32) {
            break;
        }
        d = d.offset(1)
    }
    dlen = d.wrapping_offset_from(dst) as libc::c_long as size_t;
    n = siz.wrapping_sub(dlen);
    if n == 0 as libc::c_int as libc::c_ulong {
        return dlen.wrapping_add(strlen(s));
    }
    while *s as libc::c_int != '\u{0}' as i32 {
        if n != 1 as libc::c_int as libc::c_ulong {
            let fresh1 = d;
            d = d.offset(1);
            *fresh1 = *s;
            n = n.wrapping_sub(1)
        }
        s = s.offset(1)
    }
    *d = '\u{0}' as i32 as libc::c_char;
    return dlen.wrapping_add(s.wrapping_offset_from(src) as libc::c_long as libc::c_ulong);
    // count does not include NUL
}
