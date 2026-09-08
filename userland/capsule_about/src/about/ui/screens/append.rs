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

//! Building one short label out of several pieces.
//!
//! Three screens grew the same three-line helper under three names. It is here
//! once so a fix to the bounds check is a fix everywhere, and because a routine
//! that silently truncates deserves to be read in one place rather than trusted
//! in three.

/// Append what fits and report how much landed. The caller advances by the
/// return, so a piece cut short by a full buffer shortens the label rather than
/// writing past it.
pub(super) fn put(dst: &mut [u8], src: &[u8]) -> usize {
    let n = src.len().min(dst.len());
    dst[..n].copy_from_slice(&src[..n]);
    n
}
