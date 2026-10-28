// NONOS Operating System (AGPL-3.0-or-later)

//! An integrated 22000-family part's forced power gating, as Linux v6.12's
//! `iwl_pcie_gen2_force_power_gating` does it: D0A, FORCE_ACTIVE, then
//! power and sleep gating on, FORCE_ACTIVE off, and a software reset.

use super::super::super::prph::{grab, read_no_grab, release, write_no_grab};
use super::super::super::region::Clock;
use super::super::{finish_nic_init, prepare_card_hw, sw_reset, StartError};
use super::{w, G};
use crate::regs::Mmio;

const HPM_HIPM_GEN_CFG: u32 = 0xA0_3458;
const CR_PG_EN: u32 = 1 << 0;
const CR_SLP_EN: u32 = 1 << 1;
const CR_FORCE_ACTIVE: u32 = 1 << 10;

pub(super) fn force_power_gating<M: Mmio, C: Clock>(m: &M, c: &mut C) -> Result<(), StartError> {
    finish_nic_init(m, c, G)?;
    gen_cfg(m, c, |v| v | CR_FORCE_ACTIVE)?;
    c.delay_us(20);
    gen_cfg(m, c, |v| v | CR_PG_EN | CR_SLP_EN)?;
    c.delay_us(20);
    gen_cfg(m, c, |v| v & !CR_FORCE_ACTIVE)?;
    sw_reset(m, c, G);
    prepare_card_hw(m, c)
}

// HPM_HIPM_GEN_CFG read, changed and written with the MAC awake.
fn gen_cfg<M: Mmio, C: Clock>(m: &M, c: &mut C, f: impl Fn(u32) -> u32) -> Result<(), StartError> {
    if !grab(m, c, G) {
        return Err(StartError::NoNicAccess);
    }
    write_no_grab(m, w(HPM_HIPM_GEN_CFG), f(read_no_grab(m, w(HPM_HIPM_GEN_CFG))));
    release(m, G);
    Ok(())
}
