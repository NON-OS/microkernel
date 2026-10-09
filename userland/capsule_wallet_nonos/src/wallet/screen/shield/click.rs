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

/*
 * A press on a Shield screen. Back walks one step: from review to the
 * screen it was opened from, from any other Shield screen to Shield home,
 * and from Shield home to the wallet.
 */

use nonos_app_skeleton::EventOutcome;

use crate::wallet::screen::hits::Press;
use crate::wallet::state::shield_ui::{
    FIELD_TO, SHIELD_HOME, SHIELD_PROVING, SHIELD_REVIEW, SHIELD_SETTLE,
};
use crate::wallet::state::{State, VIEW_HOME, VIEW_SHIELD};

/* Enter Shield from the wallet: ask whether a shield service is there and,
 * if it is, open this account's shield store in it. */
pub fn open(state: &mut State) -> EventOutcome {
    state.shield_ui.screen = SHIELD_HOME;
    state.shield_ui.absent_hidden = false;
    state.shield_ui.failure = None;
    state.failure = None;
    if crate::wallet::shield::open::here() {
        state.shield = crate::wallet::shield::probe::probe();
    }
    if crate::wallet::shield::open::here() && state.shield.available() {
        crate::wallet::shield::open::open(state);
    }
    state.view = VIEW_SHIELD;
    state.scroll = 0;
    EventOutcome::Repaint
}

pub fn back(state: &mut State) -> EventOutcome {
    let ui = &mut state.shield_ui;
    ui.failure = None;
    match ui.screen {
        SHIELD_HOME => state.view = VIEW_HOME,
        SHIELD_REVIEW if ui.from == SHIELD_SETTLE => ui.screen = SHIELD_PROVING,
        SHIELD_REVIEW => ui.screen = ui.from,
        _ => ui.screen = SHIELD_HOME,
    }
    state.scroll = 0;
    EventOutcome::Repaint
}

pub fn click(state: &mut State, press: Press) -> EventOutcome {
    let ui = &mut state.shield_ui;
    match press {
        Press::Back => return back(state),
        Press::Dismiss if state.failure.is_some() => state.failure = None,
        Press::Dismiss if ui.failure.is_some() => ui.failure = None,
        Press::Dismiss => ui.absent_hidden = true,
        Press::Go(screen) => {
            ui.screen = screen;
            ui.from = SHIELD_HOME;
            ui.focus = FIELD_TO;
            ui.news = None;
            ui.size = None;
            ui.failure = None;
            state.scroll = 0;
        }
        Press::Asset(a) if a != ui.asset => {
            ui.asset = a;
            ui.size = None;
        }
        Press::Pick(i) => ui.size = Some(i),
        Press::Field(f) => ui.focus = f,
        Press::Footer(n) => return super::footer::footer(state, n),
        Press::Row(super::home_receive::ROW_COPY) if ui.screen == SHIELD_HOME => {
            if let Some(addr) = &ui.nox1 {
                let copied =
                    nonos_app_skeleton::clients::clipboard::clipboard_copy(addr.as_bytes());
                state.status = if copied.is_ok() {
                    b"private address copied"
                } else {
                    b"the clipboard did not take it"
                };
            }
        }
        _ => return EventOutcome::Idle,
    }
    EventOutcome::Repaint
}
