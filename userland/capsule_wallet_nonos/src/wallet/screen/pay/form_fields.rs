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
//! The fields of the payment form, and what this account holds of the
//! asset chosen.

use alloc::format;
use alloc::string::String;

use nonos_app_skeleton::PaintBuffer;

use super::super::hits::{self, Press};
use crate::wallet::chain;
use crate::wallet::etna::parts::fact::fact;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::GAP;
use crate::wallet::screen::amounts;
use crate::wallet::screen::shield::parts::{chips, field};
use crate::wallet::send::{amount_text, asset_name, ASSET_ETH, ASSET_NOX};
use crate::wallet::state::{State, SEND_FIELD_AMOUNT, SEND_FIELD_TO};

fn held(state: &State) -> String {
    match state.send.token {
        ASSET_ETH => amounts::eth(state),
        ASSET_NOX => amounts::nox(state),
        _ => amounts::usdc(state),
    }
}

/// The chips, the two fields and the facts below them, from `y` down;
/// returns where they end.
pub(super) fn fields(state: &State, fb: &mut PaintBuffer, c: Rect, mut y: u32) -> u32 {
    y += chips(fb, c, y, &["ETH", "NOX", "USDC"], Some(state.send.token), Press::Asset) + GAP;
    let to = core::str::from_utf8(&state.send.to_hex[..state.send.to_len]).unwrap_or("");
    let to = if to.is_empty() { String::new() } else { format!("0x{to}") };
    let focused = state.send.focus == SEND_FIELD_TO;
    let (at, h) = field(fb, c, y, "TO", &to, "0x and 40 hex digits (Ctrl+V pastes)", focused);
    hits::put(Press::Field(SEND_FIELD_TO), at);
    y += h + GAP / 2;
    let name = format!("AMOUNT, {}", asset_name(state.send.token));
    let focused = state.send.focus == SEND_FIELD_AMOUNT;
    let (at, h) = field(fb, c, y, &name, &amount_text(&state.send.amount), "0.0", focused);
    hits::put(Press::Field(SEND_FIELD_AMOUNT), at);
    y += h + GAP;
    let have = format!("{} {}", held(state), asset_name(state.send.token));
    y += fact(fb, c.x, y, c.w, "this account holds", &have);
    y += fact(fb, c.x, y, c.w, "network", chain::current().name);
    let fee = if state.send.all && state.send.token == ASSET_ETH {
        "set aside from the amount on the review, paid in ETH"
    } else {
        "shown on the review, paid in ETH"
    };
    y += fact(fb, c.x, y, c.w, "network fee", fee);
    y
}
