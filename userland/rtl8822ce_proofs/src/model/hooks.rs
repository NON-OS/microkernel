// NONOS Operating System (AGPL-3.0-or-later)
//! The registers that answer: the efuse controller (a byte read sets the
//! done flag with the byte), the DBI flag and the LLT init bit (clear as
//! soon as set), the LTE-coex window (ready unless dead), and the DAC
//! calibration's sample register (whatever the test feeds it).

use std::cell::{Cell, RefCell};

use super::Chip;

#[derive(Default)]
pub struct Hw {
    /// The physical efuse, when the chip has one to read.
    pub efuse: RefCell<Option<Vec<u8>>>,
    /// The efuse controller never finishes a read.
    pub efuse_stuck: Cell<bool>,
    pub dbi_stuck: Cell<bool>,
    pub lte_dead: Cell<bool>,
    /// Feeds the DAC sample register (0x2DBC); counts each read.
    pub sample: RefCell<Option<Box<dyn FnMut() -> u32>>>,
    pub sample_reads: Cell<u32>,
}

const REG_EFUSE_CTRL: usize = 0x0030;
const REG_DBI_FLAG_B2: usize = 0x03F2;
const REG_AUTO_LLT: usize = 0x0208;
const LTE_CTRL: usize = 0x1700;
const DAC_SAMPLE: usize = 0x2DBC;

impl Hw {
    pub fn read(&self, chip: &Chip, off: usize, n: usize) -> Option<u32> {
        if off == DAC_SAMPLE && n == 4 {
            if let Some(f) = self.sample.borrow_mut().as_mut() {
                self.sample_reads.set(self.sample_reads.get() + 1);
                return Some(f());
            }
        }
        if off == LTE_CTRL && n == 4 && !self.lte_dead.get() {
            return Some(chip.get32(LTE_CTRL) | 1 << 29);
        }
        None
    }

    pub fn wrote(&self, chip: &Chip, off: usize, n: usize, v: u32) {
        if off == REG_EFUSE_CTRL && n == 4 && v & 1 << 31 == 0 && !self.efuse_stuck.get() {
            if let Some(map) = self.efuse.borrow().as_ref() {
                let addr = ((v >> 8) & 0x3FF) as usize;
                let byte = u32::from(*map.get(addr).unwrap_or(&0xFF));
                chip.set32(REG_EFUSE_CTRL, (v & !0xFF) | 1 << 31 | byte);
            }
        }
        if off == REG_DBI_FLAG_B2 && n == 1 && !self.dbi_stuck.get() {
            chip.mem.borrow_mut().insert(REG_DBI_FLAG_B2, 0);
        }
        if off == REG_AUTO_LLT && n == 1 {
            chip.mem.borrow_mut().insert(REG_AUTO_LLT, (v & !1) as u8);
        }
    }
}
