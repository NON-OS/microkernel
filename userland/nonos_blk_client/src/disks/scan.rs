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

//! Every disk a block driver is serving, with what is on it and what it
//! calls itself. A driver that is registered and did not answer is listed
//! as such rather than dropped, so the list never looks complete when it
//! is not.

use alloc::string::String;
use alloc::vec::Vec;

use super::contents::Contents;
use crate::device::{discover, BlockDevice, Identity};
use crate::driver::Driver;

pub struct Disk {
    pub driver: Driver,
    pub device: Option<BlockDevice>,
    pub contents: Contents,
    pub identity: Option<Identity>,
    /// Why there is no device behind this entry, when there is none.
    pub fault: Option<String>,
}

pub fn scan() -> Vec<Disk> {
    let mut out = Vec::new();
    for found in discover() {
        let driver = found.driver;
        match found.device {
            Ok(device) => out.push(Disk {
                driver,
                device: Some(device),
                contents: Contents::probe(&device),
                identity: device.identity().ok().flatten(),
                fault: None,
            }),
            Err(e) => out.push(Disk {
                driver,
                device: None,
                contents: Contents::Unknown,
                identity: None,
                fault: Some(alloc::format!("did not answer ({e:?})")),
            }),
        }
    }
    out
}
