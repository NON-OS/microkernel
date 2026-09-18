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

//! The window: a dialog-sized panel in the middle of the screen. Keys only;
//! an installer that could be driven by a stray click is not one to ship.

use nonos_app_skeleton::{AppManifest, WindowKind};

use super::ui::metrics::{WIN_H, WIN_W, WIN_X, WIN_Y};

const WINDOW_ID: u32 = 0x494E_5354;
const INPUT_KEY_DOWN_BIT: u32 = 1 << 0;

pub fn manifest() -> AppManifest {
    AppManifest {
        title: "Install NØNOS".as_bytes(),
        window_id: WINDOW_ID,
        kind: WindowKind::Normal,
        initial_x: WIN_X,
        initial_y: WIN_Y,
        width: WIN_W,
        height: WIN_H,
        input_kind_mask: INPUT_KEY_DOWN_BIT,
    }
}
