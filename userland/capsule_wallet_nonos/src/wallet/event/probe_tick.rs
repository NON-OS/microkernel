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

use nonos_app_skeleton::EventOutcome;

use crate::wallet::net::step::Job;
use crate::wallet::state::State;

use super::probe_finish::finish;
pub(super) use super::probe_kick::probe_kick;

/// Refresh the account, a step per tick. The expensive network
/// self-diagnostic (the Nym service, the full TLS check) runs once to confirm
/// the link; after that every refresh is a single batched round trip that
/// fetches balance, nonce, fee and the staking figures over one connection.
/// The payments still on their way, on this network and account, are read
/// first, all in one batch with the newest block and the account's nonces. No step waits on
/// the network longer than a slice (`net::step`), and only a finished job
/// touches the state. Returns Repaint only when a shown value actually
/// changed, so a steady screen does not recomposite every cycle. On a
/// failed fetch the link is marked degraded so the next refresh re-runs the
/// diagnostic and reconnects. `due` begins a refresh when none is under way.
pub fn probe_tick(state: &mut State, due: bool) -> EventOutcome {
    if state.net_job.is_none() {
        if !due {
            return EventOutcome::Idle;
        }
        state.probe_step = 0;
        let chain = crate::wallet::chain::current().id;
        let open = state.broadcast.sent.open(chain, &state.address);
        state.net_job = Some(if open.is_empty() {
            account_job(state)
        } else {
            Job::follow(open, &state.address)
        });
    }
    let Some(finished) = state.net_job.as_mut().and_then(Job::step) else {
        return EventOutcome::Idle;
    };
    state.net_job = None;
    outcome(finish(state, finished))
}

/// The diagnostic while the link is unconfirmed, the snapshot after.
pub(super) fn account_job(state: &State) -> Job {
    /* A host is read from only once it answered this network's chain id. */
    if state.net.rpc_chain_ok && state.net.rpc_host == crate::wallet::chain::rpc_host() {
        Job::snapshot(&state.address)
    } else {
        Job::probe()
    }
}

fn outcome(changed: bool) -> EventOutcome {
    if changed {
        EventOutcome::Repaint
    } else {
        EventOutcome::Idle
    }
}
