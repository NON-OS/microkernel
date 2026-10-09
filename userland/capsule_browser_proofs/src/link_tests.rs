// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! What a clicked link does, how the address text scrolls, and which late
//! results and pointer moves count.

use crate::browser::omnibox::parts::{state_restyle, Restyle, ACTIVE, FOCUS, HOVER};
use crate::browser::omnibox::{hover_update, link_action, should_commit, text_offset};
use crate::browser::omnibox::{same_document, LinkAction};

const PAGE: &str = "https://nonos.software/docs";

#[test]
fn links_navigate_scroll_or_do_nothing() {
    let anchor = link_action(PAGE, "https://nonos.software/docs#install");
    assert_eq!(anchor, LinkAction::Anchor("install".into()));
    let next = link_action(PAGE, "https://nonos.software/blog");
    assert_eq!(next, LinkAction::Navigate("https://nonos.software/blog".into()));
    assert_eq!(link_action(PAGE, "javascript:void(0)"), LinkAction::Ignore);
    assert_eq!(link_action(PAGE, "mailto:a@b.org"), LinkAction::Ignore);
    assert_eq!(link_action(PAGE, "tel:+100"), LinkAction::Ignore);
    let other = link_action("", "https://nonos.software/#x");
    assert_eq!(other, LinkAction::Navigate("https://nonos.software/#x".into()));
    assert!(same_document("https://a.org/x#1", "https://a.org/x"));
    assert!(!same_document("", "https://a.org/x"));
}

#[test]
fn address_text_scrolls_to_keep_the_caret_in_view() {
    assert_eq!(text_offset(40, 120, 300, 0), 0, "text that fits never scrolls");
    assert_eq!(text_offset(1000, 1000, 300, 0), 704, "caret at the end of long text");
    assert_eq!(text_offset(150, 1000, 300, 100), 100, "a visible caret keeps the offset");
    assert_eq!(text_offset(20, 1000, 300, 400), 20, "caret left of the view pulls it back");
}

#[test]
fn only_the_current_navigation_commits() {
    assert!(should_commit(7, 7));
    assert!(!should_commit(8, 7), "stopped or replaced");
}

#[test]
fn only_a_changed_hover_repaints() {
    let mut slot = None;
    assert!(hover_update(&mut slot, Some("/a")));
    assert!(!hover_update(&mut slot, Some("/a")));
    assert!(hover_update(&mut slot, Some("/b")));
    assert!(hover_update(&mut slot, None));
    assert!(!hover_update(&mut slot, None));
}

#[test]
fn a_state_change_restyles_only_when_a_rule_tests_it() {
    assert_eq!(state_restyle(HOVER, HOVER | ACTIVE), Restyle::Relayout);
    assert_eq!(state_restyle(HOVER, FOCUS), Restyle::Nothing);
    assert_eq!(state_restyle(FOCUS, 0), Restyle::Repaint);
    assert_eq!(state_restyle(ACTIVE, ACTIVE), Restyle::Relayout);
    assert_eq!(state_restyle(0, HOVER), Restyle::Nothing);
}
