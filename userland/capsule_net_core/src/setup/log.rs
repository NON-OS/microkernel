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

//! What the NIC choice says on the log.

use core::sync::atomic::{AtomicU32, Ordering};

// Last logged verdict per candidate: 0 unseen, 1 down, 2 no-answer. Indexed by
// position in the WiFi-then-wired order, so a change logs once, not per tick.
// Room for every candidate: past the table a candidate was never logged, and
// the USB network drivers made the list eleven long.
static PROBE_SEEN: [AtomicU32; 16] = [const { AtomicU32::new(0) }; 16];

/// A candidate found not up, logged when its verdict changed.
pub(super) fn probe_seen(idx: usize, name: &str, verdict: Option<bool>) {
    let code = if verdict.is_none() { 2 } else { 1 };
    if idx < PROBE_SEEN.len() && PROBE_SEEN[idx].swap(code, Ordering::Relaxed) != code {
        probe_log(name, verdict);
    }
}

// Which NIC was found and why it was not bound. A silent None here reads as a
// stack that never comes up, with every layer above reporting its own timeout.
fn probe_log(name: &str, verdict: Option<bool>) {
    let mut line = [0u8; 96];
    let tag: &[u8] = b"[NET-CORE] link probe ";
    let n = tag.len();
    line[..n].copy_from_slice(tag);
    let m = name.len().min(line.len() - n - 8);
    line[n..n + m].copy_from_slice(&name.as_bytes()[..m]);
    let tail: &[u8] = match verdict {
        Some(false) => b" down",
        None => b" no-answer",
        Some(true) => b" up",
    };
    line[n + m..n + m + tail.len()].copy_from_slice(tail);
    let _ = nonos_libc::mk_debug(line.as_ptr(), n + m + tail.len());
}

pub(super) fn bind_log(msg: &[u8]) {
    let _ = nonos_libc::mk_debug(msg.as_ptr(), msg.len());
}

// Which NIC the stack bound. With a wired port and a WiFi link both present
// the choice is the first thing to know, and "interface up" alone does not
// say it.
pub(super) fn bind_up_log(name: &str) {
    let mut line = [0u8; 96];
    let tag: &[u8] = b"[NET-CORE] bind: interface up on ";
    let n = tag.len();
    line[..n].copy_from_slice(tag);
    let m = name.len().min(line.len() - n);
    line[n..n + m].copy_from_slice(&name.as_bytes()[..m]);
    bind_log(&line[..n + m]);
}
