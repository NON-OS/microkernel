// NONOS Operating System (AGPL-3.0-or-later)

//! The register accesses: the BAR offsets with behaviour, and the L1
//! window at 0x40000, which reaches the chip address the remap register's
//! low half names in its upper sixteen bits (`mt7921_reg_map_l1`). Behind
//! it: the chip's identity, the MCU's driver-ownership bit
//! (`MT_TOP_LPCR_HOST_BAND0`) and the subsystem reset (`WFSYS_SW_RST_B`,
//! answered by `WFSYS_SW_INIT_DONE`).

use super::chip::{Chip, LPCTL, MISC, REMAP};

const CHIPID: u32 = 0x70010200;
const BOUND: u32 = 0x70010020;
const REV: u32 = 0x70010204;
const TOP_LPCR: u32 = 0x18060010;
const WFSYS: u32 = 0x18000140;

impl Chip {
    fn l1_addr(&self, off: u32) -> u32 {
        (self.reg(REMAP) & 0xffff) << 16 | (off - 0x40000)
    }

    pub fn read(&mut self, off: u32) -> u32 {
        if self.faults.dead {
            return u32::MAX;
        }
        self.tick();
        match off {
            LPCTL => u32::from(self.own_sync) << 2,
            MISC => u32::from(self.mcu.running && !self.faults.never_ready) * 3,
            0x40000..=0x4ffff => match self.l1_addr(off) {
                CHIPID => self.id.0,
                BOUND => self.id.1,
                REV => self.id.2,
                TOP_LPCR => u32::from(self.fw_own),
                WFSYS => {
                    let v = self.l1.get(&WFSYS).copied().unwrap_or(0);
                    v | u32::from(v & 1 != 0 && !self.faults.no_reset) << 4
                }
                a => self.l1.get(&a).copied().unwrap_or(0),
            },
            _ => self.reg(off),
        }
    }

    pub fn write(&mut self, off: u32, v: u32) {
        if self.faults.dead {
            return;
        }
        self.writes.push((off, v));
        match off {
            LPCTL if v & 1 != 0 => self.own_sync = true,
            LPCTL if v & 2 != 0 => self.own_sync = self.faults.no_own,
            LPCTL => {}
            0x40000..=0x4ffff => {
                let a = self.l1_addr(off);
                if a == TOP_LPCR && v & 2 != 0 {
                    self.fw_own = false;
                }
                self.l1.insert(a, v);
            }
            _ => {
                self.regs.insert(off, v);
                self.ring_write(off);
            }
        }
        self.tick();
    }
}
