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
use crate::constants::{LEG_QUEUE_NUM, LEG_QUEUE_PFN, LEG_QUEUE_SEL};
use crate::queue::ring_bytes;
use crate::regs::Regs;

/// The request queue's size. QUEUE_NUM is the device's to set on the
/// legacy transport: it is taken as offered whenever the split ring allows
/// it, and the ring grant is sized for it (`queue::ring_bytes`). It was
/// held to 256, which refused QEMU's queue-size=512 and 1024 outright, and
/// a size that is no power of two was taken.
pub fn queue_size(regs: Regs) -> Result<u16, &'static str> {
    regs.w16(LEG_QUEUE_SEL, 0);
    match regs.r16(LEG_QUEUE_NUM) {
        0 => fail(regs, "virtio-blk: requestq missing"),
        n if ring_bytes(n).is_none() => fail(regs, "virtio-blk: requestq size not a power of two"),
        n => Ok(n),
    }
}

/// Point the device at the ring by page number, which the register holds
/// in 32 bits; with MSI-X, at its vector first.
pub fn program(regs: Regs, ring_phys: u64, msix: bool) -> Result<(), &'static str> {
    let Ok(pfn) = u32::try_from(ring_phys >> 12) else {
        return fail(regs, "virtio-blk: ring above the legacy page range");
    };
    regs.w16(LEG_QUEUE_SEL, 0);
    if msix {
        crate::vectors::assign(regs);
    }
    regs.w32(LEG_QUEUE_PFN, pfn);
    Ok(())
}
