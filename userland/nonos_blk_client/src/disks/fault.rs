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

//! Why a driver's row is not a disk to install to.

use alloc::string::String;

use crate::driver::Driver;
use crate::error::BlkError;

pub(super) fn fault(driver: Driver, instance: u8, e: &BlkError, emmc_only: bool) -> String {
    let bus = if driver == Driver::Ahci && emmc_only {
        "eMMC"
    } else {
        core::str::from_utf8(driver.label()).unwrap_or("disk")
    };
    match e {
        BlkError::NoService if instance == 0 => {
            alloc::format!("{bus} controller present, its driver did not come up")
        }
        BlkError::NoService => {
            alloc::format!("another {bus} controller is present, no driver is serving it")
        }
        BlkError::BlockSize(n) => {
            alloc::format!("its driver serves {n}-byte blocks, which this cannot address")
        }
        _ => alloc::format!("its driver did not answer ({e:?})"),
    }
}
