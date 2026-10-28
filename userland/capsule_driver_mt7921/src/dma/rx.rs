/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Receive on a host ring, as `mt76_dma_rx_fill` and `mt76_dma_dequeue`
//! keep one: every slot but one armed with its buffer, a completed slot
//! taken at `tail` once the WFDMA set its done bit, cleared and re-armed.
//! A frame spread over several buffers is not joined: its pieces are
//! taken and dropped, as Linux drops one longer than its buffer.

use super::desc::{rx_done, BUF0, BUF1, CTL_DMA_DONE, CTL_SD_LEN0_SHIFT, CTRL, DESC_SIZE, INFO};
use super::ring::Ring;
use crate::hw::Regs;

pub enum Rx {
    /// A whole buffer: its length, copied into the caller's slice.
    Frame(usize),
    /// A completed buffer that is dropped, a piece of a longer frame or one
    /// the DMA marked.
    Dropped,
}

impl Ring {
    /// Arm every free slot and hand them to the WFDMA.
    pub fn fill(&mut self, regs: &Regs) {
        while self.queued < self.n - 1 {
            let i = self.head;
            self.bufs.fill(i as usize * self.stride, self.stride, 0);
            let d = i as usize * DESC_SIZE;
            self.desc.wr32(d + BUF0, self.slot_dev(i));
            self.desc.wr32(d + BUF1, 0);
            self.desc.wr32(d + INFO, 0);
            self.desc.wr32(d + CTRL, (self.stride.min(0x3fff) as u32) << CTL_SD_LEN0_SHIFT);
            self.head = self.next(i);
            self.queued += 1;
        }
        self.kick(regs);
    }

    /// The buffer at `tail`, if the WFDMA has finished it, copied into
    /// `out` (cut to it); the slot is then re-armed.
    pub fn take(&mut self, regs: &Regs, out: &mut [u8]) -> Option<Rx> {
        let d = self.tail as usize * DESC_SIZE;
        let ctrl = self.desc.rd32(d + CTRL);
        if self.queued == 0 || ctrl & CTL_DMA_DONE == 0 {
            return None;
        }
        let (buf1, info) = (self.desc.rd32(d + BUF1), self.desc.rd32(d + INFO));
        let (len, more, drop) = rx_done(ctrl, buf1, info);
        let piece = core::mem::replace(&mut self.skip, more);
        let whole = !piece && !more && !drop && len <= self.stride && len <= out.len();
        if whole {
            self.bufs.read(self.tail as usize * self.stride, &mut out[..len]);
        }
        self.tail = self.next(self.tail);
        self.queued -= 1;
        self.fill(regs);
        Some(if whole { Rx::Frame(len) } else { Rx::Dropped })
    }
}
