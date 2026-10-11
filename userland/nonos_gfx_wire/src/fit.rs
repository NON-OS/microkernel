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

//! The resize rule of section 12, over CAPS and MODES, so the compositor
//! and any driver that resizes on its own compute it the same way: the
//! rule itself lives in `nonos_modes::fit`.

use crate::body::caps::{Caps, MODE_RANGE};
use crate::body::mode::{Mode, MAX_MODES};
use nonos_modes::fit::Range;

/// The size to ask MODE_SET for when the host wants `want`; None for a
/// zero size, the current size, or nothing that fits.
pub fn fit(
    want: (u32, u32),
    current: (u32, u32),
    caps: &Caps,
    modes: &[Mode],
) -> Option<(u32, u32)> {
    let range = caps.has(MODE_RANGE).then_some(Range {
        max_width: caps.max_width,
        max_height: caps.max_height,
        width_align: caps.width_align as u32,
        budget: caps.surface_budget,
    });
    let mut list = [(0u32, 0u32); MAX_MODES];
    let n = modes.len().min(list.len());
    for (slot, m) in list.iter_mut().zip(modes) {
        *slot = (m.width, m.height);
    }
    nonos_modes::fit(want, current, range.as_ref(), &list[..n])
}
