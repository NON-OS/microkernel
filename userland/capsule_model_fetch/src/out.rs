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
 * What the person at the Terminal is told: written to this process's own
 * output, which only the Terminal that started it reads, never the serial
 * log. A full output inbox is waited on briefly, then the line is dropped
 * rather than holding a download for a screen nobody drains.
 */

use nonos_libc::{mk_idle_ms, mk_private_write, mk_uptime_ms};

const EBUSY: i64 = -16;
const WAIT_MS: i64 = 5_000;

/* One line. */
pub fn say(text: &str) {
    write(text.as_bytes());
    write(b"\n");
}

/* A line that the next one overwrites: the progress of a file. */
pub fn over(text: &str) {
    write(b"\r\x1b[K");
    write(text.as_bytes());
}

fn write(mut bytes: &[u8]) {
    let until = mk_uptime_ms().saturating_add(WAIT_MS);
    while !bytes.is_empty() {
        let n = mk_private_write(&bytes[..bytes.len().min(256)]);
        if n > 0 {
            bytes = &bytes[(n as usize).min(bytes.len())..];
        } else if n != EBUSY || mk_uptime_ms() > until {
            return;
        } else {
            mk_idle_ms(10);
        }
    }
}

pub use crate::size::size;
