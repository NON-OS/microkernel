// NONOS Operating System (AGPL-3.0-or-later)
//! Each defect the rule exists for, planted in a scratch tree and caught.

use crate::check::{compare, Finding};
use crate::ledger::parse;
use crate::scan::{count, names_x86_cfg, names_x86_path, Rule};
use crate::scratch::Scratch;

const GATE: &str = "#[cfg(target_arch = \"x86_64\")]\nmod port;\n";
const LEDGER: &str = "src/a.rs 1\n    port I/O. Elsewhere: refused.\n";

fn run(tree: &Scratch, ledger: &str) -> Vec<Finding> {
    compare(&parse(ledger).unwrap(), &count(tree.root(), Rule::Cfg).unwrap())
}

#[test]
fn an_agreeing_tree_passes() {
    let t = Scratch::new();
    t.put("src/a.rs", GATE);
    assert_eq!(run(&t, LEDGER), vec![]);
}

#[test]
fn a_new_x86_only_file_is_caught() {
    let t = Scratch::new();
    t.put("src/a.rs", GATE);
    t.put("src/b/c.rs", GATE);
    assert_eq!(run(&t, LEDGER), vec![Finding::Unlisted { path: "src/b/c.rs".into(), n: 1 }]);
}

#[test]
fn a_grown_file_is_caught() {
    let t = Scratch::new();
    t.put("src/a.rs", &format!("{GATE}{GATE}"));
    assert_eq!(run(&t, LEDGER), vec![Finding::Grown { path: "src/a.rs".into(), row: 1, n: 2 }]);
}

#[test]
fn a_stale_count_and_a_gone_file_are_caught() {
    let t = Scratch::new();
    t.put("src/a.rs", "mod port;\n");
    assert_eq!(run(&t, LEDGER), vec![Finding::Gone { path: "src/a.rs".into() }]);
    t.put("src/a.rs", GATE);
    let two = "src/a.rs 2\n    port I/O.\n";
    assert_eq!(run(&t, two), vec![Finding::Shrunk { path: "src/a.rs".into(), row: 2, n: 1 }]);
}

#[test]
fn a_zero_or_word_count_is_caught() {
    let t = Scratch::new();
    t.put("src/a.rs", GATE);
    let found = run(&t, "src/a.rs 0\n    why.\n");
    assert!(found.contains(&Finding::BadCount { path: "src/a.rs".into() }), "{found:?}");
}

#[test]
fn src_arch_is_left_out_and_spacing_does_not_hide_a_gate() {
    let t = Scratch::new();
    t.put("src/arch/x86_64/a.rs", GATE);
    t.put("src/a.rs", "#[cfg( target_arch=\"x86_64\" )]\nfn f() {}\n");
    assert_eq!(run(&t, LEDGER), vec![]);
}

#[test]
fn a_path_into_x86_64_counts_and_its_neighbours_do_not() {
    assert!(names_x86_path("use crate::arch::x86_64::vga::Color;"));
    assert!(names_x86_path("crate::arch::x86_64::cpu::halt()"));
    assert!(!names_x86_path("use crate::arch::aarch64::gic;"));
    assert!(!names_x86_path("use crate::arch::x86_64_compat;"));
    assert!(!names_x86_cfg("#[cfg(target_arch = \"aarch64\")]"));
}
