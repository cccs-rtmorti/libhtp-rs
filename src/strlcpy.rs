use ::libc;
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
// 1048576 is 1 Mbyte
//deflate max ratio is about 1000
// Parser states, in the order in which they are
// used as a single transaction is processed.
// Parsing functions
// Private transaction functions
// Utility functions
/*
 * Copyright (c) 1998 Todd C. Miller <Todd.Miller@courtesan.com>
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. The name of the author may not be used to endorse or promote products
 *    derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES,
 * INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY
 * AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL
 * THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS;
 * OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
 * WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR
 * OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF
 * ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
/* $Id: strlcpyu.c,v 1.4 2003/10/20 15:03:27 chrisgreen Exp $ */
/* LIBC_SCCS and not lint */
/*
 * Copy src to string dst of size siz.  At most siz-1 characters
 * will be copied.  Always NUL terminates (unless siz == 0).
 * Returns strlen(src); if retval >= siz, truncation occurred.
 */
#[no_mangle]
pub unsafe extern "C" fn strlcpy(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut siz: size_t,
) -> size_t {
    let mut d: *mut libc::c_char = dst;
    let mut s: *const libc::c_char = src;
    let mut n: size_t = siz;
    /* Copy as many bytes as will fit */
    if n != 0 as libc::c_int as libc::c_ulong && {
        n = n.wrapping_sub(1);
        (n) != 0 as libc::c_int as libc::c_ulong
    } {
        loop {
            let fresh0 = s;
            s = s.offset(1);
            let fresh1 = d;
            d = d.offset(1);
            *fresh1 = *fresh0;
            if *fresh1 as libc::c_int == 0 as libc::c_int {
                break;
            }
            n = n.wrapping_sub(1);
            if !(n != 0 as libc::c_int as libc::c_ulong) {
                break;
            }
        }
    }
    /* Not enough room in dst, add NUL and traverse rest of src */
    if n == 0 as libc::c_int as libc::c_ulong {
        if siz != 0 as libc::c_int as libc::c_ulong {
            *d = '\u{0}' as i32 as libc::c_char
        } /* NUL-terminate dst */
        loop {
            let fresh2 = s;
            s = s.offset(1);
            if !(*fresh2 != 0) {
                break;
            }
        }
    }
    return (s.wrapping_offset_from(src) as libc::c_long - 1 as libc::c_int as libc::c_long)
        as size_t;
    /* count does not include NUL */
}
