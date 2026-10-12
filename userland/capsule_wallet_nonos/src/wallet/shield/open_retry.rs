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

//! A service or keyring too slow to answer is asked again, later and later.

use super::*;

/* The longest wait between two asks of a service that does not answer. */
const RETRY_MAX_MS: i64 = 30_000;

/* No answer in time: ask again later, by itself, waiting longer each time. */
pub(super) fn later(state: &mut State) {
    let ui = &mut state.shield_ui;
    let wait = (2_000i64 << ui.tries.min(4)).min(RETRY_MAX_MS);
    ui.tries = ui.tries.saturating_add(1);
    ui.retry_at_ms = Some(mk_uptime_ms() + wait);
    ui.failure = None;
}

/* The service lost its job and its open store, as when it restarted. */
pub fn restarted(state: &mut State) {
    later(state);
}

/* The service answered: no ask is pending. */
pub(super) fn answered(state: &mut State) {
    state.shield_ui.retry_at_ms = None;
    state.shield_ui.tries = 0;
}

/* The pending ask, once its time has come. True when the screen should repaint. */
pub fn retry_due(state: &mut State) -> bool {
    match state.shield_ui.retry_at_ms {
        Some(at) if mk_uptime_ms() >= at && state.shield_ui.waiting.is_none() => {
            state.shield_ui.retry_at_ms = None;
            if here() && state.address_ready {
                probe_then_open(state);
            }
            true
        }
        _ => false,
    }
}

/* Whether the wallet is waiting to ask a slow service again. */
pub fn retrying(state: &State) -> bool {
    state.shield_ui.retry_at_ms.is_some()
}

/* A keyring too slow to answer is asked again later, not taken as a refusal. */
pub(super) fn keyring_late(state: &mut State) {
    later(state);
    state.shield_ui.failure = Some(String::from(KEYRING_LATE));
}

/* An open that went unanswered is asked again; any other refusal stands. */
pub(super) fn slow(state: &mut State) {
    if state.shield_ui.failure.as_deref() == Some(NO_ANSWER) {
        later(state);
    }
}
