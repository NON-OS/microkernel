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

//! Every disk a driver capsule is serving on this boot.
//!
//! A driver that is registered but does not answer is reported, not
//! skipped: a disk that is present and broken is something the person
//! choosing a target needs to see, and silently dropping it would make the
//! list look complete when it is not.

use alloc::vec::Vec;

use super::handle::BlockDevice;
use crate::driver::{Driver, ALL};
use crate::error::BlkError;

pub struct Found {
    pub driver: Driver,
    pub device: Result<BlockDevice, BlkError>,
}

pub fn discover() -> Vec<Found> {
    let mut out = Vec::new();
    for &driver in ALL.iter() {
        match BlockDevice::open(driver) {
            Ok(Some(device)) => out.push(Found { driver, device: Ok(device) }),
            Ok(None) => {}
            Err(e) => out.push(Found { driver, device: Err(e) }),
        }
    }
    out
}
