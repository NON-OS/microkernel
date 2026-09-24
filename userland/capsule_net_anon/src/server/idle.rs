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

use crate::manager::{
    circuit_tick, directory_tick, link_tick, pump_tick, retire_tick, sendme_tick, Manager,
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
    pump_tick(state);
    sendme_tick(state);
}
