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

//! Why the kernel refused a kill, in words.

/// Why the kernel refused the kill, in words. The terminal holds no
/// ProcessControl, so it ends only the processes it started; anything else
/// is the Process Manager's to end.
pub fn refused(rc: i64) -> &'static [u8] {
    match rc {
        -1 => b"kill: the terminal ends only what it started; Process Manager can end this one",
        _ => b"kill: the signal must be 2, 9 or 15",
    }
}
