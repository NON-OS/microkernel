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

//! What the pointer can be over, and where: the painters draw the controls
//! where `controls` puts them, and the click and the hover both ask
//! `target_at`, so a click lands on what was drawn.

use nonos_toolkit::widgets::rect::contains;
use nonos_toolkit::widgets::Rect;

use crate::viewer::caption::nav_shown;
use crate::viewer::gallery::input::hit_tile;
use crate::viewer::nav::button_rects;

/// The previous and next buttons, or a gallery tile by its index.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    Prev,
    Next,
    Tile(usize),
}

impl Target {
    /// How far a click on this steps through the images, for prev and next.
    pub fn step(self) -> Option<i32> {
        match self {
            Target::Prev => Some(-1),
            Target::Next => Some(1),
            Target::Tile(_) => None,
        }
    }
}

/// What the window shows, as far as the pointer is concerned: which view,
/// how many images the single view steps through, the window's size, and
/// the gallery's scroll and tile count.
#[derive(Clone, Copy, Debug)]
pub struct Shown {
    pub gallery: bool,
    pub images: usize,
    pub w: u32,
    pub h: u32,
    pub scroll: usize,
    pub tiles: usize,
}

/// The controls on show and where each is drawn. The single view has prev
/// and next while there is another image to step to; the gallery has none.
pub fn controls(s: &Shown) -> impl Iterator<Item = (Target, Rect)> {
    let (l, r) = button_rects(s.w, s.h);
    let on = !s.gallery && nav_shown(s.images);
    [(Target::Prev, l), (Target::Next, r)].into_iter().filter(move |_| on)
}

/// What is at `(x, y)`: a tile in the gallery, a control in the single view.
pub fn target_at(s: &Shown, x: i32, y: i32) -> Option<Target> {
    if s.gallery {
        return hit_tile(x, y, s.scroll, s.w, s.h, s.tiles).map(Target::Tile);
    }
    controls(s).find(|&(_, r)| contains(r, x, y)).map(|(t, _)| t)
}
