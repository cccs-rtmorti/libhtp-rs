use ::libc;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
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
/*
Copyright (c) 2008-2009 Bjoern Hoehrmann <bjoern@hoehrmann.de>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software
and associated documentation files (the "Software"), to deal in the Software without restriction,
including without limitation the rights to use, copy, modify, merge, publish, distribute,
sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or
substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/
// Copyright (c) 2008-2009 Bjoern Hoehrmann <bjoern@hoehrmann.de>
// See http://bjoern.hoehrmann.de/utf-8/decoder/dfa/ for details.
static mut utf8d: [uint8_t; 400] =
    [0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     9 as libc::c_int as uint8_t, 9 as libc::c_int as uint8_t,
     9 as libc::c_int as uint8_t, 9 as libc::c_int as uint8_t,
     9 as libc::c_int as uint8_t, 9 as libc::c_int as uint8_t,
     9 as libc::c_int as uint8_t, 9 as libc::c_int as uint8_t,
     9 as libc::c_int as uint8_t, 9 as libc::c_int as uint8_t,
     9 as libc::c_int as uint8_t, 9 as libc::c_int as uint8_t,
     9 as libc::c_int as uint8_t, 9 as libc::c_int as uint8_t,
     9 as libc::c_int as uint8_t, 9 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     8 as libc::c_int as uint8_t, 8 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     0xa as libc::c_int as uint8_t, 0x3 as libc::c_int as uint8_t,
     0x3 as libc::c_int as uint8_t, 0x3 as libc::c_int as uint8_t,
     0x3 as libc::c_int as uint8_t, 0x3 as libc::c_int as uint8_t,
     0x3 as libc::c_int as uint8_t, 0x3 as libc::c_int as uint8_t,
     0x3 as libc::c_int as uint8_t, 0x3 as libc::c_int as uint8_t,
     0x3 as libc::c_int as uint8_t, 0x3 as libc::c_int as uint8_t,
     0x3 as libc::c_int as uint8_t, 0x4 as libc::c_int as uint8_t,
     0x3 as libc::c_int as uint8_t, 0x3 as libc::c_int as uint8_t,
     0xb as libc::c_int as uint8_t, 0x6 as libc::c_int as uint8_t,
     0x6 as libc::c_int as uint8_t, 0x6 as libc::c_int as uint8_t,
     0x5 as libc::c_int as uint8_t, 0x8 as libc::c_int as uint8_t,
     0x8 as libc::c_int as uint8_t, 0x8 as libc::c_int as uint8_t,
     0x8 as libc::c_int as uint8_t, 0x8 as libc::c_int as uint8_t,
     0x8 as libc::c_int as uint8_t, 0x8 as libc::c_int as uint8_t,
     0x8 as libc::c_int as uint8_t, 0x8 as libc::c_int as uint8_t,
     0x8 as libc::c_int as uint8_t, 0x8 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0x1 as libc::c_int as uint8_t,
     0x2 as libc::c_int as uint8_t, 0x3 as libc::c_int as uint8_t,
     0x5 as libc::c_int as uint8_t, 0x8 as libc::c_int as uint8_t,
     0x7 as libc::c_int as uint8_t, 0x1 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0x1 as libc::c_int as uint8_t,
     0x4 as libc::c_int as uint8_t, 0x6 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0x1 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0x1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 3 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 3 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 3 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 3 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 3 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 3 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t];
static mut utf8d_allow_overlong: [uint8_t; 400] =
    [0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     9 as libc::c_int as uint8_t, 9 as libc::c_int as uint8_t,
     9 as libc::c_int as uint8_t, 9 as libc::c_int as uint8_t,
     9 as libc::c_int as uint8_t, 9 as libc::c_int as uint8_t,
     9 as libc::c_int as uint8_t, 9 as libc::c_int as uint8_t,
     9 as libc::c_int as uint8_t, 9 as libc::c_int as uint8_t,
     9 as libc::c_int as uint8_t, 9 as libc::c_int as uint8_t,
     9 as libc::c_int as uint8_t, 9 as libc::c_int as uint8_t,
     9 as libc::c_int as uint8_t, 9 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     7 as libc::c_int as uint8_t, 7 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     2 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     0x3 as libc::c_int as uint8_t, 0x3 as libc::c_int as uint8_t,
     0x3 as libc::c_int as uint8_t, 0x3 as libc::c_int as uint8_t,
     0x3 as libc::c_int as uint8_t, 0x3 as libc::c_int as uint8_t,
     0x3 as libc::c_int as uint8_t, 0x3 as libc::c_int as uint8_t,
     0x3 as libc::c_int as uint8_t, 0x3 as libc::c_int as uint8_t,
     0x3 as libc::c_int as uint8_t, 0x3 as libc::c_int as uint8_t,
     0x3 as libc::c_int as uint8_t, 0x4 as libc::c_int as uint8_t,
     0x3 as libc::c_int as uint8_t, 0x3 as libc::c_int as uint8_t,
     0x6 as libc::c_int as uint8_t, 0x6 as libc::c_int as uint8_t,
     0x6 as libc::c_int as uint8_t, 0x6 as libc::c_int as uint8_t,
     0x5 as libc::c_int as uint8_t, 0x8 as libc::c_int as uint8_t,
     0x8 as libc::c_int as uint8_t, 0x8 as libc::c_int as uint8_t,
     0x8 as libc::c_int as uint8_t, 0x8 as libc::c_int as uint8_t,
     0x8 as libc::c_int as uint8_t, 0x8 as libc::c_int as uint8_t,
     0x8 as libc::c_int as uint8_t, 0x8 as libc::c_int as uint8_t,
     0x8 as libc::c_int as uint8_t, 0x8 as libc::c_int as uint8_t,
     0 as libc::c_int as uint8_t, 0x1 as libc::c_int as uint8_t,
     0x2 as libc::c_int as uint8_t, 0x3 as libc::c_int as uint8_t,
     0x5 as libc::c_int as uint8_t, 0x8 as libc::c_int as uint8_t,
     0x7 as libc::c_int as uint8_t, 0x1 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0x1 as libc::c_int as uint8_t,
     0x4 as libc::c_int as uint8_t, 0x6 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0x1 as libc::c_int as uint8_t,
     0x1 as libc::c_int as uint8_t, 0x1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 0 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 2 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 3 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 3 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 3 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 3 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 3 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 3 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t,
     1 as libc::c_int as uint8_t, 1 as libc::c_int as uint8_t];
/* *
 * Process one byte of UTF-8 data and return a code point if one is available.
 *
 * @param[in] state
 * @param[in] codep
 * @param[in] byte
 * @return HTP_UTF8_ACCEPT for a valid character, HTP_UTF8_REJECT for an invalid character,
 *         or something else if the character has not yet been formed
 */
