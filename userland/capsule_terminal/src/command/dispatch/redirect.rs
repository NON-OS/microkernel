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
    File { path: &'a [u8], append: bool },
}

pub struct Plan<'a> {
    /// The command and its arguments, `|` between pipe stages kept.
    pub words: Vec<&'a [u8]>,
    pub input: Source<'a>,
    pub output: Sink<'a>,
}

const NULL_DEVICE: &[u8] = b"/dev/null";

pub fn plan<'a>(args: &[&'a [u8]]) -> Result<Plan<'a>, &'static [u8]> {
    let last_stage = args.iter().filter(|a| **a == b"|").count();
    let mut stage = 0;
    let mut words = Vec::with_capacity(args.len());
    let (mut input, mut output) = (Source::Terminal, Sink::Screen);
    let (mut has_input, mut has_output) = (false, false);
    let mut i = 0;
    while i < args.len() {
        let word = args[i];
        if numbered(word) {
            return Err(
                b"redirect: a numbered stream (2>, 2>&1) is not taken; only <, > and >> are",
            );
        }
        let append = match word {
            b"<" => None,
            b">" => Some(false),
            b">>" => Some(true),
            _ => {
                stage += usize::from(word == b"|");
                words.push(word);
                i += 1;
                continue;
            }
        };
        let path = match args.get(i + 1) {
            Some(p) if !p.is_empty() && !is_operator(p) => *p,
            _ if append.is_none() => return Err(b"redirect: expected a file path after <"),
            _ => return Err(b"redirect: expected a file path after >"),
        };
        match append {
            None if has_input => return Err(b"redirect: one < per command"),
            None if stage != 0 => return Err(b"redirect: < goes in the first stage of a pipe"),
            None => {
                has_input = true;
                input = if path == NULL_DEVICE { Source::Null } else { Source::File(path) };
            }
            Some(_) if has_output => return Err(b"redirect: one > or >> per command"),
            Some(_) if stage != last_stage => {
                return Err(b"redirect: > goes in the last stage of a pipe")
            }
            Some(append) => {
                has_output = true;
                output = if path == NULL_DEVICE { Sink::Null } else { Sink::File { path, append } };
            }
        }
        i += 2;
    }
    Ok(Plan { words, input, output })
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
