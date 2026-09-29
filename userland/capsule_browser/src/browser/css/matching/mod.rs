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

mod bloom;
mod class;
mod complex;
mod cx;
mod cx_nav;
mod dir;
mod form;
mod form_candidate;
mod form_check;
mod form_disabled;
mod form_group;
mod form_kind;
mod form_missing;
mod form_range;
mod form_required;
mod form_select;
mod form_shape;
mod form_text;
mod form_valid;
mod form_value;
mod has;
mod lang;
mod matches;
mod misc;
mod nth_of;
mod positions;
mod pseudo;
mod select;
mod select_id;
mod selector;
mod sibling;
mod simple;
mod state;
mod steps;
mod structural;
mod table;
mod tokens;
mod tree_scan;
mod tree_walk;
mod user;

pub use matches::{closest, matches};
pub use select::{select, select_in};
pub use selector::matches_selector;
pub use sibling::Siblings;
#[cfg(feature = "harness")]
pub use state::{element_state, set_active, set_focused, set_hovered, set_target, ElementState};
pub use steps::spent;
