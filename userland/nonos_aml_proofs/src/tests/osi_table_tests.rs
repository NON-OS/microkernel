// NONOS Operating System (AGPL-3.0-or-later)
//! The _OSI table's own rules: one row per string; nothing answered that
//! NONOS does not honour at least in part; every partial or refused row
//! names the firmware paths it would open that NONOS lacks; every row has
//! its reason; and the interpreter answers exactly the rows marked so.

use super::osi;
use crate::exec::{rows, Group, Honour, Personality};

#[test]
fn every_string_has_one_row_and_a_reason() {
    let all: Vec<_> = rows().collect();
    for (i, r) in all.iter().enumerate() {
        assert!(all[..i].iter().all(|o| o.string != r.string), "{:?} twice", r.string);
        assert!(!r.why.is_empty(), "{:?} has no reason", r.string);
    }
    assert!(all.iter().any(|r| r.group == Group::Feature));
    assert!(all.iter().any(|r| r.group == Group::Vendor));
}

#[test]
fn nothing_is_answered_that_is_not_honoured() {
    for r in rows().filter(|r| r.answered) {
        assert_ne!(r.honour, Honour::No, "{:?}", r.string);
    }
}

#[test]
fn every_gap_is_named() {
    for r in rows() {
        assert_eq!(r.honour == Honour::Yes, r.unlocks.is_empty(), "{:?}", r.string);
    }
}

#[test]
fn recalled_firmware_behaviour_never_passes_as_checked() {
    assert!(rows().all(|r| !r.verified), "a row claims a source no one read");
}

#[test]
fn the_interpreter_answers_exactly_the_rows_marked_answered() {
    for r in rows() {
        let want = if r.answered { u64::MAX } else { 0 };
        assert_eq!(osi(Personality::DEFAULT, r.string), want, "{:?}", r.string);
    }
}
