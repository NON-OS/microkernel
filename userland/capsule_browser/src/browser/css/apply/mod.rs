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

mod align;
mod align_kw;
mod aspect;
mod border;
mod clip;
mod clip_polygon;
mod decl;
mod direction;
mod display;
mod filter_hue;
mod filter_mats;
mod filter_parse;
mod flex;
mod flex_shorthand;
mod float;
mod font_family;
mod font_size;
mod grid;
mod grid_halves;
pub(super) mod grid_names;
mod grid_template;
mod list;
mod margin;
mod object;
mod origin;
mod overflow;
mod padding;
mod paint;
mod position;
mod radius;
mod shadow;
mod sizing;
mod text;
mod text_flow;
mod transform;
mod transform_fn;
mod trig;
mod visual;
mod white_space;
mod z_index;

pub use decl::apply_decl;
