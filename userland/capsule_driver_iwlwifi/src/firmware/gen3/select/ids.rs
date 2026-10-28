// NONOS Operating System (AGPL-3.0-or-later)

//! The MAC and RF ids `select` reads: CSR_HW_REV's MAC type and step and
//! CSR_HW_RF_ID's RF type, step and CDB bit, decoded as Linux v6.12's
//! iwl-csr.h macros do, with the `IWL_CFG_MAC_TYPE_*` and `IWL_CFG_RF_TYPE_*`
//! values of iwl-config.h.

pub(super) const MAC_SO: u16 = 0x37;
pub(super) const MAC_SOF: u16 = 0x43;
pub(super) const MAC_TY: u16 = 0x42;
pub(super) const MAC_MA: u16 = 0x44;
/// `IWL_CFG_MAC_TYPE_BZ` and `IWL_CFG_MAC_TYPE_GL`.
pub const MAC_BZ: u16 = 0x46;
pub const MAC_GL: u16 = 0x47;
/// `hw_rev_step` for the 8000 family on: the low nibble of CSR_HW_REV (for
/// Bz, the CNVi's step ORed in), read as the step letter `'a' + n`
/// (`iwl_drv_get_step`). Meteor Lake and Bz ship B0, Gl C0.
pub(super) const STEP_A: u8 = 0;
pub(super) const STEP_B: u8 = 1;
pub(super) const STEP_C: u8 = 2;
pub(super) const RF_HR2: u16 = 0x10A;
pub(super) const RF_HR1: u16 = 0x10C;
pub(super) const RF_GF: u16 = 0x10D;
/// `IWL_CFG_RF_TYPE_FM`; linux-firmware's gl and bz images are for its C step.
pub(super) const RF_FM: u16 = 0x112;

/// The MAC step: the low nibble of CSR_HW_REV (`hw_rev_step`).
pub fn mac_step(hw_rev: u32) -> u8 {
    (hw_rev & 0xF) as u8
}

/// `CSR_HW_REV_TYPE`: the MAC type.
pub fn mac_type(hw_rev: u32) -> u16 {
    ((hw_rev & 0x0000_FFF0) >> 4) as u16
}

/// `CSR_HW_RFID_TYPE`: the RF module type.
pub fn rf_type(rf_id: u32) -> u16 {
    ((rf_id & 0x00FF_F000) >> 12) as u16
}

/// `CSR_HW_RFID_STEP`: the RF module step.
pub fn rf_step(rf_id: u32) -> u8 {
    ((rf_id >> 8) & 0xF) as u8
}

/// `CSR_HW_RFID_IS_CDB`: a dual-radio module.
pub fn rf_is_cdb(rf_id: u32) -> bool {
    rf_id & 0x1000_0000 != 0
}
