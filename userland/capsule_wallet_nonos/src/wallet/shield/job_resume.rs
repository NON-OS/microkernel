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

//! A job the service already runs for this wallet, waited on again.

use nonos_libc::mk_uptime_ms;
use shield_wire::*;

use crate::wallet::state::shield_ui::Waiting;
use crate::wallet::state::State;

/* The op a job the service names was started with, for a wallet that
 * comes back to a job it did not see begin. */
fn op_of(name: &str) -> Option<u16> {
    Some(match name {
        "open" => OP_OPEN_WORDS,
        "sync" => OP_SYNC,
        "review shield" => OP_REVIEW_SHIELD,
        "confirm" => OP_CONFIRM,
        "quote" => OP_QUOTE,
        "send" => OP_SEND,
        "withdraw" => OP_WITHDRAW,
        "follow" => OP_FOLLOW,
        "review settle" => OP_REVIEW_SELF_SETTLE,
        "take back" => OP_TAKE_BACK,
        "sign" => OP_SIGN_FROM_FRESH,
        "scan" => OP_FRESH_ADDRESS,
        "shield fresh" => OP_SHIELD_FROM_FRESH,
        _ => return None,
    })
}

/* The service is running `name` for this wallet already: wait on it. */
pub fn resume(state: &mut State, name: &str) -> bool {
    let Some(op) = op_of(name) else { return false };
    let ui = &mut state.shield_ui;
    ui.waiting = Some(Waiting { op, since_ms: mk_uptime_ms() });
    ui.misses = 0;
    ui.poll_at_ms = 0;
    if matches!(op, OP_SEND | OP_WITHDRAW) {
        ui.screen = crate::wallet::state::shield_ui::SHIELD_PROVING;
    }
    true
}
