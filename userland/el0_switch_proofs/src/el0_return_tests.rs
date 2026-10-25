// NONOS Operating System (AGPL-3.0-or-later)
//! The switch at an IRQ's return to EL0: after the interrupt is finished,
//! only from EL0, and the user snapshot dropped once this frame resumes.

use crate::source::{code, order};

#[test]
fn switched_only_from_el0_after_the_interrupt_is_done() {
    let irq = code("arch/aarch64/exceptions/handlers/irq.rs");
    order(
        &irq,
        &[
            "let handled = dispatch_irq(acked.intid);",
            "complete_interrupt();",
            "leave_service(outer);",
            "crate::process::exit::reclaim_at_tick(from_user);",
            "if from_user {",
            "super::el0_return::at_el0_return();",
        ],
    );
    let el0 = code("arch/aarch64/exceptions/handlers/el0_return.rs");
    order(
        &el0,
        &[
            "if crate::smp::preempt_enabled() && need_reschedule() {",
            "clear_reschedule();",
            "let _ = switch(SwitchIntent::Preempt);",
            "*pcb.saved_user_context.lock() = None;",
        ],
    );
}

#[test]
fn the_switch_needs_interrupts_masked() {
    let lease = code("process/scheduler/contract/lease.rs");
    order(
        &lease,
        &[
            "pub fn acquire() -> Option<Self> {",
            "if backend::interrupts_enabled() {",
            "return None;",
        ],
    );
}
