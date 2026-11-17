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

//! What the pointer can be over, and the control at a point: the first in
//! the table that answers there. The click and the hover both ask here, so
//! neither can name what was not drawn there.

use nonos_toolkit::widgets::rect::contains;
use nonos_toolkit::widgets::Rect;

use super::frame::region::body;
use super::layout::seek_reach;
use super::screen::Route;
use super::targets::{targets, Shown};
use super::view::page_geom::{strip, tabs};

/// A control the window shows: a sidebar page, the search field, a half of
/// the grid and list switch (true is the grid), a Folders rail row and its
/// Open button, a video by its index in the list, a Details tab, and the
/// player's header buttons, transport and picture.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Target {
    Nav(usize),
    Search,
    View(bool),
    Folder(usize),
    Open,
    Tile(usize),
    Row(usize),
    Tab(usize),
    Back,
    Info,
    Play,
    Prev,
    Next,
    Seek,
    Frame,
}

/// Where `t`, drawn in `r`, answers the pointer: the seek groove a little
/// past it, as its click always did.
pub fn reach(t: Target, r: Rect) -> Rect {
    match t {
        Target::Seek => seek_reach(r),
        _ => r,
    }
}

/// The control at `(x, y)`. The Details tabs are the shared strip's to
/// place, so its own hit names them.
pub fn target_at(s: &Shown, x: i32, y: i32) -> Option<Target> {
    let page = strip(body(s.dims.0, s.dims.1));
    if let Some(i) = (s.route == Route::Details).then(|| tabs().hit(page, x, y)).flatten() {
        return Some(Target::Tab(i));
    }
    targets(s).into_iter().find(|&(t, r)| contains(reach(t, r), x, y)).map(|(t, _)| t)
}
