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

//! One source of truth for desktop type sizes and where a line of text sits inside
//! a box. `ttf` clamps every draw and measure to its floor, so every size here
//! is at or above it rather than a smaller size that would be silently raised.

mod lines;
mod scale;
mod sizes;

pub use lines::{line_h, top_y_centered, valid_str};
pub use scale::{px, scale, scaled, set_scale};
pub use sizes::{BRAND_PX, LABEL_PX, MENU_PX, META_PX, STATUS_PX, TITLE_PX, UI_PX};
