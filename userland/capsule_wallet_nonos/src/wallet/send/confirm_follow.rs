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
//! Following a sent transaction's receipt onto the chain.

use crate::wallet::act::Purpose;
use crate::wallet::state::State;

/// Whether a sent payment still waits for its receipt.
pub fn awaiting(state: &State) -> bool {
    state.broadcast.ready && !state.broadcast.receipt_ready
}

/// Take what a receipt read for `hash` said; None is no receipt yet, or
/// no answer. True when that changed. A read for a payment that is no
/// longer the one waiting is dropped.
pub fn follow(state: &mut State, hash: &[u8; 32], ok: Option<bool>) -> bool {
    if !awaiting(state) || *hash != state.broadcast.hash {
        return false;
    }
    let Some(ok) = ok else {
        return false;
    };
    state.broadcast.receipt_ready = true;
    state.broadcast.receipt_ok = ok;
    state.broadcast.unknown = false;
    state.status = match (state.broadcast.purpose, ok) {
        (Purpose::Pay, true) => b"payment confirmed",
        (Purpose::Pay, false) => b"payment reverted",
        (Purpose::Approve, true) => b"approval confirmed, review the stake next",
        (Purpose::Approve, false) => b"approval reverted, nothing was staked",
        (Purpose::Stake, true) => b"stake confirmed",
        (Purpose::Stake, false) => b"stake reverted",
        (Purpose::Unstake, true) => b"position closed",
        (Purpose::Unstake, false) => b"unstake reverted",
    };
    /* The stake is offered only once the approval it spends is in a block. */
    match (state.broadcast.purpose, ok) {
        (Purpose::Approve, true) => state.stake_step = 1,
        (Purpose::Stake, true) => {
            state.stake_step = 0;
            crate::wallet::event::stake_clear(state);
        }
        _ => {}
    }
    true
}
