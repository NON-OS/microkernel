// NONOS Operating System (AGPL-3.0-or-later)

//! From claimed part to its CPU started, in the family's way: the 22000
//! family's version 1 context info (`gen2`), or the gen3 one with its
//! peripheral scratch and image loader (AX210 and BZ): start the hardware,
//! lay out memory, kick the self-load, start the CPU.

use super::super::csr::CsrLayout;
use super::super::dev::Dev;
use super::super::gen2;
use super::super::layout::{lay_out, Board, Memory};
use super::super::plan::{CTXT_INFO, IML};
use super::super::region::{Clock, Region};
use super::super::start::{after_kick, before_kick, enable_alive_int, start_hw};
use super::super::ucode::Ucode;
use super::super::{kick, FwLayout};
use super::BootError;
use crate::regs::Mmio;

pub(super) fn start_and_kick<M: Mmio, R: Region + ?Sized, C: Clock>(
    dev: &mut Dev<'_, M, R>,
    c: &mut C,
    mem: &Memory<'_, R>,
    fw: &FwLayout<'_>,
    ucode: &Ucode<'_>,
    board: &Board,
) -> Result<(), BootError> {
    let (m, csr) = (dev.m, dev.csr);
    if csr == CsrLayout::Gen2 {
        return gen2::start_and_kick(dev, c, mem, fw, ucode, board);
    }
    start_hw(m, c, csr).map_err(BootError::Start)?;
    before_kick(m, c, csr).map_err(BootError::Start)?;
    lay_out(mem, fw, ucode.iml, board, &mut dev.rxq, &mut dev.cmdq).map_err(BootError::Layout)?;
    enable_alive_int(m);
    let base = mem.ctrl.dev();
    kick(m, base + CTXT_INFO as u64, base + IML as u64, ucode.iml.len() as u32);
    after_kick(m, c, csr, board.discrete).map_err(BootError::Start)
}
