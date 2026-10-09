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

use crate::arch::aarch64::exceptions::terminal;
use crate::arch::trap::contract::cause::TrapCause;
use crate::arch::trap::contract::frame::TrapFrame;
use crate::sys::serial::Line;

use super::{detail, label};

/// Each line is built whole and written through the fatal writer, so a CPU
/// that faulted while holding the serial lock still gets its report out, and
/// a fault inside the report parks the CPU instead of recursing.
pub(in crate::arch::trap::contract) fn report_fatal<F: TrapFrame>(frame: &F, cause: &TrapCause) {
    if !terminal::enter() {
        return;
    }
    let mut line = Line::new();
    line.str(b"[TRAP] KERNEL FATAL TRAP: ").str(label::for_cause(cause).as_bytes());
    if let Some(esr) = frame.syndrome() {
        line.str(b" esr=").hex(esr);
    }
    line.str(b" elr=")
        .hex(frame.instruction_pointer())
        .str(b" sp=")
        .hex(frame.stack_pointer())
        .str(b" origin=")
        .str(if frame.from_user() { b"EL0" } else { b"EL1" })
        .end_fatal();
    detail::report(cause);
}
