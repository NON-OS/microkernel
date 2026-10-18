// NONOS Operating System (AGPL-3.0-or-later)
//! One HBA port as the driver sees it through its registers.

use super::drive::Drive;
use super::hba_addr::Pointers;
use super::hba_global::Global;
use super::quirks::Quirks;
use super::script::Step;
use super::state::{Count, PortRegs, Times};
use crate::constants::regs::{
    PORT_CI, PORT_CMD, PORT_IS, PORT_SACT, PORT_SCTL, PORT_SERR, PORT_SIG, PORT_SSTS, PORT_TFD,
    SIG_SATA,
};

pub struct Hba {
    pub reg: PortRegs,
    /// The command list, where program() pointed PxCLB.
    pub clb: u64,
    pub drive: Drive,
    pub quirks: Quirks,
    pub count: Count,
    /// Slot 0's command FIS went to the drive.
    pub(super) sent: bool,
    pub(super) at: Times,
    /// The HBA around the port (`hba_global`); none for a bare port at 0.
    pub global: Option<Global>,
    /// PxFB, and where PxCLB and PxFB pointed as each engine went on.
    pub ptr: Pointers,
    /// The HBA's CAP.S64A, as `hba_ports` passes it on (`fetch::reach`).
    pub(super) wide: bool,
}

impl Hba {
    /// An idle port whose drive runs `script`.
    pub fn new(clb: u64, script: Vec<Step>) -> Self {
        let (reg, drive) = (PortRegs::idle(), Drive::new(script));
        let (quirks, count, at) = (Quirks::default(), Count::default(), Times::default());
        let ptr = Pointers::default();
        Self { reg, clb, drive, quirks, count, sent: false, at, global: None, ptr, wide: true }
    }

    pub(super) fn read_port(&mut self, off: u32) -> u32 {
        self.advance();
        let r = &self.reg;
        match off {
            PORT_CMD => r.cmd,
            PORT_TFD => r.tfd,
            PORT_IS => r.is_read(),
            PORT_CI => r.ci | self.quirks.sticky.0,
            PORT_SACT => r.sact | self.quirks.sticky.1,
            PORT_SCTL => r.sctl,
            PORT_SSTS => r.ssts,
            PORT_SERR => r.serr,
            PORT_SIG => SIG_SATA,
            _ => self.read_addr(off),
        }
    }

    pub(super) fn write_port(&mut self, off: u32, v: u32) {
        self.advance();
        match off {
            // Write one to clear (AHCI 1.3.1, 3.3.5 and 3.3.12).
            PORT_IS => self.reg.is &= !v,
            PORT_SERR => self.reg.serr &= !v,
            PORT_CI => self.issue(v),
            PORT_CMD => self.write_cmd(v),
            PORT_SCTL => self.write_sctl(v),
            _ => self.write_addr(off, v),
        }
        self.advance();
    }
}
