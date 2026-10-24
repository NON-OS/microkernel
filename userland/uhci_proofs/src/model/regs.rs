// NONOS Operating System (AGPL-3.0-or-later)
//! The I/O registers as the controller answers them: USBCMD's resets and
//! run bit, USBSTS's write-to-clear bits, and PORTSC's reset, enable and
//! change bits. PORTSC bit 7 reads as one, as recalled of real parts.

use super::world::World;
use crate::facts::io::*;

const PORT_ALWAYS: u16 = 1 << 7;

impl World {
    pub fn read16(&mut self, off: u16) -> u16 {
        match off {
            USBCMD => self.cmd,
            USBSTS => self.sts,
            USBINTR => self.intr,
            FRNUM => self.frnum,
            o if (PORTSC1..PORTSC1 + 4).contains(&o) => self.port((o - PORTSC1) as usize / 2),
            _ => 0xFFFF,
        }
    }

    fn port(&self, i: usize) -> u16 {
        let low = self.devs[i].as_ref().is_some_and(|d| d.low_speed);
        self.portsc[i] | PORT_ALWAYS | if low { PORT_LSDA } else { 0 }
    }

    pub fn write16(&mut self, off: u16, v: u16) {
        match off {
            USBCMD => self.command(v),
            USBSTS => self.sts &= !(v & STS_ALL & !STS_HCH),
            USBINTR => self.intr = v,
            FRNUM => self.frnum = v & 0x7FF,
            o if (PORTSC1..PORTSC1 + 4).contains(&o) => {
                self.portsc_write((o - PORTSC1) as usize / 2, v)
            }
            _ => {}
        }
    }

    pub fn write32(&mut self, off: u16, v: u32) {
        if off == FLBASEADD {
            self.flbase = v & !0xFFF;
        }
    }

    fn command(&mut self, v: u16) {
        if v & CMD_HCRESET != 0 && !self.hcreset_stuck {
            let ports = self.portsc.map(|p| p & !(PORT_PE | PORT_PR));
            let (devs, mem, ms) =
                (core::mem::take(&mut self.devs), core::mem::take(&mut self.mem), self.ms);
            *self = World { devs, mem, ms, portsc: ports, ..World::default() };
            return;
        }
        self.cmd = v;
        if v & CMD_RS != 0 {
            self.sts &= !STS_HCH
        } else {
            self.sts |= STS_HCH
        }
    }
}
