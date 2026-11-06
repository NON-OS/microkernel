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

//! What a click does: the action of the control `target_at` names there, so
//! a click acts only on what was drawn under it.

use nonos_toolkit::widgets::Rect;

use super::action::Action;
use crate::app::state::VideoApp;
use crate::ui::layout::{layout, seek_permille};
use crate::ui::screen::{Route, NAV};
use crate::ui::target::{target_at, Target};
use crate::ui::view::list_geom::{columns, grid_capacity, list_at, rows_visible};

const SKIP_SECS: i32 = 10;

pub fn click(app: &VideoApp, x: i32, y: i32) -> Action {
    let Some(target) = target_at(&app.shown(), x, y) else { return Action::None };
    match target {
        Target::Nav(i) => NAV.get(i).map_or(Action::None, |&route| Action::Goto(route)),
        Target::View(grid) if grid != app.browse.grid => Action::ToggleGrid,
        Target::Search | Target::View(_) => Action::None,
        Target::Folder(i) => Action::Folder(i.checked_sub(1)),
        Target::Open => Action::OpenSelected,
        Target::Tile(i) | Target::Row(i) => Action::OpenIndex(i),
        Target::Tab(0) => Action::Goto(Route::Player),
        Target::Tab(_) => Action::None,
        Target::Back => Action::Back,
        Target::Info => Action::Goto(Route::Details),
        Target::Play | Target::Frame => Action::TogglePlay,
        Target::Prev => Action::SeekBy(-SKIP_SECS),
        Target::Next => Action::SeekBy(SKIP_SECS),
        Target::Seek => {
            let l = layout(app.dims.0, app.dims.1);
            Action::SeekToPermille(seek_permille(l.scrub, x))
        }
    }
}

/// How many videos the page shows at once: what a click can name and the
/// painters draw.
pub fn page_step(body: Rect, grid: bool) -> usize {
    if grid {
        return grid_capacity(body).max(1);
    }
    rows_visible(list_at(body)).max(1)
}

/// How many videos share a line: the grid's columns, or one row of the list.
pub fn line_step(body: Rect, grid: bool) -> usize {
    if grid {
        return columns(body.w);
    }
    1
}
