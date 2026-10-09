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


//! Why the store has nothing for a program, in words, from the status vfs
//! latched on its boot load (capsule_vfs blk/status.rs). The number alone
//! ("the store reports error 2") told the person nothing. Pure, so the host
//! proofs hold every code.

/// Why a file is not there when vfs's store status is `code`, which is not
/// 0. Every code says what was wrong and, where there is one, what to do.
pub fn store_why(code: u32) -> &'static str {
    match code {
        1 => "no disk with the NONOS store was found on this boot",
        2 => "the disk with the store did not answer in time; restart to read it again",
        3 | 6 => "the store on this disk could not be read; it is damaged",
        7 => "the disk refused the read of the store",
        8 => "the store's table is not one vfs takes",
        9 => "a damaged store entry was left out, and this may be it",
        11 => "the store does not fit in this machine's memory",
        _ => "the store did not load",
    }
}
