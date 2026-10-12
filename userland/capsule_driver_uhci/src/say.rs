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

//! The one console line the driver gives once it serves.

use nonos_libc::mk_debug;

use crate::setup::Driver;

const DIGITS: &[u8; 16] = b"0123456789abcdef";

/// "driver.uhci: 8086:7020 started, serving driver.uhci0".
pub fn started(d: &Driver) {
    let mut line = *b"driver.uhci: vvvv:dddd started, serving driver.uhci0\n";
    for (at, v) in [(13, d.found.vendor), (18, d.found.device)] {
        for i in 0..4 {
            line[at + i] = DIGITS[(v >> (12 - 4 * i)) as usize & 0xF];
        }
    }
    let _ = mk_debug(line.as_ptr(), line.len());
}
