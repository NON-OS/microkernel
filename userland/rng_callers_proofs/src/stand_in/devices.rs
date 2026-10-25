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

//! The block device, as the volume's files reach it: a disk in memory, per
//! thread, that keeps every plan sector written.

use std::cell::RefCell;
use std::collections::BTreeMap;

#[path = "../../../../src/hardware/block_device/error.rs"]
mod block_device_error;

thread_local! {
    pub(crate) static PLAN: RefCell<BTreeMap<u64, [u8; 512]>> =
        const { RefCell::new(BTreeMap::new()) };
}

/// The plan partition's sectors written on this thread.
pub fn plan_written() -> BTreeMap<u64, [u8; 512]> {
    PLAN.with_borrow(Clone::clone)
}

pub mod hardware {
    pub mod block_device {
        pub use super::super::block_device_error::BlockDeviceError;
        use crate::stand_in::devices::PLAN;

        /// The volume's header ring: never written here, so it reads blank.
        pub fn read(_lba: u64, out: &mut [u8]) -> Result<(), BlockDeviceError> {
            out.fill(0);
            Ok(())
        }
        pub fn read_plan_sector(index: u64, out: &mut [u8]) -> Result<(), BlockDeviceError> {
            let s = PLAN.with_borrow(|p| p.get(&index).copied()).unwrap_or([0; 512]);
            out.copy_from_slice(&s[..out.len()]);
            Ok(())
        }
        pub fn write_plan_sector(index: u64, s: &[u8]) -> Result<(), BlockDeviceError> {
            let mut sector = [0u8; 512];
            sector[..s.len()].copy_from_slice(s);
            PLAN.with_borrow_mut(|p| p.insert(index, sector));
            Ok(())
        }
        pub fn flush() -> Result<(), BlockDeviceError> {
            Ok(())
        }
    }
}
