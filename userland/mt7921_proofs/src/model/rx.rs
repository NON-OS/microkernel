// NONOS Operating System (AGPL-3.0-or-later)

//! The WFDMA's receive side as the model runs it. A buffer waits until the
//! driver has armed a slot at `dma_idx`; it is written there, done set, and
//! last set unless it runs on into the next slot; `dma_idx` moves. Every
//! register access is a moment of time on the air, and a chance for what
//! waits to be written.

use nonos_libc::now;

use super::chip::Chip;
use super::mem::{word, write_mem};
use super::rings::{DONE, LAST, RX_BASE};

impl Chip {
    /// Queue `bytes` for receive ring `ring`, and write what fits now.
    pub fn push_rx(&mut self, ring: usize, bytes: Vec<u8>) {
        self.pending[ring].push_back(bytes);
        self.flush(ring);
    }

    fn flush(&mut self, ring: usize) {
        let r = RX_BASE + 16 * ring as u32;
        while let Some(mut bytes) = self.pending[ring].pop_front() {
            let (base, size, dma) = (self.reg(r), self.reg(r + 4), self.reg(r + 12));
            let d = base + 16 * dma;
            let room = (word(d + 4) >> 16 & 0x3fff) as usize;
            if size == 0 || dma == self.reg(r + 8) || room == 0 {
                self.pending[ring].push_front(bytes);
                return;
            }
            let rest = bytes.split_off(bytes.len().min(room));
            write_mem(word(d), &bytes);
            let last = if rest.is_empty() { LAST } else { 0 };
            write_mem(d + 4, &((bytes.len() as u32) << 16 | last | DONE).to_le_bytes());
            self.regs.insert(r + 12, (dma + 1) % size);
            if !rest.is_empty() {
                self.pending[ring].push_front(rest);
            }
        }
    }

    /// Time on the air, and every waiting buffer written where it fits.
    pub fn tick(&mut self) {
        let tuned = self.mcu.tuned();
        if let Some(frames) = self.air.as_mut().map(|a| a.tick(now(), tuned)) {
            for f in frames {
                self.receive(f);
            }
        }
        for ring in 0..self.pending.len() {
            self.flush(ring);
        }
    }
}
