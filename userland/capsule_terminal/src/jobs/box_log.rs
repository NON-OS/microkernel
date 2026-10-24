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

//! A boxed run's `[BOX]` line to the serial log, which is the record a
//! check of the run reads: never cut, so it is sent in the pieces mk_debug
//! takes, which the log joins. The kernel refuses a longer call whole.

use alloc::vec::Vec;

/// The most mk_debug takes at once.
pub const DEBUG_MAX: usize = 256;
/// What mk_debug answers a capsule not given the serial log, as a hardened
/// release builds every capsule: there is no log, which is not a failure.
pub const NO_LOG: i64 = -1;

/// `line` and its newline through `write` (mk_debug), piece by piece;
/// false when a log there is refused a piece, the rest then not sent.
pub fn log(line: &[u8], mut write: impl FnMut(&[u8]) -> i64) -> bool {
    let whole: Vec<u8> = [line, b"\n"].concat();
    for piece in whole.chunks(DEBUG_MAX) {
        match write(piece) {
            NO_LOG => return true,
            e if e < 0 => return false,
            _ => {}
        }
    }
    true
}
