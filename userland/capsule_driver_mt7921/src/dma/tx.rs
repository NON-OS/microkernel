/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Transmit on a host ring: a single-segment buffer per descriptor, as
//! `mt76_dma_tx_queue_skb_raw` queues an MCU command or a firmware chunk
//! and as the data ring carries each frame's TXD and TXP. A slot is the
//! host's again once the WFDMA's `dma_idx` has passed it
//! (`mt76_dma_tx_cleanup`).

use super::desc::{ctrl_one, BUF0, BUF1, CTRL, DESC_SIZE, INFO};
use super::ring::{Ring, REG_DMA_IDX};
use crate::hw::Regs;

impl Ring {
    /// Give back the slots the WFDMA has read.
    pub fn reclaim(&mut self, regs: &Regs) {
        let done = regs.rr(self.reg + REG_DMA_IDX);
        if done >= self.n as u32 {
            return; // a gone card reads all ones: nothing is trusted from it
        }
        while self.queued > 0 && self.tail != done as u16 {
            self.tail = self.next(self.tail);
            self.queued -= 1;
        }
    }

    /// Whether one more buffer fits: Linux keeps two slots free
    /// (`q->queued + 1 >= q->ndesc - 1` refuses).
    pub fn room(&self) -> bool {
        self.queued + 1 < self.n - 1
    }

    /// Copy `data` into the next slot and describe it; the slot index, or
    /// `None` when the ring is full or `data` does not fit a slot. Nothing
    /// is sent until `kick`.
    pub fn push(&mut self, data: &[u8]) -> Option<u16> {
        if !self.room() || data.is_empty() || data.len() > self.stride {
            return None;
        }
        let i = self.head;
        let off = i as usize * self.stride;
        self.bufs.write(off, data);
        self.bufs.fill(off + data.len(), self.stride - data.len(), 0);
        let d = i as usize * DESC_SIZE;
        self.desc.wr32(d + BUF0, self.slot_dev(i));
        self.desc.wr32(d + BUF1, 0);
        self.desc.wr32(d + INFO, 0);
        self.desc.wr32(d + CTRL, ctrl_one(data.len()));
        self.head = self.next(i);
        self.queued += 1;
        Some(i)
    }

    /// Wait up to `ms` for a free slot, reclaiming as the WFDMA reads.
    pub fn wait_room(&mut self, regs: &Regs, ms: u64) -> bool {
        crate::clock::until_ms(ms, || {
            self.reclaim(regs);
            self.room()
        })
    }
}
