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

//! The vectors with no handler: SError, FIQ, every slot of the SP_EL0 and
//! AArch32 groups, and an interrupt nothing is routed to. Each names itself in
//! one `[TRAP]` line with the registers the vector saved, then parks the CPU.
//!
//! ESR_EL1 is written by a synchronous exception or an SError and left alone by
//! an interrupt, so interrupt lines leave it out rather than print an earlier
//! exception's syndrome as this one's. FAR_EL1 is written only by aborts, PC
//! alignment faults and watchpoints, and not even by an abort that reports it
//! invalid, so it is printed only then.

use crate::arch::aarch64::cpu;
use crate::arch::aarch64::exceptions::frame::ExceptionFrame;
use crate::arch::aarch64::exceptions::syndrome::{decode_esr, ExceptionClass};
use crate::arch::aarch64::exceptions::terminal;
use crate::sys::serial::Line;

/// ISS bit 10 of an abort syndrome: FAR_EL1 does not hold the faulting address.
const ISS_FNV: u64 = 1 << 10;

/// Report a terminal synchronous exception or SError.
pub(super) fn fatal(tag: &[u8], frame: &ExceptionFrame) -> ! {
    if terminal::enter() {
        let mut line = Line::new();
        line.str(b"[TRAP] ").str(tag).str(b" esr=").hex(frame.esr);
        if far_is_valid(frame.esr) {
            line.str(b" far=").hex(frame.far);
        }
        registers(&mut line, frame).end_fatal();
    }
    cpu::halt()
}

/// Report a terminal interrupt. `intid` is what the GIC acknowledged, or none
/// when the vector was taken with nothing acknowledged.
pub(super) fn fatal_interrupt(tag: &[u8], intid: Option<u32>, frame: &ExceptionFrame) -> ! {
    if terminal::enter() {
        let mut line = Line::new();
        line.str(b"[TRAP] ").str(tag);
        if let Some(intid) = intid {
            line.str(b" unrouted intid=").dec(u64::from(intid));
        }
        registers(&mut line, frame).end_fatal();
    }
    cpu::halt()
}

fn far_is_valid(esr: u64) -> bool {
    let class = decode_esr(esr).class;
    if class.is_data_abort() || class.is_instruction_abort() {
        return esr & ISS_FNV == 0;
    }
    matches!(
        class,
        ExceptionClass::PcAlignment
            | ExceptionClass::WatchpointLower
            | ExceptionClass::WatchpointSame
    )
}

fn registers<'a>(line: &'a mut Line, frame: &ExceptionFrame) -> &'a mut Line {
    line.str(b" elr=").hex(frame.elr).str(b" spsr=").hex(frame.spsr).str(b" sp=").hex(frame.sp)
}
