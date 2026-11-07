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

//! A layer's content drawn over its own surface: the box of it the layer
//! names, from its corner on the layer, cut to the layer and to the damaged
//! rectangle, blended, or put down opaque for an XRGB buffer.

use crate::state::damage::Rect;
use crate::state::scene::Layer;
use crate::sw_blitter::{self, Surface};

pub fn draw(dst: Surface, layer: &Layer, src: Surface, rect: Rect) {
    let c = layer.content;
    let Some(src) = src.view(c.view) else { return };
    let (w, h) = (layer.width.saturating_sub(c.x), layer.height.saturating_sub(c.y));
    let (x, y) = (layer.x.saturating_add(c.x), layer.y.saturating_add(c.y));
    let row = if c.opaque { sw_blitter::row::opaque_row } else { sw_blitter::row::composite_row };
    sw_blitter::composite_with(dst, src, (x, y, w, h), rect, row);
}
