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

//! Bytes to text: WHATWG UTF-8 decoding with replacement, the byte order
//! mark, newline normalisation, and a value longer than any real one.

use std::borrow::Cow;

use crate::browser::dom;
use crate::browser::html::input::decode;
use crate::shape::{all, body, check};

#[test]
fn invalid_utf8_becomes_one_replacement_per_maximal_subpart() {
    assert_eq!(decode(b"a\xFFb"), "a\u{FFFD}b");
    assert_eq!(decode(b"\xF0\x9F\x98("), "\u{FFFD}(");
    assert_eq!(decode(b"\xE0\x80\x80"), "\u{FFFD}\u{FFFD}\u{FFFD}");
    assert_eq!(decode(b"\xED\xA0\x80"), "\u{FFFD}\u{FFFD}\u{FFFD}");
}

#[test]
fn a_byte_order_mark_is_dropped_and_newlines_are_normalised() {
    assert_eq!(decode(b"\xEF\xBB\xBFa\r\nb\rc\n"), "a\nb\nc\n");
    assert_eq!(decode(b"\xEF\xBB\xBF\xEF\xBB\xBFa"), "\u{FEFF}a");
    assert!(matches!(decode(b"<p>plain</p>"), Cow::Borrowed(_)));
}

#[test]
fn a_page_with_bad_bytes_still_parses_into_a_tree() {
    let d = dom::parse(b"<p>caf\xE9 \xC3</p><p>ok</p>");
    check(&d);
    assert_eq!(all(&d, "p").len(), 2);
    assert_eq!(d.inner_html(all(&d, "p")[0]), "caf\u{FFFD} \u{FFFD}");
    let frag = dom::parse_fragment(b"\xFFx", "div");
    assert_eq!(frag.inner_html(0), "\u{FFFD}x");
}

#[test]
fn carriage_returns_never_reach_the_tree() {
    assert_eq!(body("<pre>a\r\nb\rc</pre>"), "<pre>a\nb\nc</pre>");
    assert_eq!(body("<p title=\"a\r\nb\">x"), "<p title=\"a\nb\">x</p>");
}

#[test]
fn values_are_not_cut_at_a_fixed_tag_length() {
    let long = "x".repeat(20_000);
    let d = dom::parse(format!("<a href=\"{long}\" id=k>t</a>").as_bytes());
    let a = &d.nodes[all(&d, "a")[0]];
    assert_eq!(a.attr("href").map(str::len), Some(20_000));
    assert_eq!(a.attr("id"), Some("k"));
}

#[test]
fn a_value_over_one_mebibyte_is_dropped_whole() {
    let long = "x".repeat((1 << 20) + 1);
    let d = dom::parse(format!("<a href=\"{long}\" id=k>t</a>").as_bytes());
    let a = &d.nodes[all(&d, "a")[0]];
    assert_eq!(a.attr("href"), None);
    assert_eq!(a.attr("id"), Some("k"));
    assert_eq!(d.nodes.iter().filter(|n| n.text == "t").count(), 1);
}
