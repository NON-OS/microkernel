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
//! After the payment left: its hash, where to look it up, and what the
//! network has said about it so far.

use alloc::format;
use alloc::string::String;

use nonos_app_skeleton::PaintBuffer;

use super::super::hits;
use super::super::page::{edges, lead};
use crate::wallet::chain;
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::etna::parts::fact::fact;
use crate::wallet::etna::parts::value::value_block;
use crate::wallet::etna::tokens::GAP;
use crate::wallet::state::State;

fn said(state: &State) -> String {
    use crate::wallet::send::sent::{Fate, CONFIRMATIONS};
    let fate = state.sent.get(&state.broadcast_hash).map(|s| s.fate);
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
    let unknown = state.broadcast_ready && state.broadcast_unknown && !state.receipt_ready;
    if unknown && crate::wallet::send::held_back(state).is_none() {
        return "The network broke off before saying whether it took this payment, and in \
                ten minutes it has not been seen in a block, so it most likely never went. \
                A new payment may be made; this still watches for this one.";
    }
    if unknown {
        return "The network broke off before saying whether it took this payment. It is \
                not sent again; this watches for it to land in a block.";
    }
    match (state.broadcast_ready, state.receipt_ready, state.receipt_ok) {
        (false, _, _) => "Signed, and the network did not take it. Nothing was spent.",
        (true, false, _) => "Sent. Waiting for the network to put it in a block.",
        (true, true, true) => "Confirmed: the payment is in a block, and twelve blocks hold it.",
        (true, true, false) => "In a block, and the transfer reverted. Only the fee was spent.",
    }
}

pub fn done(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = super::super::status::parts(state);
    let footer = [("Done", Weight::Primary, true)];
    let spec = FrameSpec {
        number: "05",
        title: "Sent",
        back: false,
        backdrop: Some(Backdrop::History),
        failure: state.failure,
        footer: &footer,
        status: &status,
        scroll: state.scroll,
    };
    let mut l = begin(fb, &spec);
    let c = l.content;
    let mut y = lead(fb, c, &said(state));
    if state.broadcast_ready {
        let mut hash = String::from("0x");
        for b in state.broadcast_hash {
            hash.push_str(&format!("{b:02x}"));
        }
        y += value_block(fb, c.x, y, c.w, &hash) + GAP;
        y += fact(fb, c.x, y, c.w, "look it up at", chain::current().explorer);
    }
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    edges(&l, &[true]);
}
