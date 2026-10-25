// NONOS Operating System (AGPL-3.0-or-later)
//! The slips that are caught: a swap, a missing or extra letter, a near name.

use super::near;

#[test]
fn a_single_slip_is_caught() {
    assert_eq!(near("gerp"), Some("grep".into()));
    assert_eq!(near("hlep"), Some("help".into()));
    assert_eq!(near("mkdri"), Some("mkdir".into()));
}

#[test]
fn a_missing_letter_is_caught() {
    assert_eq!(near("cler"), Some("clear".into()));
    assert_eq!(near("tre"), Some("tree".into()));
}

#[test]
fn an_extra_letter_is_caught() {
    assert_eq!(near("treee"), Some("tree".into()));
    assert_eq!(near("catt"), Some("cat".into()));
}

/// The point of the feature: names carried over from another system.
#[test]
fn a_neighbouring_real_command_is_offered() {
    assert_eq!(near("historyy"), Some("history".into()));
}

#[test]
fn suggestions_are_stable_rather_than_order_dependent() {
    let first = near("gerp");
    for _ in 0..10 {
        assert_eq!(near("gerp"), first);
    }
}

/// Installed tools are candidates too: `tokie` should find `tokei`.
#[test]
fn installed_tools_are_offered_like_any_other_name() {
    assert_eq!(near("tokie"), Some("tokei".into()));
}
