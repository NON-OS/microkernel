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

/* Minimal SVG rasterizer: paths with bezier and arc segments, the basic
 * shapes, groups with affine transforms, solid fills (nonzero and evenodd)
 * and approximated strokes, supersampled 2x. Masks, filters and text are
 * skipped, never guessed. */

mod affine;
mod arc;
mod aspect;
mod attr;
mod blend;
mod brush;
mod clip;
mod color;
mod curves;
mod dash;
mod decode;
mod defs;
mod downsample;
mod draw;
mod element;
mod fill;
mod geom;
mod grad_build;
mod grad_geom;
mod grad_lut;
mod grad_paint;
mod grad_stops;
mod gradient;
mod group;
mod ink;
mod math;
mod natural;
mod num;
mod path;
mod path_curves;
mod path_num;
mod path_state;
mod path_tok;
mod pen;
mod pen_parse;
mod raster;
mod shapes;
mod state;
mod stroke;
mod stroke_join;
mod transform;
mod use_ref;
mod viewport;
mod walk;
mod walk_run;
mod xml;

pub(super) use decode::{decode_svg, MAX_SIDE};
pub(super) use natural::{is_svg, natural_size};
