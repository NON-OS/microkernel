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

//! One bring-up attempt. A failed one holds nothing afterwards: once the
//! register window is mapped the device is reset first, and
//! then the claim goes, and every grant with it.

use nonos_libc::mk_device_release;
use nonos_virtio::common::reset_until;

use super::{claim, live, mmio, pci};
use crate::clock::expired_after;
use crate::constants::RESET_MS;
use crate::discover::{find_virtio_gpu, Found};
use crate::driver::Driver;

pub fn run() -> Result<Driver, &'static str> {
    let dev = find_virtio_gpu().ok_or("virtio-gpu: device not found")?;
    let claim_epoch = claim::claim(dev.device_id)?;
    let attempt = claimed(dev, claim_epoch);
    if attempt.is_err() {
        /*
         * Whichever step failed, the claim goes, and every register, queue
         * and surface grant with it: the broker stops bus mastering first
         * and only then tears the grants down, so the next attempt can
         * claim afresh. Nothing is unmapped before that.
         */
        let _ = mk_device_release(dev.device_id);
    }
    attempt
}

fn claimed(dev: Found, claim_epoch: u64) -> Result<Driver, &'static str> {
    /*
     * No interrupt is bound: the driver polls, and DECODE takes the device
     * off its legacy line so it cannot hold a shared line up.
     */
    pci::command(dev.device_id, claim_epoch, pci::DECODE)?;
    let window = mmio::map(&dev, claim_epoch)?;
    let attempt = live::run(dev, claim_epoch, &window);
    if attempt.is_err() {
        // A command may be in flight, or a resource attached to a grant:
        // the device is stopped before the claim is given back.
        let _ = reset_until(&window.common.region, expired_after(RESET_MS));
    }
    attempt
}
