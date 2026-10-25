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

//! A confirmed send's answer: its transaction and the log entry.

use alloc::format;
use alloc::string::String;

use shield_wire::*;

use crate::wallet::state::shield_log::Kind;
use crate::wallet::state::shield_ui::*;
use crate::wallet::state::State;

use super::apply_refresh::refresh;

pub(super) fn confirmed(state: &mut State, body: &str) {
    let Some(review) = state.shield_ui.review.take() else { return };
    let settle = state.shield_ui.from == SHIELD_SETTLE;
    let kind = match (settle, review.approval) {
        (true, _) => Kind::SelfSettle,
        (_, true) => Kind::Approval,
        _ => Kind::Deposit,
    };
    let tx = field(body, "tx").map(String::from);
    refresh(state);
    let ui = &mut state.shield_ui;
    /* The transaction is named when the service named it. */
    let hash = tx.map(|t| format!(" ({t})")).unwrap_or_default();
    ui.news = Some(match kind {
        Kind::Approval => {
            format!("Approval sent{hash}. Once it lands, review the deposit again to send it.")
        }
        Kind::SelfSettle => {
            let id = ui.settling_id.take();
            let spend = match &id {
                None => ui.spend.as_mut(),
                Some(id) => ui.earlier.iter_mut().find(|s| s.id.as_ref() == Some(id)),
            };
            if let Some(spend) = spend {
                spend.state = String::from(super::follow::SETTLING);
                spend.self_settle = false;
            }
            /* Followed from now on to the block, as a lander's would be. */
            ui.link.last_follow_ms = 0;
            format!("Sent from your own account{hash}. It links this account to the payment.")
        }
        _ => format!("Deposit sent{hash}. It is spendable once it matures in the pool."),
    });
    ui.screen = if settle { SHIELD_PROVING } else { SHIELD_HOME };
    ui.size = None;
    state.scroll = 0;
}
