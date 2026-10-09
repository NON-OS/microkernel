// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! What the reader is told when a page calls `alert`, `confirm` or
//! `prompt`.
//!
//! A page's script runs to its end in one go, on a 2 s budget, so a dialog
//! that holds the script until the reader answers cannot be built here.
//! The page is answered at once (confirm false, prompt null) and the
//! reader is told what it asked and what this browser answered for them.
//! It never says yes: the reader agreed to nothing.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// Which dialog a page called (nonos_qjs::Dialog, kept apart from the
/// engine so the wording is proved on the host).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Asked {
    Alert,
    Confirm,
    Prompt,
}

/// At most this many characters of the page's message are quoted.
pub const QUOTED: usize = 160;

/// The line for the notice bubble and the status line. `count` is how many
/// dialogs the page called since the browser last looked; the last is
/// quoted and the others counted.
pub fn dialog_line(kind: Asked, text: &str, count: u32) -> String {
    let msg = quoted(text);
    const NO: &str = "This browser cannot wait for an answer, so it answered No.";
    const NONE: &str = "This browser cannot wait for an answer, so it gave none.";
    let mut line = match (kind, msg.is_empty()) {
        (Asked::Alert, false) => format!("This page says: \"{}\"", msg),
        (Asked::Alert, true) => String::from("This page showed an alert with no text."),
        (Asked::Confirm, false) => format!("This page asked \"{}\". {}", msg, NO),
        (Asked::Confirm, true) => format!("This page asked a question with no text. {}", NO),
        (Asked::Prompt, false) => format!("This page asked for text: \"{}\". {}", msg, NONE),
        (Asked::Prompt, true) => format!("This page asked for text. {}", NONE),
    };
    match count {
        0 | 1 => {}
        2 => line.push_str(" (1 earlier message from it is not shown.)"),
        n => line.push_str(&format!(" ({} earlier messages from it are not shown.)", n - 1)),
    }
    line
}

/* The message on one line: runs of white space and control characters are
 * one space, and past QUOTED characters it is cut with "...". */
fn quoted(text: &str) -> String {
    let words: Vec<&str> = text.split(|c: char| c.is_whitespace() || c.is_control()).collect();
    let one: Vec<&str> = words.into_iter().filter(|w| !w.is_empty()).collect();
    let one = one.join(" ");
    match one.char_indices().nth(QUOTED) {
        Some((at, _)) => format!("{}...", &one[..at]),
        None => one,
    }
}
