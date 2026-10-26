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

/// The content area app.rs paints; the window adds the frame.
pub const WIDTH: u32 = 360;
pub const HEIGHT: u32 = 200;
const WINDOW: (u32, u32) = window_for(WIDTH, HEIGHT);

pub fn manifest() -> AppManifest {
    AppManifest {
        title: b"std GUI proof",
        window_id: 0x5354_4447,
        kind: WindowKind::Normal,
        initial_x: 160,
        initial_y: 140,
        width: WINDOW.0,
        height: WINDOW.1,
        input_kind_mask: (1 << 0) | (1 << 5),
    }
}
