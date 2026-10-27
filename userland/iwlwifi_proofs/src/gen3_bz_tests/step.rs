// NONOS Operating System (AGPL-3.0-or-later)

//! The Bz MAC step, read as Linux's `get_crf_id` reads it: the CNVi given
//! its registers (WFPM_CTRL_REG's owner bit, through the UMAC window), then
//! CNVI_AUX_MISC_CHIP's bits 24-27, an A-step BZ-U (0x930) taken as B.

use std::cell::RefCell;
use std::collections::HashMap;

use crate::gen3::csr::CsrLayout;
use crate::gen3::csr::{GP_BZ_MAC_ACCESS_REQ, GP_MAC_STATUS};
use crate::gen3::regs::{CSR_GP_CNTRL, HBUS_TARG_PRPH_RADDR, HBUS_TARG_PRPH_RDAT};
use crate::gen3::regs::{HBUS_TARG_PRPH_WADDR, HBUS_TARG_PRPH_WDAT};
use crate::gen3::select::bz_step::step_of;
use crate::gen3::select::crf::read_crf;
use crate::gen3::select::crf::{AUX_IF_MAC_OWNER, CNVI_AUX_MISC_CHIP, WFPM_CTRL_REG};
use crate::regs::Mmio;

use super::rig::polls;

// The step as the probe reads it: the CNVi word through `read_crf`.
fn bz_mac_step<M: Mmio>(m: &M, c: &mut crate::gen3_model::Polls) -> Option<u8> {
    read_crf(m, c, CsrLayout::V2).map(|ids| step_of(ids.cnv))
}

/// CSR registers, and peripheral registers behind the HBUS window.
#[derive(Default)]
struct Cnvi {
    csr: RefCell<HashMap<usize, u32>>,
    prph: RefCell<HashMap<u32, u32>>,
}

impl Mmio for Cnvi {
    fn read32(&self, off: usize) -> u32 {
        let csr = self.csr.borrow();
        let v = csr.get(&off).copied().unwrap_or(0);
        match off {
            CSR_GP_CNTRL if v & GP_BZ_MAC_ACCESS_REQ != 0 => v | GP_MAC_STATUS,
            HBUS_TARG_PRPH_RDAT => {
                let a = csr.get(&HBUS_TARG_PRPH_RADDR).copied().unwrap_or(0) & 0xFF_FFFF;
                self.prph.borrow().get(&a).copied().unwrap_or(0)
            }
            _ => v,
        }
    }
    fn write32(&self, off: usize, val: u32) {
        if off == HBUS_TARG_PRPH_WDAT {
            let a = self.read32(HBUS_TARG_PRPH_WADDR) & 0xFF_FFFF;
            self.prph.borrow_mut().insert(a, val);
        }
        self.csr.borrow_mut().insert(off, val);
    }
}

#[test]
fn the_bz_step_comes_from_the_cnvi_with_its_registers_given() {
    let d = Cnvi::default();
    d.prph.borrow_mut().insert(CNVI_AUX_MISC_CHIP, 0x0100_0900);
    assert_eq!(bz_mac_step(&d, &mut polls()), Some(1), "BZ-I B step");
    let wfpm = d.prph.borrow()[&(WFPM_CTRL_REG + 0x30_0000)];
    assert_eq!(wfpm & AUX_IF_MAC_OWNER, AUX_IF_MAC_OWNER, "the owner bit, in the UMAC window");
    assert_eq!(d.read32(CSR_GP_CNTRL) & GP_BZ_MAC_ACCESS_REQ, 0, "access released");
    assert_eq!(step_of(0x0000_0930), 1, "an A-step BZ-U is taken as B");
    assert_eq!(step_of(0x0000_0900), 0, "an A-step BZ-I stays A (and is refused)");
    assert_eq!(step_of(0x0200_0930), 2);
}
