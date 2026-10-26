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

use x86_64::structures::idt::InterruptStackFrame;

use super::context::ExceptionContext;
use crate::arch::x86_64::diag::emit_fatal_notice_nolock;
use crate::interrupts::stats;

pub fn handle(frame: InterruptStackFrame, error_code: u64) -> ! {
    let ctx = ExceptionContext::from_frame(&frame);
    emit_fatal_notice_nolock(b"DF", ctx.instruction_pointer);
    stats::increment_exceptions();
    // Nothing allocated and no lock waited on: a double fault's stack and
    // heap are suspect, and the panic path is all that is left.
    let mut l = crate::sys::serial::Line::new();
    l.str(b"[PANIC DF] error code ").hex(error_code).str(b" cs=").hex(ctx.code_segment as u64);
    l.end_fatal();
    crate::boot::panic::fatal();
}
