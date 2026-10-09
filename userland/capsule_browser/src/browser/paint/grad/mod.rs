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

/* CSS gradient painting: linear gradients at any angle and radial gradients
 * (a circle from the box centre to its corners), parsed once, sampled into a
 * color table and filled with integer steps; conic gradients are not drawn,
 * so a box keeps its color rather than guessing. */

mod cache;
mod color;
mod composite;
mod mask_layers;
mod paint;
mod painter;
mod parse;
mod radial;
mod raster;
mod render;
mod shape;
mod split;
mod sqrt;
mod stop_list;
mod stops;
mod trig;

pub(crate) use mask_layers::{mask_layers, MaskLayer};
pub(crate) use paint::paint_gradient;
pub(crate) use render::put_pixel;
pub(crate) use shape::is_gradient;
