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
//! is empty until the owner accepts display hints (the host-services
//! decision), so today every sender is refused with -1. A hint taken for
//! output 0 of a wire driver is held and followed as a STATE hint is
//! (`setup::resize`) and answered resizable (bit 0); otherwise, or with
//! FOLLOW_HOST off (Decision 15), it is logged and answered 0.

use alloc::format;

use nonos_libc::mk_uptime_ms;

use crate::display::follow::FOLLOW_HOST;
use crate::display::hint_rule::{allowed, Hint};
use crate::protocol::{Request, E_INVAL, E_PERM};
use crate::server::respond;
use crate::setup::{display_hints::HINT_SOURCES, lookup};
use crate::state::Context;

pub fn handle(
    ctx: &mut Context,
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
    let follows = FOLLOW_HOST && hint.output == 0 && ctx.wire.is_some();
    if follows {
        ctx.resize.note((hint.width, hint.height), mk_uptime_ms());
    }
    let flags = follows as u32;
    respond::status_payload(sender_pid, req, 0, &flags.to_le_bytes(), tx)
}
