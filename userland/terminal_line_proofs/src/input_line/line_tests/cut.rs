// NONOS Operating System (AGPL-3.0-or-later)
//! Ctrl-W and Ctrl-K cut into the same ring, and Ctrl-D deletes forward.

use super::{text, typed};

// Ctrl-W and Ctrl-K feed the same ring.

#[test]
fn a_cut_word_comes_back() {
    let mut l = typed("git commit --amend");
    assert!(l.delete_word());
    assert_eq!(text(&l), "git commit ");
    assert!(l.yank());
    assert_eq!(text(&l), "git commit --amend");
}

#[test]
fn a_cut_tail_comes_back() {
    let mut l = typed("rm -rf /important");
    for _ in 0..10 {
        l.move_left();
    }
    assert!(l.kill_to_end());
    assert_eq!(text(&l), "rm -rf ");
    assert!(l.yank());
    assert_eq!(text(&l), "rm -rf /important");
}

#[test]
fn yank_inserts_at_the_caret_not_at_the_end() {
    let mut l = typed("one two");
    l.delete_word(); // holds "two"
    assert_eq!(text(&l), "one ");
    l.move_home();
    l.yank();
    assert_eq!(text(&l), "twoone ");
}

// Ctrl-D.

#[test]
fn delete_removes_forward_and_leaves_the_caret() {
    let mut l = typed("abc");
    l.move_home();
    assert!(l.delete());
    assert_eq!(text(&l), "bc");
    assert_eq!(l.cursor, 0);
}

#[test]
fn delete_at_the_end_does_nothing() {
    let mut l = typed("abc");
    assert!(!l.delete());
    assert_eq!(text(&l), "abc");
}
