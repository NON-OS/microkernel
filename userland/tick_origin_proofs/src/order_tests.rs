// NONOS Operating System (AGPL-3.0-or-later)
//! The tick's origin on aarch64 and riscv64, read from the kernel's source:
//! taken from the level the interrupt came from, recorded before the tick
//! runs, into the one per-CPU flag the dead-thread switch and band wake read.

fn code(path: &str) -> String {
    let src = std::fs::read_to_string(format!("../../src/{path}")).unwrap();
    src.lines().map(|l| l.split("//").next().unwrap_or("").trim()).collect::<Vec<_>>().join("\n")
}

fn order(src: &str, marks: &[&str]) {
    let mut from = 0;
    for mark in marks {
        let at = src[from..].find(mark).unwrap_or_else(|| panic!("no `{mark}` after {from}"));
        from += at + mark.len();
    }
}

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
    order(
        &int,
        &[
            "InterruptCode::SupervisorTimer => {",
            "crate::process::accounting::set_tick_origin(frame.is_from_user());",
            "timer::handle_timer_interrupt()",
        ],
    );
    let timer = code("arch/riscv64/timer/interrupt.rs");
    order(&timer, &["pub fn handle_timer_interrupt() {", "preemption::tick::tick();"]);
}

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
