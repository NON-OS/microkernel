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

//! ACTIVATE_DEV: the device reads the shared area and the queue table and
//! refuses a layout it cannot run, as a host does by answering non-zero.
//! Offsets are vmxnet3_defs.h's, written out here independently of the
//! driver's, so a driver offset that is wrong fails the test.

use super::device::Device;
use super::ring::Ring;

const MAGIC: u32 = 0xBABE_FEE1;

fn ring(base: u64, size: u32) -> Result<Ring, &'static str> {
    if base & 511 != 0 || size == 0 || size & 31 != 0 || size > 4096 {
        return Err("ring not 512-aligned or its size not a multiple of 32");
    }
    Ok(Ring { base, size, next: 0, gen: 1 })
}

impl Device {
    pub(super) fn activate(&mut self) -> Result<(), &'static str> {
        if (self.vrrs, self.uvrs) != (1, 1) || self.faults.refuse_activation {
            return Err("revision not selected, or refused by the test");
        }
        let (s, misc) = (self.dsa, self.dsa + 8);
        if self.mem.u32(s) != MAGIC {
            return Err("no revision 1 magic");
        }
        let (qd, qd_len) = (self.mem.u64(misc + 32), self.mem.u32(misc + 44));
        let mut queues = [0u8; 2];
        self.mem.read(misc + 54, &mut queues);
        self.mtu = self.mem.u32(misc + 48);
        if qd & 127 != 0 || qd_len < 512 || queues != [1, 1] || !(60..=9000).contains(&self.mtu) {
            return Err("queue table, queue counts or MTU");
        }
        if self.mem.u32(s + 80 + 28) & 1 == 0 {
            return Err("interrupts left enabled");
        }
        let c = qd + 16;
        self.tx = ring(self.mem.u64(c), self.mem.u32(c + 40))?;
        let data = ring(self.mem.u64(c + 8), self.mem.u32(c + 44))?;
        self.tx_comp = ring(self.mem.u64(c + 16), self.mem.u32(c + 48))?;
        let c = qd + 256 + 16;
        self.rx = ring(self.mem.u64(c), self.mem.u32(c + 40))?;
        self.rx1 = ring(self.mem.u64(c + 8), self.mem.u32(c + 44))?;
        self.rx_comp = ring(self.mem.u64(c + 16), self.mem.u32(c + 48))?;
        if data.size != self.tx.size || self.rx_comp.size < self.rx.size + self.rx1.size {
            return Err("data ring or completion ring sizes");
        }
        Ok(())
    }
}
