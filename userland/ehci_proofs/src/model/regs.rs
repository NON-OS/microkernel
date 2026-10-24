// NONOS Operating System (AGPL-3.0-or-later)
//! The registers as the controller answers them: capability registers at
//! BAR0 (no 64-bit addressing, no legacy capability), and the operational
//! ones at CAPLEN: run and halt, a host reset, the schedules' status
//! following their enables, the doorbell answered at the next frame, and
//! write-one-to-clear status.

use super::world::{World, CAPLEN, PORTS};
use crate::facts::port::PORTSC;
use crate::facts::regs::*;

impl World {
    pub fn read32(&mut self, off: u32) -> u32 {
        let port = |o: u32| (CAPLEN + PORTSC..CAPLEN + PORTSC + 4 * PORTS as u32).contains(&o);
        match off {
            CAPLENGTH => CAPLEN | 0x0100 << 16,
            HCSPARAMS => PORTS as u32 | HCS_PPC,
            HCCPARAMS => 0,
            o if o == CAPLEN + USBCMD => self.cmd,
            o if o == CAPLEN + USBSTS => self.sts,
            o if o == CAPLEN + FRINDEX => self.frindex,
            o if o == CAPLEN + CONFIGFLAG => self.configflag,
            o if port(o) => self.port_read(((o - CAPLEN - PORTSC) / 4) as usize),
            _ => 0,
        }
    }

    pub fn write32(&mut self, off: u32, v: u32) {
        let port = |o: u32| (CAPLEN + PORTSC..CAPLEN + PORTSC + 4 * PORTS as u32).contains(&o);
        match off {
            o if o == CAPLEN + USBCMD => self.command(v),
            o if o == CAPLEN + USBSTS => self.sts &= !(v & (STS_ALL | STS_IAA)),
            o if o == CAPLEN + PERIODICLISTBASE => self.periodic = v & !0xFFF,
            o if o == CAPLEN + ASYNCLISTADDR => self.async_addr = v & !0x1F,
            o if o == CAPLEN + CONFIGFLAG => self.configflag = v,
            o if port(o) => self.port_write(((o - CAPLEN - PORTSC) / 4) as usize, v),
            _ => {}
        }
    }

    fn command(&mut self, v: u32) {
        if v & CMD_HCRESET != 0 {
            if self.hcreset_stuck {
                self.cmd |= CMD_HCRESET;
                return;
            }
            (self.cmd, self.sts, self.configflag) = (0, STS_HALTED, 0);
            (self.periodic, self.async_addr) = (0, 0);
            return;
        }
        self.cmd = v;
        self.doorbell |= v & CMD_IAAD != 0;
        if v & CMD_RS != 0 {
            self.sts &= !STS_HALTED;
        } else {
            self.sts |= STS_HALTED;
        }
        let on = |bit: u32, s: u32| if v & bit != 0 && v & CMD_RS != 0 { s } else { 0 };
        self.sts = self.sts & !(STS_PSS | STS_ASS) | on(CMD_PSE, STS_PSS) | on(CMD_ASE, STS_ASS);
    }
}
