// NONOS Operating System (AGPL-3.0-or-later)
//! The line holds UTF-8: what the keymap types (an e acute, a euro sign, a
//! CJK ideograph) goes in whole, and every key that moves or deletes steps
//! over whole characters, so the line is always text a font can draw.

mod full;
mod keys;
mod paste;
mod screen;

use crate::line::Line;

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
