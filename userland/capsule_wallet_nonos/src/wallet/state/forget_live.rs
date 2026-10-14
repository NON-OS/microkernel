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

//! Dropping every figure read from the chain for the open account and network.

use super::State;

/* Everything read from the chain for the open account and network: it is
 * read again, never carried over to another account or network. */
pub fn forget_live(state: &mut State) {
    state.reads.balance_ready = false;
    state.reads.balance_wei = [0; 32];
    state.reads.usdc_ready = false;
    state.reads.usdc_units = [0; 32];
    state.reads.nonce_ready = false;
    state.reads.live_nonce = 0;
    state.reads.fee_ready = false;
    state.reads.fee_wei = 0;
    state.nox = crate::wallet::nox::NoxStatus::empty();
    state.tx.ready = false;
    state.tx.raw.clear();
    state.broadcast.armed = false;
    state.broadcast.ready = false;
    state.broadcast.receipt_ready = false;
    state.broadcast.receipt_ok = false;
    state.broadcast.hash = [0; 32];
    /* A payment reviewed or sent on the last network is not this one's: the
     * form keeps what was typed, and is reviewed again before it signs. */
    if !matches!(state.action, Some(crate::wallet::act::Action::Broadcast { .. })) {
        state.send.draft = None;
        state.send.stage = crate::wallet::send::STAGE_FORM;
    }
    /* A quote and an allowance belong to one network's contracts. */
    state.swap.quote = crate::wallet::swap::Quote::default();
    state.swap.step = 0;
    state.stake.step = 0;
    state.stake.draft = None;
    state.stake.position = 0;
    state.net.link = super::default_net();
    /* The last network's link status is not this one's. */
    state.status = b"reading the network";
    /* A refresh begun for the last account or network is not finished. */
    state.net.job = None;
    state.broadcast.unknown = false;
    /* A review read for the last account or network is dropped. A
     * broadcast under way is not: it may have left already, and is said when it
     * ends, without being taken as this account's. */
    if matches!(state.action, Some(crate::wallet::act::Action::Review { .. })) {
        state.action = None;
    }
    state.net.probe_step = 1;
}
