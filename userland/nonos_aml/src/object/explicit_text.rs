// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The explicit text and buffer conversions: ToBuffer, ToDecimalString,
//! ToHexString and ToString (sections 19.6.138, 19.6.139, 19.6.141 and
//! 19.6.143). The exact text of the decimal and hexadecimal forms of a buffer
//! is unverified against the specification; firmware reads these strings
//! only to print them.

use super::hex::{decimal, hex_fixed, hex_trimmed};
use super::{Object, Width};
use alloc::vec::Vec;

impl Object {
    /// An integer in decimal; a buffer as its bytes in decimal, separated by
    /// commas; a string unchanged.
    pub fn to_decimal_string(&self, w: Width) -> Option<Vec<u8>> {
        self.text_with(w, decimal, |b, out| decimal(b as u64, out))
    }

    /// An integer as "0x" and its digits; a buffer as "0x"-prefixed byte
    /// pairs separated by commas; a string unchanged.
    pub fn to_hex_string(&self, w: Width) -> Option<Vec<u8>> {
        let int = |v, out: &mut Vec<u8>| {
            out.extend_from_slice(b"0x");
            hex_trimmed(v, out)
        };
        let byte = |b: u8, out: &mut Vec<u8>| {
            out.extend_from_slice(b"0x");
            hex_fixed(b as u64, 2, out)
        };
        self.text_with(w, int, byte)
    }

    fn text_with(
        &self,
        w: Width,
        int: impl Fn(u64, &mut Vec<u8>),
        byte: impl Fn(u8, &mut Vec<u8>),
    ) -> Option<Vec<u8>> {
        let mut out = Vec::new();
        match self {
            Object::Integer(v) => int(w.mask(*v), &mut out),
            Object::String(s) => out.extend_from_slice(&s.borrow()),
            Object::Buffer(b) => {
                for (i, x) in b.borrow().iter().enumerate() {
                    if i > 0 {
                        out.push(b',');
                    }
                    byte(*x, &mut out);
                }
            }
            _ => return None,
        }
        Some(out)
    }
}
