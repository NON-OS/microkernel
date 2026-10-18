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
//! What virtio-gpu says of itself on the wire. CAPS: kind 1, its outputs,
//! any size in range (a host takes any), the host's size hints, no cursor
//! plane yet; FLUSH copies, so not DEVICE_READS. STATE: the size the head
//! shows and the host's wanted size; scanning out, dark while the host has
//! the scanout off, LOST once it is back until a mode set shows it.

use nonos_gfx_wire::body::caps::{HINTS, MODE_RANGE, MODE_SET};
use nonos_gfx_wire::body::state::{LOST, SCANNING_OUT};
use nonos_gfx_wire::errno::ENXIO;
use nonos_gfx_wire::{kind, Caps, Errno, State};

use super::gpu::{Gpu, GRANT_BYTES, MAX_SIDE};
use super::plug::Plug;
use crate::state::Health;

pub fn caps(g: &Gpu) -> Caps {
    Caps {
        kind: kind::VIRTIO_GPU,
        outputs: g.outputs,
        flags: MODE_SET | MODE_RANGE | HINTS,
        max_width: MAX_SIDE,
        max_height: MAX_SIDE,
        // A host takes any width: one the window manager sized included.
        width_align: 1,
        surface_budget: GRANT_BYTES,
        physical_mm: g.panel.physical_mm,
    }
}

pub fn state(g: &Gpu, output: u16) -> Result<State, Errno> {
    let h = g.head(output).ok_or(ENXIO)?;
    Ok(State {
        generation: g.generation,
        width: h.shown.width,
        height: h.shown.height,
        hint_width: h.hint.0,
        hint_height: h.hint.1,
        flags: match (g.health, h.plug) {
            (Health::Down, _) => LOST,
            (_, Plug::In) => SCANNING_OUT,
            (_, Plug::Out) => 0,
            (_, Plug::Back) => LOST,
        },
        ..State::default()
    })
}

/// A Down device takes nothing: its queue is reset for good, so no output
/// can show a frame again (`recover`).
pub fn live(g: &Gpu) -> Result<(), Errno> {
    match g.health {
        Health::Live => Ok(()),
        Health::Down => Err(ENXIO),
    }
}
