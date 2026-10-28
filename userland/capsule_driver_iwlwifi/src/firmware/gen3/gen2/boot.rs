// NONOS Operating System (AGPL-3.0-or-later)

//! From claimed 22000-family part to its CPU started, in Linux v6.12's
//! order: `_iwl_trans_pcie_start_hw` (`start_hw_gen2`), then
//! `iwl_trans_pcie_gen2_start_fw`: prepare, interrupts cleared and masked,
//! APM init, the NIC configuration (`iwl_mvm_nic_config`), the memory laid
//! out, the ALIVE interrupt enabled (`iwl_enable_fw_load_int_ctx_info`),
//! the context info's address written to CSR_CTXT_INFO_BA (64 bits, low
//! dword first, as `iwl_write64`), then the LTR and UREG_CPU_INIT_RUN.

use super::super::bringup::BootError;
use super::super::csr::CsrLayout;
use super::super::dev::Dev;
use super::super::layout::{Board, Memory};
use super::super::plan::CTXT_INFO;
use super::super::region::{Clock, Region};
use super::super::start::gen2::{bisr_wait, set_nic_config, start_hw_gen2};
use super::super::start::{after_kick, before_kick, enable_alive_int};
use super::super::ucode::Ucode;
use super::super::FwLayout;
use super::layout::lay_out_gen2;
use crate::regs::Mmio;

/// `CSR_CTXT_INFO_BA` (iwl-context-info.h).
pub const CSR_CTXT_INFO_BA: usize = 0x40;

/// Start the part, lay its memory out and start its CPU.
pub fn start_and_kick<M: Mmio, R: Region + ?Sized, C: Clock>(
    dev: &mut Dev<'_, M, R>,
    c: &mut C,
    mem: &Memory<'_, R>,
    fw: &FwLayout<'_>,
    ucode: &Ucode<'_>,
    board: &Board,
) -> Result<(), BootError> {
    let m = dev.m;
    let csr = CsrLayout::Gen2;
    // cfg/22000.c sets `bisr_workaround` on iwl_ax200_trans_cfg, the
    // family's one discrete transport.
    let bisr = board.discrete;
    start_hw_gen2(m, c, !board.discrete, bisr).map_err(BootError::Start)?;
    bisr_wait(c, bisr, 2000);
    before_kick(m, c, csr).map_err(BootError::Start)?;
    bisr_wait(c, bisr, 200);
    set_nic_config(m, board.hw_rev, ucode.phy_config);
    lay_out_gen2(mem, fw, board, &mut dev.rxq, &mut dev.cmdq).map_err(BootError::Layout)?;
    enable_alive_int(m);
    let ba = mem.ctrl.dev() + CTXT_INFO as u64;
    m.write32(CSR_CTXT_INFO_BA, ba as u32);
    m.write32(CSR_CTXT_INFO_BA + 4, (ba >> 32) as u32);
    after_kick(m, c, csr, board.discrete).map_err(BootError::Start)
}
