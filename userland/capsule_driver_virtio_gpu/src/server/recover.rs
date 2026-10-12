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

//! A control queue the device stalled on, brought back in place (plan/50
//! C1). The server runs only once the primary surface is shared, and the
//! compositor draws into its frames from then on: giving the device back
//! would have the kernel free them under it, and nothing restarts a driver.
//! So the claim and every grant stay. The device is reset, its queue
//! restarted on the same grant, and the primary resource made again under
//! the same id on the same frames and shown, at most three tries, each
//! logged. If none works the device is left reset and the driver stays up,
//! `Down`: device ops answer E_DEVICE and OP_HEALTHCHECK names the state.

use nonos_libc::mk_debug;

use crate::driver::Driver;
use crate::init::restart;
use crate::setup::prime;
use crate::state::Health;

const TRIES: [&[u8]; 3] = [
    b"driver.virtio_gpu0: control queue stalled; device reset, recovering (try 1 of 3)\n",
    b"driver.virtio_gpu0: control queue stalled; device reset, recovering (try 2 of 3)\n",
    b"driver.virtio_gpu0: control queue stalled; device reset, recovering (try 3 of 3)\n",
];
const RECOVERED: &[u8] = b"driver.virtio_gpu0: recovered in place, same surface\n";
const DOWN: &[u8] = b"driver.virtio_gpu0: down; device reset, claim and surface kept\n";

/// After every request: a stalled queue is recovered, once per stall; a
/// driver already down is left answering E_DEVICE.
pub fn settle(driver: &mut Driver) {
    if driver.control_queue.is_reset() && driver.health == Health::Live {
        recover(driver);
    }
}

fn recover(driver: &mut Driver) {
    for say in TRIES {
        let _ = mk_debug(say.as_ptr(), say.len());
        if attempt(driver).is_ok() {
            driver.recoveries = driver.recoveries.saturating_add(1);
            let _ = mk_debug(RECOVERED.as_ptr(), RECOVERED.len());
            return;
        }
        driver.control_queue.reset_device();
    }
    driver.health = Health::Down;
    let _ = mk_debug(DOWN.as_ptr(), DOWN.len());
}

fn attempt(driver: &mut Driver) -> Result<(), &'static str> {
    let (queue, features) = restart(&driver.control_queue, driver.notify)?;
    driver.control_queue = queue;
    driver.host_features = features as u32;
    driver.queue_size = driver.control_queue.layout.queue_size;
    // The reset took render context 1 with every host resource.
    driver.virgl_ready = false;
    match driver.primary {
        Some(p) => prime::show(&driver.control_queue, &driver.fences, &p),
        None => Ok(()),
    }
}
