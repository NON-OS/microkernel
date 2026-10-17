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

/*
 * The meter's running job, asked on every tick while one runs (v1.4.14):
 * JOB answers at once, so the window never waits on the network.
 */

use crate::wallet::meter::job::{closed_due, outcome, Outcome};
use crate::wallet::shield::due::Why;
use crate::wallet::state::State;

/// The meter's work on this tick: a running job, a purchase's step, or
/// the mailbox (B8). Whether the screen changed.
pub fn tick(state: &mut State) -> bool {
    (state.meter.job.is_some() && poll(state))
        || crate::wallet::order::consent::poll(state)
        || crate::wallet::order::step::poll(state)
        || crate::wallet::order::watch::poll(state)
        || crate::wallet::order::deliver::poll(state)
        || crate::wallet::order::escrow_drive::poll(state)
        || crate::wallet::shield::landed_poll::poll(state)
        || crate::wallet::shield::scan_job::derive(state)
        || crate::wallet::order::opened::poll(state)
        || crate::wallet::order::voucher_cap::poll(state)
        || crate::wallet::order::intake::take_due(state)
        || crate::wallet::publish::send_drive::poll(state)
}

/// Whether the screen changed.
pub fn poll(state: &mut State) -> bool {
    let Some(job) = state.meter.job else {
        return false;
    };
    let answer = super::call::job(job.id);
    let o = outcome(job.kind, answer.as_deref().map_err(|e| *e));
    if let Some(reclaim_to) = closed_due(&job, &o) {
        let now = nonos_libc::mk_uptime_ms().max(0) as u64;
        state.meter.landed.due.add(reclaim_to, Why::ChannelClosed, now);
    }
    match o {
        Outcome::Running => return false,
        Outcome::Sent(line) => state.meter.news = Some(line),
        Outcome::Failed(why) => state.failure = Some(why),
    }
    state.meter.job = None;
    state.meter.account = super::call::account(0);
    true
}
