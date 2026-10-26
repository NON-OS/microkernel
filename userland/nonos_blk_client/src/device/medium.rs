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

//! Which driver answer means a disk is not there rather than broken: an
//! empty slot (ENOMEDIUM) on any driver, and a unit the driver does not have
//! (ENXIO) on a driver with more than one. Kept apart from the calls, with
//! no kernel import but the error, so the host proofs mount it.

use crate::driver::Driver;
use crate::error::BlkError;

pub const E_NXIO: i32 = -6;
pub const E_NOMEDIUM: i32 = -123;

/// `Ok(None)` for an answer that says no disk is at this unit.
pub fn held<T>(driver: Driver, r: Result<T, BlkError>) -> Result<Option<T>, BlkError> {
    match r {
        Err(BlkError::Status(E_NOMEDIUM)) => Ok(None),
        Err(BlkError::Status(E_NXIO)) if driver.units() > 1 => Ok(None),
        r => r.map(Some),
    }
}
