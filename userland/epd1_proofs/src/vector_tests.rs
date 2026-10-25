// NONOS Operating System (AGPL-3.0-or-later)
//! The trampoline's targets land where the kernel's own table does, an entry
//! from EL0 through its `_scrubbed` door, which only the targets name; and
//! every return to EL0 takes the test.

use crate::asm::{code, macro_body};
use crate::source::{block, ENTER_USER, RESUME_USER, VECTORS};

#[test]
fn every_slot_lands_where_the_kernels_table_sends_it() {
    let lines = code(block(VECTORS, "__aarch64_vectors_el1:"));
    let table: Vec<&str> = lines.iter().filter_map(|l| l.strip_prefix("b   ")).collect();
    let lines = code(block(VECTORS, "__aarch64_tramp_targets:"));
    let targets: Vec<&str> = lines.iter().filter_map(|l| l.strip_prefix("TRAMP_TARGET ")).collect();
    assert_eq!((table.len(), targets.len()), (16, 16));
    for (slot, (t, target)) in table.iter().zip(&targets).enumerate() {
        let want = if (8..12).contains(&slot) { format!("{t}_scrubbed") } else { t.to_string() };
        assert_eq!(*target, want, "slot {slot}");
    }
    let doors = code(VECTORS.lines()).iter().filter(|l| l.contains("_scrubbed")).count();
    assert_eq!(doors, 4 + 1, "only the four targets and the macro's own label");
}

#[test]
fn the_scrubbed_door_skips_only_the_work() {
    let body: Vec<String> =
        macro_body(VECTORS, "LOWER_ENTRY").into_iter().filter(|l| !l.starts_with('.')).collect();
    let at = |want: &str| body.iter().position(|l| l == want).expect(want);
    let door = at("\\name\\()_scrubbed:");
    let main = at("\\name:");
    assert_eq!(body[door + 1..main], ["SAVE_GPRS", "SAVE_SYSREGS_USER", "b       8f"]);
    assert_eq!(body[main + 1..main + 4], ["SAVE_GPRS", "SAVE_SYSREGS_USER", "SPECTRE_FROM_EL0"]);
    assert_eq!(body[main + 4], "8:  PAC_FROM_EL0");
}

#[test]
fn every_return_to_el0_takes_the_test() {
    for (name, text) in [("enter_user", ENTER_USER), ("resume_user", RESUME_USER)] {
        let all = code(text.lines());
        assert!(all.contains(&"#include \"epd1.h\"".to_string()), "{name}");
        assert_eq!(all.iter().filter(|l| *l == "EXIT_TO_EL0").count(), 1, "{name}");
        assert!(!all.iter().any(|l| l == "eret"), "{name} returns past the test");
    }
    let body = macro_body(VECTORS, "LOWER_ENTRY");
    assert_eq!(body.iter().filter(|l| *l == "EXIT_TO_EL0").count(), 1);
    assert!(!body.iter().any(|l| l == "eret"));
    for kind in ["sync", "irq", "fiq", "serror"] {
        let line = format!("LOWER_ENTRY __aarch64_{kind}_lower, aarch64_exc_{kind}_lower");
        assert!(code(VECTORS.lines()).contains(&line), "{kind}");
    }
}
