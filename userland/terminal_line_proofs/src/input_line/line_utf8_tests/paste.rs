// NONOS Operating System (AGPL-3.0-or-later)
//! A paste puts printable text at the cursor and stops on a character.

use super::{text, typed};
use crate::line::Line;
use crate::term::dimensions::LINE_MAX;

// Paste.

#[test]
fn a_paste_inserts_printable_text_tabs_as_spaces_and_drops_controls() {
    let mut l = typed("ls ");
    let p = l.paste("/tmp/\u{1b}[31mrésumé\tfile");
    assert!(p.changed && !p.full);
    assert_eq!(text(&l), "ls /tmp/[31mrésumé file");
}

#[test]
fn a_paste_lands_at_the_cursor() {
    let mut l = typed("cat x");
    l.move_left();
    l.paste("naïve/");
    assert_eq!(text(&l), "cat naïve/x");
    assert_eq!(l.cursor, "cat naïve/".len());
}

#[test]
fn a_paste_that_overflows_stops_on_a_character_and_says_so() {
    let mut l = Line::new();
    for _ in 0..LINE_MAX - 2 {
        l.insert_char('a');
    }
    let p = l.paste("é€");
    assert!(p.changed && p.full);
    assert_eq!(l.len, LINE_MAX);
    assert!(text(&l).ends_with('é'));
}

#[test]
fn pasting_nothing_printable_changes_nothing() {
    let mut l = typed("x");
    let p = l.paste("\u{7}\u{8}");
    assert!(!p.changed && !p.full);
    assert_eq!(text(&l), "x");
}
