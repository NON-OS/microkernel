// NONOS Operating System (AGPL-3.0-or-later)

//! What the card says it is, read in Linux v6.12's `iwl_pci_probe` order:
//! with the NIC taken and in D0A (`iwl_pcie_prepare_card_hw`,
//! `iwl_finish_nic_init`, which setup's wake runs), the CRF, CNVi and WFPM
//! ids (`get_crf_id`, `crf`), then CSR_HW_RF_ID, built from the CRF id on a
//! blank OTP (`map_crf_id`). A Bz MAC's step is the CNVi's, ORed into
//! hw_rev. All of it before `_iwl_trans_pcie_start_hw`'s software reset.

use super::super::csr::CsrLayout;
use super::super::region::Clock;
use super::super::regs::CSR_HW_RF_ID;
use super::bz_step::step_of;
use super::crf::{read_crf, rf_id};
use super::ids::{mac_type, MAC_BZ};
use crate::regs::Mmio;

/// `hw_rev` with a Bz MAC's step ORed in, and the RF id; `None` when the
/// MAC would not wake for the CRF read (Linux's probe fails with -EIO).
pub fn probe<M: Mmio, C: Clock>(
    m: &M,
    c: &mut C,
    csr: CsrLayout,
    hw_rev: u32,
) -> Option<(u32, u32)> {
    let ids = read_crf(m, c, csr)?;
    let rf = rf_id(m.read32(CSR_HW_RF_ID), ids);
    let bz = csr == CsrLayout::V2 && mac_type(hw_rev) == MAC_BZ;
    let step = if bz { u32::from(step_of(ids.cnv)) } else { 0 };
    Some((hw_rev | step, rf))
}
