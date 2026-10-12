// NONOS Operating System (AGPL-3.0-or-later)
//! The root ports: a device connected once its port is powered; its line
//! in K-state while a low-speed device sits there unreset; a reset the
//! driver ends (PR written back to 0) that enables the port only for a
//! high-speed device; the owner bit handing it to the companion, after
//! which this controller sees nothing there.

use super::world::{Speed, World};
use crate::facts::port::*;

impl World {
    fn present(&self, i: usize) -> bool {
        self.devs[i].is_some() && self.ports[i] & PORT_PP != 0 && self.ports[i] & PORT_OWNER == 0
    }

    pub fn port_read(&mut self, i: usize) -> u32 {
        let mut v = self.ports[i];
        if self.present(i) {
            v |= PORT_CCS;
            if self.speed[i] == Speed::Low && v & PORT_PE == 0 {
                v |= LS_K << PORT_LS_SHIFT;
            }
        }
        v
    }

    pub fn port_write(&mut self, i: usize, v: u32) {
        let was = self.ports[i];
        let keep = PORT_PE | PORT_PR | PORT_PP | PORT_OWNER;
        let mut p = (was & !(v & PORT_WC)) & (PORT_WC | keep);
        p = p & !(PORT_PP | PORT_OWNER) | v & (PORT_PP | PORT_OWNER);
        if v & PORT_PE == 0 {
            p &= !PORT_PE;
        }
        if v & PORT_OWNER != 0 && was & PORT_OWNER == 0 {
            self.handed[i] += 1;
            p &= !PORT_PE;
        }
        if v & PORT_PR != 0 && was & PORT_PR == 0 {
            self.resets[i] += 1;
            p = (p | PORT_PR) & !PORT_PE;
        } else if v & PORT_PR == 0 && was & PORT_PR != 0 {
            p &= !PORT_PR;
            if self.present(i) && self.speed[i] == Speed::High {
                p |= PORT_PE;
            }
            if let Some(d) = self.devs[i].as_mut() {
                d.reset();
            }
        }
        self.ports[i] = p;
    }
}
