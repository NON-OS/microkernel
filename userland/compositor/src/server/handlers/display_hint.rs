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

//! NCMP 0x0009 DISPLAY_HINT: the host's wanted size, from a guest service.
//! Only a service named in the kernel's HINT_SOURCES may send one; that list
//! is empty until the owner accepts display hints (Decision 15), so today
//! every sender is refused with -1. The answer's flags say whether the size
//! will be applied (bit 0, resizable); resizing is its own unit (G 6), so a
//! hint taken now is logged and answered 0.

use alloc::format;

use crate::display::hint_rule::{allowed, Hint};
use crate::protocol::{Request, E_INVAL, E_PERM};
use crate::server::respond;
use crate::setup::{displays::HINT_SOURCES, lookup};

pub fn handle(
    sender_pid: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> Result<(), &'static str> {
    // Looked up per hint: a source that restarted holds a new pid.
    let sources = HINT_SOURCES.iter().filter_map(|n| lookup(n)).map(|e| e.pid);
    if !allowed(sender_pid, sources) {
        return respond::status(sender_pid, req, E_PERM, tx);
    }
    let Some(hint) = Hint::decode(body) else {
        return respond::status(sender_pid, req, E_INVAL, tx);
    };
    if !hint.is_probe() {
        let (w, h, out) = (hint.width, hint.height, hint.output);
        crate::say::say(&format!("hint {}x{} for output {} from pid {}", w, h, out, sender_pid));
    }
    let flags: u32 = 0;
    respond::status_payload(sender_pid, req, 0, &flags.to_le_bytes(), tx)
}
