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

mod affine;
mod attr_px;
mod auto_repeat_n;
mod box_kind;
mod build;
mod collect;
mod collect_items;
mod content_width;
mod display_list;
mod element;
mod element_box;
mod element_field;
mod element_img;
mod element_svg;
mod field_label;
mod flex_col;
mod flex_row;
mod float_ctx;
mod flow;
mod flush_line;
mod flush_run;
mod fx;
mod geom;
mod grid_children;
mod grid_place;
mod image_box;
mod img_src;
mod inline_items;
mod layout;
mod layout_block;
mod layout_box;
mod layout_flex;
mod layout_grid;
mod layout_inline;
mod layout_table;
mod leaf;
mod list_marker;
mod min_content_width;
mod post;
mod pseudo_box;
mod replaced_size;
mod srcset;
mod svg_serialize;
pub mod table_columns;
mod text_transform;
mod track_widths;
mod tree;
mod walk;
mod wrap_items;
mod wrap_mixed;
mod wrap_runs;

pub use affine::Affine;
pub use build::build;
pub use display_list::{BoxDocument, Content, Fragment};
pub use fx::{rel, Clip, ClipR, Fx, Rel};
pub(crate) use geom::{abs_out_of_flow, border_box_w, ctx, edges_x, edges_y, shift_down};
pub use layout::layout;
