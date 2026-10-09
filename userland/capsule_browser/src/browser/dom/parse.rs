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

use crate::browser::html::input::decode;
use crate::browser::html::tokenizer::TextMode;

use super::builder::{fragment_state, Builder};
use super::node::Ns;
use super::quirks::Quirks;
use super::tree::Dom;

/// Parse a whole document: the WHATWG tokenizer and tree builder, so the
/// tree is the one every other browser builds from the same bytes, with
/// html, head and body always present. Never fails: bytes that are not
/// UTF-8 decode to U+FFFD, and past the node cap the tree stops growing
/// with `truncated` saying so.
pub fn parse(bytes: &[u8]) -> Dom {
    let src = decode(bytes);
    let mut b = Builder::document();
    b.run(&src, TextMode::Data);
    b.finish()
}

/// Parse markup as the contents of the element `context_tag` names, the
/// way innerHTML does (13.4). A foreign context carries its namespace as
/// a prefix, "svg path" or "math mi" (see `Node::context_tag`); a bare name
/// is an HTML element. Node 0's children are the fragment's top-level
/// nodes, and no html, head or body is added. The fragment is parsed as a
/// no-quirks document.
pub fn parse_fragment(input: &[u8], context_tag: &str) -> Dom {
    let (ns, tag) = match context_tag.split_once(' ') {
        Some(("svg", tag)) => (Ns::Svg, tag),
        Some(("math", tag)) => (Ns::MathMl, tag),
        _ => (Ns::Html, context_tag),
    };
    let src = decode(input);
    let mut b = Builder::fragment(tag, ns, Quirks::No);
    b.run(&src, fragment_state(tag, ns));
    b.finish()
}
