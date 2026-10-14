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

//! Taking a finished refresh job's result into the state.

use crate::wallet::net::step::Finished;
use crate::wallet::state::State;

use super::probe_apply::apply;
use super::probe_follow::followed_now;
use super::probe_tick::account_job;

/// Take a finished job's result, returning whether anything shown changed.
pub(super) fn finish(state: &mut State, finished: Finished) -> bool {
    match finished {
        Finished::Probe(net) => {
            let changed = net.status != state.status;
            state.net = net;
            state.status = state.net.status;
            changed
        }
        Finished::Snapshot { snap, address, chain, host } => {
            /* Asked for an account or a network no longer open. */
            if address != state.address || chain != crate::wallet::chain::current().id {
                return false;
            }
            match snap {
                Some(snap) => {
                    /* A new block read is news on Home, whatever else moved. */
                    let mut newer = false;
                    if let Some(block) = snap.head {
                        let route = state.net.route.map_or("", |r| r.label());
                        let at_ms = nonos_libc::mk_uptime_ms();
                        let read =
                            crate::wallet::net::last_read::LastRead { block, host, route, at_ms };
                        let slot =
                            &mut state.last_read[usize::from(crate::wallet::chain::is_sepolia())];
                        newer = slot.map_or(true, |r| r.block != block || r.host != host);
                        *slot = Some(read);
                    }
                    apply(state, snap) | newer
                }
                None => {
                    // The link dropped; fall back to the diagnostic next refresh.
                    state.net.rpc_chain_ok = false;
                    state.status = b"reconnecting to the network";
                    true
                }
            }
        }
        Finished::Follow { followed, address, chain } => {
            let here = address == state.address && chain == crate::wallet::chain::current().id;
            let changed = match followed {
                Some(f) if here => followed_now(state, f),
                _ => false,
            };
            /* The account is read in the same refresh, as it always was. */
            state.net_job = Some(account_job(state));
            changed
        }
    }
}
