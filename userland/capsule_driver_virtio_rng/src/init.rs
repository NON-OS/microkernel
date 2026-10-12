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

//! The virtio init handshake on the legacy register window, split where bus
//! mastering goes on (`setup::legacy`): `negotiate` resets the device and
//! settles features, `program` hands it the ring, `driver_ok` lets it run.
//! A step that does not hold leaves the device marked FAILED.

use super::constants::{
    LEGACY_MAX, LEG_GUEST_FEATURES, LEG_HOST_FEATURES, LEG_QUEUE_NUM, LEG_QUEUE_PFN, LEG_QUEUE_SEL,
    LEG_STATUS, STATUS_ACKNOWLEDGE, STATUS_DRIVER, STATUS_DRIVER_OK, STATUS_FAILED,
    STATUS_FEATURES_OK,
};
use super::regs::Regs;

/// Reset, ACKNOWLEDGE, DRIVER, no feature bits (virtio-rng needs none, and
/// claiming one the device does not offer is a protocol violation), then
/// FEATURES_OK, read back.
pub fn negotiate(regs: Regs) -> Result<(), &'static str> {
    regs.w8(LEG_STATUS, 0);
    regs.w8(LEG_STATUS, STATUS_ACKNOWLEDGE);
    regs.w8(LEG_STATUS, regs.r8(LEG_STATUS) | STATUS_DRIVER);
    let _host = regs.r32(LEG_HOST_FEATURES);
    regs.w32(LEG_GUEST_FEATURES, 0);
    regs.w8(LEG_STATUS, regs.r8(LEG_STATUS) | STATUS_FEATURES_OK);
    if regs.r8(LEG_STATUS) & STATUS_FEATURES_OK == 0 {
        return fail(regs, "virtio: features-ok rejected");
    }
    Ok(())
}

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

pub fn driver_ok(regs: Regs) {
    regs.w8(LEG_STATUS, regs.r8(LEG_STATUS) | STATUS_DRIVER_OK);
}

fn fail<T>(regs: Regs, why: &'static str) -> Result<T, &'static str> {
    regs.w8(LEG_STATUS, regs.r8(LEG_STATUS) | STATUS_FAILED);
    Err(why)
}
