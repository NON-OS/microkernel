// NONOS Operating System (AGPL-3.0-or-later)
//! The root ports: connected once powered with a device there, a reset the
//! hub times (10 ms, as recalled) that enables the port and resets the
//! device, low bits written as commands and change bits cleared by a one.

use super::world::World;
use crate::facts::port::*;

const PORT_PRS: u32 = 1 << 4;
const PORT_PPS: u32 = 1 << 8;
const SET_ENABLE: u32 = 1 << 1;
const CHANGES: u32 = 0x1F << 16;

impl World {
    fn connected(&self, i: usize) -> bool {
        self.devs[i].is_some() && (self.powered || self.ports[i] & PORT_PPS != 0)
    }

    pub fn port_read(&mut self, i: usize) -> u32 {
        let mut v = self.ports[i];
        if self.connected(i) {
            v |= PORT_CCS;
            if self.devs[i].as_ref().unwrap().low_speed {
                v |= PORT_LSDA;
            }
        }
        if self.powered {
            v |= PORT_PPS;
        }
        v
    }

    pub fn port_write(&mut self, i: usize, v: u32) {
        let p = &mut self.ports[i];
        *p &= !(v & CHANGES);
        if v & PORT_CLEAR_ENABLE != 0 {
            *p &= !PORT_PES;
        }
        if v & PORT_SET_POWER != 0 {
            *p |= PORT_PPS;
        }
        let connected = self.connected(i);
        let p = &mut self.ports[i];
        if v & SET_ENABLE != 0 && connected {
            *p |= PORT_PES;
        }
        if v & PORT_SET_RESET != 0 && connected {
            *p = (*p | PORT_PRS) & !PORT_PES;
            self.reset_left[i] = 10;
        }
    }

    pub fn port_tick(&mut self) {
        for i in 0..self.ports.len() {
            if self.reset_left[i] == 0 {
                continue;
            }
            self.reset_left[i] -= 1;
            if self.reset_left[i] == 0 {
                self.ports[i] = (self.ports[i] & !PORT_PRS) | PORT_PES | PORT_PRSC;
                if let Some(d) = self.devs[i].as_mut() {
                    d.reset();
                }
            }
        }
    }
}
