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

use super::super::controller::parse_memory32_fixed;
use super::super::crs::{resource_bytes, scan_large_descriptors, walk_large_descriptors};
use super::super::types::GpioController;

const LEAD_MEMORY32_FIXED: u8 = 0x86;

/// Fill a `GpioController` from its `_CRS` ResourceTemplate, preferring the
/// length-delimited descriptor walk and falling back to a raw byte scan when
/// the wrapper offset is unusual, exactly like `parse_controller_crs`.
pub(super) fn parse_gpio_crs(crs_value: &[u8], ctl: &mut GpioController) {
    if let Some(bytes) = resource_bytes(crs_value) {
        walk_large_descriptors(bytes, |lead, data| apply_memory32(lead, data, ctl));
    }
    if ctl.window_count == 0 {
        scan_large_descriptors(crs_value, |lead, data| apply_memory32(lead, data, ctl));
    }
}

/// Keep every Memory32Fixed window in `_CRS` order: an Intel PCH lists one
/// per community, in the order Linux's barno counts them. A window whose
/// base is zero is one the firmware patches in at run time (coreboot
/// gpio.asl writes PCRB into it), so no static window is kept from that
/// template at all. Returns true when the walk can stop.
fn apply_memory32(lead: u8, data: &[u8], ctl: &mut GpioController) -> bool {
    if lead != LEAD_MEMORY32_FIXED {
        return false;
    }
    match parse_memory32_fixed(data) {
        Some((base, size)) if size != 0 => !ctl.push_window(u64::from(base), u64::from(size)),
        // A whole descriptor that decodes to nothing has a zero base or
        // length: a run-time window. Drop what was kept so a half-static
        // template never shifts the community numbering.
        _ if data.len() >= 9 => {
            ctl.window_count = 0;
            true
        }
        _ => false,
    }
}
