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

//! The modern sequence: the common and notify regions, reset (waited for by
//! the clock), ACKNOWLEDGE, DRIVER, features (VERSION_1, and ACCESS_PLATFORM
//! whenever offered; virtio-rng has no device features), FEATURES_OK, the
//! two DMA grants, Bus Master, the request queue, DRIVER_OK. A step that
//! fails after the window is mapped resets the device before the caller
//! releases the claim, which takes every grant with it.

use nonos_virtio::common::{accept, driver_ok, reset_until, start_until};
use nonos_virtio::{map_window, ModernCaps, VirtioError, Window};

use super::super::driver::Driver;
use super::super::{dma, pci};
use super::queue;
use crate::clock::expired_after;
use crate::constants::RESET_MS;
use crate::discover::Found;
use crate::queue::Queue;
use crate::transport::{LibcBroker, Modern, Transport, NEED};

pub fn run(dev: Found, claim_epoch: u64, caps: &ModernCaps) -> Result<Driver, &'static str> {
    let mut broker = LibcBroker::new(dev.device_id, claim_epoch);
    let window = map_window(&mut broker, caps, NEED).map_err(VirtioError::message)?;
    let live = live(dev, claim_epoch, &window);
    if live.is_err() {
        let _ = reset_until(&window.common.region, expired_after(RESET_MS));
    }
    live
}

fn live(dev: Found, claim_epoch: u64, window: &Window) -> Result<Driver, &'static str> {
    let common = window.common.region;
    start_until(&common, expired_after(RESET_MS)).map_err(VirtioError::message)?;
    accept(&common, 0).map_err(VirtioError::message)?;
    let dma = dma::map_both(dev.device_id, claim_epoch)?;
    pci::command(dev.device_id, claim_epoch, pci::MASTER)?;
    let ready = queue::setup(&common, window.notify_area(), dma.ring.device_addr)?;
    driver_ok(&common);
    let notify = window.notify.region;
    Ok(Driver {
        device_id: dev.device_id,
        queue: Queue::new(dma.ring.view, dma.buf.view, dma.buf.device_addr, ready.queue_size),
        transport: Transport::Modern(Modern { common, notify, doorbell: ready.doorbell }),
    })
}
