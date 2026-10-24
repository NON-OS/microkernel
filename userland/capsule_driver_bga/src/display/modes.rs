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

//! MODES: the modes the monitor's EDID lists that fit, and the mode shown
//! now. Preferred is EDID's when it fits, else the current one (section
//! 12: the device's own mode, then EDID's, then the current; DISPI has no
//! mode of its own).

use super::bga::Bga;
use nonos_gfx_wire::body::mode::{CURRENT, FROM_DEVICE, PREFERRED};
use nonos_gfx_wire::Mode;

/// Write the list into `out`; how many.
pub fn modes(b: &Bga, out: &mut [Mode]) -> usize {
    let (w, h) = (b.mode.width, b.mode.height);
    let pref = if b.panel.preferred.is_none() { PREFERRED } else { 0 };
    let mut n = 0;
    let mut current_listed = false;
    for m in &b.panel.modes[..b.panel.len] {
        let Some(slot) = out.get_mut(n) else { break };
        *slot = *m;
        if (m.width, m.height) == (w, h) {
            slot.flags |= CURRENT | FROM_DEVICE | pref;
            current_listed = true;
        }
        n += 1;
    }
    if !current_listed {
        if let Some(slot) = out.get_mut(n) {
            *slot = Mode { width: w, height: h, flags: CURRENT | FROM_DEVICE | pref };
            n += 1;
        }
    }
    n
}
