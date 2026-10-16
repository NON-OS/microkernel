// NONOS Operating System (AGPL-3.0-or-later)
//! A modeled 8821C whose last driver died with its DMA engines armed. Its
//! power-off takes (REG_CR reads 0xEA, the engines disarmed) only when
//! `powers_off`; its events go in `log`, so their order shows.

use core::cell::{Cell, RefCell};

use crate::pwr_mock::Chip;
use crate::regs::Mmio;

pub struct Armed {
    pub chip: Chip,
    pub powers_off: bool,
    pub armed: Cell<bool>,
    pub log: RefCell<Vec<&'static str>>,
}

/// A warm card (REG_CR powered) or a cold one (0xEA), answering its chip id.
pub fn armed(warm: bool, powers_off: bool) -> Armed {
    let chip = Chip::new(if warm { 0x00 } else { 0xEA });
    chip.regs.borrow_mut()[0xFC] = 0x21; // REG_SYS_CFG1: alive
    Armed { chip, powers_off, armed: Cell::new(warm), log: RefCell::new(Vec::new()) }
}

impl Mmio for Armed {
    fn read8(&self, off: usize) -> u8 {
        self.chip.read8(off)
    }
    fn write8(&self, off: usize, val: u8) {
        self.chip.write8(off, val);
        // Card emulation to card disable: 0x05 bit 2.
        if off == 0x05 && val & 0x04 != 0 && self.powers_off && self.armed.replace(false) {
            self.chip.regs.borrow_mut()[0x100] = 0xEA;
            self.log.borrow_mut().push("powered_off");
        }
    }
    fn read16(&self, off: usize) -> u16 {
        self.chip.read16(off)
    }
    fn write16(&self, off: usize, val: u16) {
        for (i, b) in val.to_le_bytes().iter().enumerate() {
            self.write8(off + i, *b);
        }
    }
    fn read32(&self, off: usize) -> u32 {
        self.chip.read32(off)
    }
    fn write32(&self, off: usize, val: u32) {
        for (i, b) in val.to_le_bytes().iter().enumerate() {
            self.write8(off + i, *b);
        }
    }
}
