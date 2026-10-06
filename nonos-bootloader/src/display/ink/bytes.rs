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

/// `n` bytes of `b` from `at`, little-endian; None past the end.
pub fn le(b: &[u8], at: usize, n: usize) -> Option<u32> {
    let s = b.get(at..at.checked_add(n)?)?;
    Some(s.iter().rev().fold(0u32, |v, &x| (v << 8) | x as u32))
}
