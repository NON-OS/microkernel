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

//! Moving the wallet between Ethereum mainnet and Sepolia.
//!
//! The address is the same on both, so nothing about the account changes.
//! Every figure read from the old network is dropped at once, so no balance,
//! nonce or fee from one chain is ever shown or signed against on the other,
//! and the link is checked again, since the new network is another host.

use super::State;

/* Said when a switch would leave something going out on the old network. */
pub const SWITCH_BUSY: &str = "A payment or a shield job is under way on this network. \
     Let it finish, then change the network.";

/* Ok(false) when the network picked is already this one. A broadcast or a
 * shield job under way holds the network: its answer belongs to the old
 * one, so the switch waits for it rather than leave it half followed. */
pub fn switch_network(state: &mut State, sepolia: bool) -> Result<bool, &'static str> {
    if crate::wallet::chain::is_sepolia() == sepolia {
        return Ok(false);
    }
    let sending = matches!(state.action, Some(crate::wallet::act::Action::Broadcast { .. }));
    if sending || state.shield_ui.waiting.is_some() {
        return Err(SWITCH_BUSY);
    }
    crate::wallet::chain::pick(sepolia);
    /* Kept, so the next boot opens on this network, on a boot that keeps anything. */
    if crate::wallet::vault::persistent() {
        let _ = crate::wallet::vault::remember_network(sepolia);
    }
    forget_live(state);
    /* The shield runs on one network: its store is closed on leaving it, and
     * opened at once on arriving where it runs, so its address is ready. */
    crate::wallet::shield::open::forget_network(state);
    if crate::wallet::chain::current().shield {
        crate::wallet::shield::open::ensure(state);
    } else {
        state.receive_private = false;
    }
    Ok(true)
}

/* Everything read from the chain for the open account and network: it is
 * read again, never carried over to another account or network. */
pub fn forget_live(state: &mut State) {
    state.balance_ready = false;
    state.balance_wei = [0; 32];
    state.usdc_ready = false;
    state.usdc_units = [0; 32];
    state.nonce_ready = false;
    state.live_nonce = 0;
    state.fee_ready = false;
    state.fee_wei = 0;
    state.nox = crate::wallet::nox::NoxStatus::empty();
    state.tx_ready = false;
    state.tx_raw.clear();
    state.broadcast_armed = false;
    state.broadcast_ready = false;
    state.receipt_ready = false;
    state.receipt_ok = false;
    state.broadcast_hash = [0; 32];
    /* A payment reviewed or sent on the last network is not this one's: the
     * form keeps what was typed, and is reviewed again before it signs. */
    if !matches!(state.action, Some(crate::wallet::act::Action::Broadcast { .. })) {
        state.send_draft = None;
        state.send_stage = crate::wallet::send::STAGE_FORM;
    }
    /* A quote and an allowance belong to one network's contracts. */
    state.swap_quote = crate::wallet::swap::Quote::default();
    state.swap_step = 0;
    state.stake_step = 0;
    state.stake_draft = None;
    state.stake_position = 0;
    state.net = super::default_net();
    /* The last network's link status is not this one's. */
    state.status = b"reading the network";
    /* A refresh begun for the last account or network is not finished. */
    state.net_job = None;
    state.broadcast_unknown = false;
    /* A review read for the last account or network is dropped. A
     * broadcast under way is not: it may have left already, and is said when it
     * ends, without being taken as this account's. */
    if matches!(state.action, Some(crate::wallet::act::Action::Review { .. })) {
        state.action = None;
    }
    state.probe_step = 1;
}
