// NONOS Operating System (AGPL-3.0-or-later)
//! What the terminal says about itself, held to what it does.
//!
//! `help` named eleven commands that `help <command>` answered with "no usage
//! entry yet", among them five it advertised as text commands (`sort`, `uniq`,
//! `cut`, `nl`, `tac`, `rev`) that ran only after a `|` and said "not found"
//! on their own. `whoami` and `version` printed a fixed signing claim and a
//! namespace no manifest carries. The pages and the dispatch are held in
//! tables/help_pages_tests.rs; these hold who the terminal says it is to the
//! real registry parser, and its identity lines to what they may claim.

use crate::receipt_own::{describe, entry_for, own_signer, signer, ENTRY_LEN};

const WHOAMI: &str = include_str!("../../capsule_terminal/src/command/builtin/whoami.rs");
const VERSION: &str = include_str!("../../capsule_terminal/src/command/builtin/version.rs");
const SPLASH: &str = include_str!("../../capsule_terminal/src/paint/fetch.rs");

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
