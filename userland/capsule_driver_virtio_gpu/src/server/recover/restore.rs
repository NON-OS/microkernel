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

//! One recovery try (`tries`): the device reset and its queue restarted on
//! the same grant with the features settled again, then the primary resource
//! made again under the same id on the same frames and shown.

use crate::driver::Driver;
use crate::init::restart;
use crate::setup::prime;

pub fn attempt(driver: &mut Driver) -> Result<(), &'static str> {
    let (queue, features) = restart(&driver.control_queue, driver.notify)?;
    driver.control_queue = queue;
    driver.host_features = features as u32;
    driver.queue_size = driver.control_queue.layout.queue_size;
    match driver.primary {
        Some(p) => prime::show(&driver.control_queue, &driver.fences, &p),
        None => Ok(()),
    }
}
