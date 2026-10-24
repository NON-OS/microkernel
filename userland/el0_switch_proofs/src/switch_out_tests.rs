// NONOS Operating System (AGPL-3.0-or-later)
//! What a task leaves on a CPU it is switched away from is kept before it
//! is requeued, after which another CPU may take it up.

use crate::source::{code, order};

#[test]
fn kept_before_requeued_at_both_switches() {
    let keep = "crate::arch::keep_outgoing_user_state();";
    let preempt = code("process/scheduler/preemption/switch.rs");
    order(&preempt, &["save_fpu_state(curr_pid);", keep, "requeue(curr_pid);"]);
    let yielded = code("process/scheduler/preemption/yield_body/perform.rs");
    order(&yielded, &["save_fpu_state(pid);", keep, "requeue(pid);"]);
    let out = code("arch/outgoing.rs");
    order(
        &out,
        &[
            "pub(crate) fn keep_outgoing_user_state() {",
            "#[cfg(target_arch = \"aarch64\")]",
            "crate::arch::aarch64::fpu::save_outgoing();",
            "crate::arch::aarch64::context::keep_outgoing_tls();",
        ],
    );
}

#[test]
fn registers_live_at_el0_are_saved() {
    let fpu = code("arch/aarch64/fpu/switch.rs");
    order(
        &fpu,
        &["pub fn save_outgoing() {", "if slot.enabled && slot.dirty {", "save(&mut slot.ctx)"],
    );
    // Handed back live, EL0 may write them with no trap to say so.
    order(
        &fpu,
        &[
            "pub fn prepare_incoming() {",
            "if slot.valid {",
            "slot.enabled = true;",
            "slot.dirty = true;",
        ],
    );
    let tls = code("arch/aarch64/context/tls.rs");
    order(&tls, &["pub(crate) fn keep_outgoing_tls() {", "pcb.set_tls_base(read_user_tls());"]);
    assert!(tls.contains("asm!(\"mrs {}, tpidr_el0\""));
}
