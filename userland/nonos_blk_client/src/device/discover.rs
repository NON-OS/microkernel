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
        for k in 0..unserved(driver, on_bus, served) {
            out.push(Found {
                driver,
                instance: served.saturating_add(k),
                device: Err(BlkError::NoService),
            });
        }
    }
    out
}

/// Controllers with no service behind them. An NVMe controller is one
/// disk, so each one past those served is one missing. The SATA driver
/// serves a disk from whichever of the machine's AHCI controllers holds
/// one, and a virtio-blk device can be the medium this boot came from, so
/// for those only a driver serving nothing is a missing disk.
fn unserved(driver: Driver, on_bus: usize, served: u8) -> u8 {
    let missing = match driver {
        Driver::Nvme => on_bus.saturating_sub(served as usize),
        Driver::Ahci | Driver::VirtioBlk => usize::from(served == 0 && on_bus > 0),
    };
    missing.min(u8::MAX as usize) as u8
}
