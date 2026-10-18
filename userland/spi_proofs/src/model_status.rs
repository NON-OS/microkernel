// NONOS Operating System (AGPL-3.0-or-later)
//! The model's status register: each read is a tick, on which frames in
//! flight land in the receive FIFO, or overrun it when it is full.

use crate::facts::{SSSR_BSY, SSSR_RNE, SSSR_ROR, SSSR_TNF};
use crate::model::Inner;

impl Inner {
    pub fn status(&mut self) -> u32 {
        for f in self.flight.iter_mut() {
            f.1 = f.1.saturating_sub(1);
        }
        while self.flight.front().is_some_and(|f| f.1 == 0) {
            let (b, _) = self.flight.pop_front().unwrap();
            if self.rx.len() >= self.depth {
                self.ror = true;
            } else {
                self.rx.push_back(b);
            }
        }
        let mut v = 0;
        if self.flight.len() < self.depth {
            v |= SSSR_TNF;
        }
        if !self.rx.is_empty() || self.stuck_rne {
            v |= SSSR_RNE;
        }
        if !self.flight.is_empty() {
            v |= SSSR_BSY;
        }
        if self.ror {
            v |= SSSR_ROR;
        }
        v
    }
}
