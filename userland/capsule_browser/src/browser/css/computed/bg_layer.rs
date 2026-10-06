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

use crate::browser::layout::boxmodel::Rel;

/// A box's background image layer: its size, its position (px plus
/// per-mille of the room left once the tile is placed, 0% 0% by default)
/// and whether it repeats. A url mask-image lays out the same way.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BgLayer {
    pub size: BgSize,
    pub pos: [Rel; 2],
    pub repeat: bool,
}

impl BgLayer {
    pub const INITIAL: BgLayer = BgLayer { size: BgSize::Auto, pos: [(0, 0); 2], repeat: true };
}

/// background-size: the image's natural size, scaled to cover or to fit
/// the box, or a width and height where auto keeps the image's aspect.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BgSize {
    Auto,
    Cover,
    Contain,
    Wh(BgLen, BgLen),
}

/// One side of a background-size: auto, px, or per-mille of the box side.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BgLen {
    Auto,
    Px(u16),
    Pct(u16),
}
