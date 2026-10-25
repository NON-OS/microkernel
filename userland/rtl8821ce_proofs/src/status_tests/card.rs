// NONOS Operating System (AGPL-3.0-or-later)
//! An efuse with a factory address burned in, which can stall on one byte
//! and, at the worst, leave that byte in the control register's low bits.

use std::cell::Cell;

use crate::regs::Mmio;

/// The factory address: no byte of it is one the reply's fixed words hold.
pub(super) const FACTORY: [u8; 6] = [0x5C, 0xA7, 0x3E, 0xD9, 0x61, 0xB4];
const REG_EFUSE_CTRL: usize = 0x0030;
const REG_LDO_EFUSE_CTRL: usize = 0x0034;
const EF_FLAG: u32 = 1 << 31;

pub(super) struct Efuse {
    pub(super) phys: [u8; 512],
    pub(super) stall_at: Option<usize>,
    addr: Cell<usize>,
}

impl Efuse {
    /// The RF option at logical 0xCA (block 25, word 1), then the address at
    /// logical 0xD0 (block 26, words 0 to 2), in the packed physical format.
    pub(super) fn new(stall_at: Option<usize>) -> Self {
        let mut phys = [0xFF; 512];
        let packed = [&[0x2F, 0x3D, 0x22, 0x00][..], &[0x4F, 0x38], &FACTORY];
        phys[..12].copy_from_slice(&packed.concat());
        Self { phys, stall_at, addr: Cell::new(0) }
    }
}

impl Mmio for Efuse {
    fn read8(&self, _: usize) -> u8 {
        0
    }
    fn write8(&self, _: usize, _: u8) {}
    fn read16(&self, _: usize) -> u16 {
        0
    }
    fn write16(&self, _: usize, _: u16) {}
    fn read32(&self, off: usize) -> u32 {
        let a = self.addr.get();
        match off {
            REG_EFUSE_CTRL if Some(a) == self.stall_at => (a as u32) << 8 | u32::from(self.phys[a]),
            REG_EFUSE_CTRL => EF_FLAG | (a as u32) << 8 | u32::from(self.phys[a]),
            // Every bit set, the address's bytes among them: only the bank and
            // LDO bits may be kept.
            REG_LDO_EFUSE_CTRL => u32::from_le_bytes([FACTORY[0], 0xFF, FACTORY[2], 0xFF]),
            _ => 0,
        }
    }
    fn write32(&self, off: usize, val: u32) {
        if off == REG_EFUSE_CTRL {
            self.addr.set(((val >> 8) & 0x3FF) as usize);
        }
    }
}

/// Whether `reply` holds the factory address anywhere, whole or any byte of
/// it in the words that are not counters or the stalled address.
pub(super) fn carries_factory(reply: &[u8]) -> bool {
    let fixed = [25..29, 33..45];
    reply.windows(6).any(|w| w == FACTORY)
        || fixed.iter().any(|r| reply[r.clone()].iter().any(|b| FACTORY.contains(b)))
}
