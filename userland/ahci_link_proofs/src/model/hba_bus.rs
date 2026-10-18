// NONOS Operating System (AGPL-3.0-or-later)
//! Where a register access lands: a bare port answers at offset 0; on a
//! whole HBA (`hba_global`) the port is port 0, its registers at PORT_BASE,
//! and below them lie the HBA's own.

use nonos_libc::Engines;

use super::hba::Hba;
use crate::constants::regs::{CMD_CR, CMD_FR, CMD_FRE, CMD_ST, PORT_BASE, PORT_STRIDE};

impl Hba {
    pub fn read(&mut self, off: u32) -> u32 {
        if self.global.is_none() {
            return self.read_port(off);
        }
        self.pass_width();
        match off.checked_sub(PORT_BASE) {
            Some(o) if o < PORT_STRIDE => self.read_port(o),
            // Another port, or one this HBA does not have.
            Some(o) => self.extra(o / PORT_STRIDE).map_or(0, |p| p.read_port(o % PORT_STRIDE)),
            None => self.read_global(off),
        }
    }

    pub fn write(&mut self, off: u32, v: u32) {
        if self.global.is_none() {
            return self.write_port(off, v);
        }
        self.pass_width();
        match off.checked_sub(PORT_BASE) {
            Some(o) if o < PORT_STRIDE => self.write_port(o, v),
            Some(o) => self.extra(o / PORT_STRIDE).map_or((), |p| p.write_port(o % PORT_STRIDE, v)),
            None => self.write_global(off, v),
        }
    }

    /// The device as the broker shim's watch reads it: HBA resets ended, and
    /// the engine bits of every port, and of each of the first four.
    pub fn engines(&self) -> Engines {
        let resets = self.global.as_ref().map_or(0, |g| g.resets);
        let extra = self.global.iter().flat_map(|g| g.extra.iter());
        let bits = |p: &Hba| p.reg.cmd & (CMD_ST | CMD_CR | CMD_FRE | CMD_FR);
        let all: Vec<u32> = core::iter::once(self).chain(extra).map(bits).collect();
        let mut ports = [0; 4];
        ports.iter_mut().zip(&all).for_each(|(p, &b)| *p = b);
        Engines { resets, running: all.iter().fold(0, |a, &b| a | b), ports }
    }
}
