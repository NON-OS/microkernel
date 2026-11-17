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

extern crate alloc;

use alloc::string::String;

use nonos_app_skeleton::{EventOutcome, KEY_ENTER};

use super::event_browse::on_browse_key;
use super::event_head::on_head;
use super::event_side::on_side;
use super::info_act::run_info;
use super::list_click::sort_by;
use super::navigate::navigate;
use super::screen::Screen;
use super::screen_hit::{screen_hit, ScreenHit};
use super::sel_hit::run;
use super::selection::toggle;
use super::state::State;
use super::target::Target;
use super::target_at::target_at;

/// Resolve a click against the target table the painters and the hover read.
/// A click on the band's or the info panel's own ground names nothing, as does
/// any click while a modal dialog is up, so it is `Idle` rather than reaching
/// the row underneath.
pub fn on_click(state: &mut State, x: u32, y: u32) -> EventOutcome {
    let Some(t) = target_at(state, x as i32, y as i32) else { return EventOutcome::Idle };
    match t {
        Target::Side(i) => on_side(state, i),
        Target::Head(hit) => on_head(state, hit),
        Target::Sort(col) => sort_by(state, col),
        Target::Check(index) => {
            state.cursor = index;
            toggle(state);
            EventOutcome::Repaint
        }
        Target::Row(index) | Target::Cell(index) => {
            state.cursor = index;
            on_browse_key(state, KEY_ENTER)
        }
        Target::Band(i) => run(state, i),
        Target::Quick(_) | Target::Untag(_) | Target::AddTag | Target::Perms => run_info(state, t),
        Target::Cat(_) | Target::Stor(_) | Target::Chip(_) | Target::Line(_) => on_screen(state, t),
    }
}

// A stacked surface resolves to a path to open or a tag to filter by. Clicking
// the active tag again clears the filter, which is the only way back to the
// bare chip row.
fn on_screen(state: &mut State, t: Target) -> EventOutcome {
    match screen_hit(state, t) {
        Some(ScreenHit::Open(path)) => {
            navigate(state, path.as_str());
            EventOutcome::Repaint
        }
        Some(ScreenHit::Go(screen)) => {
            state.screen = screen;
            EventOutcome::Repaint
        }
        Some(ScreenHit::Filter(kind)) => {
            match state.screen {
                Screen::Recents => state.recents_filter = kind,
                _ => state.hit_filter = kind,
            }
            EventOutcome::Repaint
        }
        Some(ScreenHit::Tag(name)) => {
            state.tag_filter =
                if state.tag_filter == name { String::new() } else { name };
            EventOutcome::Repaint
        }
        None => EventOutcome::Idle,
    }
}
