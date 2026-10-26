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

//! Physical memory and the RAM disk a live boot's volume is made on: the
//! free memory a test sets, and a RAM disk that only records its start.

use std::cell::Cell;

thread_local! {
    static RAM_STARTED: Cell<Option<u64>> = const { Cell::new(None) };
    static FREE: Cell<u64> = const { Cell::new(8 << 30) };
}

/// How many sectors the RAM disk was started with, if it was.
pub fn ram_started() -> Option<u64> {
    RAM_STARTED.get()
}

pub fn reserve() -> u64 {
    1 << 30
}

pub fn start(sectors: u64) -> Result<(), crate::hardware::block_device::BlockDeviceError> {
    RAM_STARTED.set(Some(sectors));
    Ok(())
}

/// The RAM disk given back, when keeping turns on.
pub fn stop() {
    RAM_STARTED.set(None);
}

pub mod memory {
    pub mod phys {
        pub fn free_memory() -> u64 {
            super::super::FREE.get()
        }
    }
}
