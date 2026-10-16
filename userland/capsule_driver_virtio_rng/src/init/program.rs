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

use super::fail::fail;
use crate::constants::STATUS_DRIVER_OK;
use crate::constants::{LEGACY_MAX, LEG_QUEUE_NUM, LEG_QUEUE_PFN, LEG_QUEUE_SEL, LEG_STATUS};
use crate::regs::Regs;

/// Select the requestq and give the device the ring's page number. The
/// legacy QUEUE_NUM is read-only: the device lays the ring out for that
/// size and finds the available ring after that many descriptors, so the
/// size is taken as offered, or refused when the region cannot hold it. It
/// was cut to 16, where a device with more never found a request.
pub fn program(regs: Regs, queue_phys: u64) -> Result<u16, &'static str> {
    regs.w16(LEG_QUEUE_SEL, 0);
    let qsize = regs.r16(LEG_QUEUE_NUM);
    if qsize == 0 {
        return fail(regs, "virtio: requestq missing");
    }
    if qsize > LEGACY_MAX {
        return fail(regs, "virtio-rng: legacy queue does not fit the ring region");
    }
    // The page frame number register is 32 bits wide.
    let Ok(pfn) = u32::try_from(queue_phys >> 12) else {
        return fail(regs, "virtio-rng: ring above the legacy page range");
    };
    regs.w32(LEG_QUEUE_PFN, pfn);
    Ok(qsize)
}

/// The ring programmed, then bus mastering (`master`), then DRIVER_OK. A
/// device left live by a dead predecessor holds that one's ring, in frames
/// since freed, until it is given this one, so it may reach memory only
/// after `program`, which is also the last step that refuses it. Bus Master
/// used to go on before the ring and its refusals.
pub fn start(
    regs: Regs,
    queue_phys: u64,
    master: impl FnOnce() -> Result<(), &'static str>,
) -> Result<u16, &'static str> {
    let size = program(regs, queue_phys)?;
    master()?;
    driver_ok(regs);
    Ok(size)
}

pub fn driver_ok(regs: Regs) {
    regs.w8(LEG_STATUS, regs.r8(LEG_STATUS) | STATUS_DRIVER_OK);
}
