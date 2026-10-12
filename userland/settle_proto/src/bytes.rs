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
//! Fixed-width reads at an offset, checked (rule 1): None past the end, never
//! a panic and never an overflowed index.

/// The N bytes at `at`, or None unless all of them are there.
pub(crate) fn fixed<const N: usize>(b: &[u8], at: usize) -> Option<[u8; N]> {
    b.get(at..at.checked_add(N)?)?.try_into().ok()
}

/// `len` as a u32 length field; a length past u32 saturates, and the
/// frame it heads then fails every exact-length check.
pub(crate) fn len_u32(len: usize) -> u32 {
    u32::try_from(len).unwrap_or(u32::MAX)
}

/// `len` as a u8 count, the same way.
pub(crate) fn len_u8(len: usize) -> u8 {
    u8::try_from(len).unwrap_or(u8::MAX)
}

/// `len` as a u16 length field, the same way.
pub(crate) fn len_u16(len: usize) -> u16 {
    u16::try_from(len).unwrap_or(u16::MAX)
}
