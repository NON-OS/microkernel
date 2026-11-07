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

//! A row of 32-bit pixels seen as the little-endian bytes the rows take.

/// `px` as bytes. Every byte pattern is a valid `u8`, and the length is
/// exactly the words', so the view is sound.
pub fn bytes_mut(px: &mut [u32]) -> &mut [u8] {
    let len = px.len() * 4;
    // SAFETY: the bytes are the words' own, borrowed for as long as they
    // are; `u8` has no alignment or validity requirement. The pixels are
    // little-endian, which is what the rows assume, on every target here.
    unsafe { core::slice::from_raw_parts_mut(px.as_mut_ptr().cast::<u8>(), len) }
}

/// `px` as bytes, read only.
pub fn bytes(px: &[u32]) -> &[u8] {
    // SAFETY: as `bytes_mut`.
    unsafe { core::slice::from_raw_parts(px.as_ptr().cast::<u8>(), px.len() * 4) }
}
