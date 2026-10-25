// NONOS Operating System (AGPL-3.0-or-later)
//! What reads the tick's origin, read from the kernel's source: the one
//! per-CPU flag behind the dead-thread switch and band wake, and the
//! lock-free tick point every arch reaches with the same origin.

use crate::source::{code, order};

#[test]
fn the_dead_thread_switch_and_band_wake_read_what_was_set() {
    let flag = "crate::smp::percpu::current().tick_from_user";
    let split = code("process/accounting/split.rs");
    order(&split, &["pub fn set_tick_origin(from_user: bool) {", flag, ".store(from_user,"]);
    let tick = code("process/scheduler/preemption/tick.rs");
    order(
        &tick,
        &[
            "pub fn tick() {",
            "give_way()",
            "&& ",
            flag,
            "&& super::super::selection::is_dead(pid)",
            "set_reschedule();",
        ],
    );
    let band = code("process/scheduler/preemption/band_wake.rs");
    order(&band, &["pub(super) fn give_way() -> bool {", flag, "if !from_user ||"]);
}

/// After the tick, with the origin it recorded: the one point where a tick
/// holding no kernel lock does deferred work, on all three arches.
#[test]
fn every_arch_reaches_the_lock_free_tick_with_its_origin() {
    let x86 = code("interrupts/isr/timer_trampoline/handler.rs");
    order(
        &x86,
        &[
            "crate::process::accounting::set_tick_origin(from_user);",
            "timer::on_timer_interrupt();",
            "crate::process::exit::reclaim_at_tick(from_user);",
        ],
    );
    let a64 = code("arch/aarch64/exceptions/handlers/irq.rs");
    order(
        &a64,
        &[
            "let handled = dispatch_irq(acked.intid);",
            "complete_interrupt();",
            "if is_preemption_tick(acked.intid) {",
            "crate::process::exit::reclaim_at_tick(from_user);",
        ],
    );
    let rv = code("arch/riscv64/timer/interrupt.rs");
    order(
        &rv,
        &[
            "crate::process::scheduler::preemption::tick();",
            "crate::process::exit::reclaim_at_tick(from_user);",
        ],
    );
    let reclaim = code("process/exit/reclaim.rs");
    assert!(
        reclaim.contains("let holds_no_lock = from_user || crate::process::accounting::is_idle();")
    );
}
