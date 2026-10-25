// NONOS Operating System (AGPL-3.0-or-later)
//! The real tree against the real ledgers: every x86_64-only line outside
//! src/arch is accounted for, and no row is stale.

use std::path::Path;

use crate::check::compare;
use crate::ledger::parse;
use crate::scan::{count, names_x86_cfg, names_x86_path};

/// The root `scan::count` walks from; it reads `../../src`.
const ROOT: &str = "../..";
const SRC: &str = "../../src";

fn holds(ledger: &str, hit: fn(&str) -> bool) {
    assert!(Path::new(SRC).is_dir(), "the kernel's source is not here");
    let text = std::fs::read_to_string(ledger).unwrap();
    let rows = parse(&text).unwrap_or_else(|e| panic!("{ledger}: {e:?}"));
    let tree = count(Path::new(ROOT), hit).unwrap();
    let found = compare(&rows, &tree);
    let lines: Vec<String> = found.iter().map(|f| format!("  {f}")).collect();
    assert!(found.is_empty(), "{ledger} disagrees with the tree:\n{}", lines.join("\n"));
}

#[test]
fn every_x86_64_cfg_outside_arch_says_why() {
    holds("../../nonos-ci/arch-reach/cfg-x86_64.txt", names_x86_cfg);
}

#[test]
fn every_x86_64_path_outside_arch_says_why() {
    holds("../../nonos-ci/arch-reach/arch-x86_64-uses.txt", names_x86_path);
}

#[test]
fn the_tree_is_really_read() {
    // A walk that found nothing would agree with an empty ledger; the tree
    // has well over a hundred such lines, so this guards the walk itself.
    let tree = count(Path::new(ROOT), names_x86_cfg).unwrap();
    assert!(tree.values().sum::<usize>() > 100, "only {} lines", tree.len());
    assert!(!tree.keys().any(|k| k.starts_with("src/arch/")));
}
