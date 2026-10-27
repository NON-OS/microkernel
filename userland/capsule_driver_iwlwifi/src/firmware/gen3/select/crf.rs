// NONOS Operating System (AGPL-3.0-or-later)

//! What Linux v6.12's probe reads of the MAC and its companion RF before it
//! names the firmware (pcie/drv.c `iwl_pci_probe`, `get_crf_id`,
//! `map_crf_id`), for every family: with the MAC held awake, the peripheral
//! registers are given to the MAC (WFPM_CTRL_REG's AUX_IF_MAC_OWNER), then
//! the CRF id (SD_REG_VER, at SD_REG_VER_GEN2 from AX210 on), the CNVi id
//! (CNVI_AUX_MISC_CHIP) and the WFPM OTP word (its CDB and jacket bits) are
//! read. CSR_HW_RF_ID names the RF; on a blank OTP it names no RF type, and
//! the RF id is built from the CRF id instead, as `map_crf_id` builds it.

use super::super::csr::CsrLayout;
use super::super::prph::{grab, read_no_grab, release, write_no_grab};
use super::super::region::Clock;
use super::ids::rf_type;
use crate::regs::Mmio;

/// UMAC `WFPM_CTRL_REG` and its owner bit; plain `CNVI_AUX_MISC_CHIP`,
/// `SD_REG_VER` and `SD_REG_VER_GEN2`; UMAC `WFPM_OTP_CFG1_ADDR`.
pub const WFPM_CTRL_REG: u32 = 0xA0_3030;
pub const AUX_IF_MAC_OWNER: u32 = 0x8000_0000;
pub const CNVI_AUX_MISC_CHIP: u32 = 0xA2_00B0;
pub const SD_REG_VER: u32 = 0xA2_9600;
pub const SD_REG_VER_GEN2: u32 = 0xA2_B800;
pub const WFPM_OTP_CFG1: u32 = 0xA0_3098;

/// The three words `get_crf_id` reads.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct CrfIds {
    pub crf: u32,
    pub cnv: u32,
    pub wfpm: u32,
}

fn umac(csr: CsrLayout, addr: u32) -> u32 {
    addr.wrapping_add(csr.umac_offset()) & csr.prph_mask()
}

/// `get_crf_id`, or `None` when the MAC would not wake for it.
pub fn read_crf<M: Mmio, C: Clock>(m: &M, c: &mut C, csr: CsrLayout) -> Option<CrfIds> {
    grab(m, c, csr).then_some(())?;
    let ctrl = umac(csr, WFPM_CTRL_REG);
    write_no_grab(m, ctrl, read_no_grab(m, ctrl) | AUX_IF_MAC_OWNER);
    let sd = if csr == CsrLayout::Gen2 { SD_REG_VER } else { SD_REG_VER_GEN2 };
    let ids = CrfIds {
        crf: read_no_grab(m, sd & csr.prph_mask()),
        cnv: read_no_grab(m, CNVI_AUX_MISC_CHIP & csr.prph_mask()),
        wfpm: read_no_grab(m, umac(csr, WFPM_OTP_CFG1)),
    };
    release(m, csr);
    Some(ids)
}

/// The RF id the firmware is chosen by: CSR_HW_RF_ID when it names an RF
/// type, else `map_crf_id`'s from the CRF id (type, step, then the CDB and
/// jacket bits), else the blank value, which `select` refuses.
pub fn rf_id(csr_rf_id: u32, ids: CrfIds) -> u32 {
    if rf_type(csr_rf_id) != 0 {
        return csr_rf_id;
    }
    let rf: u32 = match ids.crf & 0xFFF {
        0x201 => 0x108,
        0x202 => 0x105,
        0x501 => 0x10C,
        0x503 | 0x504 => 0x10A,
        0x410 => 0x10D,
        0x910 => 0x112,
        0xA10 => 0x113,
        _ => return csr_rf_id,
    };
    let bit = |v: u32, b: u32| (v >> b) & 1;
    let cdb = bit(ids.wfpm, 4) | bit(ids.crf, 19);
    let jacket = bit(ids.wfpm, 5) | bit(ids.cnv, 19);
    rf << 12 | ((ids.crf >> 24) & 0xF) << 8 | cdb << 28 | jacket << 29
}
