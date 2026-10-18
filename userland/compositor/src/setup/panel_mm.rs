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

use nonos_libc::nonos_display_physical_mm;

use crate::state::Context;

/// The physical size of the panel shown on: CAPS' on the wire (the driver
/// read its monitor's EDID), the kernel's on GOP (the boot panel's EDID).
pub fn panel_mm(ctx: &Context) -> Option<(u32, u32)> {
    let mut mm: u32 = 0;
    if let Some(link) = ctx.wire {
        mm = link.physical_mm;
    } else if !ctx.gop_mode || nonos_display_physical_mm(0, &mut mm as *mut u32) < 0 {
        return None;
    }
    let (w, h) = (mm & 0xFFFF, mm >> 16);
    (w != 0 && h != 0).then_some((w, h))
}
