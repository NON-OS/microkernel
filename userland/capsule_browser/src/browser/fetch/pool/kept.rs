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
        while let Some(at) = self.idle.iter().rposition(|i| i.serves(url)) {
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

    /// Keep `idle` for a later request, closing the oldest kept connection
    /// if that would be more connections than the pool allows.
    pub fn park<W: Wire>(&mut self, w: &mut W, idle: Idle) {
        let (_, all) = Pool::limits(w.mixnet());
        self.idle.push(idle);
        while self.live.len() + self.idle.len() > all && !self.idle.is_empty() {
            let old = self.idle.remove(0);
            w.close(old.handle);
        }
    }
}
