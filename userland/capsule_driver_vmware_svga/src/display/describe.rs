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

//! What the adapter says of itself on the wire: CAPS (kind SVGA II, one
//! output, any mode in its range, no hardware cursor) and its STATE.

use super::svga::Svga;
use nonos_gfx_wire::body::caps::{MODE_RANGE, MODE_SET};
use nonos_gfx_wire::{kind, Caps, State};

pub fn caps(s: &Svga) -> Caps {
    Caps {
        kind: kind::SVGA_II,
        outputs: 1,
        flags: MODE_SET | MODE_RANGE,
        max_width: s.max.0,
        max_height: s.max.1,
        // The adapter takes any width; the pitch it reports is honoured.
        width_align: 1,
        surface_budget: s.frame_room(),
        // SVGA II reports no panel size.
        physical_mm: 0,
    }
}

pub fn state(s: &Svga, flags: u32) -> State {
    State {
        generation: s.generation,
        width: s.mode.width,
        height: s.mode.height,
        flags,
        ..State::default()
    }
}
