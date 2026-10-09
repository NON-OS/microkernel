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

//! The pool's kept connections: finding one for a request, and holding one.

use super::idle::Idle;
use super::slots::Pool;
use crate::browser::fetch::wire::{Wire, READABLE, WRITABLE};
use crate::browser::net::mixnet::Way;
use crate::browser::url::Url;

/* A connection kept this long without a request is closed, not tried. */
const IDLE_MAX_MS: i64 = 20_000;

impl Pool {
    /*
     * A server closes a connection it has kept long enough, often with nothing
     * a read can tell from quiet. Old connections are not tried; one the
     * transport no longer calls writable was reset; and one with bytes
     * waiting between responses holds the server's goodbye, not an answer.
     */
    /// A kept connection that serves `url`, the most recent first.
    pub fn take_idle<W: Wire>(&mut self, w: &mut W, url: &Url) -> Option<Idle> {
        let now = w.now_ms();
        let way = w.way(&url.host);
        while let Some(at) = self.idle.iter().rposition(|i| i.serves(url, way)) {
            let idle = self.idle.remove(at);
            let fresh = now.wrapping_sub(idle.since_ms) <= IDLE_MAX_MS;
            let quiet = |bits: u8| bits & WRITABLE != 0 && bits & READABLE == 0;
            if fresh && w.poll(idle.handle).is_ok_and(quiet) {
                return Some(idle);
            }
            w.close(idle.handle);
        }
        None
    }

    /// Free a stream at the proxy `way` leaves through, for a navigation:
    /// the oldest kept connection there goes first, then the oldest fetch
    /// still running there, which belongs to the page being left. False
    /// when nothing of the pool's holds one.
    pub fn free_stream<W: Wire>(&mut self, w: &mut W, way: Way) -> bool {
        while !w.room(way) {
            if let Some(at) = self.idle.iter().position(|i| same_proxy(i.way, way)) {
                let old = self.idle.remove(at);
                w.close(old.handle);
            } else if let Some(at) = self.live.iter().position(|f| same_proxy(f.way, way)) {
                let old = self.live.remove(at);
                w.close(old.handle);
            } else {
                return false;
            }
        }
        true
    }

    /// Keep `idle` for a later request, closing the oldest kept connection
    /// if that would be more connections than the pool allows.
    pub fn park<W: Wire>(&mut self, w: &mut W, idle: Idle) {
        /* A connection through a proxy holds one of its few streams, so it
         * is kept under the proxied limit; kept under the direct one, eight
         * of them could leave the next navigation no stream at all. */
        let (_, all) = Pool::limits(idle.way.proxied());
        self.idle.push(idle);
        while self.live.len() + self.idle.len() > all && !self.idle.is_empty() {
            let old = self.idle.remove(0);
            w.close(old.handle);
        }
    }
}

/// Whether `a` and `b` go through the same proxy, whose streams they share.
fn same_proxy(a: Way, b: Way) -> bool {
    match (a, b) {
        (Way::Proxy { port: x, .. }, Way::Proxy { port: y, .. }) => x == y,
        _ => false,
    }
}
