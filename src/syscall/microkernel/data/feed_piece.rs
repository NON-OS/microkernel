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

//! The next piece a feed copies in: where in the caller it starts and how
//! long it is. A source address that would wrap is no piece, so the bound
//! holds in the handler and not only in usercopy.

/// The piece of a `len`-byte feed from `buf` once `done` bytes have come,
/// at most `most` long. None when `buf + done` wraps or `done` is past `len`.
pub(super) const fn next_piece(buf: u64, len: u64, done: u64, most: u64) -> Option<(u64, u64)> {
    let Some(src) = buf.checked_add(done) else { return None };
    let Some(left) = len.checked_sub(done) else { return None };
    Some((src, if left < most { left } else { most }))
}
