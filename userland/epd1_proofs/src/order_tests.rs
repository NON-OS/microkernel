// NONOS Operating System (AGPL-3.0-or-later)
//! On every entry from EL0 through the trampoline, the speculation work
//! runs before the page's one indirect branch, from the same macro the
//! vectors use, so EL0's branch history steers no branch of the kernel's.

use crate::asm::{code, indirect, macro_body, op};
use crate::source::{block, SPECTRE_ENTRY, SPECTRE_WORK, TRAMPOLINE, VECTORS};

#[test]
fn the_work_runs_before_the_only_indirect_branch_from_el0() {
    let routine: Vec<String> =
        code(block(TRAMPOLINE, "from_el0:")).into_iter().filter(|l| !l.starts_with('.')).collect();
    let work = routine.iter().position(|l| l == "SPECTRE_WORK w1, w3").expect("the work");
    let branches: Vec<usize> = (0..routine.len()).filter(|&i| indirect(&routine[i])).collect();
    assert_eq!(branches, [routine.len() - 1], "one indirect branch, last");
    assert!(work < branches[0]);
    // The word is the data page's second, and an empty word skips to the end.
    let word = routine.iter().position(|l| l.starts_with("ldr     w1, __epd1_trampoline + 0x1008"));
    assert!(word.is_some_and(|w| w < work));
    assert!(routine.contains(&"cbz     w1, 9f".to_string()));
}

#[test]
fn the_work_itself_branches_only_directly() {
    let work = macro_body(SPECTRE_WORK, "SPECTRE_WORK");
    assert!(work.iter().any(|l| op(l) == "smc") && work.iter().any(|l| op(l) == "hvc"));
    assert!(work.iter().all(|l| !indirect(l)));
    assert_eq!(work.last().map(String::as_str), Some("9:"));
}

#[test]
fn the_vectors_run_the_same_work() {
    let entry = macro_body(VECTORS, "SPECTRE_FROM_EL0");
    assert_eq!(entry.last().map(String::as_str), Some("SPECTRE_WORK w9, w10"));
    assert!(code(VECTORS.lines()).contains(&"#include \"spectre_work.h\"".to_string()));
    assert!(code(TRAMPOLINE.lines()).contains(&"#include \"spectre_work.h\"".to_string()));
}

/// The trampoline's word is the vectors' word: every publish folds the
/// same CPU word into both, by the same `merge`.
#[test]
fn every_publish_feeds_the_trampolines_copy() {
    let publish = block(SPECTRE_ENTRY, "pub(super) fn publish(word: u32) {");
    let body: String = publish.concat().split_whitespace().collect();
    assert!(body.contains("Some(merge(now,word))"));
    assert!(body.contains("mirror_entry_word(|now|merge(now,word))"));
}
