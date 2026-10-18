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

//! Host memory standing in for the three DMA grants, and a part that can
//! reach only them.

use crate::constants::layout::CONTROL_BYTES;
use crate::constants::queue::{RX_POOL_BYTES, TX_POOL_BYTES};
use crate::model::{install, Memory, Part, Region};
use crate::setup::Driver;

pub const CONTROL_DEV: u32 = 0x0010_0000;
pub const RX_DEV: u32 = 0x0020_0000;
pub const TX_DEV: u32 = 0x0040_0000;
/// Grant ids as setup would hold them, so a test can read them back.
pub const GRANTS: [u64; 5] = [11, 12, 13, 14, 15];

pub struct Bench {
    // u128 for the sixteen-byte alignment the rings need.
    pub control: Vec<u128>,
    pub rx: Vec<u8>,
    pub tx: Vec<u8>,
}

impl Bench {
    pub fn new() -> Self {
        Self {
            control: vec![0; CONTROL_BYTES / 16],
            rx: vec![0; RX_POOL_BYTES],
            tx: vec![0; TX_POOL_BYTES],
        }
    }

    /// A part out of reset that reaches exactly the three grants.
    pub fn part(&mut self) -> Part {
        let control = self.control.as_mut_ptr() as *mut u8;
        let regions = vec![
            Region { dev: CONTROL_DEV, host: control, len: CONTROL_BYTES },
            Region { dev: RX_DEV, host: self.rx.as_mut_ptr(), len: RX_POOL_BYTES },
            Region { dev: TX_DEV, host: self.tx.as_mut_ptr(), len: TX_POOL_BYTES },
        ];
        Part::new(Memory { regions })
    }

    /// The part installed on this thread with `tune` applied, and the driver.
    pub fn with(&mut self, tune: impl FnOnce(&mut Part)) -> Driver {
        let mut part = self.part();
        tune(&mut part);
        install(part);
        self.driver()
    }
}
