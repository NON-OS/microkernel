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

//! MODES for one output: with mode setting, the modes its monitor lists,
//! the one shown marked current; without, the shown mode alone, marked
//! FROM_EDID when the monitor lists it too.

use nonos_gfx_wire::body::mode::{CURRENT, FROM_DEVICE, FROM_EDID, PREFERRED};
use nonos_gfx_wire::Mode;

use super::frames::Frames;
use super::head::Head;

pub fn modes<F: Frames>(h: &Head<F>, out: &mut [Mode]) -> usize {
    let shown = (h.scan.width, h.scan.height);
    let listed = &h.panel.modes[..h.panel.len];
    let mut n = 0;
    if h.modeset.is_some() {
        for m in listed.iter().filter(|m| (m.width, m.height) != shown) {
            let Some(slot) = out.get_mut(n) else { return n };
            *slot = *m;
            n += 1;
        }
    }
    let Some(slot) = out.get_mut(n) else { return n };
    let on_list = listed.iter().any(|m| (m.width, m.height) == shown);
    let edid = if on_list { FROM_EDID } else { 0 };
    let own = h.modeset.is_none() || h.panel.preferred.is_none_or(|p| p == shown);
    let pref = if own { PREFERRED } else { 0 };
    *slot = Mode { width: shown.0, height: shown.1, flags: CURRENT | FROM_DEVICE | edid | pref };
    n + 1
}
