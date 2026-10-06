// NONOS Operating System (AGPL-3.0-or-later)
//! The line holds UTF-8: what the keymap types (an e acute, a euro sign, a
//! CJK ideograph) goes in whole, and every key that moves or deletes steps
//! over whole characters, so the line is always text a font can draw.

use crate::line::Line;
use crate::line_window::{cells_of, fit_cells, window};
use crate::term::dimensions::LINE_MAX;

fn typed(s: &str) -> Line {
    let mut l = Line::new();
    for ch in s.chars() {
        assert!(l.insert_char(ch), "room for {ch}");
    }
    l
}

fn text(l: &Line) -> &str {
    core::str::from_utf8(l.as_bytes()).expect("the line is always UTF-8")
}

#[test]
fn typed_characters_go_in_whole() {
    let l = typed("café €5 日本");
    assert_eq!(text(&l), "café €5 日本");
    assert_eq!(l.cursor, "café €5 日本".len());
}

#[test]
fn backspace_takes_a_whole_character() {
    let mut l = typed("naïve€");
    assert!(l.backspace());
    assert_eq!(text(&l), "naïve");
    l.move_left();
    l.move_left();
    assert!(l.backspace(), "the two byte i");
    assert_eq!(text(&l), "nave");
}

#[test]
fn delete_takes_the_whole_character_under_the_cursor() {
    let mut l = typed("a日b");
    l.move_home();
    l.move_right();
    assert!(l.delete());
    assert_eq!(text(&l), "ab");
    assert_eq!(l.cursor, 1);
}

#[test]
fn the_arrows_step_one_character_of_any_length() {
    let mut l = typed("é€😀");
    let mut stops = vec![l.cursor];
    while l.move_left() {
        stops.push(l.cursor);
    }
    assert_eq!(stops, vec![9, 5, 2, 0]);
    while l.move_right() {}
    assert_eq!(l.cursor, 9);
}

#[test]
fn typing_in_the_middle_keeps_the_rest_intact() {
    let mut l = typed("über");
    l.move_home();
    l.move_right();
    assert!(l.insert_char('ß'));
    assert_eq!(text(&l), "üßber");
    assert_eq!(l.cursor, 4);
}

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

// What the screen shows.

#[test]
fn cells_count_characters_not_bytes() {
    assert_eq!(cells_of("café".as_bytes()), 4);
    assert_eq!(cells_of("日本".as_bytes()), 4, "wide characters take two cells");
    assert_eq!(fit_cells("日本語".as_bytes(), 5), "日本".len(), "a half fitting one stays out");
}

#[test]
fn the_cursor_cell_is_counted_in_characters() {
    let body = "éé€x".as_bytes();
    // After three characters (seven bytes), the cursor is in the fourth cell.
    let (start, stop, cell) = window(body, 7, 20);
    assert_eq!((start, stop, cell), (0, body.len(), 3));
}

#[test]
fn a_long_line_scrolls_by_cells_and_keeps_the_cursor_on_screen() {
    let line: String = "é".repeat(30);
    let body = line.as_bytes();
    let (start, stop, cell) = window(body, body.len(), 10);
    assert_eq!(cell, 9, "the cursor in the last cell");
    assert_eq!(&body[start..stop], "é".repeat(9).as_bytes(), "nine characters before it");
    assert!(core::str::from_utf8(&body[start..stop]).is_ok());
}

#[test]
fn a_wide_character_cut_by_the_left_edge_is_left_out() {
    let body = "a日本語x".as_bytes();
    // Cursor at the end: 8 cells to the cursor, 5 on screen, scroll by 4.
    let (start, stop, cell) = window(body, body.len(), 5);
    assert!(core::str::from_utf8(&body[start..stop]).is_ok());
    assert_eq!(&body[start..stop], "語x".as_bytes());
    assert_eq!(cell, 3);
}
