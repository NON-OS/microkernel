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

//! What bga says of itself on the wire: CAPS (kind 2, DISPI, one output,
//! any mode in range, no hardware cursor) and its STATE: the mode it set or
//! firmware left, with the flags `health` found.

use super::bga::{Bga, MAX_SIDE, WIDTH_ALIGN};
use nonos_gfx_wire::body::caps::{MODE_RANGE, MODE_SET};
use nonos_gfx_wire::{kind, Caps, State};

pub fn caps(b: &Bga) -> Caps {
    Caps {
        kind: kind::DISPI,
        outputs: 1,
        flags: MODE_SET | MODE_RANGE,
        max_width: MAX_SIDE,
        max_height: MAX_SIDE,
        width_align: WIDTH_ALIGN,
        // The largest frame is what video memory holds; no cursor slot.
        surface_budget: b.vram_len as u64,
        physical_mm: b.panel.physical_mm,
    }
}

pub fn state(b: &Bga, flags: u32) -> State {
    State {
        generation: b.generation,
        width: b.mode.width,
        height: b.mode.height,
        flags,
        ..State::default()
    }
}
