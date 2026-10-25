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

//! Every disk a driver capsule is serving on this boot, and every
//! controller on the bus that should have one and has none.
//!
//! A driver that is registered but does not answer is reported, not
//! skipped: a disk that is present and broken is something the person
//! choosing a target needs to see, and silently dropping it would make the
//! list look complete when it is not. For the same reason a controller
//! with no service behind it is reported too. A driver that gave up has
//! exited and taken its service name with it; the controller it gave up on
//! is still on the bus, and its entry carries [`BlkError::NoService`].

use alloc::vec::Vec;

use super::handle::BlockDevice;
use super::unserved::unserved;
use crate::driver::{Controller, Driver, ALL};
use crate::error::BlkError;

pub struct Found {
    pub driver: Driver,
    pub instance: u8,
    pub device: Result<BlockDevice, BlkError>,
}

/// `present` is what is on the bus, from `present::controllers`.
pub fn discover(present: &[Controller]) -> Vec<Found> {
    let mut out = Vec::new();
    for &driver in ALL.iter() {
        let mut served = 0u8;
        for instance in 0..driver.instances() {
            let device = match BlockDevice::open(driver, instance) {
                Ok(Some(device)) => Ok(device),
                Ok(None) => continue,
                Err(e) => Err(e),
            };
            served += 1;
            out.push(Found { driver, instance, device });
        }
        let on_bus = present.iter().filter(|&&c| driver.serves(c)).count();
        for k in 0..unserved(one_disk_each(driver), on_bus, served) {
            out.push(Found {
                driver,
                instance: served.saturating_add(k),
                device: Err(BlkError::NoService),
            });
        }
    }
    out
}

/// An NVMe controller is one disk, a UFS host one device. SATA and eMMC serve
/// their disk from whichever controller holds one, a virtio-blk device can be
/// the medium this boot came from, and a card reader is never counted missing.
fn one_disk_each(driver: Driver) -> bool {
    matches!(driver, Driver::Nvme | Driver::Ufs)
}
