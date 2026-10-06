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

//! The field readers every body is made of. Each bound is checked against
//! the buffer that arrived rather than the length claiming to describe it.

/// A length-prefixed string at `at` (four bytes of length, then the
/// bytes), and where the next field starts.
pub(crate) fn lp(body: &[u8], at: usize) -> Option<(&[u8], usize)> {
    let len = u32_at(body, at)? as usize;
    let start = at.checked_add(4)?;
    let end = start.checked_add(len)?;
    Some((body.get(start..end)?, end))
}

pub(crate) fn u32_at(body: &[u8], at: usize) -> Option<u32> {
    let end = at.checked_add(4)?;
    Some(u32::from_le_bytes(body.get(at..end)?.try_into().ok()?))
}

/// Past `n` fixed bytes at `at`, provided they arrived.
pub(crate) fn skip(body: &[u8], at: usize, n: usize) -> Option<usize> {
    let end = at.checked_add(n)?;
    (end <= body.len()).then_some(end)
}

/// Past a count and that many length-prefixed strings.
pub(crate) fn skip_list(body: &[u8], at: usize) -> Option<usize> {
    let n = u32_at(body, at)?;
    (0..n).try_fold(at.checked_add(4)?, |at, _| lp(body, at).map(|(_, end)| end))
}
