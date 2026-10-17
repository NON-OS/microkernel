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
use super::claim;
use super::claimed::claimed;
use crate::discover::find_virtio_gpu;
use crate::driver::Driver;
use nonos_libc::mk_device_release;

/// One bring-up attempt. A failed one holds nothing afterwards.
pub fn run() -> Result<Driver, &'static str> {
    let dev = find_virtio_gpu();
    let dev = dev.ok_or("virtio-gpu: device not found")?;
    let claim_epoch = claim::claim(dev.device_id)?;
    let attempt = claimed(dev, claim_epoch);
    if attempt.is_err() {
        /*
         * bring_up, scanouts::seed, create_primary and the queue layout
         * returned their errors with the claim, the register grants, the
         * queue DMA and any surface DMA still held, so every attempt after
         * the first failed at claim. Releasing the claim takes every grant
         * with it; the steps that roll back on their own have released it
         * already and this answers "not claimed".
         */
        let _ = mk_device_release(dev.device_id);
    }
    attempt
}
