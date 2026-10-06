// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Lay out everything the boot ROM reads, as Linux v6.12
//! `iwl_pcie_ctxt_info_gen3_init` and `iwl_pcie_init_fw_sec` do: each firmware
//! section copied into its own block, the image loader, the peripheral
//! scratch (control flags, free ring address, the three image maps) and the
//! context info (scratch, info page, status word, command and completion
//! rings with their sizes). The receive and command rings are initialised
//! here too; the device is told nothing until the kick.

use alloc::vec::Vec;

use super::cmdq::CmdQueue;
use super::plan::{
    plan_firmware, Block, CMD_RING, CMD_TFDS, CTXT_INFO, IML, PAGE, PRPH_INFO, PRPH_SCRATCH,
    RB_STTS, RX_FREE, RX_RING, RX_USED,
};
use super::prph_scratch::flags::{CTRL_IMR_DEBUG_EN, CTRL_MTR_FORMAT, CTRL_MTR_MODE};
use super::prph_scratch::PRPH_SCRATCH_REPORTED;
use super::region::{zero, Region};
use super::rxq::RxQueue;
use super::{
    CtxtInfoGen3, DramImage, FwLayout, PrphScratch, CTXT_INFO_GEN3_SIZE, PRPH_SCRATCH_SIZE,
};

/// Why the memory could not be laid out.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LayoutError {
    /// A section is empty or too large, or the regions are not the planned ones.
    Plan,
    /// More than 64 sections in one image map.
    TooManySections,
    /// A region refused a write (shorter than planned).
    Write,
}

/// What the layout needs to know about the adapter.
pub struct Board {
    pub hw_rev: u32,
    pub imr_enabled: bool,
    /// CSR_HW_RF_ID, which picks the platform NVM section with the MAC type.
    pub rf_id: u32,
    /// The platform NVM file for this image, when the tree carries one.
    pub pnvm: Option<&'static [u8]>,
}

/// The firmware regions to request: the sizes [`plan_firmware`] gives for
/// the sections in LMAC, UMAC, paging order.
pub fn firmware_region_sizes(fw: &FwLayout<'_>) -> Option<Vec<usize>> {
    plan_firmware(&section_lens(fw)).map(|p| p.regions)
}

fn section_lens(fw: &FwLayout<'_>) -> Vec<usize> {
    fw.lmac.iter().chain(fw.umac.iter()).chain(fw.virt.iter()).map(|s| s.data.len()).collect()
}

/// `TFD_QUEUE_CB_SIZE` and `RX_QUEUE_CB_SIZE`: ring sizes as the log2 the
/// context info carries.
pub const MTR_SIZE: u16 = CMD_RING.trailing_zeros() as u16 - 3;
pub const MCR_SIZE: u16 = RX_RING.trailing_zeros() as u16;

/// The DMA memory of one bring-up: the control region, the receive buffers,
/// and the firmware regions [`firmware_region_sizes`] asked for, in order.
pub struct Memory<'r, R: Region + ?Sized> {
    pub ctrl: &'r R,
    pub rbs: &'r R,
    pub fw: &'r [&'r R],
}

/// Write everything into `mem` and initialise both queues.
pub fn lay_out<R: Region + ?Sized>(
    mem: &Memory<'_, R>,
    fw: &FwLayout<'_>,
    iml: &[u8],
    board: &Board,
    rxq: &mut RxQueue,
    cmdq: &mut CmdQueue,
) -> Result<(), LayoutError> {
    let (ctrl, rbs, fw_regions) = (mem.ctrl, mem.rbs, mem.fw);
    let plan = plan_firmware(&section_lens(fw)).ok_or(LayoutError::Plan)?;
    if plan.regions.len() != fw_regions.len()
        || plan.regions.iter().zip(fw_regions).any(|(&want, r)| r.len() < want)
    {
        return Err(LayoutError::Plan);
    }
    let sections = fw.lmac.iter().chain(fw.umac.iter()).chain(fw.virt.iter());
    let mut addrs = Vec::with_capacity(plan.blocks.len());
    for (s, b) in sections.zip(&plan.blocks) {
        let Block { region, off, .. } = *b;
        let r = fw_regions[region];
        if !r.write(off, s.data) {
            return Err(LayoutError::Write);
        }
        addrs.push(r.dev() + off as u64);
    }
    let (lmac, rest) = addrs.split_at(fw.lmac.len());
    let (umac, virt) = rest.split_at(fw.umac.len());

    if !zero(ctrl, CTXT_INFO, RB_STTS + PAGE) || !ctrl.write(IML, iml) {
        return Err(LayoutError::Write);
    }
    if !rxq.init(ctrl, rbs) || !cmdq.init(ctrl) {
        return Err(LayoutError::Write);
    }

    let mut flags = CTRL_MTR_MODE | CTRL_MTR_FORMAT;
    if board.imr_enabled {
        flags |= CTRL_IMR_DEBUG_EN;
    }
    let scratch = PrphScratch {
        mac_id: board.hw_rev as u16,
        version: 0,
        control_flags: flags,
        control_flags_ext: 0,
        pnvm_base: 0,
        pnvm_size: 0,
        reduce_power_base: 0,
        reduce_power_size: 0,
        free_rbd_addr: ctrl.dev() + RX_FREE as u64,
        dram: DramImage { umac, lmac, virt },
    };
    let mut buf = [0u8; PRPH_SCRATCH_SIZE];
    if !scratch.write(&mut buf) {
        return Err(LayoutError::TooManySections);
    }
    if !ctrl.write(PRPH_SCRATCH, &buf) {
        return Err(LayoutError::Write);
    }

    let base = ctrl.dev();
    let info = CtxtInfoGen3 {
        prph_info_base: base + PRPH_INFO as u64,
        prph_scratch_base: base + PRPH_SCRATCH as u64,
        prph_scratch_size: PRPH_SCRATCH_REPORTED as u32,
        cr_head_idx_arr_base: base + RB_STTS as u64,
        tr_tail_idx_arr_base: base + (PRPH_INFO + PAGE / 2) as u64,
        cr_tail_idx_arr_base: base + (PRPH_INFO + 3 * PAGE / 4) as u64,
        tr_idx_arr_size: 0,
        cr_idx_arr_size: 0,
        mtr_base: base + CMD_TFDS as u64,
        mcr_base: base + RX_USED as u64,
        mtr_size: MTR_SIZE,
        mcr_size: MCR_SIZE,
    };
    let mut ci = [0u8; CTXT_INFO_GEN3_SIZE];
    if !info.write(&mut ci) || !ctrl.write(CTXT_INFO, &ci) {
        return Err(LayoutError::Write);
    }
    Ok(())
}
