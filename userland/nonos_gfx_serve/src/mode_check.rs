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

//! Whether a MODE_SET size is one the device takes (section 12, Modes): a
//! MODE_RANGE device any size from 640x480 to its maximum with the width
//! a multiple of its alignment and the frame plus the cursor slot within
//! its surface budget; any other device only the modes it lists.

use nonos_gfx_wire::body::caps::{HW_CURSOR, MODE_RANGE};
use nonos_gfx_wire::body::mode::MAX_MODES;
use nonos_gfx_wire::errno::{EINVAL, ENOSYS};
use nonos_gfx_wire::{Caps, Errno, Mode};

use super::display::Display;
use super::primary::Primary;

pub fn check<D: Display>(d: &D, output: u16, w: u32, h: u32) -> Result<(), Errno> {
    let caps: Caps = d.caps();
    if !caps.has(nonos_gfx_wire::body::caps::MODE_SET) {
        return Err(ENOSYS);
    }
    if caps.has(MODE_RANGE) {
        let align = caps.width_align.max(1) as u32;
        let (len, _, _) = Primary::len(w, h, caps.has(HW_CURSOR)).ok_or(EINVAL)?;
        let ok = (640..=caps.max_width).contains(&w)
            && (480..=caps.max_height).contains(&h)
            && w.is_multiple_of(align)
            && len as u64 <= caps.surface_budget;
        return if ok { Ok(()) } else { Err(EINVAL) };
    }
    let mut modes = [Mode::default(); MAX_MODES];
    let n = d.modes(output, &mut modes)?;
    if modes[..n].iter().any(|m| m.width == w && m.height == h) {
        Ok(())
    } else {
        Err(EINVAL)
    }
}
