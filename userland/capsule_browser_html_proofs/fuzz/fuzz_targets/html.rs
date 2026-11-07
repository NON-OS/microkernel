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

//! Any bytes as a page: the decoder, the tokenizer in every text mode and the
//! tree builder never panic and always end (a hang is a libFuzzer timeout).
//! The document and the fragment built from them hold the arena invariants,
//! the depth cap, the node cap and both attribute budgets, and the tree
//! serialises back to markup.

#![no_main]

use capsule_browser_html_proofs::browser::dom::limits::{MAX_ATTRS, MAX_DEPTH, MAX_NODES};
use capsule_browser_html_proofs::browser::dom::{parse, parse_fragment, Dom};
use capsule_browser_html_proofs::browser::html::input::decode;
use capsule_browser_html_proofs::browser::html::tokenizer::{TextMode, Token, Tokenizer, MAX_TAG_ATTRS};
use capsule_browser_html_proofs::shape::{check, depth};
use libfuzzer_sys::fuzz_target;

const CONTEXTS: &[&str] = &["div", "tbody", "tr", "td", "select", "title", "svg svg", "math mi", "table", "template"];
const MODES: &[TextMode] =
    &[TextMode::Data, TextMode::Rcdata, TextMode::Rawtext, TextMode::ScriptData, TextMode::Plaintext];

fn hold(d: &Dom) {
    check(d);
    assert!(d.nodes.len() <= MAX_NODES, "{} nodes", d.nodes.len());
    assert!(depth(d) <= MAX_DEPTH + 1, "depth {}", depth(d));
    let held: usize = d.nodes.iter().map(|n| n.attrs.len()).sum();
    assert!(held <= MAX_ATTRS, "{held} attributes");
    assert!(d.nodes.iter().all(|n| n.attrs.len() <= MAX_TAG_ATTRS));
    let _ = d.inner_html(0);
}

fuzz_target!(|data: &[u8]| {
    let Some((&pick, markup)) = data.split_first() else { return };
    hold(&parse(markup));
    hold(&parse_fragment(markup, CONTEXTS[pick as usize % CONTEXTS.len()]));
    let src = decode(markup);
    let mut t = Tokenizer::with_state(&src, MODES[pick as usize % MODES.len()], "script");
    while !matches!(t.next_token(), Token::Eof) {}
});
