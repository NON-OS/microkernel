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
//! What the Sent screen says of the payment: its fate once known, or what
//! the network has said before that.

use alloc::format;
use alloc::string::String;

use crate::wallet::state::State;

pub(super) fn said(state: &State) -> String {
    use crate::wallet::send::sent::{Fate, CONFIRMATIONS};
    let fate = state.broadcast.sent.get(&state.broadcast.hash).map(|s| s.fate);
    let head = match fate {
        Some(Fate::InBlock { block, ok: true }) => Some(format!(
            "In block {block}. It is said to be confirmed once {CONFIRMATIONS} blocks hold it."
        )),
        Some(Fate::InBlock { block, ok: false }) => {
            Some(format!("In block {block}, and the transfer reverted. Only the fee was spent."))
        }
        Some(Fate::Replaced) => Some(String::from(
            "Another transaction from this account used the same nonce and landed instead. \
             This payment will never be made.",
        )),
        Some(Fate::Dropped) => Some(String::from(
            "No block has this payment and no node holds it any more, after thirty \
             minutes. Nothing was paid, and a new payment may be made.",
        )),
        _ => None,
    };
    head.unwrap_or_else(|| String::from(said_before(state)))
}

fn said_before(state: &State) -> &'static str {
    let unknown =
        state.broadcast.ready && state.broadcast.unknown && !state.broadcast.receipt_ready;
    if unknown && crate::wallet::send::held_back(state).is_none() {
        return "The network broke off before saying whether it took this payment, and in \
                ten minutes it has not been seen in a block, so it most likely never went. \
                A new payment may be made; this still watches for this one.";
    }
    if unknown {
        return "The network broke off before saying whether it took this payment. It is \
                not sent again; this watches for it to land in a block.";
    }
    match (state.broadcast.ready, state.broadcast.receipt_ready, state.broadcast.receipt_ok) {
        (false, _, _) => "Signed, and the network did not take it. Nothing was spent.",
        (true, false, _) => "Sent. Waiting for the network to put it in a block.",
        (true, true, true) => "Confirmed: the payment is in a block, and twelve blocks hold it.",
        (true, true, false) => "In a block, and the transfer reverted. Only the fee was spent.",
    }
}
