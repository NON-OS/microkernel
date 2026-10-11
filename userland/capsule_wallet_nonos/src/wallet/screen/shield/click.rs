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

use super::landed_move::{MOVE_MAX, ROW_MOVE};
use crate::wallet::screen::hits::Press;
use crate::wallet::shield::scan_job;
use crate::wallet::state::shield_ui::{FIELD_TO, SHIELD_HOME};
use crate::wallet::state::State;

pub use super::nav::{back, open};

pub fn click(state: &mut State, press: Press) -> EventOutcome {
    let moves = ROW_MOVE..ROW_MOVE + MOVE_MAX;
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
            ui.to_meter = false;
            ui.to_deposit = None;
            ui.failure = None;
            state.scroll = 0;
        }
        Press::Asset(a) if a != ui.asset => {
            ui.asset = a;
            ui.size = None;
        }
        Press::Pick(i) => ui.size = Some(i),
        Press::Dest(d) => (ui.to_meter, ui.to_deposit) = (d == 1, None),
        Press::Field(f) => ui.focus = f,
        Press::Footer(n) => return super::footer::footer(state, n),
        Press::Row(super::landed::ROW_SCAN) if ui.screen == SHIELD_HOME => scan_job::start(state),
        Press::Row(super::landed::ROW_FURTHER) if ui.screen == SHIELD_HOME => {
            scan_job::further(state)
        }
        Press::Row(r) if ui.screen == SHIELD_HOME && moves.contains(&r) => {
            return super::landed_rows::click(state, r)
        }
        Press::Row(super::home_receive::ROW_COPY) if ui.screen == SHIELD_HOME => {
            super::home_receive::copy(state)
        }
        _ => return EventOutcome::Idle,
    }
    EventOutcome::Repaint
}
