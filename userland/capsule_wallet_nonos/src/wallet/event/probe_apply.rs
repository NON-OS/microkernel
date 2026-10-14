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

//! Applying a finished snapshot of the account to the state.

use crate::wallet::net::read_snapshot::Snapshot;
use crate::wallet::state::State;

/// Apply a snapshot, returning whether anything shown changed.
pub(super) fn apply(state: &mut State, snap: Snapshot) -> bool {
    use super::reading::take;
    let staking = crate::wallet::chain::current().staking.is_some();
    let asked = if staking { 9 } else { 5 };
    let came = [
        snap.eth_balance.is_some(),
        snap.nonce.is_some(),
        snap.fee.is_some(),
        snap.nox_balance.is_some(),
        snap.usdc_balance.is_some(),
        snap.claimable.is_some(),
        snap.positions.is_some(),
        snap.passes.is_some(),
        snap.stats.is_some(),
    ]
    .iter()
    .filter(|c| **c)
    .count();
    if snap.nonce.is_some() && !state.nonce_ready {
        state.send.nonce = snap.nonce.unwrap_or(0);
    }
    let mut changed = take(snap.eth_balance, &mut state.balance_wei, &mut state.balance_ready);
    changed |= take(snap.nonce, &mut state.live_nonce, &mut state.nonce_ready);
    changed |= take(snap.fee, &mut state.fee_wei, &mut state.fee_ready);
    let nox = &mut state.nox;
    changed |= take(snap.nox_balance, &mut nox.balance_wei, &mut nox.balance_ready);
    changed |= take(snap.usdc_balance, &mut state.usdc_units, &mut state.usdc_ready);
    let nox = &mut state.nox;
    changed |= take(snap.claimable, &mut nox.claimable_wei, &mut nox.claimable_ready);
    changed |= take(snap.positions, &mut nox.positions, &mut nox.positions_ready);
    changed |= take(snap.passes, &mut nox.passes, &mut nox.passes_ready);
    let total = snap.stats.as_ref().map(|s| s.total);
    let rewards = snap.stats.as_ref().map(|s| s.rewards);
    let apr = snap.stats.as_ref().and_then(|s| s.apr);
    changed |= take(total, &mut nox.total_staked_wei, &mut nox.stats_ready);
    if let Some(r) = rewards {
        nox.rewards_distributed_wei = r;
    }
    /* A rate that cannot be worked out (nothing staked) is not the last one. */
    changed |= take(apr, &mut nox.apr_bps, &mut nox.apr_ready);
    let status: &'static [u8] = match came {
        0 => {
            /* An answer with no readings in it: the link is checked again. */
            state.net.rpc_chain_ok = false;
            b"the network answered without any readings, checking the link again"
        }
        n if n < asked => b"some readings did not come, shown as not read",
        _ => state.net.status,
    };
    changed |= status != state.status;
    state.status = status;
    changed
}
