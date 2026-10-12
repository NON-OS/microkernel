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

//! A display with several heads, as Intel's pipes are: each output has
//! its own mode and its own scanout memory, set and flushed alone. A head
//! can be marked lost, as an underrun or a dropped link leaves it, and
//! reports so on its next STATE.

use nonos_gfx_wire::body::caps::{MODE_RANGE, MODE_SET};
use nonos_gfx_wire::Caps;

#[derive(Clone, Default)]
pub struct Head {
    pub size: (u32, u32),
    pub fb: Vec<u32>,
    pub lost: bool,
    /// Mode sets this head has had, re-programs after a loss included.
    pub sets: u32,
}

pub struct Heads {
    pub caps: Caps,
    pub heads: Vec<Head>,
    pub fail_set: bool,
}

impl Heads {
    pub fn new(sizes: &[(u32, u32)]) -> Self {
        let caps = Caps {
            kind: 7,
            outputs: sizes.len() as u16,
            flags: MODE_SET | MODE_RANGE,
            max_width: 4096,
            max_height: 2160,
            width_align: 8,
            surface_budget: 4096 * 2160 * 4,
            physical_mm: 0,
        };
        let head = |&size| Head { size, fb: vec![0; 4096 * 2160], ..Head::default() };
        Heads { caps, heads: sizes.iter().map(head).collect(), fail_set: false }
    }
}
