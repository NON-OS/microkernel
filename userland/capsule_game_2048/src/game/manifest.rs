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

use nonos_app_skeleton::{window_for, AppManifest, WindowKind};

/// The content area the game is laid out for (paint.rs); the window asked
/// for is that plus the frame around it.
pub const WIDTH: u32 = 360;
pub const HEIGHT: u32 = 480;
const WINDOW: (u32, u32) = window_for(WIDTH, HEIGHT);

const WINDOW_ID: u32 = 0x3230_3438;
const INPUT_KEY_DOWN_BIT: u32 = 1 << 0;

pub fn manifest() -> AppManifest {
    AppManifest {
        title: b"2048",
        window_id: WINDOW_ID,
        kind: WindowKind::Normal,
        initial_x: 380,
        initial_y: 120,
        width: WINDOW.0,
        height: WINDOW.1,
        input_kind_mask: INPUT_KEY_DOWN_BIT,
    }
}
