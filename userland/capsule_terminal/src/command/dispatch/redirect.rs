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

//! What a line's redirects ask for. `< f` feeds the command a file, `> f`
//! writes its output to one and `>> f` adds to one; `/dev/null` on either
//! side is nothing in and nothing kept. Every redirect leaves the words, so
//! no operator or file name ever reaches the command as an argument, and one
//! the terminal cannot honour is refused before anything runs.

use alloc::vec::Vec;

#[path = "redirect_plan.rs"]
mod redirect_plan;

pub use redirect_plan::plan;

/// Where the command's input comes from.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Source<'a> {
    /// The keyboard, as with no redirect.
    Terminal,
    /// `< /dev/null`: the input is over before it starts.
    Null,
    File(&'a [u8]),
}

/// Where the command's output goes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Sink<'a> {
    Screen,
    /// `> /dev/null`: run it, keep nothing.
    Null,
    File {
        path: &'a [u8],
        append: bool,
    },
}

pub struct Plan<'a> {
    /// The command and its arguments, `|` between pipe stages kept.
    pub words: Vec<&'a [u8]>,
    pub input: Source<'a>,
    pub output: Sink<'a>,
}

/// Whether a word is one of the operators the tokenizer splits out.
pub fn is_operator(word: &[u8]) -> bool {
    matches!(word, b"<" | b">" | b">>" | b"|") || numbered(word)
}

/// `2>`, `2>>`, `2>&1`, `>&2`: a redirect of a stream other than input and
/// output, as the tokenizer keeps it whole.
fn numbered(word: &[u8]) -> bool {
    let digits = word.iter().take_while(|b| b.is_ascii_digit()).count();
    let rest = &word[digits..];
    matches!(rest.first(), Some(b'<' | b'>')) && (digits > 0 || rest.contains(&b'&'))
}
