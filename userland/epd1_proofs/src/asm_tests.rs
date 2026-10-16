// NONOS Operating System (AGPL-3.0-or-later)
//! The order the trampoline's assembly keeps, read from the sources: EPD1
//! cleared before the first load on every entry from EL0 and set before the
//! TLB is emptied on the way out, every slot landing where the kernel's own
//! table sends it, and every return to EL0 taking the test.

use crate::source::{block, define, ENTER_USER, RESUME_USER, TRAMPOLINE, VECTORS};

/// The instruction or directive on each line, comments and blanks gone.
fn code<'a>(lines: impl IntoIterator<Item = &'a str>) -> Vec<String> {
    let strip = |l: &str| l.split("/*").next().unwrap_or("").trim().to_string();
    let skip = |l: &&str| l.trim_start().starts_with('*') || l.trim_start().starts_with("/*");
    lines.into_iter().filter(|l| !skip(l)).map(strip).filter(|l| !l.is_empty()).collect()
}

fn mem(op: &str) -> bool {
    ["ldr", "str", "ldp", "stp"].iter().any(|m| op.split_whitespace().next() == Some(m))
}

#[test]
fn entries_from_el0_clear_epd1_before_the_first_load() {
    let slots: Vec<(u32, u32)> = code(TRAMPOLINE.lines())
        .iter()
        .filter_map(|l| l.strip_prefix("TRAMP_ENTRY "))
        .map(|a| a.split_once(", ").map(|(n, f)| (n.parse().unwrap(), f.parse().unwrap())).unwrap())
        .collect();
    assert_eq!(slots, (0..16).map(|n| (n, u32::from(n >= 8))).collect::<Vec<_>>());
    let body = code(block(TRAMPOLINE, ".macro TRAMP_ENTRY slot, from_el0"));
    let at = |want: &str| body.iter().position(|l| l.starts_with(want)).expect(want);
    let first_load = body.iter().position(|l| mem(l)).expect("the targets' load");
    assert!(at("bic") < at("msr     tcr_el1") && at("msr     tcr_el1") < at("isb"));
    assert!(at("isb") < at(".endif") && at(".endif") < first_load);
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

#[test]
fn every_slot_lands_where_the_kernels_table_sends_it() {
    let lines = code(block(VECTORS, "__aarch64_vectors_el1:"));
    let table: Vec<&str> = lines.iter().filter_map(|l| l.strip_prefix("b   ")).collect();
    let lines = code(block(VECTORS, "__aarch64_tramp_targets:"));
    let targets: Vec<&str> = lines.iter().filter_map(|l| l.strip_prefix("TRAMP_TARGET ")).collect();
    assert_eq!(table.len(), 16);
    assert_eq!(table, targets);
}

#[test]
fn every_return_to_el0_takes_the_test() {
    for (name, text) in [("enter_user", ENTER_USER), ("resume_user", RESUME_USER)] {
        let all = code(text.lines());
        assert!(all.contains(&"#include \"epd1.h\"".to_string()), "{name}");
        assert_eq!(all.iter().filter(|l| *l == "EXIT_TO_EL0").count(), 1, "{name}");
        assert!(!all.iter().any(|l| l == "eret"), "{name} returns past the test");
    }
    for kind in ["sync", "irq", "fiq", "serror"] {
        let body = code(block(VECTORS, &format!("__aarch64_{kind}_lower:")));
        assert_eq!(body.iter().filter(|l| *l == "EXIT_TO_EL0").count(), 1, "{kind}");
        assert!(!body.iter().any(|l| l == "eret"), "{kind} returns past the test");
    }
}
