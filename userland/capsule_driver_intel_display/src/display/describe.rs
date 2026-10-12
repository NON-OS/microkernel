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

//! What the engine says of itself on the wire: CAPS (kind 7, one output,
//! firmware's mode only: no MODE_SET or MODE_RANGE until mode setting
//! joins the wire; no HW_CURSOR yet; DEVICE_READS off, because FLUSH
//! copies; the panel's size from its EDID), and MODES, the one mode it
//! scans out.

use nonos_gfx_wire::body::mode::{CURRENT, FROM_DEVICE, FROM_EDID, PREFERRED};
use nonos_gfx_wire::{kind, Caps, Mode};

use super::frames::Frames;
use super::intel::Intel;
use crate::engine::access::{Clock, Mmio};

pub fn caps<M: Mmio, F: Frames, C: Clock>(d: &Intel<M, F, C>) -> Caps {
    let (w, h) = (d.scan.width, d.scan.height);
    Caps {
        kind: kind::INTEL,
        outputs: 1,
        flags: 0,
        max_width: w,
        max_height: h,
        width_align: 1,
        surface_budget: d.scan.stride as u64 * h as u64,
        physical_mm: d.panel.physical_mm,
    }
}

/// The mode shown, the one a takeover offers, marked FROM_EDID when the
/// monitor lists it too.
pub fn modes<M: Mmio, F: Frames, C: Clock>(d: &Intel<M, F, C>, out: &mut [Mode]) -> usize {
    let Some(slot) = out.first_mut() else { return 0 };
    let (width, height) = (d.scan.width, d.scan.height);
    let listed =
        d.panel.modes[..d.panel.len].iter().any(|m| (m.width, m.height) == (width, height));
    let edid = if listed { FROM_EDID } else { 0 };
    *slot = Mode { width, height, flags: PREFERRED | CURRENT | FROM_DEVICE | edid };
    1
}
