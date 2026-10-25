// NONOS Operating System (AGPL-3.0-or-later)
//! A modeled device whose DMA engines a predecessor left armed, logging its
//! own events among the broker calls (`note`) so their order shows.

use std::cell::Cell;

use nonos_libc::note;

use crate::gen3::regs::{CSR_RESET, RESET_MASTER_DISABLED, RESET_STOP_MASTER, RESET_SW_RESET};
use crate::regs::Mmio;

/// A device left with its DMA engines armed: they stop `stops_after` reads
/// after the stop is asked (never, with `None`), and only a reset disarms it.
pub struct Armed {
    reset: Cell<u32>,
    reads: Cell<u32>,
    stops_after: Option<u32>,
    pub armed: Cell<bool>,
}

impl Mmio for Armed {
    fn read32(&self, off: usize) -> u32 {
        let asked = self.reset.get() & RESET_STOP_MASTER != 0;
        if off == CSR_RESET && asked && self.reset.get() & RESET_MASTER_DISABLED == 0 {
            self.reads.set(self.reads.get() + 1);
            if self.stops_after.is_some_and(|k| self.reads.get() >= k) {
                self.reset.set(self.reset.get() | RESET_MASTER_DISABLED);
                note("master_stopped", 0);
            }
        }
        if off == CSR_RESET {
            self.reset.get()
        } else {
            0
        }
    }
    fn write32(&self, off: usize, val: u32) {
        if off != CSR_RESET {
            return;
        }
        if val & RESET_STOP_MASTER != 0 && self.reset.get() & RESET_STOP_MASTER == 0 {
            note("stop_master", 0);
        }
        if val & RESET_SW_RESET != 0 && self.armed.replace(false) {
            note("sw_reset", 0);
        }
        self.reset.set(val);
    }
}

pub fn armed(stops_after: Option<u32>) -> Armed {
    Armed { reset: Cell::new(0), reads: Cell::new(0), stops_after, armed: Cell::new(true) }
}
