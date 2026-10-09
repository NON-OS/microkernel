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

//! What ends up as content: merged text, template and noscript children,
//! and the newline a pre or textarea drops.

use crate::browser::dom::node::NodeKind;
use crate::shape::{all, body, check, doc, inner};

#[test]
fn text_merges_unless_a_comment_stood_between() {
    let d = doc("<p>a</x>b<!--c-->d</p>");
    check(&d);
    let p = &d.nodes[all(&d, "p")[0]];
    let texts: Vec<&str> = p.children.iter().map(|&c| d.nodes[c].text.as_str()).collect();
    assert_eq!(texts, ["ab", "d"]);
    assert!(p.children.iter().all(|&c| d.nodes[c].kind == NodeKind::Text));
}

#[test]
fn template_and_noscript_contents_are_ordinary_children() {
    let d = doc("<body><template><td>x</td></template><noscript><p>y</p></noscript>");
    check(&d);
    assert_eq!(inner(&d, "template"), "<td>x</td>");
    assert_eq!(inner(&d, "noscript"), "<p>y</p>");
    let head = doc("<head><noscript><link rel=a></noscript></head>");
    assert_eq!(inner(&head, "noscript"), "<link rel=\"a\">");
}

#[test]
fn preformatted_text_drops_one_leading_newline() {
    let cases = [
        ("<pre>\nx</pre>", "<pre>x</pre>"),
        ("<pre>\n\nx</pre>", "<pre>\nx</pre>"),
        ("<textarea>\nx&amp;</textarea>", "<textarea>x&amp;</textarea>"),
        ("<image src=a>", "<img src=\"a\">"),
        ("<p>a\0b</p>", "<p>ab</p>"),
    ];
    for (html, want) in cases {
        assert_eq!(body(html), want, "{html}");
    }
}
