// NONOS Operating System (AGPL-3.0-or-later)
//! The completion words are read from exactly the parts listed, and complete.rs
//! joins exactly the groups the completion test joins; a new part or group
//! fails here before it can go unread.

use super::{COMMANDS_LINE, WORD_PARTS};

const EVENT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../capsule_terminal/src/event");

#[test]
fn the_word_parts_are_the_directorys_word_files() {
    let mut found: Vec<String> = std::fs::read_dir(EVENT)
        .expect("event directory")
        .filter_map(|e| e.ok()?.file_name().into_string().ok())
        .filter(|n| n.starts_with("complete_words") && n.ends_with(".rs"))
        .collect();
    found.sort();
    let mut listed: Vec<String> = WORD_PARTS.iter().map(|s| s.to_string()).collect();
    listed.sort();
    assert_eq!(found, listed);
}

#[test]
fn complete_rs_joins_the_groups_the_test_joins() {
    let src = include_str!("../../../capsule_terminal/src/event/complete.rs");
    assert_eq!(src.matches("const COMMANDS").count(), 1);
    assert!(src.contains(COMMANDS_LINE), "COMMANDS is no longer the five groups");
}
