// NONOS Operating System (AGPL-3.0-or-later)
//! An SSP core modelled from the fact sheet, not from any driver: frames
//! written to SSDR go to a device that answers each with a byte of its own
//! after `latency` status reads; RNE and TNF follow the FIFOs, BSY while a
//! frame is in flight, ROR when more answers come than the FIFO holds.

use std::cell::RefCell;
use std::collections::VecDeque;

use crate::core::Regs32;
use crate::facts::{SSDR, SSSR, SSSR_ROR};

pub struct Ssp {
    pub inner: RefCell<Inner>,
}

pub struct Inner {
    pub sent: Vec<u8>,
    pub rx: VecDeque<u8>,
    pub flight: VecDeque<(u8, u32)>,
    pub answer: Box<dyn FnMut(u8) -> u8>,
    pub latency: u32,
    pub ror: bool,
    pub depth: usize,
    /// A core whose receive flag never clears.
    pub stuck_rne: bool,
}

impl Ssp {
    pub fn new(answer: impl FnMut(u8) -> u8 + 'static, latency: u32) -> Self {
        let i = Inner {
            sent: vec![],
            rx: VecDeque::new(),
            flight: VecDeque::new(),
            answer: Box::new(answer),
            latency,
            ror: false,
            depth: 16,
            stuck_rne: false,
        };
        Self { inner: RefCell::new(i) }
    }
}

impl Regs32 for Ssp {
    fn read32(&self, off: u64) -> u32 {
        let mut s = self.inner.borrow_mut();
        match off {
            SSSR => s.status(),
            SSDR => s.rx.pop_front().unwrap_or(0) as u32,
            _ => 0,
        }
    }

    fn write32(&self, off: u64, v: u32) {
        let mut s = self.inner.borrow_mut();
        match off {
            SSDR => {
                let b = v as u8;
                s.sent.push(b);
                let reply = (s.answer)(b);
                let lat = s.latency.max(1);
                s.flight.push_back((reply, lat));
            }
            SSSR if v & SSSR_ROR != 0 => s.ror = false,
            _ => {}
        }
    }
}
