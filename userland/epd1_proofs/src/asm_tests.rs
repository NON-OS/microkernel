// NONOS Operating System (AGPL-3.0-or-later)
//! The trampoline's own order, read from its source: EPD1 cleared before
//! any load on every entry from EL0, set before the TLB is emptied on the
//! way out, and the page one page alone.

use crate::asm::{code, macro_body, mem};
use crate::source::{define, TRAMPOLINE};

#[test]
fn entries_from_el0_clear_epd1_before_any_load() {
    let slots: Vec<(u32, u32)> = code(TRAMPOLINE.lines())
        .iter()
        .filter_map(|l| l.strip_prefix("TRAMP_ENTRY "))
        .map(|a| a.split_once(", ").map(|(n, f)| (n.parse().unwrap(), f.parse().unwrap())).unwrap())
        .collect();
    assert_eq!(slots, (0..16).map(|n| (n, u32::from(n >= 8))).collect::<Vec<_>>());
    let body = macro_body(TRAMPOLINE, "TRAMP_ENTRY");
    let at = |want: &str| body.iter().position(|l| l.starts_with(want)).expect(want);
    let from_el0 = &body[at(".if")..at(".else")];
    let order =
        ["mrs     x30, tcr_el1", "bic", "msr     tcr_el1", "isb", "mov", "b       from_el0"];
    let mut seen = from_el0.iter().skip(1);
    for want in order {
        assert!(seen.any(|l| l.starts_with(want)), "{want} out of order in the EL0 slots");
    }
    assert!(from_el0.iter().all(|l| !mem(l)), "an EL0 slot loads before the routine");
}

#[test]
fn the_exit_sets_epd1_then_empties_the_tlb_and_loads_nothing() {
    assert_eq!(define("EPD1_EXIT_OFFSET"), 0x800);
    let all = code(TRAMPOLINE.lines());
    let from = all.iter().position(|l| l == ".balign EPD1_EXIT_OFFSET").expect("the exit");
    let exit = &all[from + 1..];
    let want = ["hint    #36", "mrs     x30, tcr_el1", "orr     x30, x30, #TCR_EPD1"];
    let more = ["msr     tcr_el1, x30", "isb", "tlbi    vmalle1", "dsb     nsh", "isb"];
    assert_eq!(exit[..8], want.iter().chain(&more).map(|s| s.to_string()).collect::<Vec<_>>()[..]);
    let eret = exit.iter().position(|l| l == "eret").expect("eret");
    assert!(exit[..eret].iter().all(|l| !mem(l)));
    assert_eq!(all.iter().filter(|l| *l == ".balign 0x1000").count(), 2, "one page, alone");
}
