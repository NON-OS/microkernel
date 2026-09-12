// NONOS Operating System (AGPL-3.0-or-later)
//! What the line editor has to do, written as the key sequences people type.
//!
//! The kill keys are the reason this file exists. Ctrl-U, Ctrl-W and Ctrl-K
//! used to destroy text with no way back, and losing a typed line to one
//! keystroke is the failure a terminal is least forgiven for.

use crate::line::Line;

fn typed(s: &str) -> Line {
    let mut l = Line::new();
    for b in s.bytes() {
        l.insert(b);
    }
    l
}

fn text(l: &Line) -> String {
    String::from_utf8(l.as_bytes().to_vec()).unwrap()
}

#[test]
fn typing_leaves_the_caret_at_the_end() {
    let l = typed("ls -la /data");
    assert_eq!(text(&l), "ls -la /data");
    assert_eq!(l.cursor, 12);
}

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

// Word motion, and its agreement with the word kill.

#[test]
fn word_left_skips_the_gap_and_lands_on_the_word() {
    let mut l = typed("ls -la /data");
    assert!(l.move_word_left());
    assert_eq!(l.cursor, 7); // '/data'
    assert!(l.move_word_left());
    assert_eq!(l.cursor, 3); // '-la'
}

#[test]
fn word_right_lands_on_the_next_word_not_on_the_space() {
    let mut l = typed("ls -la /data");
    l.move_home();
    assert!(l.move_word_right());
    assert_eq!(l.cursor, 3);
    assert!(l.move_word_right());
    assert_eq!(l.cursor, 7);
}

/// Ctrl-W must delete exactly what Ctrl-Left would have skipped, or the two
/// keys disagree about where a word begins and the editor feels arbitrary.
#[test]
fn cut_word_and_word_left_agree_on_the_boundary() {
    for line in ["ls -la /data", "a  b   c", "one", "  leading", "trailing  "] {
        let mut nav = typed(line);
        let mut cut = typed(line);
        let moved = nav.move_word_left();
        let removed = cut.delete_word();
        assert_eq!(moved, removed, "{line}: one acted and the other did not");
        assert_eq!(nav.cursor, cut.cursor, "{line}: landed in different places");
    }
}

#[test]
fn word_motion_stops_at_the_ends() {
    let mut l = typed("word");
    l.move_home();
    assert!(!l.move_word_left());
    l.move_end();
    assert!(!l.move_word_right());
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

/// The buffer is fixed width, and a yank is the one path that inserts many
/// bytes at once. It has to stop at the edge rather than run past it.
#[test]
fn yanking_into_a_nearly_full_line_stays_in_bounds() {
    let wide = "x".repeat(crate::term::dimensions::COLS);
    let mut l = typed(&wide);
    assert_eq!(l.len, crate::term::dimensions::COLS);
    l.kill_line();
    l.yank();
    assert!(l.len <= crate::term::dimensions::COLS);
    l.yank();
    assert!(l.len <= crate::term::dimensions::COLS);
    assert!(l.cursor <= l.len);
}
