/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Chip-address register access: `mt7921_rr`, `mt7921_wr`, `mt7921_rmw`
//! and `mt7921_reg_map_l1` of Linux v6.12 `mt7921/pci.c` and `mt7921.h`.
//! An address no window reaches is not accessed at all: it reads as all
//! ones and a write to it is dropped (Linux logs it and touches offset 0).

use alloc::boxed::Box;

use super::addr::{HIF_REMAP_BASE_L1, HIF_REMAP_L1};
use super::bus::Bus;
use super::map::{fixed, remapped};

pub struct Regs {
    bus: Box<dyn Bus>,
}

impl Regs {
    pub fn new(bus: Box<dyn Bus>) -> Self {
        Self { bus }
    }

    /// The BAR offset of `addr`, opening the L1 window onto it first when
    /// that is how it is reached: the window's base is the address's upper
    /// half, written into the remap register's low half and read back to
    /// push the write, as `mt7921_reg_map_l1` does.
    fn offset(&self, addr: u32) -> Option<u32> {
        if let Some(off) = fixed(addr) {
            return Some(off);
        }
        if !remapped(addr) {
            return None;
        }
        let cur = self.bus.read32(HIF_REMAP_L1);
        self.bus.write32(HIF_REMAP_L1, (cur & !0xffff) | (addr >> 16));
        let _ = self.bus.read32(HIF_REMAP_L1);
        Some(HIF_REMAP_BASE_L1 + (addr & 0xffff))
    }

    pub fn rr(&self, addr: u32) -> u32 {
        self.offset(addr).map_or(u32::MAX, |off| self.bus.read32(off))
    }

    pub fn wr(&self, addr: u32, val: u32) {
        if let Some(off) = self.offset(addr) {
            self.bus.write32(off, val);
        }
    }

    /// `mt76_rmw`: the bits of `mask` replaced by `val`. The value written.
    pub fn rmw(&self, addr: u32, mask: u32, val: u32) -> u32 {
        let v = (self.rr(addr) & !mask) | val;
        self.wr(addr, v);
        v
    }

    pub fn set(&self, addr: u32, bits: u32) {
        self.rmw(addr, 0, bits);
    }

    pub fn clear(&self, addr: u32, bits: u32) {
        self.rmw(addr, bits, 0);
    }

    /// `mt76_rmw_field`: `val` shifted into the contiguous field `mask`.
    pub fn rmw_field(&self, addr: u32, mask: u32, val: u32) {
        self.rmw(addr, mask, (val << mask.trailing_zeros()) & mask);
    }
}
