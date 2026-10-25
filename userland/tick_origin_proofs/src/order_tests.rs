// NONOS Operating System (AGPL-3.0-or-later)
//! The tick's origin on aarch64 and riscv64, read from the kernel's source:
//! taken from the level the interrupt came from and recorded before the
//! tick runs.

use crate::source::{code, order};

/// The 16 slots of the aarch64 vector table, in table order: each 0x80
/// apart, the IRQ of each group second (Arm ARM vector offsets, recalled).
fn vector_slots() -> Vec<String> {
    let src = std::fs::read_to_string("../../src/arch/aarch64/asm/vectors.S").unwrap();
    let table = src.split("__aarch64_vectors_el1:").nth(1).unwrap();
    let table = table.split(".size __aarch64_vectors_el1").next().unwrap();
    table.lines().filter_map(|l| l.trim().strip_prefix("b ")).map(|t| t.trim().into()).collect()
}

#[test]
fn aarch64_takes_it_from_the_vector_before_the_tick() {
    let slots = vector_slots();
    assert_eq!(slots.len(), 16);
    assert_eq!(slots[5], "__aarch64_irq_current", "current EL, SP_ELx, IRQ");
    assert_eq!(slots[9], "__aarch64_irq_lower", "lower EL, AArch64, IRQ");
    let asm = std::fs::read_to_string("../../src/arch/aarch64/asm/vectors.S").unwrap();
    assert!(asm.contains("LOWER_ENTRY __aarch64_irq_lower, aarch64_exc_irq_lower"));
    assert!(asm.contains("TRAMP_TARGET __aarch64_irq_lower_scrubbed"));
    order(&asm, &["__aarch64_irq_current:", "bl      aarch64_exc_irq_current"]);

    let irq = code("arch/aarch64/exceptions/handlers/irq.rs");
    order(&irq, &["fn aarch64_exc_irq_current(", "handle(frame, b\"IRQ EL1\", false)"]);
    order(&irq, &["fn aarch64_exc_irq_lower(", "handle(frame, b\"IRQ EL0\", true)"]);
    order(
        &irq,
        &[
            "fn handle(frame: &ExceptionFrame, tag: &[u8], from_user: bool) {",
            "if is_preemption_tick(acked.intid) {",
            "crate::process::accounting::set_tick_origin(from_user);",
            "let handled = dispatch_irq(acked.intid);",
        ],
    );
    let handler = code("arch/aarch64/timer/preemption/handler.rs");
    assert!(handler.contains("use crate::process::scheduler::preemption::tick;"));
    order(&handler, &["fn timer_tick(", "tick();"]);
}

#[test]
fn riscv64_takes_it_from_the_frame_before_the_tick() {
    let int = code("arch/riscv64/interrupts/handlers/interrupt.rs");
    assert!(int.contains(
        "InterruptCode::SupervisorTimer => timer::handle_timer_interrupt(frame.is_from_user()),"
    ));
    let timer = code("arch/riscv64/timer/interrupt.rs");
    order(
        &timer,
        &[
            "pub fn handle_timer_interrupt(from_user: bool) {",
            "crate::process::accounting::set_tick_origin(from_user);",
            "crate::process::scheduler::preemption::tick();",
        ],
    );
}
