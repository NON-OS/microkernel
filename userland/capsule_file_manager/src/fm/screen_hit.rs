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
use super::screen::Screen;
use super::screen_list::Line;
use super::screen_recents::recents_lines;
use super::screen_search::search_lines;
use super::state::State;
use super::tags_geom::chip_at;
use super::tags_rows::tag_lines;

/// What a click on a stacked screen resolved to: a path to open, or a tag to
/// filter by. Browse has its own row and cell hit-tests and is not routed here.
pub enum ScreenHit {
    Open(String),
    Tag(String),
}

/// Test `(x, y)` against the very line list the active screen's painter drew
/// from. Heading lines carry no path, so a click on a section title is inert
/// rather than opening the row beneath it.
pub fn screen_hit(state: &State, x: u32, y: u32) -> Option<ScreenHit> {
    if state.screen == Screen::Tags {
        if let Some(name) = chip_at(state, x, y) {
            return Some(ScreenHit::Tag(name));
        }
    }
    line_at(&lines_of(state), y).map(ScreenHit::Open)
}

// The active screen's one layout pass. Browse and Shared lay out no lines.
fn lines_of(state: &State) -> Vec<Line> {
    let now = mk_time_millis().max(0) as u64;
    match state.screen {
        Screen::Home => home_lines(state, now),
        Screen::Recents => recents_lines(state, now),
        Screen::Search => search_lines(state),
        Screen::Tags => tag_lines(state),
        Screen::Browse | Screen::Shared => Vec::new(),
    }
}

fn line_at(lines: &[Line], y: u32) -> Option<String> {
    lines
        .iter()
        .find(|l| l.head.is_none() && y >= l.y && y < l.y + l.h)
        .map(|l| l.path.clone())
}
