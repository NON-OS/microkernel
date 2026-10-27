// NONOS Operating System (AGPL-3.0-or-later)

//! DMA regions as plain memory with a device address, and a clock that
//! re-checks a condition a fixed number of times.

use std::cell::RefCell;
use std::rc::Rc;

use crate::gen3::region::{Clock, Region};

pub struct Mem {
    pub bytes: RefCell<Vec<u8>>,
    pub dev: u64,
}

impl Mem {
    pub fn new(len: usize, dev: u64) -> Rc<Self> {
        Rc::new(Self { bytes: RefCell::new(vec![0; len]), dev })
    }
}

impl Region for Mem {
    fn len(&self) -> usize {
        self.bytes.borrow().len()
    }
    fn dev(&self) -> u64 {
        self.dev
    }
    fn write(&self, off: usize, src: &[u8]) -> bool {
        let mut b = self.bytes.borrow_mut();
        match off.checked_add(src.len()) {
            Some(end) if end <= b.len() => {
                b[off..end].copy_from_slice(src);
                true
            }
            _ => false,
        }
    }
    fn read(&self, off: usize, dst: &mut [u8]) -> bool {
        let b = self.bytes.borrow();
        match off.checked_add(dst.len()) {
            Some(end) if end <= b.len() => {
                dst.copy_from_slice(&b[off..end]);
                true
            }
            _ => false,
        }
    }
}

/// A clock that re-checks a condition a fixed number of times. Its time is
/// `delays_us`: a settle adds its length, a wait that runs out adds the whole
/// wait, and a wait that is answered adds a millisecond.
pub struct Polls {
    pub per_wait: u32,
    pub delays_us: u64,
}

impl Clock for Polls {
    fn poll_for(&mut self, ms: u32, ready: &mut dyn FnMut() -> bool) -> bool {
        let met = (0..self.per_wait).any(|_| ready());
        self.delays_us += if met { 1000 } else { u64::from(ms) * 1000 };
        met
    }
    // The model has no CPU to keep busy: a sleepless wait is the same wait.
    fn spin_for(&mut self, ms: u32, ready: &mut dyn FnMut() -> bool) -> bool {
        self.poll_for(ms, ready)
    }
    fn delay_us(&mut self, us: u32) {
        self.delays_us += us as u64;
    }
    fn now_ms(&mut self) -> u64 {
        self.delays_us / 1000
    }
}
