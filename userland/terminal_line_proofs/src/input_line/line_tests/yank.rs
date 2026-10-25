// NONOS Operating System (AGPL-3.0-or-later)
//! Ctrl-U empties the line into the ring and Ctrl-Y gives it back, within the line's bounds.

use super::{text, typed};
use crate::line::Line;

// Ctrl-U, then Ctrl-Y.

#[test]
fn a_killed_line_comes_back_whole() {
    let mut l = typed("cargo build --release");
    assert!(l.kill_line());
    assert_eq!(text(&l), "");
    assert!(l.yank());
    assert_eq!(text(&l), "cargo build --release");
}

#[test]
fn killing_an_empty_line_does_not_erase_what_is_held() {
    let mut l = typed("something worth keeping");
    l.kill_line();
    // A second Ctrl-U on the now empty line must not overwrite the ring with
    // nothing, because that is the exact moment before someone presses Ctrl-Y.
    assert!(!l.kill_line());
    assert!(l.yank());
    assert_eq!(text(&l), "something worth keeping");
}

#[test]
fn yanking_with_nothing_held_is_a_no_op() {
    let mut l = Line::new();
    assert!(!l.yank());
    assert_eq!(text(&l), "");
}

#[test]
fn held_text_survives_being_yanked_so_it_can_be_placed_twice() {
    let mut l = typed("abc");
    l.kill_line();
    l.yank();
    l.yank();
    assert_eq!(text(&l), "abcabc");
}

/// The buffer is fixed width, and a yank is the one path that inserts many
/// bytes at once. It has to stop at the edge rather than run past it.
#[test]
fn yanking_into_a_nearly_full_line_stays_in_bounds() {
    let wide = "x".repeat(crate::term::dimensions::LINE_MAX);
    let mut l = typed(&wide);
    assert_eq!(l.len, crate::term::dimensions::LINE_MAX);
    l.kill_line();
    l.yank();
    assert!(l.len <= crate::term::dimensions::LINE_MAX);
    l.yank();
    assert!(l.len <= crate::term::dimensions::LINE_MAX);
    assert!(l.cursor <= l.len);
}
