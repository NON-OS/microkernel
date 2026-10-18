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

use super::negotiate::fail;
use crate::constants::{LEG_CFG_CAPACITY, LEG_STATUS, STATUS_DRIVER_OK};
use crate::regs::Regs;

/// With MSI-X on, the two vector registers come before the device config.
const MSIX_CONFIG_SHIFT: usize = 4;

/// The capacity in 512-byte sectors; a disk of none is refused. It is read
/// once: a resize is not handled (plan/57 names it its own unit), and the
/// configuration-change interrupt is pointed nowhere (`vectors`).
pub fn capacity(regs: Regs, msix: bool) -> Result<u64, &'static str> {
    let at = LEG_CFG_CAPACITY + if msix { MSIX_CONFIG_SHIFT } else { 0 };
    match regs.r64(at) {
        0 => fail(regs, "virtio-blk: zero capacity"),
        sectors => Ok(sectors),
    }
}

pub fn driver_ok(regs: Regs) {
    regs.w8(LEG_STATUS, regs.r8(LEG_STATUS) | STATUS_DRIVER_OK);
}
