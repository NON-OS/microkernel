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

//! What the boot framebuffer says of itself on the wire: CAPS with kind 6
//! and no flags (no mode setting, no cursor), its one mode, and a state
//! that is always that mode, scanning out.

use super::fb::Fb;
use nonos_gfx_wire::body::mode::{CURRENT, FROM_DEVICE, PREFERRED};
use nonos_gfx_wire::body::state::SCANNING_OUT;
use nonos_gfx_wire::{kind, Caps, Mode, State};

pub fn caps(f: &Fb) -> Caps {
    Caps {
        kind: kind::BOOT_FB,
        outputs: 1,
        flags: 0,
        max_width: f.width,
        max_height: f.height,
        width_align: 1,
        surface_budget: f.width as u64 * f.height as u64 * 4,
        physical_mm: f.physical_mm,
    }
}

pub fn mode(f: &Fb) -> Mode {
    Mode { width: f.width, height: f.height, flags: PREFERRED | CURRENT | FROM_DEVICE }
}

pub fn state(f: &Fb) -> State {
    State {
        generation: 1,
        width: f.width,
        height: f.height,
        flags: SCANNING_OUT,
        ..State::default()
    }
}