#[no_mangle]
pub unsafe extern "C" fn htp_utf8_decode(mut state: *mut uint32_t,
                                         mut codep: *mut uint32_t,
                                         mut byte: uint32_t) -> uint32_t {
    let mut type_0: uint32_t = utf8d[byte as usize] as uint32_t;
    *codep =
        if *state != 0 as libc::c_int as libc::c_uint {
            (byte & 0x3f as libc::c_uint) | *codep << 6 as libc::c_int
        } else { ((0xff as libc::c_int >> type_0) as libc::c_uint) & byte };
    *state =
        utf8d[(256 as libc::c_int as
                   libc::c_uint).wrapping_add((*state).wrapping_mul(16 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)).wrapping_add(type_0)
                  as usize] as uint32_t;
    return *state;
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
/* LibHTP changes:
 *
 *     - Changed the name of the function from "decode" to "utf8_decode"
 *     - Created a separate header file
 *     - Copied the license from the web page
 *     - Created a copy of the data and function "utf8_decode_allow_overlong", which
 *       does not treat overlong characters as invalid.
 */
/*
Copyright (c) 2008-2009 Bjoern Hoehrmann <bjoern@hoehrmann.de>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software
and associated documentation files (the "Software"), to deal in the Software without restriction,
including without limitation the rights to use, copy, modify, merge, publish, distribute,
sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or
substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/
/* *
 * Process one byte of UTF-8 data and return a code point if one is available. Allows
 * overlong characters in input.
 *
 * @param[in] state
 * @param[in] codep
 * @param[in] byte
 * @return HTP_UTF8_ACCEPT for a valid character, HTP_UTF8_REJECT for an invalid character,
 *         or something else if the character has not yet been formed
 */
#[no_mangle]
pub unsafe extern "C" fn htp_utf8_decode_allow_overlong(mut state:
                                                            *mut uint32_t,
                                                        mut codep:
                                                            *mut uint32_t,
                                                        mut byte: uint32_t)
 -> uint32_t {
    let mut type_0: uint32_t =
        utf8d_allow_overlong[byte as usize] as uint32_t;
    *codep =
        if *state != 0 as libc::c_int as libc::c_uint {
            (byte & 0x3f as libc::c_uint) | *codep << 6 as libc::c_int
        } else { ((0xff as libc::c_int >> type_0) as libc::c_uint) & byte };
    *state =
        utf8d[(256 as libc::c_int as
                   libc::c_uint).wrapping_add((*state).wrapping_mul(16 as
                                                                        libc::c_int
                                                                        as
                                                                        libc::c_uint)).wrapping_add(type_0)
                  as usize] as uint32_t;
    return *state;
}
