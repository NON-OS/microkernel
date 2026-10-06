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

use crate::controller::remap::{
    may_remap, remapped_nvme, MAX_REMAP, REMAP_CAP, REMAP_DCC, REMAP_DCC_STRIDE, VSCAP,
};
use crate::log::Line;
use crate::regs::Regs;

/// Say when Intel RST has hidden NVMe drives behind this controller, so a
/// laptop whose internal NVMe is missing from the disk list says why. The
/// drives stay out of reach, as on Linux; the firmware's SATA mode must be
/// set to AHCI for the NVMe driver to find them.
pub(super) fn say_remapped(regs: Regs, abar_size: u64, mapped: u64) {
    if !may_remap(abar_size, mapped) {
        return;
    }
    let mut dcc = [0u32; MAX_REMAP];
    // SAFETY: may_remap checked the mapped ABAR reaches past the last slot's
    // class code, the highest register read here.
    let (vscap, cap) = unsafe {
        for (i, class) in dcc.iter_mut().enumerate() {
            *class = regs.r32(REMAP_DCC + i as u32 * REMAP_DCC_STRIDE);
        }
        (regs.r32(VSCAP), regs.r32(REMAP_CAP))
    };
    let n = remapped_nvme(vscap, cap, dcc);
    if n == 0 {
        return;
    }
    Line::new()
        .text(b"Intel RST hides ")
        .num(u64::from(n))
        .text(b" NVMe drive(s) behind this controller; set the firmware's SATA mode to AHCI")
        .send();
}
