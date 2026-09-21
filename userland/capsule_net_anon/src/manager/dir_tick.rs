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

//! One step of the directory bootstrap, run when nothing else is due.

use crate::directory::consensus::is_stale;
use crate::trace;

use super::dir_certs::gather as gather_certs;
use super::dir_join;
use super::dir_load::load;
use super::state::{Bootstrap, Manager};

/*
 * Staged, and each stage retried, because the mixnet transport learned this the
 * expensive way: a stage that fetched once and treated an empty answer as
 * success installed a directory with no gateways in it and refused every send for
 * the life of the boot. Here nothing advances until the stage after it has
 * something real, and Ready is only reached with a path actually drawable.
 *
 * One fetch per turn, never a run of them. Every fetch is a TCP connection this
 * capsule waits on, and the turn it happens in is the turn the link and the
 * circuits do not get.
 */
/*
 * How long to wait before trying the directory again after a stage got nowhere.
 * Short enough that a lease arriving is acted on promptly, long enough that seven
 * instant refusals per turn stop being a busy loop.
 */
const RETRY_MS: u64 = 750;

pub fn tick(state: &mut Manager, now: u64) {
    if now < state.retry_after {
        return;
    }
    match state.bootstrap {
        Bootstrap::Cold => anchor(state, now),
        Bootstrap::Anchored => load(state, now),
        Bootstrap::Joining => dir_join::tick(state),
        Bootstrap::Ready if is_stale(state.fresh_until, now) => {
            trace::say(b"consensus no longer fresh, refetching");
            state.bootstrap = Bootstrap::Anchored;
        }
        Bootstrap::Ready => {}
    }
}

fn anchor(state: &mut Manager, now: u64) {
    let certs = gather_certs(state.tcp_port, now);
    if certs.is_empty() {
        /*
         * Nothing anchored. Before the lease that is every authority refusing
         * instantly, and nothing about the answer can change until the stack has
         * an address, so the next sweep waits rather than spinning.
         */
        state.retry_after = now.saturating_add(RETRY_MS);
        return;
    }
    state.certs = certs;
    state.bootstrap = Bootstrap::Anchored;
}
