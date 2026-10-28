// NONOS Operating System (AGPL-3.0-or-later)

//! The free ring's write index handed to the device, and the family's
//! used-ring and free-ring entries (`iwl_pcie_rxq_inc_wr_ptr`,
//! `iwl_pcie_get_rxb`, `iwl_pcie_restock_bd`).

use super::super::csr::{CsrLayout, WRPTR_RX_Q0};
use super::super::plan::{RX_FREE, RX_FREE_DESC, RX_USED, RX_USED_DESC};
use super::super::region::{get16, put16, put64, Region};
use super::super::regs::{HBUS_TARG_WRPTR, RFH_Q0_FRBDCB_WIDX_TRG};
use super::queue::RxQueue;
use crate::regs::Mmio;

/// `iwl_rx_completion_desc_bz` and the 22000 family's entry: four bytes.
const USED_DESC_SHORT: usize = 4;

impl RxQueue {
    /// Tell the device how far the free ring is stocked (a multiple of eight):
    /// V1 and Gen2 through RFH_Q_FRBDCB_WIDX_TRG, V2 through HBUS_TARG_WRPTR
    /// with queue 0 at 512.
    pub fn kick<M: Mmio>(&mut self, m: &M) {
        let actual = self.write & !7;
        if actual != self.write_actual {
            self.write_actual = actual;
            match self.csr {
                CsrLayout::V1 | CsrLayout::Gen2 => m.write32(RFH_Q0_FRBDCB_WIDX_TRG, actual as u32),
                CsrLayout::V2 => m.write32(HBUS_TARG_WRPTR, actual as u32 | WRPTR_RX_Q0),
            }
        }
    }

    /// The buffer id used-ring entry `i` names, and its flags byte (`None`
    /// when it could not be read). V1 is `iwl_rx_completion_desc` (32 bytes:
    /// `rbid` 4, `flags` 6), V2 `iwl_rx_completion_desc_bz` (`rbid` 0,
    /// `flags` 2), Gen2 a `__le32` whose low 12 bits are the id, no flags.
    pub(super) fn used<R: Region + ?Sized>(&self, ctrl: &R, i: usize) -> Option<(u16, Option<u8>)> {
        let (id_at, flags_at) = match self.csr {
            CsrLayout::V1 => (RX_USED + i * RX_USED_DESC + 4, RX_USED + i * RX_USED_DESC + 6),
            CsrLayout::V2 => (RX_USED + i * USED_DESC_SHORT, RX_USED + i * USED_DESC_SHORT + 2),
            CsrLayout::Gen2 => {
                return get16(ctrl, RX_USED + i * USED_DESC_SHORT).map(|v| (v & 0x0FFF, Some(0)))
            }
        };
        let vid = get16(ctrl, id_at)?;
        let mut flags = [0u8; 1];
        Some((vid, ctrl.read(flags_at, &mut flags).then_some(flags[0])))
    }

    /// Free-ring entry `i` for buffer `vid` at device address `addr`: V1 and
    /// V2 `iwl_rx_transfer_desc` (`rbid` 0, `addr` 8), Gen2 a `__le64` of the
    /// page-aligned address with the id in its low 12 bits.
    pub(super) fn free<R: Region + ?Sized>(&self, ctrl: &R, i: usize, vid: u16, addr: u64) -> bool {
        match self.csr {
            CsrLayout::Gen2 => put64(ctrl, RX_FREE + i * 8, addr | u64::from(vid & 0x0FFF)),
            CsrLayout::V1 | CsrLayout::V2 => {
                let at = RX_FREE + i * RX_FREE_DESC;
                put16(ctrl, at, vid) && put64(ctrl, at + 8, addr)
            }
        }
    }
}
