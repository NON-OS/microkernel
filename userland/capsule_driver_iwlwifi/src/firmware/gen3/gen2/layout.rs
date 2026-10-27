// NONOS Operating System (AGPL-3.0-or-later)

//! Everything the 22000 family's ROM reads, laid out as Linux v6.12
//! `iwl_pcie_ctxt_info_init` does: the sections in their blocks, the rings
//! cleared and stocked, and the context info at the start of the control
//! region, its control word `IWL_CTXT_INFO_TFD_FORMAT_LONG`, the receive
//! ring's size as its log2 (`RX_QUEUE_CB_SIZE`, bits 4-7) and the 2 KiB
//! buffer size (`IWL_CTXT_INFO_RB_SIZE_2K`, bits 9-12), the command ring's
//! `TFD_QUEUE_CB_SIZE` of 32 entries.

use super::super::cmdq::CmdQueue;
use super::super::csr::CsrLayout;
use super::super::layout::{place, Board, LayoutError, Memory, MCR_SIZE};
use super::super::plan::{CMD_TFDS, CTXT_INFO, PAGE, RB_STTS, RX_FREE, RX_USED};
use super::super::region::{zero, Region};
use super::super::rxq::RxQueue;
use super::super::FwLayout;
use super::ctxt_info::{CtxtInfoV1, CTXT_INFO_V1_SIZE};

const TFD_FORMAT_LONG: u32 = 0x100;
const RB_SIZE_2K: u32 = 0x2;

/// The context info's control word.
pub const CONTROL: u32 = TFD_FORMAT_LONG | (MCR_SIZE as u32) << 4 | RB_SIZE_2K << 9;

/// Write everything into `mem` and initialise both queues.
pub fn lay_out_gen2<R: Region + ?Sized>(
    mem: &Memory<'_, R>,
    fw: &FwLayout<'_>,
    board: &Board,
    rxq: &mut RxQueue,
    cmdq: &mut CmdQueue,
) -> Result<(), LayoutError> {
    let (ctrl, rbs) = (mem.ctrl, mem.rbs);
    let placed = place(mem, fw)?;
    let (umac, lmac, virt) = placed.maps();
    if !zero(ctrl, CTXT_INFO, RB_STTS + PAGE) || !rxq.init(ctrl, rbs) || !cmdq.init(ctrl) {
        return Err(LayoutError::Write);
    }
    let base = ctrl.dev();
    let info = CtxtInfoV1 {
        mac_id: board.hw_rev as u16,
        control_flags: CONTROL,
        free_rbd: base + RX_FREE as u64,
        used_rbd: base + RX_USED as u64,
        status_wr_ptr: base + RB_STTS as u64,
        cmd_queue: base + CMD_TFDS as u64,
        cmd_queue_size: CsrLayout::Gen2.cmd_ring().trailing_zeros() as u8 - 3,
        umac,
        lmac,
        virt,
    };
    let mut ci = [0u8; CTXT_INFO_V1_SIZE];
    if !info.write(&mut ci) {
        return Err(LayoutError::TooManySections);
    }
    if !ctrl.write(CTXT_INFO, &ci) {
        return Err(LayoutError::Write);
    }
    Ok(())
}
