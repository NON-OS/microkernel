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

//! A device whose reset takes a chosen number of status reads to settle.

use core::cell::Cell;

use crate::common::{CommonCfg, DEVICE_STATUS};

/// Only the status byte: after a write of zero it reads non-zero for
/// `settle` more reads, then zero.
pub struct SlowReset {
    pub status: Cell<u8>,
    settle: Cell<u32>,
    pub reads: Cell<u32>,
}

impl SlowReset {
    pub fn new(settle: u32) -> Self {
        Self { status: Cell::new(0x0F), settle: Cell::new(settle), reads: Cell::new(0) }
    }
}

impl CommonCfg for SlowReset {
    fn r8(&self, off: usize) -> u8 {
        assert_eq!(off, DEVICE_STATUS);
        self.reads.set(self.reads.get() + 1);
        match self.settle.get() {
            0 => self.status.get(),
            n => {
                self.settle.set(n - 1);
                0x0F
            }
        }
    }
    fn r16(&self, _: usize) -> u16 {
        0
    }
    fn r32(&self, _: usize) -> u32 {
        0
    }
    fn w8(&self, off: usize, value: u8) {
        assert_eq!(off, DEVICE_STATUS);
        self.status.set(value);
    }
    fn w16(&self, _: usize, _: u16) {}
    fn w32(&self, _: usize, _: u32) {}
}
