// NONOS Operating System (AGPL-3.0-or-later)

//! The 22000 family's image, as Linux v6.12 pcie/drv.c chooses it: the AX200
//! (8086:2723) runs `iwl_ax200_cfg_cc` (cc-a0) whatever its MAC and RF say;
//! a Qu or QuZ CNVi with an HR radio runs, by `iwl_dev_info_table` and the
//! two hw_rev fix-ups after it, Qu-b0-hr-b0 on a Qu MAC of step B,
//! Qu-c0-hr-b0 on step C (`CSR_HW_REV_TYPE_QU_C0`), and QuZ-a0-hr-b0 on a
//! QuZ MAC (its MAC rows: any step for HR1, step B for HR2; its Comet Lake
//! subsystem rows, `iwl_ax201_cfg_quz_hr`: any). The subsystem rows name the
//! card (AX201, Killer 1650s/i) and otherwise choose what the MAC and step
//! do; the broker's record carries no subsystem id, so the driver takes what
//! a listed one would: a Qu B-step 160 MHz HR2 module (no MAC row) runs
//! Qu-b0, a QuZ part of any step QuZ (`subsystems` in the proofs compares
//! every row). A JF radio (9461, 9462, 9560) runs a 9000-family image, not
//! carried.

use super::ids::*;
use super::{Image, Refusal};

const MAC_QU: u16 = 0x33;
const MAC_QUZ: u16 = 0x35;
/// The discrete AX200's PCI device id.
pub const AX200: u16 = 0x2723;

pub(super) fn qu_image(pci_device: u16, hw_rev: u32, rf_id: u32) -> Result<Image, Refusal> {
    let rf = rf_type(rf_id);
    if pci_device == AX200 {
        return Ok(Image::CcHr);
    }
    if rf_is_cdb(rf_id) {
        return Err(Refusal::CdbNotBundled);
    }
    if !matches!(rf, RF_HR1 | RF_HR2) {
        return Err(Refusal::RfNotBundled(rf));
    }
    match (mac_type(hw_rev), mac_step(hw_rev)) {
        (MAC_QU, STEP_B) => Ok(Image::QuB0Hr),
        (MAC_QU, STEP_C) => Ok(Image::QuC0Hr),
        (MAC_QUZ, _) => Ok(Image::QuZHr),
        (MAC_QU, step) => Err(Refusal::MacStepNotBundled(step)),
        (mac, _) => Err(Refusal::MacNotBundled(mac)),
    }
}
