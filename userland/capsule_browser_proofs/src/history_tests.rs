// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! Session history: new visits drop the forward entries, a reload or a
//! redirect rewrites the entry in place, and the list is bounded.

use crate::browser::omnibox::parts::HISTORY_CAP;
use crate::browser::omnibox::History;

fn visited(urls: &[&str]) -> History {
    let mut h = History::new();
    urls.iter().for_each(|u| h.push(u));
    h
}

#[test]
fn push_back_forward_and_truncation() {
    let mut h = visited(&["a", "b", "c"]);
    assert_eq!(h.current(), Some("c"));
    assert!(h.can_back() && !h.can_forward());
    assert_eq!(h.back(), Some("b"));
    assert_eq!(h.back(), Some("a"));
    assert_eq!(h.back(), None);
    assert_eq!(h.forward(), Some("b"));
    h.push("d");
    assert_eq!(h.entries, ["a", "b", "d"]);
    assert!(!h.can_forward());
    h.push("d");
    assert_eq!(h.entries.len(), 3, "loading the shown address adds nothing");
}

#[test]
fn reload_and_redirect_replace_in_place() {
    let mut h = visited(&["a", "b"]);
    h.replace("b");
    assert_eq!(h.entries, ["a", "b"], "reload keeps one entry");
    h.back();
    h.replace("a2");
    assert_eq!(h.entries, ["a2", "b"], "a redirect after Back rewrites that entry");
    assert_eq!(h.index, 0);
    let mut empty = History::new();
    empty.replace("x");
    assert_eq!(empty.entries, ["x"]);
}

#[test]
fn history_is_capped_at_a_hundred_entries() {
    let mut h = History::new();
    for i in 0..250 {
        h.push(&alloc::format!("u{i}"));
    }
    assert_eq!(HISTORY_CAP, 100);
    assert_eq!(h.entries.len(), 100);
    assert_eq!(h.current(), Some("u249"));
    assert_eq!(h.entries[0], "u150");
    assert_eq!(h.index, 99);
}
