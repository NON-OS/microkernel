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

/// A 128-bit figure as decimal digits, most significant first.
///
/// `format_u64` covers every amount the chain hands back, but a figure still
/// being typed can pass 64 bits before the reader is finished, and truncating it
/// mid-keystroke would show them someone else's number. It also takes a plain
/// slice rather than a fixed array, because the caller here is splitting one
/// buffer between a whole part and a fraction.
///
/// Zero prints as a single "0" rather than as nothing, so a field that has been
/// cleared still reads as a number.
pub fn format_u128(v: u128, out: &mut [u8]) -> usize {
    if out.is_empty() {
        return 0;
    }
    if v == 0 {
        out[0] = b'0';
        return 1;
    }
    let mut tmp = [0u8; 39];
    let mut n = 0;
    let mut rest = v;
    while rest > 0 && n < tmp.len() {
        tmp[n] = b'0' + (rest % 10) as u8;
        rest /= 10;
        n += 1;
    }
    let n = n.min(out.len());
    for (i, slot) in out[..n].iter_mut().enumerate() {
        *slot = tmp[n - 1 - i];
    }
    n
}
