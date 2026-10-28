// NONOS Operating System (AGPL-3.0-or-later)

//! A card that wakes on its family's access request, with peripheral
//! registers behind the HBUS window, and the probe run on it.

use std::cell::RefCell;
use std::collections::HashMap;

use crate::gen3::csr::{CsrLayout, GP_BZ_MAC_ACCESS_REQ, GP_MAC_STATUS};
use crate::gen3::regs::*;
use crate::gen3::select::crf::{CNVI_AUX_MISC_CHIP, SD_REG_VER, SD_REG_VER_GEN2, WFPM_OTP_CFG1};
use crate::gen3::select::probe::probe;
use crate::gen3_model::Polls;
use crate::regs::Mmio;

/// CSR registers that wake on an access request, and peripheral ones.
#[derive(Default)]
pub(super) struct Card(RefCell<HashMap<usize, u32>>, pub(super) RefCell<HashMap<u32, u32>>);

impl Mmio for Card {
    fn read32(&self, off: usize) -> u32 {
        let v = self.0.borrow().get(&off).copied().unwrap_or(0);
        let a = self.0.borrow().get(&HBUS_TARG_PRPH_RADDR).copied().unwrap_or(0) & 0xFF_FFFF;
        match off {
            CSR_GP_CNTRL if v & GP_MAC_ACCESS_REQ != 0 => v | GP_MAC_CLOCK_READY,
            CSR_GP_CNTRL if v & GP_BZ_MAC_ACCESS_REQ != 0 => v | GP_MAC_STATUS,
            HBUS_TARG_PRPH_RDAT => self.1.borrow().get(&a).copied().unwrap_or(0),
            _ => v,
        }
    }
    fn write32(&self, off: usize, val: u32) {
        if off == HBUS_TARG_PRPH_WDAT {
            let a = self.read32(HBUS_TARG_PRPH_WADDR) & 0xFF_FFFF;
            self.1.borrow_mut().insert(a, val);
        }
        self.0.borrow_mut().insert(off, val);
    }
}

pub(super) fn card(csr: CsrLayout, rf_id: u32, crf: u32, cnv: u32, wfpm: u32) -> Card {
    let (m, off) = (csr.prph_mask(), csr.umac_offset());
    let sd = if csr == CsrLayout::Gen2 { SD_REG_VER } else { SD_REG_VER_GEN2 };
    let c = Card::default();
    c.0.borrow_mut().insert(CSR_HW_RF_ID, rf_id);
    let regs = [(sd & m, crf), (CNVI_AUX_MISC_CHIP & m, cnv), ((WFPM_OTP_CFG1 + off) & m, wfpm)];
    c.1.borrow_mut().extend(regs);
    c
}

pub(super) fn run(c: &Card, csr: CsrLayout, hw_rev: u32) -> Option<(u32, u32)> {
    probe(c, &mut Polls { per_wait: 64, delays_us: 0 }, csr, hw_rev)
}
