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

//! Locking, and forgetting an account's or a network's shield.

use super::*;

/* The wallet locked: the service forgets the store's keys too, and stops a
 * proof under way. It answers at once, whatever the store is doing. */
pub fn lock(state: &mut State) {
    let _ = call(OP_LOCK, &[]);
    let ui = &mut state.shield_ui;
    ui.waiting = None;
    ui.misses = 0;
    ui.poll_at_ms = 0;
    ui.phase = None;
    ui.permille = None;
    ui.opened = false;
    ui.retry_at_ms = None;
    ui.tries = 0;
    ui.nox1 = None;
    ui.held = [None, None];
    ui.review = None;
    ui.quote = None;
    ui.live = false;
    ui.wait_left = None;
    ui.withdraw_to = None;
}

/* Another account is open: nothing of the last one's shield is kept, its
 * history and the spend it followed included. */
pub fn forget_account(state: &mut State) {
    lock(state);
    state.shield_ui = crate::wallet::state::shield_ui::ShieldUi::default();
}

/* The network changed: the store is closed and nothing it said is shown
 * on the other network. The history and the spend being followed stay,
 * since only the shield's network has them, and are read again once the
 * store opens there. */
pub fn forget_network(state: &mut State) {
    lock(state);
    let ui = &mut state.shield_ui;
    ui.screen = crate::wallet::state::shield_ui::SHIELD_HOME;
    ui.phase = None;
    ui.permille = None;
    ui.news = None;
    ui.failure = None;
}
