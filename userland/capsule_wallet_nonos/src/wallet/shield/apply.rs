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

//! A finished job's answer, put where the screens read it: each op has one
//! place, and an answer the service did not give is never filled in.

use nonos_libc::mk_uptime_ms;
use shield_wire::*;

use crate::wallet::state::State;

use super::apply_confirmed::confirmed;
use super::apply_open::{opened, taken_back};
use super::apply_proved::proved;
use super::apply_refresh::refresh;
use super::apply_review::{quoted, reviewed};
use super::apply_values::number;

pub fn apply(state: &mut State, op: u16, body: &str) {
    match op {
        OP_OPEN_WORDS | OP_OPEN_KEY => opened(state, body),
        OP_SYNC => {
            let ui = &mut state.shield_ui;
            ui.link.last_sync_ms = mk_uptime_ms();
            if let (Some(l), Some(m)) = (number(body, "wait_leaves"), number(body, "wait_minutes"))
            {
                ui.wait_left = Some((l, m));
            }
            refresh(state);
        }
        OP_REVIEW_SHIELD | OP_REVIEW_SELF_SETTLE => reviewed(state, op, body),
        OP_CONFIRM => confirmed(state, body),
        OP_QUOTE => quoted(state, body),
        OP_SEND | OP_WITHDRAW => proved(state, op, body),
        OP_SIGN_FROM_FRESH => crate::wallet::order::escrow_sign::signed(state, body),
        OP_FRESH_ADDRESS => crate::wallet::shield::scan_job::derived(state, body),
        OP_SHIELD_FROM_FRESH => super::fresh_move_job::applied(state, body),
        OP_FOLLOW => {
            super::follow::followed(state, body);
            refresh(state);
        }
        OP_TAKE_BACK => taken_back(state, body),
        _ => {}
    }
}
