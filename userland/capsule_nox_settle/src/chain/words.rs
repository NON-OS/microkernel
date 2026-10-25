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

//! ABI words read strictly: a number whose high bytes are not zero is not
//! the number, and a bool is 0 or 1.

pub fn word_u128(w: &[u8]) -> Option<u128> {
    let ([hi, lo], []) = w.as_chunks::<16>() else { return None };
    (*hi == [0; 16]).then_some(u128::from_be_bytes(*lo))
}

pub fn word_u64(w: &[u8]) -> Option<u64> {
    word_u128(w).and_then(|v| u64::try_from(v).ok())
}

pub fn word_bool(w: &[u8]) -> Option<bool> {
    match word_u128(w)? {
        0 => Some(false),
        1 => Some(true),
        _ => None,
    }
}

/// An address as its argument word.
pub fn addr_word(a: &[u8; 20]) -> [u8; 32] {
    let mut w = [0u8; 32];
    w[12..].copy_from_slice(a);
    w
}
