// NONOS Operating System (AGPL-3.0-or-later)
//! The port's address registers: PxCLB and PxCLBU name the command list the
//! fetch follows (`Hba::clb`), PxFB and PxFBU the FIS area. An HBA reset
//! leaves them as they were (AHCI 1.3.1, 10.4.3), so on a port a dead driver
//! left armed they still name its frames until the driver writes its own. On
//! an HBA without CAP.S64A the upper halves read 0 and name nothing (`reach`).

use super::fetch::reach;
use super::hba::Hba;
use crate::constants::regs::{CMD_FRE, CMD_ST, PORT_CLB, PORT_CLBU, PORT_FB, PORT_FBU};

#[derive(Default)]
pub struct Pointers {
    pub fb: u64,
    /// PxFB each time FRE went on, and PxCLB each time ST did.
    pub fb_at_fre: Vec<u64>,
    pub clb_at_st: Vec<u64>,
}

/// The low or the high half of `at`, set to `v`.
fn half(at: u64, high: bool, v: u32) -> u64 {
    if high {
        at & 0xffff_ffff | u64::from(v) << 32
    } else {
        at & !0xffff_ffff | u64::from(v)
    }
}

impl Hba {
    pub(super) fn read_addr(&self, off: u32) -> u32 {
        let (clb, fb) = (reach(self.clb, self.wide), reach(self.ptr.fb, self.wide));
        let v = [(PORT_CLB, clb), (PORT_CLBU, clb >> 32), (PORT_FB, fb), (PORT_FBU, fb >> 32)];
        v.iter().find_map(|&(at, v)| (at == off).then_some(v as u32)).unwrap_or(0)
    }

    pub(super) fn write_addr(&mut self, off: u32, v: u32) {
        match off {
            PORT_CLB | PORT_CLBU => self.clb = half(self.clb, off == PORT_CLBU, v),
            PORT_FB | PORT_FBU => self.ptr.fb = half(self.ptr.fb, off == PORT_FBU, v),
            _ => {}
        }
    }

    /// Before a PxCMD write `v`: where each engine it turns on will point.
    pub(super) fn note_rise(&mut self, v: u32) {
        let rises = |bit: u32| v & bit != 0 && self.reg.cmd & bit == 0;
        if rises(CMD_FRE) {
            self.ptr.fb_at_fre.push(reach(self.ptr.fb, self.wide));
        }
        if rises(CMD_ST) {
            self.ptr.clb_at_st.push(reach(self.clb, self.wide));
        }
    }
}
