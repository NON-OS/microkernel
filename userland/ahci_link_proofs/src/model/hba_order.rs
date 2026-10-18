// NONOS Operating System (AGPL-3.0-or-later)
//! The order a PxCMD write is held to, as AHCI 1.3.1, 10.1.2 has it: FIS
//! receive goes off (FRE cleared) only once the command engine is off, ST
//! clear and CR read clear. A write that clears FRE while ST is set, or CR
//! still reads 1, is a fault the test sees: it panics.

use super::hba::Hba;
use crate::constants::regs::{CMD_CR, CMD_FRE, CMD_ST};

impl Hba {
    /// Hold the PxCMD write `v` to that order, then note where each engine it
    /// turns on will point (`note_rise`).
    pub(super) fn before_cmd(&mut self, v: u32) {
        let was = self.reg.cmd;
        if was & CMD_FRE != 0 && v & CMD_FRE == 0 && was & (CMD_ST | CMD_CR) != 0 {
            panic!("FRE cleared with PxCMD {was:#x}, the engine not off (AHCI 1.3.1, 10.1.2)");
        }
        self.note_rise(v);
    }
}
