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

//! What the capsule does in the gaps between requests.

use nonos_libc::mk_time_millis;

use crate::stream::StreamStage;

use crate::manager::{
    circuit_tick, directory_tick, link_tick, onion_tick, pump_tick, retire_tick, sendme_tick,
    Manager,
};

/*
 * Order matters and was learned the hard way in the mixnet transport. The
 * directory comes first: it is fetched over net.tcp, needs no circuit, and
 * holding a link across a thirty second fetch is how one is lost. Then the link,
 * then a circuit over it. The pump runs last because the three above are what
 * create something to read.
 *
 * SENDMEs are paid immediately after the pump, in the same gap that consumed the
 * windows. Deferring them to the next idle turn is how a circuit stalls under a
 * fast download.
 *
 * Retirement runs before the build, not after: a spent circuit still in the table
 * counts against the cap, so a client that had built its three and retired two
 * would never draw a replacement exit.
 */
pub fn idle(state: &mut Manager, now: u64) {
    directory_tick(state, now);
    link_tick(state, now);
    retire_tick(state, now);
    circuit_tick(state, now);
    onion_tick(state, now);
    pump_tick(state, now);
    sendme_tick(state);
}

/// How long the serve loop waits for a request before idling, when nothing
/// is moving.
pub const IDLE_MS: u64 = 200;
/// The same while streams are open or a lookup runs. Relay cells are read
/// only on the idle path, so this is the delay a reply waits before it is
/// read: at 200 ms every round trip of a page load paid up to that again.
pub const BUSY_MS: u64 = 5;

/// How long to wait for a request: short while a stream is opening or open
/// or a lookup runs. A stream that has ended and waits only for its caller
/// to close it does not count, so a caller that never closes cannot hold
/// the loop at the short wait.
pub fn wait_ms(state: &Manager) -> u64 {
    let live = state.streams.iter().any(|s| !matches!(s.stage, StreamStage::Ended(_)));
    if live || !state.onion.is_empty() {
        BUSY_MS
    } else {
        IDLE_MS
    }
}

/*
 * The idle work is the transport itself: the directory, the link, the
 * circuits, reading cells off the link and paying SENDMEs. It used to run only
 * when no request came within IDLE_MS, so a caller polling a stream faster
 * than that kept every cell it was waiting for unread. It now runs at least
 * every IDLE_MS whatever the request traffic.
 */
pub fn idle_due(state: &mut Manager, now: u64, quiet: bool, last: &mut i64) {
    let ms = mk_time_millis();
    if quiet || ms.wrapping_sub(*last) >= wait_ms(state) as i64 {
        idle(state, now);
        *last = ms;
    }
}
