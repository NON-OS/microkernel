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

//! The window an app laid out at a fixed size asks for. A manifest names the
//! whole window at one to one, shadow and frame too, and the app is painted
//! into the content area inside them (frame_rect.rs content_rect), so the
//! window is that content plus the frame around it.

use super::metrics::{BORDER_PX, SHADOW_MARGIN, TITLEBAR_H};

/// The window, at one to one, whose content area is `w` by `h`.
pub const fn window_for(w: u32, h: u32) -> (u32, u32) {
    (w + 2 * (SHADOW_MARGIN + BORDER_PX), h + 2 * SHADOW_MARGIN + TITLEBAR_H + BORDER_PX)
}
