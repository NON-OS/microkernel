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

//! Host memory standing in for the three DMA grants, at bus addresses above
//! 4 GiB, and a device that can reach only them.

use crate::constants::layout::CONTROL_BYTES;
use crate::constants::ring::{RX_POOL_BYTES, TX_POOL_BYTES};
use crate::model::{install, Device, Memory, Region};

pub const CONTROL_DEV: u64 = 0x1_0000_0000;
pub const RX_DEV: u64 = 0x2_0000_0000;
pub const TX_DEV: u64 = 0x3_0000_0000;
/// Grant ids as setup would hold them: claim, BAR0, BAR1, control, pools.
pub const GRANTS: [u64; 6] = [11, 12, 13, 14, 15, 16];

pub struct Bench {
    // u128 for the alignment the 64-bit descriptor fields need.
    pub control: Vec<u128>,
    pub rx: Vec<u8>,
    pub tx: Vec<u8>,
}

impl Bench {
    pub fn new() -> Self {
        let control = vec![0; CONTROL_BYTES / 16];
        Self { control, rx: vec![0; RX_POOL_BYTES], tx: vec![0; TX_POOL_BYTES] }
    }

    pub fn device(&mut self) -> Device {
        let control = self.control.as_mut_ptr() as *mut u8;
        let regions = vec![
            Region { dev: CONTROL_DEV, host: control, len: CONTROL_BYTES },
            Region { dev: RX_DEV, host: self.rx.as_mut_ptr(), len: RX_POOL_BYTES },
            Region { dev: TX_DEV, host: self.tx.as_mut_ptr(), len: TX_POOL_BYTES },
        ];
        Device::new(Memory { regions })
    }

    /// The device installed on this thread with `tune` applied, and a driver.
    pub fn with(&mut self, tune: impl FnOnce(&mut Device)) -> crate::setup::Driver {
        let mut device = self.device();
        tune(&mut device);
        install(device);
        self.driver()
    }
}
