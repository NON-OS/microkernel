// NONOS Operating System (AGPL-3.0-or-later)
//! A full line, a yank and a recalled line stop on a character boundary.

use super::{text, typed};
use crate::line::Line;
use crate::term::dimensions::LINE_MAX;

#[test]
fn a_full_line_refuses_a_character_rather_than_split_it() {
    let mut l = Line::new();
    for _ in 0..LINE_MAX - 1 {
        assert!(l.insert_char('a'));
    }
    assert!(!l.insert_char('€'), "three bytes in one free byte");
    assert!(l.insert_char('b'));
    assert_eq!(l.len, LINE_MAX);
    assert!(text(&l).ends_with('b'));
}

#[test]
fn kill_and_yank_carry_whole_characters() {
    let mut l = typed("echo grüße");
    assert!(l.delete_word());
    assert_eq!(text(&l), "echo ");
    assert!(l.yank());
    assert_eq!(text(&l), "echo grüße");
    l.move_home();
    assert!(l.kill_to_end());
    assert!(l.yank());
    assert!(l.yank());
    assert_eq!(text(&l), "echo grüßeecho grüße");
}

#[test]
fn a_yank_into_a_nearly_full_line_stops_at_a_whole_character() {
    let mut l = typed("€€");
    l.kill_line();
    for _ in 0..LINE_MAX - 4 {
        l.insert_char('a');
    }
    assert!(l.yank(), "one euro fits");
    assert_eq!(l.len, LINE_MAX - 1);
    assert!(text(&l).ends_with('€'));
}

#[test]
fn a_recalled_line_too_long_is_cut_at_a_character_boundary() {
    let mut src = vec![b'a'; LINE_MAX - 1];
    src.extend_from_slice("é".as_bytes());
    let mut l = Line::new();
    l.replace(&src);
    assert_eq!(l.len, LINE_MAX - 1);
    assert!(core::str::from_utf8(l.as_bytes()).is_ok());
}
