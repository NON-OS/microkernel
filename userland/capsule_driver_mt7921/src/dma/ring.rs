/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! One host ring: its descriptor array, a buffer per slot, and the four
//! registers the WFDMA keeps for it, `desc_base`, `ring_size`, `cpu_idx`,
//! `dma_idx` (`struct mt76_queue_regs`). Reset as Linux v6.12 `mt76/dma.c`
//! `__mt76_dma_queue_reset` and `mt76_dma_sync_idx` do.

use super::desc::{CTL_DMA_DONE, CTRL, DESC_SIZE};
use super::grant::Grant;
use crate::hw::Regs;

pub const REG_BASE: u32 = 0;
pub const REG_SIZE: u32 = 4;
pub const REG_CPU_IDX: u32 = 8;
pub const REG_DMA_IDX: u32 = 12;

pub struct Ring {
    pub(super) desc: Grant,
    pub(super) bufs: Grant,
    pub(super) stride: usize,
    pub(super) n: u16,
    /// The address of this ring's first register.
    pub(super) reg: u32,
    pub(super) head: u16,
    pub(super) tail: u16,
    pub(super) queued: u16,
    /// Inside a frame longer than one buffer: its pieces are dropped.
    pub(super) skip: bool,
}

impl Ring {
    /// A ring of `n` slots of `stride` bytes; `None` when the grants are too
    /// small for them or `n` is under two (a ring keeps one slot empty).
    pub fn new(desc: Grant, bufs: Grant, stride: usize, n: u16, reg: u32) -> Option<Ring> {
        let fits = desc.size() >= n as usize * DESC_SIZE && bufs.size() >= n as usize * stride;
        let (head, tail, queued, skip) = (0, 0, 0, false);
        (fits && n >= 2).then_some(Ring { desc, bufs, stride, n, reg, head, tail, queued, skip })
    }

    /// Every descriptor marked done (the host's), both indices zeroed, the
    /// array and its size given to the WFDMA, and the host's view taken from
    /// the index the WFDMA reads back.
    pub fn reset(&mut self, regs: &Regs) {
        for i in 0..self.n as usize {
            self.desc.fill(i * DESC_SIZE, DESC_SIZE, 0);
            self.desc.wr32(i * DESC_SIZE + CTRL, CTL_DMA_DONE);
        }
        regs.wr(self.reg + REG_CPU_IDX, 0);
        regs.wr(self.reg + REG_DMA_IDX, 0);
        regs.wr(self.reg + REG_BASE, self.desc.dev(0));
        regs.wr(self.reg + REG_SIZE, self.n as u32);
        let at = (regs.rr(self.reg + REG_DMA_IDX) % self.n as u32) as u16;
        (self.head, self.tail, self.queued, self.skip) = (at, at, 0, false);
    }

    /// Hand the WFDMA everything up to `head` (`mt76_dma_kick_queue`).
    pub fn kick(&self, regs: &Regs) {
        regs.wr(self.reg + REG_CPU_IDX, self.head as u32);
    }

    pub(super) fn next(&self, i: u16) -> u16 {
        (i + 1) % self.n
    }

    /// Slot `i`'s buffer, as the chip addresses it.
    pub(super) fn slot_dev(&self, i: u16) -> u32 {
        self.bufs.dev(i as usize * self.stride)
    }
}
