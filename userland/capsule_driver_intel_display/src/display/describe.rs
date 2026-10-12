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

//! What the engine says of itself on the wire. CAPS: kind 7, one output, a
//! list device (MODE_SET, no MODE_RANGE) when mode setting was found, else
//! firmware's mode alone; no HW_CURSOR; DEVICE_READS off (FLUSH copies).
//! MODES: with mode setting, the modes the monitor lists, the one shown
//! marked current; without, the shown mode alone.

use nonos_gfx_wire::body::caps::MODE_SET;
use nonos_gfx_wire::body::mode::{CURRENT, FROM_DEVICE, FROM_EDID, PREFERRED};
use nonos_gfx_wire::{kind, Caps, Mode};

use super::frames::Frames;
use super::intel::Intel;
use crate::engine::access::{Clock, Mmio};

pub fn caps<M: Mmio, F: Frames, C: Clock>(d: &Intel<M, F, C>) -> Caps {
    let listed = &d.panel.modes[..d.panel.len];
    let set = d.modeset.is_some();
    let biggest = |f: fn(&Mode) -> u32| listed.iter().map(f).max().unwrap_or(0);
    let max_width = biggest(|m| m.width).max(d.scan.width);
    let max_height = biggest(|m| m.height).max(d.scan.height);
    let frame = |w: u32, h: u32| (w * 4).next_multiple_of(64) as u64 * h as u64;
    Caps {
        kind: kind::INTEL,
        outputs: 1,
        flags: if set { MODE_SET } else { 0 },
        max_width: if set { max_width } else { d.scan.width },
        max_height: if set { max_height } else { d.scan.height },
        width_align: 1,
        surface_budget: if set {
            frame(max_width, max_height)
        } else {
            frame(d.scan.width, d.scan.height)
        },
        physical_mm: d.panel.physical_mm,
    }
}

pub fn modes<M: Mmio, F: Frames, C: Clock>(d: &Intel<M, F, C>, out: &mut [Mode]) -> usize {
    let shown = (d.scan.width, d.scan.height);
    let listed = &d.panel.modes[..d.panel.len];
    let on_list = listed.iter().any(|m| (m.width, m.height) == shown);
    let mut n = 0;
    if d.modeset.is_some() {
        for m in listed.iter().filter(|m| (m.width, m.height) != shown) {
            let Some(slot) = out.get_mut(n) else { return n };
            *slot = *m;
            n += 1;
        }
    }
    let Some(slot) = out.get_mut(n) else { return n };
    let edid = if on_list { FROM_EDID } else { 0 };
    let pref = if d.modeset.is_none() || d.panel.preferred.is_none_or(|p| p == shown) {
        PREFERRED
    } else {
        0
    };
    *slot = Mode { width: shown.0, height: shown.1, flags: CURRENT | FROM_DEVICE | edid | pref };
    n + 1
}
