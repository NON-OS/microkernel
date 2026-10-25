// NONOS Operating System (AGPL-3.0-or-later)
//! nonos-ci/arch-reach/capsules.txt names capsules the catalogue has and
//! user targets the tree has, each once, so an excuse cannot outlive its
//! capsule or name a target the lane never builds.

use crate::ledger::{parse, Row};

const TARGETS: [&str; 3] = ["x86_64-nonos-user", "aarch64-nonos-user", "riscv64-nonos-user"];
const TARGET_FILES: [&str; 3] = [
    "../../userland/x86_64-nonos-user.json",
    "../../userland/aarch64-nonos-user.json",
    "../../userland/riscv64-nonos-user.json",
];

/// Every capsule directory the catalogue names (tools/nix/capsules.json).
fn catalogue() -> Vec<String> {
    let text = std::fs::read_to_string("../../tools/nix/capsules.json").unwrap();
    let dirs: Vec<String> = text
        .lines()
        .filter_map(|l| l.trim().strip_prefix("\"dir\": \""))
        .filter_map(|l| l.strip_suffix("\","))
        .map(String::from)
        .collect();
    assert!(dirs.len() > 100, "the catalogue was not read: {}", dirs.len());
    dirs
}

/// Why a row is wrong, or None.
fn refuse(dirs: &[String], row: &Row) -> Option<String> {
    if !dirs.contains(&row.key) {
        return Some(format!("{}: no such capsule", row.key));
    }
    let targets: Vec<&str> = row.value.split(',').collect();
    for (i, t) in targets.iter().enumerate() {
        if !TARGETS.contains(t) {
            return Some(format!("{}: no user target {t}", row.key));
        }
        if targets[..i].contains(t) {
            return Some(format!("{}: {t} named twice", row.key));
        }
    }
    None
}

fn refusals(text: &str) -> Vec<String> {
    let rows = parse(text).unwrap_or_else(|e| panic!("capsules.txt: {e:?}"));
    let dirs = catalogue();
    rows.iter().filter_map(|r| refuse(&dirs, r)).collect()
}

#[test]
fn every_excused_capsule_and_target_exists() {
    assert!(TARGET_FILES.iter().all(|f| std::path::Path::new(f).is_file()));
    let text = std::fs::read_to_string("../../nonos-ci/arch-reach/capsules.txt").unwrap();
    assert_eq!(refusals(&text), Vec::<String>::new());
}

#[test]
fn a_row_for_a_gone_capsule_is_refused() {
    let found = refusals("userland/capsule_gone aarch64-nonos-user\n    why\n");
    assert_eq!(found, vec!["userland/capsule_gone: no such capsule".to_string()]);
}

#[test]
fn an_unknown_or_repeated_target_is_refused() {
    let row = |v: &str| format!("userland/capsule_nonos_install {v}\n    why\n");
    assert_eq!(refusals(&row("mips-nonos-user")).len(), 1);
    assert_eq!(refusals(&row("aarch64-nonos-user,aarch64-nonos-user")).len(), 1);
    assert_eq!(refusals(&row("aarch64-nonos-user,riscv64-nonos-user")), Vec::<String>::new());
}
