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

/// "PROV".
const WINDOW_ID: u32 = 0x5052_4F56;
/// Keys only: Enter agrees, Esc closes.
const INPUT_KEY_DOWN_BIT: u32 = 1 << 0;

pub const WIDTH: u32 = 760;
pub const HEIGHT: u32 = 380;
/// The window: the content above, laid out at that size, plus the frame.
const WINDOW: (u32, u32) = window_for(WIDTH, HEIGHT);

pub fn manifest() -> AppManifest {
    AppManifest {
        title: b"Prove this device",
        window_id: WINDOW_ID,
        kind: WindowKind::Normal,
        initial_x: 300,
        initial_y: 180,
        width: WINDOW.0,
        height: WINDOW.1,
        input_kind_mask: INPUT_KEY_DOWN_BIT,
    }
}
