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
//! Reading a length-prefixed field from a request body.

/// The field at the head of `buf`, and what follows it; None when `buf`
/// does not hold the four length bytes or the length they state.
pub(crate) fn take_lp(buf: &[u8]) -> Option<(&[u8], &[u8])> {
    let len: [u8; 4] = buf.get(..4)?.try_into().ok()?;
    let end = usize::try_from(u32::from_le_bytes(len)).ok()?.checked_add(4)?;
    Some((buf.get(4..end)?, buf.get(end..)?))
}

/// The field at the head of `buf` as text; None when it is not UTF-8.
pub(crate) fn read_lp_str(buf: &[u8]) -> Option<&str> {
    core::str::from_utf8(take_lp(buf)?.0).ok()
}
