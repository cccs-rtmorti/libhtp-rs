// Copyright (c) 2008-2009 Bjoern Hoehrmann <bjoern@hoehrmann.de>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge, publish, distribute,
// sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
// NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
// Copyright (c) 2008-2009 Bjoern Hoehrmann <bjoern@hoehrmann.de>
// See http://bjoern.hoehrmann.de/utf-8/decoder/dfa/ for details.
use crate::{
    unicode_bestfit_map::UnicodeBestfitMap,
    util::{FlagOperations, HtpFlags},
};

static utf8d: [u8; 400] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    8, 8, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    0xa, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x4, 0x3, 0x3, 0xb, 0x6, 0x6,
    0x6, 0x5, 0x8, 0x8, 0x8, 0x8, 0x8, 0x8, 0x8, 0x8, 0x8, 0x8, 0x8, 0, 0x1, 0x2, 0x3, 0x5, 0x8,
    0x7, 0x1, 0x1, 0x1, 0x4, 0x6, 0x1, 0x1, 0x1, 0x1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 3, 1, 3, 1, 1, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 3, 1, 3, 1, 1, 1, 1, 1,
    1, 1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];
static utf8d_allow_overlong: [u8; 400] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9,
    7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x3, 0x4, 0x3, 0x3, 0x6, 0x6, 0x6,
    0x6, 0x5, 0x8, 0x8, 0x8, 0x8, 0x8, 0x8, 0x8, 0x8, 0x8, 0x8, 0x8, 0, 0x1, 0x2, 0x3, 0x5, 0x8,
    0x7, 0x1, 0x1, 0x1, 0x4, 0x6, 0x1, 0x1, 0x1, 0x1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 3, 1, 3, 1, 1, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 3, 1, 3, 1, 1, 1, 1, 1,
    1, 1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];

#[derive(Clone)]
pub struct Utf8Decoder {
    bestfit_map: UnicodeBestfitMap,
    state: u32,
    seq: u32,
    codepoint: u32,
    pub flags: u64,
    pub seen_valid: bool,
    pub decoded_bytes: Vec<u8>,
}

impl Utf8Decoder {
    /// Make a new owned Utf8Decoder
    pub fn new(bestfit_map: UnicodeBestfitMap) -> Self {
        Self {
            bestfit_map,
            state: 0,
            seq: 0,
            codepoint: 0,
            flags: 0,
            seen_valid: false,
            decoded_bytes: Vec::new(),
        }
    }

    /// Decode utf8 byte using best-fit map.
    fn decode_byte(&mut self, encoded_byte: u8) {
        self.seq = self.seq.wrapping_add(1);
        self.decode_byte_allow_overlong(encoded_byte as u32);
        match self.state {
            0 => {
                if self.seq == 1 {
                    // ASCII character, which we just copy.
                    self.decoded_bytes.push(self.codepoint as u8);
                } else {
                    // A valid UTF-8 character, which we need to convert.
                    self.seen_valid = true;
                    // Check for overlong characters and set the flag accordingly.
                    match self.seq {
                        2 => {
                            if self.codepoint < 0x80 {
                                self.flags.set(HtpFlags::PATH_UTF8_OVERLONG)
                            }
                        }
                        3 => {
                            if self.codepoint < 0x800 {
                                self.flags.set(HtpFlags::PATH_UTF8_OVERLONG)
                            }
                        }
                        4 => {
                            if self.codepoint < 0x10000 {
                                self.flags.set(HtpFlags::PATH_UTF8_OVERLONG)
                            }
                        }
                        _ => {}
                    }
                    // Special flag for half-width/full-width evasion.
                    if self.codepoint >= 0xff00 && self.codepoint <= 0xffef {
                        self.flags.set(HtpFlags::PATH_HALF_FULL_RANGE)
                    }
                    // Use best-fit mapping to convert to a single byte.
                    self.decoded_bytes.push(self.bestfit_codepoint());
                }
                self.seq = 0;
            }
            1 => {
                // Invalid UTF-8 character.
                self.flags.set(HtpFlags::PATH_UTF8_INVALID);
                // Output the replacement byte, replacing one or more invalid bytes.
                // If the invalid byte was first in a sequence, consume it. Otherwise,
                // assume it's the starting byte of the next character.
                self.state = 0;
                self.codepoint = 0;
                self.decoded_bytes.push(self.bestfit_map.replacement_byte);
                if self.seq != 1 {
                    self.decode_byte(encoded_byte);
                }
                self.seq = 0;
            }
            _ => {
                // The character is not yet formed.
            }
        }
    }

    /// Decode a UTF-8 encoded path. Replaces a possibly-invalid utf8 byte stream
    /// with an ascii stream, storing the result in self.decoded_bytes. Overlong
    /// characters will be decoded and invalid characters will be replaced with
    /// the replacement byte specified in the bestfit_map. Best-fit mapping will be used
    /// to convert UTF-8 into a single-byte stream.
    pub fn decode_and_validate(&mut self, input: &[u8]) {
        //Reset all internals
        self.state = 0;
        self.seq = 0;
        self.codepoint = 0;
        self.flags = 0;
        self.decoded_bytes.clear();
        self.decoded_bytes.reserve(input.len());
        self.seen_valid = false;
        for byte in input {
            self.decode_byte(*byte);
        }
        // Did the input stream seem like a valid UTF-8 string?
        if self.seen_valid && !self.flags.is_set(HtpFlags::PATH_UTF8_INVALID) {
            self.flags.set(HtpFlags::PATH_UTF8_VALID)
        }
    }

    /// Process one byte of UTF-8 data and set the code point if one is available. Allows
    /// overlong characters in input.
    ///
    /// Sets the state to ACCEPT(0) for a valid character, REJECT(1) for an invalid character,
    ///         or OTHER(u32) if the character has not yet been formed
    fn decode_byte_allow_overlong(&mut self, byte: u32) {
        let type_0: u32 = utf8d_allow_overlong[byte as usize] as u32;
        self.codepoint = if self.state != 0 {
            (byte & 0x3f) | self.codepoint << 6
        } else {
            (0xff >> type_0) & byte
        };
        self.state = utf8d[(256u32)
            .wrapping_add((self.state).wrapping_mul(16))
            .wrapping_add(type_0) as usize] as u32;
    }

    /// Convert a Unicode codepoint into a single-byte, using best-fit
    /// mapping (as specified in the provided configuration structure).
    ///
    /// Returns converted single byte
    fn bestfit_codepoint(&self) -> u8 {
        // Is it a single-byte codepoint?
        if self.codepoint < 0x100 {
            return self.codepoint as u8;
        }
        self.bestfit_map.get(self.codepoint)
    }
}
