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
use alloc::vec::Vec;

use crate::browser::html::tokenizer::TextMode;

use super::super::super::tree::Dom;
use super::mode::{Entry, Mode};

/// The tree builder (13.2.6): the parser state around the `Dom` it fills.
pub struct Builder {
    pub dom: Dom,
    pub(in super::super) open: Vec<usize>,
    /// Beside `open`, one word per entry: see `meta_of`.
    pub(in super::super) open_meta: Vec<u16>,
    pub(in super::super) fmt: Vec<Entry>,
    pub(in super::super) mode: Mode,
    pub(in super::super) orig: Mode,
    pub(in super::super) tmpl: Vec<Mode>,
    pub(in super::super) head: Option<usize>,
    pub(in super::super) form: Option<usize>,
    /// The fragment parser's context element, a node outside the tree.
    pub(in super::super) context: Option<usize>,
    pub(in super::super) frameset_ok: bool,
    pub(in super::super) foster: bool,
    pub(in super::super) skip_lf: bool,
    pub(in super::super) table_text: String,
    /// Where the last comment would have gone, as (parent, child index): text
    /// inserted right there starts a new node instead of merging.
    pub(in super::super) comment_at: (usize, usize),
    /// The tokenizer state a start tag asked for, read after each token.
    pub(in super::super) switch_to: Option<TextMode>,
    /// Per node: `ON_STACK` and `IN_FMT`, so membership is one lookup.
    pub(in super::super) flags: Vec<u8>,
    /// Per node: depth when it was attached, for the nesting cap.
    pub(in super::super) depth: Vec<u16>,
    /// Open elements per tag-name bucket, so a name with nothing open is
    /// refused without walking the stack.
    pub(in super::super) open_names: [u16; 64],
    pub(in super::super) attrs_used: usize,
    pub(in super::super) attr_bytes: usize,
    /// Set when parsing has to end: the node cap, or "stop parsing".
    pub(in super::super) stopped: bool,
}

pub(in super::super) const ON_STACK: u8 = 1;
pub(in super::super) const IN_FMT: u8 = 2;
