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

//! Tree construction in body content (13.2.6.4): implied elements, implied
//! end tags, the adoption agency and formatting reconstruction.

use crate::browser::dom::quirks::Quirks;
use crate::shape::{body, check, doc};

fn each(cases: &[(&str, &str)]) {
    for (html, want) in cases {
        assert_eq!(body(html), *want, "{html}");
    }
}

#[test]
fn html_head_and_body_are_always_there() {
    let d = doc("x");
    check(&d);
    assert_eq!(d.inner_html(0), "<html><head></head><body>x</body></html>");
    let t = doc("<title>t</title><p>a");
    assert_eq!(t.inner_html(0), "<html><head><title>t</title></head><body><p>a</p></body></html>");
    assert_eq!(body("<body>a</body>b</html>c"), "abc");
}

#[test]
fn implied_end_tags_close_what_the_specification_closes() {
    each(&[
        ("<p>a<p>b<div>c</div>", "<p>a</p><p>b</p><div>c</div>"),
        ("</p>x<body></p>y", "x<p></p>y"),
        ("<h1>a<h2>b</h1>c", "<h1>a</h1><h2>b</h2>c"),
        ("<ul><li>a<li>b</ul>", "<ul><li>a</li><li>b</li></ul>"),
        ("<dl><dt>a<dd>b<dt>c</dl>", "<dl><dt>a</dt><dd>b</dd><dt>c</dt></dl>"),
        ("<form><form><input></form>", "<form><input></form>"),
        ("<p><button>a<p>b</button>c", "<p><button>a<p>b</p></button>c</p>"),
    ]);
}

#[test]
fn misnested_formatting_is_adopted_and_reopened() {
    each(&[
        ("<b>1<p>2</b>3</p>", "<b>1</b><p><b>2</b>3</p>"),
        ("<a>1<a>2", "<a>1</a><a>2</a>"),
        ("<p><b>x<p>y", "<p><b>x</b></p><p><b>y</b></p>"),
        ("<b><i>x</b>y</i>", "<b><i>x</i></b><i>y</i>"),
        ("<nobr>a<nobr>b", "<nobr>a</nobr><nobr>b</nobr>"),
    ]);
    let noah = body("<p><b><b><b><b>x</p>y");
    assert_eq!(noah.matches("<b>").count(), 4 + 3, "{noah}");
}

#[test]
fn a_table_closes_a_paragraph_except_in_quirks_mode() {
    let standard = doc("<!DOCTYPE html><p><table></table>");
    assert_eq!(standard.quirks, Quirks::No);
    assert_eq!(standard.inner_html(0).matches("</p><table>").count(), 1);
    let quirks = doc("<p><table></table>");
    assert_eq!(quirks.quirks, Quirks::Full);
    assert_eq!(quirks.inner_html(0).matches("<p><table></table></p>").count(), 1);
    let xhtml = doc("<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Frameset//EN\" \"x\">");
    assert_eq!(xhtml.quirks, Quirks::Limited);
}
