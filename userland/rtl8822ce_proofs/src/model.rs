// NONOS Operating System (AGPL-3.0-or-later)
//! A modelled RTL8822CE register file: bytes that read back what was
//! written, every write logged with its width, and the few registers whose
//! hardware answers (`hooks`): the efuse read-out, the DBI and LLT flags
//! that clear, the LTE-coex window's ready bit, and the DAC sample register.

pub mod hooks;
mod mmio;

use std::cell::RefCell;
use std::collections::HashMap;

/// A write: offset, width in bytes, value.
pub type Write = (usize, usize, u32);

#[derive(Default)]
pub struct Chip {
    pub mem: RefCell<HashMap<usize, u8>>,
    pub writes: RefCell<Vec<Write>>,
    pub hw: hooks::Hw,
}

impl Chip {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set32(&self, off: usize, v: u32) {
        let mut m = self.mem.borrow_mut();
        for (i, b) in v.to_le_bytes().iter().enumerate() {
            m.insert(off + i, *b);
        }
    }
    pub fn get32(&self, off: usize) -> u32 {
        let m = self.mem.borrow();
        u32::from_le_bytes(core::array::from_fn(|i| *m.get(&(off + i)).unwrap_or(&0)))
    }
    /// The writes to `off`, in order.
    pub fn at(&self, off: usize) -> Vec<u32> {
        self.writes.borrow().iter().filter(|w| w.0 == off).map(|w| w.2).collect()
    }
    /// Where in the log the first write to `off` is.
    pub fn first(&self, off: usize) -> Option<usize> {
        self.writes.borrow().iter().position(|w| w.0 == off)
    }
    pub(super) fn get(&self, off: usize, n: usize) -> u32 {
        if let Some(v) = self.hw.read(self, off, n) {
            return v;
        }
        let m = self.mem.borrow();
        (0..n).fold(0, |v, i| v | u32::from(*m.get(&(off + i)).unwrap_or(&0)) << (8 * i))
    }
    pub(super) fn put(&self, off: usize, n: usize, v: u32) {
        self.writes.borrow_mut().push((off, n, v));
        {
            let mut m = self.mem.borrow_mut();
            for i in 0..n {
                m.insert(off + i, (v >> (8 * i)) as u8);
            }
        }
        self.hw.wrote(self, off, n, v);
    }
}
