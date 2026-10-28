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

//! A stream whose socket is let go (closed, or its last holder exited)
//! with written bytes still queued stays in the table, held by no socket,
//! and the serve loop carries it on each pass until it ends (linger.rs).
//! It keeps its stream number at net.socks5 until then. When the family
//! ends, the loop carries every one on before the capsule exits.

use nonos_libc::{mk_idle_ms, mk_uptime_ms};

use super::entry::Phase;
use super::linger::{ends, linger};
use super::table;

/// How often a closing stream is carried on while the family waits.
const TICK_MS: u64 = 10;

/// Entry `h`'s socket is let go; `abort` drops what is queued.
pub fn close(h: u32, abort: bool) {
    let now = mk_uptime_ms();
    let kept = table::with(h, |e| {
        e.out.clear();
        let Phase::Open(link) = &mut e.phase else { return false };
        link.rx.clear();
        e.closing = linger(link.queue.queued() && !abort, now);
        e.closing.is_some()
    });
    if kept != Some(true) {
        drop(table::remove(h));
    }
}

/// Carry every closing stream a frame on, and end each one that is done.
pub fn settle() {
    for h in table::closing() {
        let now = mk_uptime_ms();
        let done = table::with(h, |e| match (&mut e.phase, e.closing) {
            (Phase::Open(link), Some(until)) if !ends(link.queue.queued(), until, now) => {
                link.carry();
                !link.queue.queued()
            }
            _ => true,
        });
        if done == Some(true) {
            drop(table::remove(h));
        }
    }
}

/// When the serve loop next carries closing streams on; None with none.
pub fn wake_ms() -> Option<u64> {
    (!table::closing().is_empty()).then_some(TICK_MS)
}

/// The family has ended: carry every closing stream on until each is done.
pub fn flush() {
    while !table::closing().is_empty() {
        settle();
        let _ = mk_idle_ms(TICK_MS);
    }
}
