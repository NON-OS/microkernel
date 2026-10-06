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

//! Fragment parsing (13.4), what innerHTML runs: the context element picks
//! the insertion mode and tokenizer state, and nothing wraps the result.

use crate::browser::dom::node::{NodeKind, Ns};
use crate::browser::dom::parse_fragment;
use crate::shape::{all, check};

fn frag(html: &str, context: &str) -> String {
    let d = parse_fragment(html.as_bytes(), context);
    check(&d);
    d.inner_html(0)
}

#[test]
fn a_row_set_on_a_tbody_is_a_row() {
    assert_eq!(frag("<tr><td>x", "tbody"), "<tr><td>x</td></tr>");
    assert_eq!(frag("<td>x", "tr"), "<td>x</td>");
    assert_eq!(frag("<td>x", "template"), "<td>x</td>");
    assert_eq!(frag("<li>a<li>b", "ul"), "<li>a</li><li>b</li>");
}

#[test]
fn inner_html_never_adds_html_head_or_body() {
    assert_eq!(frag("<p>a</p><body class=b><head><html>", "div"), "<p>a</p>");
    assert_eq!(frag("x", "div"), "x");
    assert_eq!(frag("", "div"), "");
    let d = parse_fragment(b"<title>t</title>x", "div");
    assert!(all(&d, "html").is_empty() && all(&d, "body").is_empty());
}

#[test]
fn the_context_sets_the_tokenizer_state() {
    let title = parse_fragment(b"<b>&amp;", "title");
    assert_eq!(title.nodes[1].text, "<b>&");
    let script = parse_fragment(b"<b>&amp;", "script");
    assert_eq!(script.nodes[1].text, "<b>&amp;");
    let pt = parse_fragment(b"</plaintext>", "plaintext");
    assert_eq!(pt.nodes[1].text, "</plaintext>");
    assert_eq!(frag("<option>a<option>b", "select"), "<option>a</option><option>b</option>");
}

#[test]
fn a_foreign_context_parses_foreign_content() {
    let d = parse_fragment(b"<path/><circle></circle><p>x", "svg svg");
    check(&d);
    assert_eq!(d.nodes[all(&d, "path")[0]].ns, Ns::Svg);
    assert_eq!(d.nodes[all(&d, "p")[0]].ns, Ns::Html);
    let m = parse_fragment(b"<mi>x</mi>", "math math");
    assert_eq!(m.nodes[all(&m, "mi")[0]].ns, Ns::MathMl);
    let svg = &d.nodes[all(&d, "circle")[0]];
    assert_eq!(svg.context_tag(), "svg circle");
    assert!(d.nodes.iter().skip(1).all(|n| n.kind != NodeKind::Document));
}
