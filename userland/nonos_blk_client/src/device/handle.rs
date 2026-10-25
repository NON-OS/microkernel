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

//! The handle: which driver, which instance of it, which disk behind that
//! instance, on which port, how big, and the blocks its disk is addressed in.

use nonos_libc::mk_service_lookup;

use super::addressed::geometry_of;
use super::geometry::Geometry;
use super::medium::held;
use crate::driver::Driver;
use crate::error::BlkError;

#[derive(Clone, Copy, Debug)]
pub struct BlockDevice {
    pub driver: Driver,
    /// Which of the driver's service names answered: 0 for `driver.nvme0`.
    pub instance: u8,
    /// Which disk behind that service, 0 first; only the block wire's
    /// drivers have more than unit 0.
    pub unit: u16,
    pub port: u32,
    /// The size in 512-byte sectors, whatever the disk's own block size.
    pub sectors: u64,
    pub geometry: Geometry,
}

impl BlockDevice {
    /// The port instance `instance` of the driver's service answers on,
    /// `None` when no such service is registered on this boot.
    pub fn lookup(driver: Driver, instance: u8) -> Option<u32> {
        let name = driver.service(instance)?;
        let (mut port, mut pid) = (0u32, 0u32);
        let rc = mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
        (rc >= 0 && port != 0).then_some(port)
    }

    /// Ask unit `unit` behind `port` how big its disk is and what blocks it
    /// is addressed in. `None` when it holds no disk: a reader with no card,
    /// or a unit its driver does not have; an error when it did not answer.
    pub fn at(
        driver: Driver,
        instance: u8,
        port: u32,
        unit: u16,
    ) -> Result<Option<BlockDevice>, BlkError> {
        let Some(geometry) = held(driver, geometry_of(driver, port, unit))? else {
            return Ok(None);
        };
        let mut dev = BlockDevice { driver, instance, unit, port, sectors: 0, geometry };
        let Some(blocks) = held(driver, dev.capacity_blocks())? else {
            return Ok(None);
        };
        dev.sectors = geometry.sectors(blocks);
        Ok(Some(dev))
    }
}
