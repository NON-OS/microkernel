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

//! What the personality says, through its debug channel: `say` for what went
//! wrong and for what a guest prints, `note` for how a run is going.

use nonos_libc::mk_debug;

pub(super) fn say(line: &[u8]) {
    let _ = mk_debug(line.as_ptr(), line.len());
}

/// A line about how a run is going, not about anything wrong with it. The
/// terminal's `linux` command shows only the program's own output and what
/// went wrong, so these stay out of it.
pub(super) fn note(line: &[u8]) {
    if !super::terminal::started() {
        say(line);
    }
}
