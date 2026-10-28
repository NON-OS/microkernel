// NONOS Operating System (AGPL-3.0-or-later)

//! The Bz MAC step. On a Bz MAC CSR_HW_REV carries no step; Linux v6.12
//! (pcie/drv.c `get_crf_id`) reads CNVI_AUX_MISC_CHIP with the peripheral
//! registers given to the MAC (`crf`), takes the step from bits 24-27, reads
//! an A step of a BZ-U part (product 0x930) as B, and ORs the step into
//! hw_rev. Gl, discrete, keeps its CSR_HW_REV step.

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
