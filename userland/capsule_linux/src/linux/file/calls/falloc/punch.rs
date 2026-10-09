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

/*
 * The bytes a PUNCH_HOLE with KEEP_SIZE zeroes. Pure, so the host proofs
 * hold it.
 *
 * Only the file's own bytes are zeroed, and the file is the family's copy
 * as it is held when the hole is punched, never a length the descriptor
 * remembers: that can be the length of a name since unlinked or renamed,
 * and zeroes sized from it were one allocation of whatever size the guest
 * had asked a refused call for.
 */

/*
 * `[at, at + len)` cut to a file of `size` bytes, or None when no byte of it
 * is inside the file.
 */
pub fn punched(at: u64, len: u64, size: u64) -> Option<(u64, u64)> {
    let end = at.saturating_add(len).min(size);
    (at < end).then_some((at, end))
}
