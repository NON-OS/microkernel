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

//! The end of one bring-up attempt. Setup has taken every grant; programming
//! the part either works, or every grant goes back before the attempt reports
//! failure, so the next attempt claims the device afresh instead of meeting
//! its own leftover claim.

use crate::setup::Driver;

use super::run::bring_up;

pub fn finish(mut driver: Driver) -> Result<Driver, &'static str> {
    match bring_up(&mut driver) {
        Ok(()) => Ok(driver),
        Err(e) => {
            driver.release();
            Err(e)
        }
    }
}
