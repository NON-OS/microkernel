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

//! The modern sequence: PCI command, the common and notify regions, the
//! same two DMA grants as the legacy path, then reset, ACKNOWLEDGE, DRIVER,
//! features (VERSION_1, and ACCESS_PLATFORM when offered; virtio-rng has no
//! device features), FEATURES_OK, the request queue, DRIVER_OK. A step that
//! fails returns its error and `sequence::run` releases the claim, which
//! takes every grant with it.

use nonos_virtio::common::{accept, driver_ok, start};
use nonos_virtio::{map_window, ModernCaps, Need, VirtioError};

use super::super::dma;
use super::super::driver::Driver;
use super::super::registers::RegisterGrant;
use super::{pci, queue};
use crate::constants::ENTROPY_BUF_LEN;
use crate::discover::Found;
use crate::queue::Queue;
use crate::transport::{LibcBroker, Modern, Transport};

pub fn run(dev: Found, claim_epoch: u64, caps: &ModernCaps) -> Result<Driver, &'static str> {
    pci::enable(dev.device_id, claim_epoch)?;
    let mut broker = LibcBroker::new(dev.device_id, claim_epoch);
    let window = map_window(&mut broker, caps, Need { isr: false, device: false })
        .map_err(VirtioError::message)?;
    let register_grant = RegisterGrant::Modern(window);
    let queue_dma = dma::map_queue(dev.device_id, claim_epoch, register_grant)?;
    let buf_dma = dma::map_buffer(dev.device_id, claim_epoch, register_grant, &queue_dma)?;

    let common = window.common.region;
    start(&common).map_err(VirtioError::message)?;
    accept(&common, 0).map_err(VirtioError::message)?;
    let ready = queue::setup(&common, window.notify_area(), queue_dma.device_addr)?;
    driver_ok(&common);

    let queue = Queue::new(
        queue_dma.user_va,
        queue_dma.device_addr,
        buf_dma.user_va,
        buf_dma.device_addr,
        ENTROPY_BUF_LEN as u32,
        ready.queue_size,
    );
    Ok(Driver {
        device_id: dev.device_id,
        claim_epoch,
        register_grant,
        queue_grant: queue_dma.grant_id,
        buf_grant: buf_dma.grant_id,
        queue,
        transport: Transport::Modern(Modern {
            notify: window.notify.region,
            doorbell: ready.doorbell,
        }),
    })
}
