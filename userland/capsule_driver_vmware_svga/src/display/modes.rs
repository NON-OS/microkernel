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

//! MODES: the mode shown now, preferred (the adapter has no EDID), then the
//! default and the smaller modes bring-up would choose, each that fits.

use super::svga::Svga;
use crate::constants::mode::{DEFAULT_MODE, FALLBACK_MODES};
use nonos_gfx_wire::body::mode::{CURRENT, FROM_DEVICE, PREFERRED};
use nonos_gfx_wire::Mode;

/// Write the list into `out`; how many.
pub fn modes(s: &Svga, out: &mut [Mode]) -> usize {
    let now = (s.mode.width, s.mode.height);
    let others = core::iter::once(DEFAULT_MODE).chain(FALLBACK_MODES);
    let others = others.filter(|&m| m != now && s.fits(m.0, m.1));
    let current = Mode { width: now.0, height: now.1, flags: CURRENT | FROM_DEVICE | PREFERRED };
    let all = core::iter::once(current).chain(others.map(|(w, h)| Mode {
        width: w,
        height: h,
        flags: 0,
    }));
    let mut n = 0;
    for (slot, m) in out.iter_mut().zip(all) {
        *slot = m;
        n += 1;
    }
    n
}
