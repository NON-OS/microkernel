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

//! The wheel: a section's page scrolled a notch at a time, or the search's
//! results while a search is up, never past the end of either.

use crate::settings::state::{searching, view_h, State};
use crate::settings::ui::bytes::as_str;
use crate::settings::ui::results;
use crate::settings::ui::results_geom;
use crate::settings::ui::scroll::max_scroll;
use nonos_app_skeleton::EventOutcome;

// Pixels moved per wheel notch.
const WHEEL_STEP: u32 = 48;

pub fn wheel(state: &mut State, delta_y: i32) -> EventOutcome {
    let steps = delta_y.unsigned_abs().min(10) * WHEEL_STEP;
    if steps == 0 {
        return EventOutcome::Idle;
    }
    if searching(state) {
        return wheel_results(state, delta_y, steps);
    }
    let i = state.section.index();
    let limit = max_scroll(state, view_h(state));
    state.scroll_px[i] = if delta_y > 0 {
        state.scroll_px[i].saturating_sub(steps)
    } else {
        (state.scroll_px[i] + steps).min(limit)
    };
    EventOutcome::Repaint
}

fn wheel_results(state: &mut State, delta_y: i32, steps: u32) -> EventOutcome {
    let n = results::count(as_str(state.search.text.as_slice()));
    let limit = results_geom::max_scroll(n, view_h(state));
    state.search.scroll = if delta_y > 0 {
        state.search.scroll.saturating_sub(steps)
    } else {
        (state.search.scroll + steps).min(limit)
    };
    EventOutcome::Repaint
}
