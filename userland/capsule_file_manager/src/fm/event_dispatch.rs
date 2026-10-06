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

use nonos_app_skeleton::{EventOutcome, InputEvent, InputKind};

use super::event_browse::on_browse_key;
use super::event_click::on_click;
use super::event_mode::route;
use super::event_query;
use super::preview_paint::VISIBLE_LINES;
use super::row_geom::visible_rows;
use super::screen::Screen;
use super::state::{Mode, State, ViewKind};
use super::wheel;

pub fn on_event(state: &mut State, event: InputEvent) -> EventOutcome {
    if event.kind == InputKind::Wheel {
        return wheel(state, event);
    }
    if event.kind == InputKind::ButtonDown {
        if event.x < 0 || event.y < 0 {
            return EventOutcome::Idle;
        }
        return on_click(state, event.x as u32, event.y as u32);
    }
    if !event.is_key_down() {
        return EventOutcome::Idle;
    }
    if state.screen == Screen::Search && matches!(state.mode, Mode::Browse) {
        return event_query::on_key(state, event);
    }
    if let Some(outcome) = route(state, event) {
        return outcome;
    }
    on_browse_key(state, event.code)
}

// The wheel scrolls what is on screen: an opened file, or the Browse listing.
// It used to move the listing whatever was shown, by three entries even in the
// icon grid, where that is less than a line and the view moved a line only
// every few notches, and an opened file could be read only with the arrows.
// The stacked screens (Home, Recents, Tags, Search) lay out what fits and do
// not scroll, so the wheel leaves the hidden listing alone there.
fn wheel(state: &mut State, event: InputEvent) -> EventOutcome {
    if event.delta_y == 0 {
        return EventOutcome::Idle;
    }
    let moved = if matches!(state.mode, Mode::Preview) {
        match state.preview.as_mut() {
            Some(p) => {
                let next = wheel::preview(p.scroll, p.lines.len(), VISIBLE_LINES, event.delta_y);
                core::mem::replace(&mut p.scroll, next) != next
            }
            None => false,
        }
    } else if state.screen == Screen::Browse {
        let cols = match state.view {
            ViewKind::Grid => state.grid_cols as usize,
            ViewKind::List => 1,
        };
        let visible = visible_rows(state);
        let len = state.entries.len();
        let next = wheel::listing(state.scroll, len, visible, cols, event.delta_y);
        core::mem::replace(&mut state.scroll, next) != next
    } else {
        false
    };
    if moved {
        EventOutcome::Repaint
    } else {
        EventOutcome::Idle
    }
}
