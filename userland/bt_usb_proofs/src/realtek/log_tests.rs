// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! No log line of the capsule carries an address, a name, a key or a
//! passkey: every `say` call, and every number it prints, is read from the
//! capsule's own source, and none names one. The passkey reaches Settings
//! by NBTC's PASSKEY alone.

use std::path::{Path, PathBuf};

const SECRET: [&str; 8] = ["passkey", "addr", "identity", "irk", "ltk", "key", "name", "peer"];

fn sources(dir: &Path, out: &mut Vec<PathBuf>) {
    for e in std::fs::read_dir(dir).expect("readable").flatten() {
        let p = e.path();
        if p.is_dir() {
            sources(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

/// The text of each call `open(...)` in `text`, string literals blanked.
fn calls<'a>(text: &'a str, open: &str) -> Vec<String> {
    let mut out = Vec::new();
    for (at, _) in text.match_indices(open) {
        let (mut depth, mut end) = (0i32, at);
        for (i, c) in text[at..].char_indices() {
            depth += match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };
            if c == ')' && depth == 0 {
                end = at + i;
                break;
            }
        }
        let call = &text[at + open.len()..end];
        let blank: Vec<&str> = call.split('"').step_by(2).collect();
        out.push(blank.join("").to_lowercase());
    }
    out
}

fn named(call: &str) -> Option<&'static str> {
    SECRET.into_iter().find(|s| call.contains(s))
}

fn capsule() -> Vec<(PathBuf, String)> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../capsule_driver_bt_usb/src");
    let mut files = Vec::new();
    sources(&root, &mut files);
    files.into_iter().map(|p| (p.clone(), std::fs::read_to_string(p).expect("text"))).collect()
}

#[test]
fn no_log_line_names_a_secret() {
    let files = capsule();
    assert!(files.len() > 60, "the capsule's source was not found");
    for (path, text) in &files {
        for call in calls(text, "say(&[").into_iter().chain(calls(text, "digits(")) {
            assert_eq!(named(&call), None, "{}: a log line carries {call}", path.display());
        }
    }
}

#[test]
fn the_rule_catches_the_passkey_line_it_replaced() {
    let old = r#"say(&[b"to pair, type ", &digits(u64::from(passkey), 6), b" then Enter"]);"#;
    assert_eq!(calls(old, "say(&[").iter().find_map(|c| named(c)), Some("passkey"));
    assert_eq!(calls(old, "digits(").iter().find_map(|c| named(c)), Some("passkey"));
}
