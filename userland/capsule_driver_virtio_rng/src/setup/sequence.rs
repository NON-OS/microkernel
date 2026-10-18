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

use nonos_libc::mk_device_release;

use super::driver::Driver;
use super::{claim, legacy, modern, pci};
use crate::discover::{find_virtio_rng, Found};
use crate::transport;

/// One bring-up attempt. A failed one holds nothing afterwards.
pub fn run() -> Result<Driver, &'static str> {
    let dev = find_virtio_rng().ok_or("no virtio-rng device")?;

    let claim_epoch = claim::claim(dev.device_id)?;

    let attempt = claimed(dev, claim_epoch);
    if attempt.is_err() {
        /*
         * Whichever step failed, the device was reset if the handshake had
         * reached it, and now the claim goes, and every MMIO, PIO and DMA
         * grant with it: the broker stops bus mastering first and only then
         * tears the grants down, so the next attempt can claim afresh.
         */
        let _ = mk_device_release(dev.device_id);
    }
    attempt
}

fn claimed(dev: Found, claim_epoch: u64) -> Result<Driver, &'static str> {
    pci::command(dev.device_id, claim_epoch, pci::DECODE)?;
    match transport::probe(&dev, claim_epoch)? {
        Some(caps) => modern::run(dev, claim_epoch, &caps),
        None => legacy::run(dev, claim_epoch),
    }
}
