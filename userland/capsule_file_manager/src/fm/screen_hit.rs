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
use alloc::vec::Vec;

use nonos_libc::mk_time_millis;

use super::home_geom::home_lines;
use super::home_hit::{home_open, home_target};
use super::recents_chip_paint::chip_at as kind_at;
use super::recents_chips::KindChip;
use super::screen::Screen;
use super::screen_list::Line;
use super::screen_recents::{recents_chip_row, recents_lines};
use super::screen_search::{search_chip_row, search_lines};
use super::state::State;
use super::tags_geom::{chip_at, chip_slots};
use super::tags_rows::tag_lines;
use super::target::Target;

/// What a click on a stacked screen resolved to: a path to open, a tag to filter
/// by, or another surface to switch to. Browse has its own row and cell
/// hit-tests and is not routed here.
pub enum ScreenHit {
    Open(String),
    Tag(String),
    Go(Screen),
    // A filetype chip on Recents or Search; the payload is the filter it
    // selects, `None` being the All chip.
    Filter(Option<super::filetype::Kind>),
}

/// Test `(x, y)` against the very line list the active screen's painter drew
/// from. Heading lines carry no path, so a section title names nothing rather
/// than the row beneath it.
pub fn screen_target(state: &State, x: u32, y: u32) -> Option<Target> {
    if state.screen == Screen::Home {
        if let Some(t) = home_target(state, x, y) {
            return Some(t);
        }
    }
    if state.screen == Screen::Tags {
        if let Some(i) = chip_at(state, x, y) {
            return Some(Target::Chip(i));
        }
    }
    if let Some(i) = kind_chip_at(state, x, y) {
        return Some(Target::Chip(i));
    }
    line_at(&lines_of(state), y).map(Target::Line)
}

/// What a click on `t` resolves to on the screen on show: a path to open, a
/// tag to filter by, a filter chip or another surface.
pub fn screen_hit(state: &State, t: Target) -> Option<ScreenHit> {
    match t {
        Target::Cat(_) | Target::Stor(_) => home_open(t),
        Target::Chip(i) if state.screen == Screen::Tags => {
            chip_slots(state).into_iter().nth(i).map(|s| ScreenHit::Tag(s.name))
        }
        Target::Chip(i) => kind_row(state).get(i).map(|c| ScreenHit::Filter(c.kind)),
        Target::Line(i) => lines_of(state).into_iter().nth(i).map(|l| ScreenHit::Open(l.path)),
        _ => None,
    }
}

// The filetype chip row, tested against the very slots its surface painted. A
// lone All chip means the surface has no data and drew no row at all, so it is
// not offered as a hit target either.
fn kind_chip_at(state: &State, x: u32, y: u32) -> Option<usize> {
    let chips = kind_row(state);
    if chips.len() < 2 {
        return None;
    }
    kind_at(&chips, x, y)
}

/// The filetype chip row of Recents or Search, empty on any other screen.
fn kind_row(state: &State) -> Vec<KindChip> {
    match state.screen {
        Screen::Recents => recents_chip_row(state),
        Screen::Search => search_chip_row(state),
        _ => Vec::new(),
    }
}

// The active screen's one layout pass. Browse lays out no lines.
fn lines_of(state: &State) -> Vec<Line> {
    let now = mk_time_millis().max(0) as u64;
    match state.screen {
        Screen::Home => home_lines(state, now),
        Screen::Recents => recents_lines(state, now),
        Screen::Search => search_lines(state),
        Screen::Tags => tag_lines(state),
        Screen::Browse => Vec::new(),
    }
}

fn line_at(lines: &[Line], y: u32) -> Option<usize> {
    lines.iter().position(|l| l.head.is_none() && y >= l.y && y < l.y + l.h)
}
