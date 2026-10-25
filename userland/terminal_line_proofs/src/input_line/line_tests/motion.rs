// NONOS Operating System (AGPL-3.0-or-later)
//! Where the caret lands: after typing, and by word in either direction.

use super::{text, typed};

#[test]
fn typing_leaves_the_caret_at_the_end() {
    let l = typed("ls -la /data");
    assert_eq!(text(&l), "ls -la /data");
    assert_eq!(l.cursor, 12);
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
