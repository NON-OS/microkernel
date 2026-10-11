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

//! A review's and a quote's answers, where the screens read them.

use alloc::string::String;

use shield_wire::*;

use crate::wallet::state::shield_ui::*;
use crate::wallet::state::State;

use super::apply_values::text;

pub(super) fn reviewed(state: &mut State, op: u16, body: &str) {
    let ui = &mut state.shield_ui;
    if let Some(why) = field(body, "refusal") {
        ui.failure = Some(String::from(why));
        return;
    }
    ui.review = Some(Review {
        id: text(body, "id"),
        approval: field(body, "approval") == Some("1"),
        amount: text(body, "amount"),
        pool_fee: text(body, "pool_fee"),
        shielded: text(body, "shielded"),
        max_network_fee: text(body, "max_network_fee"),
        valid_for: text(body, "valid_for"),
    });
    ui.from = if op == OP_REVIEW_SELF_SETTLE { SHIELD_SETTLE } else { SHIELD_DEPOSIT };
    ui.screen = SHIELD_REVIEW;
    state.scroll = 0;
}
pub(super) fn quoted(state: &mut State, body: &str) {
    if crate::wallet::order::fees::quoted(state, body) {
        return;
    }
    let ui = &mut state.shield_ui;
    if let Some(why) = field(body, "refusal") {
        ui.failure = Some(String::from(why));
        return;
    }
    ui.quote = Some(Quote {
        network_fee: text(body, "network_fee"),
        protocol_fee: text(body, "protocol_fee"),
        total_fee: text(body, "total_fee"),
    });
    ui.from = ui.screen;
    ui.screen = SHIELD_REVIEW;
    state.scroll = 0;
}
