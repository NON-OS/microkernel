// NONOS Operating System (AGPL-3.0-or-later)
//! What the terminal says about itself, held to what it does.
//!
//! `help` named eleven commands that `help <command>` answered with "no usage
//! entry yet", among them five it advertised as text commands (`sort`, `uniq`,
//! `cut`, `nl`, `tac`, `rev`) that ran only after a `|` and said "not found"
//! on their own. `whoami` and `version` printed a fixed signing claim and a
//! namespace no manifest carries. These read the sources and the real
//! registry parser, so neither side can drift.

use crate::help_layout::GROUPS;
use crate::receipt_own::{describe, entry_for, own_signer, signer, ENTRY_LEN};

const HELP_ONE: &str = include_str!("../../capsule_terminal/src/command/builtin/help_one.rs");
const EXEC: &str = include_str!("../../capsule_terminal/src/command/dispatch/exec.rs");
const NOX: &str = include_str!("../../capsule_terminal/src/command/builtin/nox/dispatch.rs");
const WHOAMI: &str = include_str!("../../capsule_terminal/src/command/builtin/whoami.rs");
const VERSION: &str = include_str!("../../capsule_terminal/src/command/builtin/version.rs");
const SPLASH: &str = include_str!("../../capsule_terminal/src/paint/fetch.rs");

/// Every `b"..."` in the usage table, in order; the table is triples.
fn usage_names() -> Vec<String> {
    let from = HELP_ONE.find("const USAGE").unwrap();
    let to = from + HELP_ONE[from..].find("\n];").unwrap();
    let strings: Vec<String> = HELP_ONE[from..to]
        .split("b\"")
        .skip(1)
        .filter_map(|p| p.split_once('"').map(|(s, _)| s.to_string()))
        .collect();
    assert_eq!(strings.len() % 3, 0, "the usage table is not triples");
    strings.chunks(3).map(|t| t[0].clone()).collect()
}

fn grouped() -> Vec<String> {
    GROUPS
        .iter()
        .flat_map(|(_, list)| std::str::from_utf8(list).unwrap().split_whitespace())
        .map(str::to_string)
        .collect()
}

#[test]
fn every_command_help_lists_has_a_usage_page() {
    let names = usage_names();
    assert!(names.len() >= 60, "parsed {} entries", names.len());
    for name in grouped() {
        assert!(names.contains(&name), "help lists {name} and help {name} has no page");
    }
}

/// Answered by the shell's own table or the nox table, by name.
fn dispatched(name: &str) -> bool {
    let arm = format!("b\"{name}\"");
    EXEC.lines().chain(NOX.lines()).any(|l| l.contains(&arm) && l.contains("=>"))
        || EXEC.contains(&format!("{arm} |"))
        || NOX.contains(&format!("{arm} |"))
}

#[test]
fn every_command_help_lists_runs_on_its_own() {
    for name in grouped() {
        assert!(dispatched(&name), "help lists {name} and nothing dispatches it");
    }
}

#[test]
fn no_page_promises_what_the_command_does_not_do() {
    assert!(!HELP_ONE.contains("or update their time"), "touch updates no time");
    assert!(!HELP_ONE.contains("let a stopped job carry on"), "nothing is ever stopped");
    assert!(!HELP_ONE.contains("the registered services and the pids"), "service takes one name");
    assert!(!HELP_ONE.contains("no argument goes home\""), "cd's home is $HOME");
    assert!(!HELP_ONE.contains("yet"));
}

fn registry(entries: &[(u32, u8, u8)]) -> Vec<u8> {
    let mut out = Vec::new();
    for &(pid, first, authority) in entries {
        let mut e = vec![0u8; ENTRY_LEN];
        e[..4].copy_from_slice(&pid.to_be_bytes());
        e[4] = first;
        e[5] = 0xab;
        e[44] = authority;
        out.extend_from_slice(&e);
    }
    out
}

#[test]
fn who_signed_this_terminal_comes_from_its_own_registry_entry() {
    let regs = registry(&[(7, 0x10, 0), (42, 0x1f, 255), (9, 0x00, 3)]);
    assert_eq!(entry_for(&regs, 42).map(|e| e[44]), Some(255));
    assert!(entry_for(&regs, 5).is_none());
    assert_eq!(signer(0), b"vendor");
    assert_eq!(signer(255), b"publisher");
    assert_eq!(signer(3), b"developer");
    let line = String::from_utf8(describe(entry_for(&regs, 42).unwrap())).unwrap();
    assert_eq!(line, "publisher, measurement 1fab00000000");
}

#[test]
fn an_unknown_signer_is_said_to_be_unknown() {
    let regs = registry(&[(7, 0, 0)]);
    let missing = String::from_utf8(own_signer(Ok(&regs), 8)).unwrap();
    assert!(missing.starts_with("unknown: this terminal is not in"));
    let refused = String::from_utf8(own_signer(Err(-1), 7)).unwrap();
    assert!(refused.starts_with("unknown: the kernel did not hand over"));
}

#[test]
fn identity_lines_carry_no_fixed_signing_claim() {
    for (name, src) in [("whoami", WHOAMI), ("version", VERSION), ("splash", SPLASH)] {
        assert!(!src.contains("Ed25519"), "{name} states a signature scheme as fact");
        assert!(!src.contains("app.terminal0"), "{name} names a namespace no manifest has");
        assert!(src.contains("own_line"), "{name} does not read the registry");
    }
    assert!(!VERSION.contains("b\"NONOS terminal v0.1\""));
    assert!(VERSION.contains("include_str!(\"../../../../../VERSION\")"));
}
