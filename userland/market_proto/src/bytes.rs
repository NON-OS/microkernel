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

/// `len` as a u32 length field, or None when it does not fit: an encoder
/// refuses rather than write a length that says something false.
pub(crate) fn len_u32(len: usize) -> Option<u32> {
    u32::try_from(len).ok()
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    #[test]
    fn a_length_one_past_its_u32_is_refused_not_saturated() {
        let most = usize::try_from(u32::MAX).unwrap();
        assert_eq!(super::len_u32(most), Some(u32::MAX));
        assert_eq!(super::len_u32(most.checked_add(1).unwrap()), None);
    }
}
