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

//! An unanswered ask, waited on longer each time, and a cancel.

use alloc::string::String;

use shield_wire::OP_CANCEL;

use super::client::call;
use crate::wallet::state::State;

const LOST: &str = "The shield service stopped answering, so what it was doing is not known. \
     The wallet opens the shield again by itself and reads what it holds.";
/* Asks in a row without an answer before the job is said lost: with the
 * waits below, about two minutes of a service that does not answer. */
pub const MISSES: u8 = 8;
const POLL_MAX_MS: i64 = 30_000;

/* A result ask the service did not answer: ask later, waiting longer each
 * time, and after MISSES in a row stop waiting and say so. */
pub(super) fn missed(state: &mut State, now: i64) -> bool {
    let ui = &mut state.shield_ui;
    ui.misses = ui.misses.saturating_add(1);
    if ui.misses < MISSES {
        ui.poll_at_ms = now + (1_000i64 << ui.misses.min(5)).min(POLL_MAX_MS);
        return false;
    }
    ui.waiting = None;
    ui.misses = 0;
    ui.poll_at_ms = 0;
    ui.opened = false;
    ui.phase = None;
    ui.permille = None;
    if ui.screen == crate::wallet::state::shield_ui::SHIELD_PROVING && ui.spend.is_none() {
        ui.screen = ui.from;
    }
    super::open::restarted(state);
    state.shield_ui.failure = Some(String::from(LOST));
    true
}

/* Stop a proof at its next phase. The job then ends as failed. */
pub fn cancel() {
    let _ = call(OP_CANCEL, &[]);
}
