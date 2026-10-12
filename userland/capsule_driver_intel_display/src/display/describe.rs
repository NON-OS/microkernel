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

//! What the engine says of itself on the wire. CAPS: kind 7, one output a
//! taken-over pipe, MODE_SET (a list device, no MODE_RANGE) when any output
//! has mode setting, the largest frame any output may take, no HW_CURSOR,
//! DEVICE_READS off (FLUSH copies), the first output's panel size.

use nonos_gfx_wire::body::caps::MODE_SET;
use nonos_gfx_wire::{kind, Caps};

use super::frames::Frames;
use super::head::Head;
use super::intel::Intel;
use crate::engine::access::{Clock, Mmio};

/// The largest mode `h` may show: its monitor's biggest when it can be
/// set, else the one it shows.
fn largest<F: Frames>(h: &Head<F>) -> (u32, u32) {
    let shown = (h.scan.width, h.scan.height);
    let listed = h.panel.modes[..h.panel.len].iter().map(|m| (m.width, m.height));
    let biggest = |a: (u32, u32), b: (u32, u32)| (a.0.max(b.0), a.1.max(b.1));
    match h.modeset {
        Some(_) => listed.fold(shown, biggest),
        None => shown,
    }
}

pub fn caps<M: Mmio, F: Frames, C: Clock>(d: &Intel<M, F, C>) -> Caps {
    let heads = || d.heads.iter().flatten();
    let (max_width, max_height) =
        heads().map(largest).fold((0, 0), |a, b| (a.0.max(b.0), a.1.max(b.1)));
    let set = heads().any(|h| h.modeset.is_some());
    Caps {
        kind: kind::INTEL,
        outputs: d.count() as u16,
        flags: if set { MODE_SET } else { 0 },
        max_width,
        max_height,
        width_align: 1,
        surface_budget: (max_width * 4).next_multiple_of(64) as u64 * max_height as u64,
        physical_mm: heads().next().map_or(0, |h| h.panel.physical_mm),
    }
}
