// NONOS Operating System (AGPL-3.0-or-later)

//! The platform-NVM step after ALIVE, for a part that reports a SKU id
//! (`iwl_pnvm_load_pnvm_to_trans`): give the firmware this SKU's section
//! when the tree carries the file, ring the doorbell, wait for completion.

use super::super::cmds::{PNVM_INIT_COMPLETE_NTFY, REGULATORY_AND_NVM_GROUP};
use super::super::dev::{Dev, Flow};
use super::super::layout::Board;
use super::super::plan::{pnvm_off, PRPH_SCRATCH};
use super::super::pnvm;
use super::super::prph::write_umac;
use super::super::region::{Clock, Region};
use super::super::regs::{UREG_DOORBELL_TO_ISR6, UREG_DOORBELL_TO_ISR6_PNVM};
use super::super::select::{mac_type, rf_type};
use super::{BootError, PNVM_MS};
use crate::regs::Mmio;

// Lay this SKU's platform NVM section out after the image loader and point
// the peripheral scratch at it; its version, or `None` when the file has no
// section for this adapter or it does not fit the reserved space.
fn place_pnvm<R: Region + ?Sized>(
    ctrl: &R,
    file: &[u8],
    sku: [u32; 3],
    board: &Board,
    iml_len: usize,
) -> Option<u32> {
    let p = pnvm::parse(file, sku, mac_type(board.hw_rev), rf_type(board.rf_id))?;
    let placed = pnvm::lay_out(ctrl, pnvm_off(iml_len)?, &p)?;
    pnvm::point_scratch(ctrl, PRPH_SCRATCH, placed).then_some(p.version)
}

/// Without the file, or with no matching section, the doorbell still rings
/// and the firmware runs on its defaults, as Linux's does.
pub(super) fn pnvm_step<M: Mmio, R: Region + ?Sized, C: Clock>(
    dev: &mut Dev<'_, M, R>,
    c: &mut C,
    ctrl: &R,
    board: &Board,
    iml_len: usize,
    sku: [u32; 3],
) -> Result<(), BootError> {
    if let Some(file) = board.pnvm {
        dev.pnvm = place_pnvm(ctrl, file, sku, board, iml_len);
    }
    let (m, csr) = (dev.m, dev.csr);
    if !write_umac(m, c, csr, UREG_DOORBELL_TO_ISR6, UREG_DOORBELL_TO_ISR6_PNVM) {
        return Err(BootError::NoNicAccess);
    }
    dev.wait(c, PNVM_MS, &mut |p| {
        if p.group == REGULATORY_AND_NVM_GROUP && p.cmd == PNVM_INIT_COMPLETE_NTFY {
            Flow::Done
        } else {
            Flow::Continue
        }
    })
    .map_err(BootError::Pnvm)
}
