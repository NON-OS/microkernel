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

//! A VRAM display: a framebuffer in its own memory, a list of modes or a
//! range, an optional hardware cursor. set_mode shows the new shadow at
//! once; flush copies the rectangles in with the shipping copy.

use nonos_gfx_wire::body::caps::{HW_CURSOR, MODE_RANGE, MODE_SET};
use nonos_gfx_wire::{Caps, Cursor, Mode};

pub struct Vram {
    pub caps: Caps,
    pub modes: Vec<Mode>,
    pub size: (u32, u32),
    pub fb: Vec<u32>,
    pub fail_set: bool,
    pub cursor: Option<(Cursor, Vec<u8>)>,
}

impl Vram {
    pub fn new(range: bool, cursor: bool) -> Self {
        let flags =
            MODE_SET | if range { MODE_RANGE } else { 0 } | if cursor { HW_CURSOR } else { 0 };
        let caps = Caps {
            kind: 2,
            outputs: 1,
            flags,
            max_width: 2560,
            max_height: 1600,
            width_align: 8,
            surface_budget: 2560 * 1600 * 4 + 16384,
            physical_mm: 0,
        };
        let modes = vec![
            Mode { width: 1024, height: 768, flags: 3 },
            Mode { width: 1280, height: 720, flags: 0 },
        ];
        Vram {
            caps,
            modes,
            size: (1024, 768),
            fb: vec![0; 2560 * 1600],
            fail_set: false,
            cursor: None,
        }
    }
}
