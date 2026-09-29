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

/// The window a terminal opens at.
///
/// Sized to hold eighty columns, which is not a preference: it is the width
/// every command-line tool has assumed since terminals were hardware, and the
/// width this shell's own `help` is written to. At the previous 520 by 300 the
/// text area was about fifty-eight columns and `help` was clipped at the right
/// edge, silently, with no wrap and no scroll to reach the rest. At 760 by
/// 460 `help` alone filled the window, so its output scrolled the command
/// that asked for it out of sight; this holds a hundred columns and room to
/// read what a command printed under it.
pub const WIDTH: u32 = 960;
pub const HEIGHT: u32 = 540;

const INPUT_KEY_DOWN_BIT: u32 = 1 << 0;

pub fn manifest() -> AppManifest {
    AppManifest {
        title: b"Terminal",
        window_id: 0x5445_524D,
        kind: WindowKind::Normal,
        // Centred on a 1280 by 720 screen, clear of the top bar and the dock.
        initial_x: 160,
        initial_y: 90,
        width: WIDTH,
        height: HEIGHT,
        input_kind_mask: INPUT_KEY_DOWN_BIT,
    }
}
