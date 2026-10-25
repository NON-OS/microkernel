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

use nonos_app_skeleton::{AppManifest, WindowKind};
use nonos_toolkit::decorations::{BORDER_PX, SHADOW_MARGIN, TITLEBAR_H};

/// The content area the game is laid out for (paint.rs).
pub const WIDTH: u32 = 360;
pub const HEIGHT: u32 = 480;

/* A manifest names the whole window at one to one, frame and shadow too, so
 * the window asked for is the content plus the frame around it. */
const WINDOW_W: u32 = WIDTH + 2 * (SHADOW_MARGIN + BORDER_PX);
const WINDOW_H: u32 = HEIGHT + 2 * SHADOW_MARGIN + TITLEBAR_H + BORDER_PX;

const WINDOW_ID: u32 = 0x3230_3438;
const INPUT_KEY_DOWN_BIT: u32 = 1 << 0;

pub fn manifest() -> AppManifest {
    AppManifest {
        title: b"2048",
        window_id: WINDOW_ID,
        kind: WindowKind::Normal,
        initial_x: 380,
        initial_y: 120,
        width: WINDOW_W,
        height: WINDOW_H,
        input_kind_mask: INPUT_KEY_DOWN_BIT,
    }
}
