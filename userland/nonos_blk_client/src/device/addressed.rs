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

//! How a disk is addressed, asked in its driver's own dialect.

use super::geometry::Geometry;
use crate::driver::Driver;
use crate::error::BlkError;

/// How the disk at `unit` behind `port` is addressed, in its driver's dialect.
pub(super) fn geometry_of(driver: Driver, port: u32, unit: u16) -> Result<Geometry, BlkError> {
    match driver {
        Driver::Nvme => super::nvme::geometry(port),
        Driver::Wire | Driver::Ufs | Driver::Ide | Driver::Scsi => {
            super::wire::geometry(port, unit)
        }
        Driver::Ahci | Driver::VirtioBlk | Driver::Rtsx => Ok(Geometry::SECTORS),
    }
}
