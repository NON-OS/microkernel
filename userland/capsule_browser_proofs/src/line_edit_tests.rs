// NONOS Operating System (AGPL-3.0-or-later)
//! The address bar's text editing. The live bug: every key appended, so
//! typing nonos.software gave https://example.com/https://nonos.software/.

use crate::browser::omnibox::LineEdit;
use crate::browser::url::parse;

fn with(text: &str) -> LineEdit {
    let mut e = LineEdit::new();
    e.set(text);
    e
}

#[test]
fn focus_selects_all_so_typing_replaces_the_address() {
    let mut e = with("https://example.com/");
    e.select_all();
    for c in "nonos.software".chars() {
        e.replace_selection(c.encode_utf8(&mut [0u8; 4]));
    }
    assert_eq!(e.text, "nonos.software");
    let u = parse(&e.text).expect("parses");
    assert_eq!(u.host, "nonos.software");
}

#[test]
fn caret_moves_and_inserts_in_the_middle() {
    let mut e = with("exmple.com");
    e.home(false);
    e.right(false, false);
    e.right(false, false);
    e.replace_selection("a");
    assert_eq!(e.text, "example.com");
    assert_eq!(e.caret, 3);
}

#[test]
fn word_backspace_and_delete_stop_at_url_separators() {
    let mut e = with("https://example.com/path");
    e.backspace_word();
    assert_eq!(e.text, "https://example.com/");
    e.backspace_word();
    assert_eq!(e.text, "https://example.");
    e.home(false);
    e.delete_word();
    assert_eq!(e.text, "://example.");
}

#[test]
fn shift_home_selects_and_backspace_removes_the_selection() {
    let mut e = with("abc def");
    e.left(true, false);
    assert_eq!(e.caret, 4);
    e.home(true);
    assert_eq!(e.selection(), (0, 4));
    assert_eq!(e.selected(), "abc ");
    e.backspace();
    assert_eq!(e.text, "def");
    e.end(false);
    e.left(false, true);
    assert_eq!(e.selected(), "f");
}

#[test]
fn multibyte_text_edits_on_character_boundaries() {
    let mut e = with("a\u{e4}/\u{f6}");
    e.left(false, false);
    assert_eq!(e.caret, 4);
    e.backspace();
    assert_eq!(e.text, "a\u{e4}\u{f6}");
    e.backspace();
    assert_eq!(e.text, "a\u{f6}");
    e.delete();
    assert_eq!(e.text, "a");
}
