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

//! A timer frame's physical timer, CNTBaseN (Arm ARM DDI 0487, the
//! memory-mapped view of the generic timer; offsets recalled, unverified):
//! CNTPCT at 0x00, CNTFRQ at 0x10, CNTP_CVAL at 0x20, CNTP_CTL at 0x2C.

use core::ptr::{read_volatile, write_volatile};

use crate::arch::aarch64::mmu::pointer::mapped;

pub(super) const CNTPCT_LO: u64 = 0x00;
pub(super) const CNTPCT_HI: u64 = 0x04;
const CNTFRQ: u64 = 0x10;
const CNTP_CVAL_LO: u64 = 0x20;
const CNTP_CVAL_HI: u64 = 0x24;
const CNTP_CTL: u64 = 0x2c;
const CTL_ENABLE: u32 = 1;

#[derive(Clone, Copy)]
pub(super) struct Frame {
    va: u64,
}

impl Frame {
    /// The frame at `va`, a window the kernel mapped and keeps.
    pub(super) fn new(va: u64) -> Self {
        Self { va }
    }

    pub(super) fn read(&self, offset: u64) -> u32 {
        // SAFETY: a register in the frame's mapped window.
        unsafe { read_volatile(mapped::<u32>(self.va, offset)) }
    }

    fn write(&self, offset: u64, value: u32) {
        // SAFETY: as `read`.
        unsafe { write_volatile(mapped::<u32>(self.va, offset), value) };
    }

    pub(super) fn frequency(&self) -> u32 {
        self.read(CNTFRQ)
    }

    /// Hold `compare`, or nothing. Off while it changes, so a half-written
    /// value never fires; one already past fires at once (a level interrupt).
    pub(super) fn hold(&self, compare: Option<u64>) {
        self.write(CNTP_CTL, 0);
        if let Some(at) = compare {
            self.write(CNTP_CVAL_LO, at as u32);
            self.write(CNTP_CVAL_HI, (at >> 32) as u32);
            self.write(CNTP_CTL, CTL_ENABLE);
        }
    }
}
