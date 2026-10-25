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

//! The lines a driver says when it does not start.

use crate::mk_debug;

/// The one line a driver logs when the machine has none of its devices.
pub fn say_absent(driver: &[u8]) {
    say(&[driver, b": no controller present, not started"]);
}

pub(super) fn say_gave_up(driver: &[u8], reason: &str) {
    say(&[driver, b": device present, bring-up failed; not started (", reason.as_bytes(), b")"]);
}

/// One console line from `parts`, cut to fit and always ending in a newline,
/// so the next line on the serial console starts on its own.
fn say(parts: &[&[u8]]) {
    let mut line = [0u8; 160];
    let room = line.len() - 1;
    let mut n = 0usize;
    for part in parts {
        for &b in *part {
            if n == room {
                break;
            }
            line[n] = b;
            n += 1;
        }
    }
    line[n] = b'\n';
    let _ = mk_debug(line.as_ptr(), n + 1);
}
