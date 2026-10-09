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

//! The assignment walk: its state, and where it starts.

use super::port::{Assigned, ConfigPort};
use super::window::Window;

/// Number the buses behind the VMD depth first, size every memory BAR and
/// place it in `window`, and open each bridge's window over what sits behind
/// it. Endpoints are left with memory decode on and bus mastering off; their
/// drivers turn mastering on. Bridges get memory decode and mastering, since
/// a bridge with mastering off drops its children's DMA.
pub fn assign<C: ConfigPort>(
    cfg: &mut C,
    bus_start: u8,
    bus_count: u16,
    window: Window,
) -> Assigned {
    let last_bus = (bus_start as u16 + bus_count.max(1) - 1).min(255) as u8;
    let mut walk = Walk {
        cfg,
        next_bus: bus_start as u16 + 1,
        last_bus,
        cursor: window.base,
        window,
        out: Assigned::default(),
    };
    walk.bus(bus_start, 0);
    walk.out.last_bus = (walk.next_bus - 1) as u8;
    walk.out
}

pub(super) struct Walk<'a, C: ConfigPort> {
    pub(super) cfg: &'a mut C,
    pub(super) next_bus: u16,
    pub(super) last_bus: u8,
    pub(super) cursor: u64,
    pub(super) window: Window,
    pub(super) out: Assigned,
}

pub(super) fn present(id: u32) -> bool {
    let vendor = id & 0xFFFF;
    vendor != 0xFFFF && vendor != 0
}

pub(super) fn align_up(value: u64, align: u64) -> Option<u64> {
    let mask = align - 1;
    value.checked_add(mask).map(|v| v & !mask)
}
