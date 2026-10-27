// NONOS Operating System (AGPL-3.0-or-later)

//! The WFDMA's transmit side as the model runs it. A transmit ring's
//! `cpu_idx` write sends every descriptor from `dma_idx` up to it: the
//! buffer is read from the driver's memory, the descriptor marked done,
//! `dma_idx` moved, and the buffer handed on (ring 17 to the MCU, 16 to
//! the download, 0 to the air).

use super::chip::Chip;
use super::mem::{read_mem, word, write_mem};

pub const TX_BASE: u32 = 0xd4300;
pub const RX_BASE: u32 = 0xd4500;
pub const DONE: u32 = 1 << 31;
pub const LAST: u32 = 1 << 30;

impl Chip {
    pub(super) fn ring_write(&mut self, off: u32) {
        if (TX_BASE..TX_BASE + 18 * 16).contains(&off) && off % 16 == 8 {
            self.run_tx(((off - TX_BASE) / 16) as usize);
        }
    }

    fn run_tx(&mut self, ring: usize) {
        let r = TX_BASE + 16 * ring as u32;
        let (base, size) = (self.reg(r), self.reg(r + 4).max(1));
        while self.reg(r + 12) != self.reg(r + 8) {
            let at = self.reg(r + 12);
            let d = base + 16 * at;
            let (buf, ctrl) = (word(d), word(d + 4));
            let bytes = read_mem(buf, (ctrl >> 16 & 0x3fff) as usize).unwrap_or_default();
            write_mem(d + 4, &(ctrl | DONE).to_le_bytes());
            self.regs.insert(r + 12, (at + 1) % size);
            match ring {
                17 => self.command(&bytes),
                16 => self.mcu.scatter(&bytes),
                0 => self.transmit(&bytes),
                _ => {}
            }
        }
    }
}
