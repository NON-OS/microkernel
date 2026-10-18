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

//! The codec's registers. Each access first takes the codec access
//! semaphore: reading CAS sets it, and the mixer access that follows clears
//! it. A semaphore that stays held is not waited on past a few
//! milliseconds; as Linux's intel8x0 does, the access goes ahead, since a
//! read of a register the codec does not implement never releases it.

use nonos_libc::Deadline;

use super::port::Port;
use crate::constants::bus::{CAS, CAS_BUSY};

const SEMAPHORE_MS: u64 = 2;

#[derive(Clone, Copy)]
pub struct Mixer {
    nam: Port,
    nabm: Port,
}

impl Mixer {
    pub const fn new(nam: Port, nabm: Port) -> Self {
        Self { nam, nabm }
    }

    fn take_semaphore(&self) {
        let deadline = Deadline::after_ms(SEMAPHORE_MS);
        while self.nabm.read8(CAS) & CAS_BUSY != 0 && !deadline.expired() {
            core::hint::spin_loop();
        }
    }

    pub fn read(&self, reg: u8) -> u16 {
        self.take_semaphore();
        self.nam.read16(reg as u16)
    }

    pub fn write(&self, reg: u8, value: u16) {
        self.take_semaphore();
        self.nam.write16(reg as u16, value);
    }
}
