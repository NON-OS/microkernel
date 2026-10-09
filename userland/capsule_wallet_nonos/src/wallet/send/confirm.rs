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

/// How long a transaction that may have gone holds the next one back: the
/// receipt is read every dozen seconds, and one not seen in this long most
/// likely never reached the node.
pub const HOLD_MS: i64 = 10 * 60 * 1000;

/// Why no new transaction may be reviewed now, or None. A broadcast whose
/// outcome is unknown may still land, and an approve not yet in a block
/// gives the stake nothing to spend: either holds the next signature back
/// until its receipt is read, or for `HOLD_MS` at most.
pub fn held_back(state: &State) -> Option<&'static str> {
    let now = nonos_libc::mk_uptime_ms();
    let chain = chain::current().id;
    if state.sent.unknown_waiting(chain, &state.address, now, HOLD_MS) {
        return Some(
            "The last transaction may still land. A new one waits for its receipt, \
             for ten minutes at most, so nothing is paid twice.",
        );
    }
    if !awaiting(state) {
        return None;
    }
    let recent = now.wrapping_sub(state.sent_at_ms) < HOLD_MS;
    if recent && state.follow_purpose == Purpose::Approve {
        return Some("Waiting for the approval to land in a block before the stake.");
    }
    None
}

/// Sign the reviewed payment and record it for the broadcast.
pub fn sign(state: &mut State) -> Result<(), &'static str> {
    let draft = state.send_draft.clone().ok_or("Nothing to confirm. Review the payment again.")?;
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
    state.tx_nonce = draft.nonce;
    state.tx_cost = draft.value.saturating_add(draft.fee_cap_wei());
    /* What leaves the token balance: a payment's or a stake's amount. An
     * approval or an unstake moves none of it from here. */
    let moves = matches!(draft.purpose, Purpose::Pay | Purpose::Stake);
    state.tx_token =
        (moves && draft.asset != super::ASSET_ETH).then_some((draft.asset, draft.amount));
    Ok(())
}

/// Whether a sent payment still waits for its receipt.
pub fn awaiting(state: &State) -> bool {
    state.broadcast_ready && !state.receipt_ready
}

/// Take what a receipt read for `hash` said; None is no receipt yet, or
/// no answer. True when that changed. A read for a payment that is no
/// longer the one waiting is dropped.
pub fn follow(state: &mut State, hash: &[u8; 32], ok: Option<bool>) -> bool {
    if !awaiting(state) || *hash != state.broadcast_hash {
        return false;
    }
    let Some(ok) = ok else {
        return false;
    };
    state.receipt_ready = true;
    state.receipt_ok = ok;
    state.broadcast_unknown = false;
    state.status = match (state.follow_purpose, ok) {
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
    match (state.follow_purpose, ok) {
        (Purpose::Approve, true) => state.stake_step = 1,
        (Purpose::Stake, true) => {
            state.stake_step = 0;
            crate::wallet::event::stake_clear(state);
        }
        _ => {}
    }
    true
}
