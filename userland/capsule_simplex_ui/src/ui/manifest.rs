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

pub const WIDTH: u32 = 980;
pub const HEIGHT: u32 = 660;
const WINDOW_ID: u32 = 0x5358_4D55;
const TITLE: &[u8] = b"SimpleX";
const INPUT_KEY_DOWN_BIT: u32 = 1 << 0;
const INPUT_BUTTON_DOWN_BIT: u32 = 1 << 5;

pub fn manifest() -> AppManifest {
    AppManifest {
        title: TITLE,
        window_id: WINDOW_ID,
        kind: WindowKind::Normal,
        initial_x: 140,
        initial_y: 80,
        width: WIDTH,
        height: HEIGHT,
        input_kind_mask: INPUT_KEY_DOWN_BIT | INPUT_BUTTON_DOWN_BIT,
    }
}
