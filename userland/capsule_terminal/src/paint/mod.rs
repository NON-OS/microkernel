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

mod block_chrome;
mod block_meta;
mod compose;
mod constants;
mod draw_cursor;
mod draw_input_line;
mod fetch;
mod fetch_arch;
pub(crate) mod fetch_banner;
pub(crate) mod fetch_boot;
mod fetch_palette;
pub mod fetch_uptime;
mod fetch_version;
pub mod find_view;
pub mod footer;
mod geometry;
pub mod header;
mod line_chars;
mod line_text;
mod line_window;
pub(crate) mod metrics;
mod palette;
pub mod palette_ptr;
pub mod palette_view;
mod prompt;
mod rail_addr;
pub mod rail_column;
mod rail_disk;
mod rail_fmt;
mod rail_geom;
pub mod rail_left;
mod rail_lists;
mod rail_metric;
pub mod rail_nav;
mod rail_net;
pub mod rail_ptr;
pub mod rail_scroll;
pub mod rail_spot;
mod rail_sys;
mod rail_telemetry;
mod rail_text;
mod shade;
mod spark;
mod spark_smooth;
pub mod strip;
pub mod strip_ptr;
pub mod strip_spot;
mod suggestion;
mod syntax;
pub mod tab_bar;
pub mod tab_label;
mod user_host;
mod vt;

pub use compose::paint_tabs;
pub use constants::TEXT_LEFT;
pub use geometry::grid_size;
pub use tab_bar::draw_tab_bar;
pub use vt::{Rows, Shade};
