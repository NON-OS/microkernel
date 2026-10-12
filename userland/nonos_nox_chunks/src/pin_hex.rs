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

//! The pin's lowercase hex, written and read without a table index or a
//! subtraction (rule 1).

/// A nibble's lowercase hex digit.
fn digit(n: u8) -> u8 {
    char::from_digit(u32::from(n), 16).and_then(|c| u8::try_from(c).ok()).unwrap_or(b'0')
}

pub(super) fn put(out: &mut [u8], bytes: &[u8]) {
    for (pair, b) in out.chunks_exact_mut(2).zip(bytes) {
        pair.copy_from_slice(&[digit(b >> 4), digit(b & 15)]);
    }
}

/// A lowercase hex digit's value; anything else is refused.
fn nibble(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' | b'a'..=b'f' => u8::try_from(char::from(c).to_digit(16)?).ok(),
        _ => None,
    }
}

pub(super) fn take<const N: usize>(s: &[u8]) -> Option<[u8; N]> {
    (s.chunks_exact(2).len() >= N).then_some(())?;
    let mut out = [0u8; N];
    for (o, pair) in out.iter_mut().zip(s.chunks_exact(2)) {
        let [hi, lo] = pair else { return None };
        *o = nibble(*hi)? << 4 | nibble(*lo)?;
    }
    Some(out)
}
