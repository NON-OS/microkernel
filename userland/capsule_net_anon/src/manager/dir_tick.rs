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

use crate::directory::authority::REQUIRED_SIGNATURES;
use crate::directory::consensus::is_stale;
use crate::trace;

use super::cert_rule::enough;
use super::dir_certs::sweep as sweep_certs;
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
 * One step of one fetch per turn, and no waiting inside a turn: the turn a
 * fetch waits in is the turn the link, the circuits and every caller of the
 * service do not get.
 */
/*
 * How long to wait before trying the directory again after a stage got nowhere.
 * Short enough that a lease arriving is acted on promptly, long enough that a
 * sweep of seven instant refusals does not restart at once. The manager's clock is in
 * whole seconds (server::runner::seconds), so this is one second; it was
 * written as 750 against that clock and waited twelve and a half minutes.
 */
const RETRY_SECONDS: u64 = 1;

pub fn tick(state: &mut Manager, now: u64) {
    if now < state.retry_after {
        return;
    }
    match state.bootstrap {
        Bootstrap::Cold => anchor(state, now),
        Bootstrap::Anchored => load(state, now),
        Bootstrap::Joining => dir_join::tick(state),
        Bootstrap::Ready if is_stale(state.fresh_until, now) => {
            trace::say(b"consensus no longer fresh, refetching while it serves");
            state.refreshing = true;
            state.bootstrap = Bootstrap::Anchored;
        }
        Bootstrap::Ready => {}
    }
}

fn anchor(state: &mut Manager, now: u64) {
    let Some(held) = sweep_certs(state, now) else { return };
    if !enough(held, REQUIRED_SIGNATURES) {
        /*
         * Too few for any consensus to pass. Before the lease that is every
         * authority refusing instantly; after it, the ones that failed are
         * asked again. What did anchor is kept, so the next sweep asks only
         * the rest, and it waits rather than spinning.
         */
        trace::say_two(b"authority certs held and needed", held as u64, REQUIRED_SIGNATURES as u64);
        state.retry_after = now.saturating_add(RETRY_SECONDS);
        return;
    }
    state.bootstrap = Bootstrap::Anchored;
}
