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

//! When the follow and the pool sync are due while Shield is open.

use nonos_libc::mk_uptime_ms;
use shield_wire::*;

use crate::wallet::state::State;

const EVERY_MS: i64 = 60_000;
/* The pool read again while Shield is open, for receipts and maturity. */
const SYNC_EVERY_MS: i64 = 180_000;

/* Whether the pool was never read for this open store, or not for a sync
 * interval: opening Shield again within it reads nothing new. */
pub fn sync_stale(ui: &crate::wallet::state::shield_ui::ShieldUi, now: i64) -> bool {
    ui.link.last_sync_ms == 0 || now - ui.link.last_sync_ms >= SYNC_EVERY_MS
}

/* Whether any spend is not yet in a block, however it is being landed. */
fn following(state: &State) -> bool {
    let ui = &state.shield_ui;
    ui.spend.as_ref().is_some_and(|s| s.tx.is_none()) || !ui.earlier.is_empty()
}

/* Start whatever background read is due. True when one started. */
pub fn due(state: &mut State) -> bool {
    let ui = &state.shield_ui;
    if !ui.opened || ui.waiting.is_some() {
        return false;
    }
    let now = mk_uptime_ms();
    if following(state) && now - ui.link.last_follow_ms >= EVERY_MS {
        state.shield_ui.link.last_follow_ms = now;
        return super::job::start(state, OP_FOLLOW, &[]);
    }
    if sync_stale(ui, now) {
        state.shield_ui.link.last_sync_ms = now;
        return super::job::start(state, OP_SYNC, &[]);
    }
    false
}
