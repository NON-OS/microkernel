// NONOS Operating System (AGPL-3.0-or-later)
//! What the line editor has to do, written as the key sequences people type.
//!
//! The kill keys are the reason this file exists. Ctrl-U, Ctrl-W and Ctrl-K
//! used to destroy text with no way back, and losing a typed line to one
//! keystroke is the failure a terminal is least forgiven for.

mod cut;
mod motion;
mod yank;

use crate::line::Line;

fn typed(s: &str) -> Line {
    let mut l = Line::new();
    for ch in s.chars() {
        l.insert_char(ch);
    }
    l
}

fn text(l: &Line) -> String {
    String::from_utf8(l.as_bytes().to_vec()).unwrap()
}
