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

//! What the window shows that decides where its controls sit, and the control
//! at a point: the first in the table that answers there. A click and the
//! hover both ask here, so neither can name what was not drawn there.

extern crate alloc;

use alloc::vec::Vec;

use super::geometry::{page, shell};
use super::screen::lib_geom::tab_hit;
use super::target::{reach, Target};
use super::targets::targets;
use super::view::View;

/// What the window shows that decides where its controls sit.
pub struct Shown {
    pub view: View,
    pub dims: (u32, u32),
    pub scroll: usize,
    /// Whether the search field holds text, and whether that is an address.
    pub query: bool,
    pub address: bool,
    /// Rows the page lists, tracks in the library and in the queue.
    pub rows: usize,
    pub tracks: usize,
    pub queue: usize,
    /// Each download's button labels, and whether any download has finished.
    pub acts: Vec<Vec<&'static str>>,
    pub finished: bool,
}

/// The control at `(x, y)`. The Library's tabs are the shared strip's to
/// place, so its own hit names them.
pub fn target_at(s: &Shown, x: i32, y: i32) -> Option<Target> {
    let p = page(&shell(s.dims.0, s.dims.1));
    if let Some(i) = (s.view == View::Library).then(|| tab_hit(p, x, y)).flatten() {
        return Some(Target::Tab(i));
    }
    targets(s).into_iter().find(|&(t, r)| reach(t, r).contains(x, y)).map(|(t, _)| t)
}
