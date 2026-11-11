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

//! The one target table for the whole window: the click, the hover and every
//! painter's hover and press come from `target_at`.

use nonos_toolkit::widgets::State as Look;

use super::browse_hit::browse_at;
use super::event_side::shown_rows;
use super::header_hit::head_hit;
use super::info_hit::info_at;
use super::layout::metrics;
use super::layout_measure::pane;
use super::screen::Screen;
use super::screen_hit::screen_target;
use super::sel_geom::band_at;
use super::sidebar_rows::side_at;
use super::state::{Mode, State};
use super::target::Target;

/// What `(x, y)` names, resolved against the chrome in the order it is
/// stacked: the sidebar owns its whole column, the header its whole band, and
/// only what is left reaches the active surface. A modal dialog is
/// keyboard-driven, so nothing is named while one is up.
pub fn target_at(state: &State, x: i32, y: i32) -> Option<Target> {
    if !matches!(state.mode, Mode::Browse) || x < 0 || y < 0 {
        return None;
    }
    let (x, y, m) = (x as u32, y as u32, metrics());
    if x < m.sidebar_w {
        return side_at(&shown_rows(state), y).map(Target::Side);
    }
    if y < m.header_h {
        return head_hit(state, x, y).map(Target::Head);
    }
    if state.screen != Screen::Browse {
        return screen_target(state, x, y);
    }
    if !state.selected.is_empty() {
        if let Some(t) = band_at(state.win_w, state.win_h, x, y) {
            return t;
        }
    }
    if let Some(t) = info_at(state, x, y) {
        return t;
    }
    browse_at(&pane(state), x, y)
}

/// The hover and press the target `t` draws.
pub fn look(state: &State, t: Target) -> Look {
    state.hover.look(t, |x, y| target_at(state, x, y))
}

/// Names the target under the pointer again; true when that changed.
pub fn rehover(state: &mut State) -> bool {
    let next = state.hover.pointer.position().and_then(|(x, y)| target_at(state, x, y));
    state.hover.settle(next)
}
