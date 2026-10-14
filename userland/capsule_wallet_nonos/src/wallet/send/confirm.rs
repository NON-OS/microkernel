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
//! Signing a reviewed payment and following it onto the chain.

use super::Draft;
use crate::wallet::act::Purpose;
use crate::wallet::chain;
use crate::wallet::ipc::{sign_tx, TxRequest};
use crate::wallet::state::{record_tx, State};

#[path = "confirm_follow.rs"]
mod confirm_follow;
#[path = "confirm_hold.rs"]
mod confirm_hold;
pub use confirm_follow::{awaiting, follow};
pub use confirm_hold::held_back;

/// Sign the reviewed payment and record it for the broadcast.
pub fn sign(state: &mut State) -> Result<(), &'static str> {
    let draft = state.send.draft.clone().ok_or("Nothing to confirm. Review the payment again.")?;
    sign_draft(state, &draft, b"send")
}

/// Sign a reviewed draft and record it (`record_tx`), with its own hash,
/// for the broadcast. The network is checked once more, so a draft
/// made on one chain is never signed after a switch. Nothing goes out here.
pub fn sign_draft(
    state: &mut State,
    draft: &Draft,
    kind: &'static [u8],
) -> Result<(), &'static str> {
    if draft.chain_id != chain::current().id {
        return Err("The network changed since this review. Review the payment again.");
    }
    let tx = TxRequest {
        to: draft.to,
        value: draft.value,
        data: &draft.data,
        nonce: draft.nonce,
        gas: draft.gas,
        max_priority: draft.tip,
        max_fee: draft.max_fee,
    };
    let raw = sign_tx(state.keyring_port, state.owner_pid, state.wallet_id, draft.chain_id, &tx)
        .map_err(|_| "The keyring would not sign this transaction.")?;
    let mut hash = [0u8; 32];
    if !crate::wallet::tx_hash::tx_hash(&raw, &mut hash) {
        return Err("The signed transaction could not be hashed.");
    }
    record_tx(state, kind, &raw, hash);
    state.tx.nonce = draft.nonce;
    state.tx.cost = draft.value.saturating_add(draft.fee_cap_wei());
    /* What leaves the token balance: a payment's or a stake's amount. An
     * approval or an unstake moves none of it from here. */
    let moves = matches!(draft.purpose, Purpose::Pay | Purpose::Stake);
    state.tx.token =
        (moves && draft.asset != super::ASSET_ETH).then_some((draft.asset, draft.amount));
    Ok(())
}
