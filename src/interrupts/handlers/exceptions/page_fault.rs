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

use crate::memory::addr::VirtAddr;
use x86_64::structures::idt::InterruptStackFrame;

use super::context::{log_page_fault, ExceptionContext, PageFaultContext, PageFaultErrorCode};
use crate::arch::x86_64::diag::emit_fatal_notice;
use crate::interrupts::safety::set_interrupt_context;
use crate::interrupts::stats;
use crate::memory::hardening;
use crate::memory::paging::manager::api as paging;
use crate::security::observability::redact::redact_address;

/// # Safety
/// Page fault handler. Sets interrupt context for safe nesting detection.
pub fn handle(frame: InterruptStackFrame, error_code: u64) {
    let _ctx = set_interrupt_context();

    let accessed_address = super::fault_address::fault_address();
    crate::arch::x86_64::diag::dump_trap(b"PF", &frame, Some(error_code), Some(accessed_address));
    let exception = ExceptionContext::from_frame(&frame);
    let error = PageFaultErrorCode::from_bits(error_code);

    let ctx = PageFaultContext { exception, accessed_address, error_code: error };

    // dump_trap has the frame. The entry allocates and locks, so a kernel
    // fault, which may hold the heap or the logger, skips it.
    if exception.is_user_mode() {
        log_page_fault(&ctx);
    }
    stats::increment_page_faults();

    if try_handle_fault(&ctx, error_code) {
        return;
    }

    if exception.is_user_mode() {
        terminate_user_process(&ctx);
    } else {
        kernel_panic(&ctx);
    }
}

fn try_handle_fault(ctx: &PageFaultContext, error_code: u64) -> bool {
    let virt_addr = VirtAddr::new(ctx.accessed_address);

    if hardening::check_guard_page_access(virt_addr) {
        crate::sys::serial::Line::new().str(b"[PF] guard page violation").end_fatal();
        return false;
    }

    if let Ok(()) = paging::handle_page_fault(virt_addr, error_code) {
        crate::log::logger::log_debug!("Page fault handled successfully");
        return true;
    }

    false
}

fn terminate_user_process(ctx: &PageFaultContext) -> ! {
    crate::log::logger::log_error!(
        "Segmentation fault: user process accessed invalid address {}",
        redact_address(ctx.accessed_address)
    );
    crate::process::exit::exit_and_yield(-11, true)
}

fn kernel_panic(ctx: &PageFaultContext) {
    emit_fatal_notice(b"PF", ctx.exception.instruction_pointer);
    // What was attempted, as one fatal line with nothing allocated: the fault
    // may have come while this CPU held the heap's or the port's lock.
    let what: &[u8] = if ctx.error_code.is_instruction_fetch() {
        b"[PANIC PF] attempted to execute from a non-executable page"
    } else if ctx.error_code.is_write() {
        b"[PANIC PF] attempted to write to a read-only page"
    } else {
        b"[PANIC PF] attempted to read from a non-present page"
    };
    crate::sys::serial::Line::new().str(what).end_fatal();
    crate::boot::panic::fatal();
}
