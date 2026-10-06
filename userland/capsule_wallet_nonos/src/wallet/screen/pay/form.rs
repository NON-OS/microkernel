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
//! The payment form: what to send, to whom, and what this account holds.

use alloc::format;
use alloc::string::String;

use nonos_app_skeleton::PaintBuffer;

use super::super::hits::{self, Press};
use super::super::page::{edges, lead};
use crate::wallet::chain;
use crate::wallet::etna::backdrop::Backdrop;
use crate::wallet::etna::frame::{begin, end};
use crate::wallet::etna::frame_spec::FrameSpec;
use crate::wallet::etna::parts::action::Weight;
use crate::wallet::etna::parts::fact::fact;
use crate::wallet::etna::tokens::GAP;
use crate::wallet::screen::amounts;
use crate::wallet::screen::shield::parts::{chips, field};
use crate::wallet::send::{amount_text, asset_name, ASSET_ETH, ASSET_NOX};
use crate::wallet::state::{State, SEND_FIELD_AMOUNT, SEND_FIELD_TO};

const LEAD: &str = "Public: the amount, both addresses and the time are visible to anyone. \
     For a private payment, use Shield.";

pub fn ready(state: &State) -> bool {
    state.send_to_len == 40 && !state.send_amount.is_zero()
}

fn held(state: &State) -> String {
    match state.send_token {
        ASSET_ETH => amounts::eth(state),
        ASSET_NOX => amounts::nox(state),
        _ => amounts::usdc(state),
    }
}

pub fn form(state: &State, fb: &mut PaintBuffer) {
    hits::clear();
    let status = super::super::status::parts(state);
    /* While the network is read for the review the button says so. */
    let working = crate::wallet::act::working(state);
    let ok = ready(state) && working.is_none();
    let all = working.is_none() && crate::wallet::send::held(state, state.send_token).is_some();
    let footer = [
        (working.unwrap_or("Review payment"), Weight::Primary, ok),
        ("Use all", Weight::Secondary, all),
    ];
    let spec = FrameSpec {
        number: "05",
        title: "Send",
        back: true,
        backdrop: Some(Backdrop::Send),
        failure: state.failure,
        footer: &footer,
        status: &status,
        scroll: state.scroll,
    };
    let mut l = begin(fb, &spec);
    let c = l.content;
    let mut y = lead(fb, c, LEAD);
    y += chips(fb, c, y, &["ETH", "NOX", "USDC"], Some(state.send_token), Press::Asset) + GAP;
    let to = core::str::from_utf8(&state.send_to_hex[..state.send_to_len]).unwrap_or("");
    let to = if to.is_empty() { String::new() } else { format!("0x{to}") };
    let focused = state.send_focus == SEND_FIELD_TO;
    let (at, h) = field(fb, c, y, "TO", &to, "0x and 40 hex digits (Ctrl+V pastes)", focused);
    hits::put(Press::Field(SEND_FIELD_TO), at);
    y += h + GAP / 2;
    let name = format!("AMOUNT, {}", asset_name(state.send_token));
    let focused = state.send_focus == SEND_FIELD_AMOUNT;
    let (at, h) = field(fb, c, y, &name, &amount_text(&state.send_amount), "0.0", focused);
    hits::put(Press::Field(SEND_FIELD_AMOUNT), at);
    y += h + GAP;
    let have = format!("{} {}", held(state), asset_name(state.send_token));
    y += fact(fb, c.x, y, c.w, "this account holds", &have);
    y += fact(fb, c.x, y, c.w, "network", chain::current().name);
    let fee = if state.send_all && state.send_token == ASSET_ETH {
        "set aside from the amount on the review, paid in ETH"
    } else {
        "shown on the review, paid in ETH"
    };
    y += fact(fb, c.x, y, c.w, "network fee", fee);
    hits::reach(y, state.scroll, l.content_bottom);
    end(fb, &spec, &mut l);
    edges(&l, &[ok, all]);
}
