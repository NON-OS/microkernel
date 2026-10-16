// NONOS Operating System (AGPL-3.0-or-later)
//! A deadline handed off before the CPU's timer can stop and taken back
//! after, and a timer-stop state entered only with a frame ready.

use crate::order_tests::{code, order};

const HAND: &str = include_str!("../../../src/arch/aarch64/timer/broadcast/hand.rs");
const START: &str = include_str!("../../../src/arch/aarch64/timer/broadcast/start.rs");
const CALL: &str = include_str!("../../../src/arch/aarch64/idle_states/call.rs");
const SELECT: &str = include_str!("../../../src/arch/aarch64/idle_states/select.rs");
const ALLOW: &str = include_str!("../../../src/arch/aarch64/idle_states/allow.rs");

#[test]
fn a_deadline_is_handed_off_before_the_state_and_taken_back_after() {
    let call = code(CALL);
    order(
        &call,
        &[
            "if state.timer_stops && !broadcast::hand_off(cpu, lead) {",
            "return Err(PsciError::NotPresent);",
            "psci::cpu_suspend(",
            "powerdown::enter(cpu, state.param)",
            "if state.timer_stops {\nbroadcast::take_back(cpu);",
            "entered\n}",
        ],
    );
    assert!(call.contains("let lead = microseconds_to_ticks(u64::from(state.exit_us));"));
    assert!(code(HAND).contains("PhysicalTimer::get_compare_value().saturating_sub(lead)"));
}

#[test]
fn a_timer_stop_state_only_with_a_frame() {
    let select = code(SELECT);
    assert!(select.contains("(allow.timer_stop || !state.timer_stops)"));
    assert!(code(ALLOW).contains("timer_stop: broadcast::ready()"));
    assert!(code(HAND).contains("FRAME.get().is_some()"));
    // The frame is ready only once its counter is checked and its
    // interrupt taken.
    order(
        &code(START),
        &[
            "if seen < before || seen > after {",
            "register_irq_handler(",
            "FRAME.call_once(",
            "enable_irq(",
        ],
    );
}
