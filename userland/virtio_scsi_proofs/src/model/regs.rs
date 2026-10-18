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

//! The three register regions the driver reaches through `CommonCfg`.

use nonos_virtio::common::CommonCfg;

use super::Shared;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Which {
    Common,
    Device,
    Notify,
}

pub struct Regs {
    pub m: Shared,
    pub which: Which,
}

impl Regs {
    pub fn new(m: &Shared, which: Which) -> Self {
        Self { m: m.clone(), which }
    }
}

impl CommonCfg for Regs {
    fn r8(&self, off: usize) -> u8 {
        match self.which {
            Which::Common => self.m.common.r8(off),
            _ => panic!("byte read of the {:?} region at {off}", self.which),
        }
    }

    fn r16(&self, off: usize) -> u16 {
        match self.which {
            Which::Common => self.m.common.r16(off),
            Which::Device => match off {
                30 => self.m.rest.borrow().cfg.max_target,
                28 => 0,
                _ => panic!("16-bit read of device configuration {off}"),
            },
            Which::Notify => panic!("read of a doorbell"),
        }
    }

    fn r32(&self, off: usize) -> u32 {
        match self.which {
            Which::Common => self.m.common.r32(off),
            Which::Device => {
                let r = self.m.rest.borrow();
                match off {
                    0 => r.cfg.num_queues,
                    8 => r.cfg.max_sectors,
                    20 => r.cfg.sense_size,
                    24 => r.cfg.cdb_size,
                    32 => r.cfg.max_lun,
                    _ => panic!("32-bit read of device configuration {off}"),
                }
            }
            Which::Notify => panic!("read of a doorbell"),
        }
    }

    fn w8(&self, off: usize, value: u8) {
        match self.which {
            Which::Common => {
                // A reset forgets the queue: the device starts its indices
                // over, and a host that stalled a request has let it go.
                if off == crate::common::DEVICE_STATUS && value == 0 {
                    let mut r = self.m.rest.borrow_mut();
                    r.seen_avail = 0;
                    r.used_idx = 0;
                    r.stall = false;
                }
                self.m.common.w8(off, value)
            }
            _ => panic!("byte write of the {:?} region at {off}", self.which),
        }
    }

    fn w16(&self, off: usize, value: u16) {
        match self.which {
            Which::Common => self.m.common.w16(off, value),
            Which::Notify => self.m.notify(off, value),
            Which::Device => panic!("16-bit write of device configuration {off}"),
        }
    }

    fn w32(&self, off: usize, value: u32) {
        match self.which {
            Which::Common => self.m.common.w32(off, value),
            Which::Device => {
                let mut r = self.m.rest.borrow_mut();
                if r.cfg.sizes_fixed {
                    return;
                }
                match off {
                    20 => r.cfg.sense_size = value,
                    24 => r.cfg.cdb_size = value,
                    _ => panic!("write of read-only device configuration {off}"),
                }
            }
            Which::Notify => panic!("32-bit doorbell"),
        }
    }
}
