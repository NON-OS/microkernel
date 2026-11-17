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

use super::settings_cells::px;

/// Fixed geometry the window layout is built from, in logical pixels.
pub const TOOLBAR_H: i32 = 56;
pub const SIDEBAR_FULL: i32 = 232;
pub const SIDEBAR_RAIL: i32 = 64;
/// Below this width `SidebarMode::Auto` resolves to the rail.
pub const AUTO_FULL_MIN_W: i32 = 1180;
/// A sidebar that would leave the content narrower than this is folded away.
pub const MIN_CONTENT_W: i32 = 480;

/// The toolbar's height at the display scale.
pub fn toolbar_h() -> i32 {
    px(TOOLBAR_H as u32)
}

/// The full sidebar's width at the display scale.
pub fn sidebar_full() -> i32 {
    px(SIDEBAR_FULL as u32)
}

/// The rail's width at the display scale.
pub fn sidebar_rail() -> i32 {
    px(SIDEBAR_RAIL as u32)
}
