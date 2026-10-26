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

//! The window manager's rectangle and its clamp, as the placement code
//! (`cascade`, mounted at the crate root) names them: `crate::geometry`.

#[path = "../../../capsule_wm/src/geometry/constrain.rs"]
mod constrain;
#[path = "../../../capsule_wm/src/geometry/rect.rs"]
mod rect;

pub use constrain::clamp_to_display;
pub use rect::Rect;
