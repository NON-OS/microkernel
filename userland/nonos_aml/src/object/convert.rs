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

//! The implicit conversions an operator applies to its operands (section
//! 19.3.5.7, the data type conversion rules).

use super::hex::hex_value;
use super::{Object, Width};
use alloc::vec::Vec;

impl Object {
    /// Buffer to Integer takes the first bytes little-endian, zero-extended;
    /// String to Integer reads hexadecimal digits up to the first character
    /// that is not one, at most as many as the width holds.
    pub fn as_integer(&self, w: Width) -> Option<u64> {
        match self {
            Object::Integer(v) => Some(w.mask(*v)),
            Object::Buffer(b) => {
                let b = b.borrow();
                Some(b.iter().take(w.bytes()).rev().fold(0, |v, x| (v << 8) | *x as u64))
            }
            Object::String(s) => Some(
                s.borrow()
                    .iter()
                    .take(w.bytes() * 2)
                    .map_while(|c| hex_value(*c))
                    .fold(0, |v, d| (v << 4) | d),
            ),
            _ => None,
        }
    }

    /// Integer to Buffer is the integer's bytes little-endian at the width.
    /// String to Buffer copies the characters without the NUL (unverified:
    /// the conversion table's wording on the terminator is not to hand).
    pub fn as_bytes(&self, w: Width) -> Option<Vec<u8>> {
        match self {
            Object::Integer(v) => Some(v.to_le_bytes()[..w.bytes()].to_vec()),
            Object::Buffer(b) | Object::String(b) => Some(b.borrow().clone()),
            _ => None,
        }
    }
}
