// NONOS Operating System (AGPL-3.0-or-later)

//! The free ring's write index handed to the device, and where a used-ring
//! entry's buffer id and flags lie, by the family's CSR layout
//! (`iwl_pcie_rxq_inc_wr_ptr`, `iwl_pcie_get_rxb`).

use super::super::csr::{CsrLayout, WRPTR_RX_Q0};
use super::super::plan::{RX_USED, RX_USED_DESC};
use super::super::regs::{HBUS_TARG_WRPTR, RFH_Q0_FRBDCB_WIDX_TRG};
use super::queue::RxQueue;
use crate::regs::Mmio;

/// `iwl_rx_completion_desc_bz`: `rbid` 0, `flags` 2, four bytes.
const USED_DESC_BZ: usize = 4;

impl RxQueue {
    /// Tell the device how far the free ring is stocked (a multiple of eight):
    /// V1 through RFH_Q_FRBDCB_WIDX_TRG, V2 through HBUS_TARG_WRPTR with
    /// queue 0 at 512.
    pub fn kick<M: Mmio>(&mut self, m: &M) {
        let actual = self.write & !7;
        if actual != self.write_actual {
            self.write_actual = actual;
            match self.csr {
                CsrLayout::V1 => m.write32(RFH_Q0_FRBDCB_WIDX_TRG, actual as u32),
                CsrLayout::V2 => m.write32(HBUS_TARG_WRPTR, actual as u32 | WRPTR_RX_Q0),
            }
        }
    }

    /// Where used-ring entry `i`'s buffer id and flags are. V1 is
    /// `iwl_rx_completion_desc` (32 bytes: `rbid` 4, `flags` 6).
    pub(super) fn used_entry(&self, i: usize) -> (usize, usize) {
        match self.csr {
            CsrLayout::V1 => (RX_USED + i * RX_USED_DESC + 4, RX_USED + i * RX_USED_DESC + 6),
            CsrLayout::V2 => (RX_USED + i * USED_DESC_BZ, RX_USED + i * USED_DESC_BZ + 2),
        }
    }
}
