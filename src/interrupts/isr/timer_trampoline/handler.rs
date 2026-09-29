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

use super::send_eoi::send_eoi;
use crate::interrupts::safety::set_interrupt_context;
use crate::interrupts::stats;
use crate::interrupts::timer;
use crate::process::userspace::types::UserContext;

/// Rust C-ABI body of the timer trampoline.
///
/// On entry, `ctx` points at a stack-resident region whose layout
/// matches the first 160 bytes of `UserContext` (15 GPRs + iretq
/// frame). The pointer is valid only for the duration of this call;
/// the trampoline reuses the memory on return.
///
/// When the trap originated from CPL=3, this function snapshots the
/// frame onto the current PCB's `saved_user_context` so the scheduler
/// resume hook can iretq back into the capsule via
/// `restore_user_context_iretq`. Replaces any prior snapshot — a
/// later context write overwrites earlier ones, and the scheduler
/// `take()`s the most recent one.
#[no_mangle]
pub(crate) extern "C" fn timer_trap_handler(ctx: *mut UserContext) {
    // SAFETY: eK@nonos.systems — `ctx` was produced by the trampoline
    // above and points at 160 bytes of valid stack memory laid out as
    // the leading fields of `UserContext`. We read those fields here;
    // `fs_base` and `gs_base` (the trailing 16 bytes of the full
    // struct) are not read because the trampoline does not write them.
    let frame = unsafe { &*ctx };
    let from_user = (frame.cs & 3) == 3;

    #[cfg(feature = "dbg-ring")]
    if from_user
        && (frame.cs != crate::process::userspace::USER_CS as u64
            || frame.ss != crate::process::userspace::USER_DS as u64)
    {
        crate::log::dbg_ring::dbg_emit_2u64(0x5346_0001, frame.cs, frame.ss);
    }

    if from_user {
        if let Some(pcb) = crate::process::current_process() {
            let snapshot = UserContext {
                r15: frame.r15,
                r14: frame.r14,
                r13: frame.r13,
                r12: frame.r12,
                r11: frame.r11,
                r10: frame.r10,
                r9: frame.r9,
                r8: frame.r8,
                rdi: frame.rdi,
                rsi: frame.rsi,
                rbp: frame.rbp,
                rbx: frame.rbx,
                rdx: frame.rdx,
                rcx: frame.rcx,
                rax: frame.rax,
                rip: frame.rip,
                cs: frame.cs,
                rflags: frame.rflags,
                rsp: frame.rsp,
                ss: frame.ss,
                fs_base: 0,
                gs_base: 0,
            };
            *pcb.saved_user_context.lock() = Some(snapshot);
        }
    }

    let _ctx_guard = set_interrupt_context();
    stats::increment_timer();
    // EOI before the tick work: on_timer_interrupt can preempt-switch away
    // inside this interrupt, and a deferred EOI leaves the timer vector
    // in-service at the LAPIC — no further ticks, sleep wakeups, or
    // preemption until the preempted context happens to resume. Sending it
    // here is safe: IF stays clear until the iretq, so the handler cannot
    // re-enter; a next tick pends and fires after the return.
    send_eoi();
    crate::process::accounting::set_tick_origin(from_user);
    timer::on_timer_interrupt();
    /*
     * Back here means this frame, not the snapshot, is what resumes: either
     * no switch happened or the task came back on its kernel context. A
     * snapshot left behind would later resume the task at this old rip, so a
     * guest parked in a syscall woke inside code it had already left.
     */
    if from_user {
        if let Some(pcb) = crate::process::current_process() {
            *pcb.saved_user_context.lock() = None;
        }
    }
    super::reclaim::on_tick(from_user);
    /*
     * A guest thread its supervisor asked to stop is parked here, running
     * no code, until it is answered; the frame it resumes from is this one,
     * which a signal answer rewrites to enter the handler.
     */
    if from_user {
        drop(_ctx_guard);
        let words = ctx.cast::<[u64; crate::process::foreign::TICK_FRAME_WORDS]>();
        /*
         * SAFETY: eK@nonos.systems - the trampoline's 160-byte frame read
         * above, still on this thread's kernel stack and restored from on the
         * way out; the 20 words are exactly that frame, nothing past it.
         */
        crate::process::foreign::on_user_tick(unsafe { &mut *words });
    }
}
