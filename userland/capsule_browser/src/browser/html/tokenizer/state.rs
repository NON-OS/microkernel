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

use alloc::string::String;

/// The text states the tree builder switches the tokenizer between. The
/// markup states inside tags, comments and doctypes are not listed: each is
/// read to its end in one step, so none is ever left open between tokens.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TextMode {
    Data,
    Rcdata,
    Rawtext,
    ScriptData,
    Plaintext,
}

/// The WHATWG tokenizer (13.2.5) over text already decoded and normalised.
pub struct Tokenizer<'a> {
    pub(super) src: &'a str,
    pub(super) pos: usize,
    /// The state the next token is read in, set by the tree builder.
    pub mode: TextMode,
    /// Whether the adjusted current node is foreign: only there `<![CDATA[`
    /// opens a section rather than a bogus comment.
    pub foreign: bool,
    /// The last start tag emitted, which is what makes an end tag in raw
    /// text "appropriate". Empty before the first one.
    pub(super) last_start: String,
    /// Set when a tag had more attributes, or a longer value, than is kept.
    pub dropped_attrs: bool,
}

impl<'a> Tokenizer<'a> {
    pub fn new(src: &'a str) -> Self {
        Tokenizer {
            src,
            pos: 0,
            mode: TextMode::Data,
            foreign: false,
            last_start: String::new(),
            dropped_attrs: false,
        }
    }

    /// A tokenizer starting in `mode`, as if `last_start` had been the last
    /// start tag, for the fragment parser and the tokenizer conformance tests.
    pub fn with_state(src: &'a str, mode: TextMode, last_start: &str) -> Self {
        let mut t = Tokenizer::new(src);
        t.mode = mode;
        t.last_start.push_str(last_start);
        t
    }
}

/// The tokenizer's whitespace: tab, LF, FF and space. CR never reaches it.
pub(super) fn is_ws(c: u8) -> bool {
    matches!(c, b'\t' | b'\n' | b'\x0C' | b' ')
}
