// NONOS Operating System (AGPL-3.0-or-later)
//! A root port's PORTSC written: the change bits cleared by a one, a reset
//! begun and, when PR is written back to zero, ended (the device back at
//! address 0), and the port enabled or disabled.

use super::world::World;
use crate::facts::io::*;

impl World {
    pub(super) fn portsc_write(&mut self, i: usize, v: u16) {
        let mut p = self.portsc[i] & !(v & PORT_WC);
        if v & PORT_PR != 0 {
            p = (p | PORT_PR) & !PORT_PE;
        } else if p & PORT_PR != 0 {
            p &= !PORT_PR;
            if let Some(d) = self.devs[i].as_mut() {
                d.reset();
            }
        } else if v & PORT_PE != 0 && p & PORT_CCS != 0 {
            p |= PORT_PE;
        } else if v & PORT_PE == 0 {
            p &= !PORT_PE;
        }
        self.portsc[i] = p;
    }
}
