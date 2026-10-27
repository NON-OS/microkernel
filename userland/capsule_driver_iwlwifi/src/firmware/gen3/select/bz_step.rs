// NONOS Operating System (AGPL-3.0-or-later)

//! The Bz MAC step. On a Bz MAC CSR_HW_REV carries no step; Linux v6.12
//! (pcie/drv.c `get_crf_id`) gives the CNVi its peripheral registers
//! (WFPM_CTRL_REG's AUX_IF_MAC_OWNER), reads CNVI_AUX_MISC_CHIP, takes the
//! step from bits 24-27, reads an A step of a BZ-U part (product 0x930) as B,
//! and ORs the step into hw_rev. Gl, discrete, keeps its CSR_HW_REV step.

use super::super::csr::CsrLayout;
use super::super::prph::{grab, read_no_grab, release, write_no_grab, UMAC_PRPH_OFFSET};
use super::super::region::Clock;
use crate::regs::Mmio;

/// UMAC `WFPM_CTRL_REG` and `WFPM_AUX_CTL_AUX_IF_MAC_OWNER_MSK`.
pub const WFPM_CTRL_REG: u32 = 0xA0_3030;
pub const AUX_IF_MAC_OWNER: u32 = 0x8000_0000;
/// `CNVI_AUX_MISC_CHIP`, a plain (non-UMAC) peripheral register.
pub const CNVI_AUX_MISC_CHIP: u32 = 0xA2_00B0;
const PROD_TYPE_BZ_U: u32 = 0x930;
const STEP_A: u8 = 0;
const STEP_B: u8 = 1;

/// The MAC step a CNVI_AUX_MISC_CHIP value names.
pub fn step_of(cnv: u32) -> u8 {
    let step = ((cnv >> 24) & 0xF) as u8;
    if cnv & 0xFFF == PROD_TYPE_BZ_U && step == STEP_A {
        STEP_B
    } else {
        step
    }
}

/// Read the Bz MAC step with the MAC held awake (V2 layout), or `None` when
/// it would not wake.
pub fn bz_mac_step<M: Mmio, C: Clock>(m: &M, c: &mut C) -> Option<u8> {
    let csr = CsrLayout::V2;
    if !grab(m, c, csr) {
        return None;
    }
    let ctrl = WFPM_CTRL_REG + UMAC_PRPH_OFFSET;
    write_no_grab(m, ctrl, read_no_grab(m, ctrl) | AUX_IF_MAC_OWNER);
    let cnv = read_no_grab(m, CNVI_AUX_MISC_CHIP);
    release(m, csr);
    Some(step_of(cnv))
}
