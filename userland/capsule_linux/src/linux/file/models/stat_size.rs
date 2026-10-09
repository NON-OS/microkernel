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
 * The size `stat` reports for a model. One pinned and not brought onto the
 * volume yet (the volume says ENOENT) is the size its pin says, the size it
 * will have once opened, so a program that sizes a model before it opens it
 * (qwenchat plans its memory from every part's size) is not told nothing is
 * there. Any other answer of the volume stands: a pinned file's other
 * refusals (ENODEV with no NONOS disk at all, EIO, ENOMEM when a live
 * session's volume in memory cannot be held) and every unpinned name's. Pure, so
 * capsule_linux_proofs holds the rule.
 */

const ENOENT: i64 = 2;

/* `pinned` the pin's length if the name is pinned; `got` what the volume said. */
pub fn stat_size(pinned: Option<u64>, got: i64) -> Result<u64, i64> {
    match pinned {
        Some(bytes) if got == -ENOENT => Ok(bytes),
        _ if got < 0 => Err(-got),
        _ => Ok(got as u64),
    }
}
