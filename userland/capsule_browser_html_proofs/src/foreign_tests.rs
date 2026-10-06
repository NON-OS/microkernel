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

//! Foreign content (13.2.6.5): SVG and MathML namespaces, their name case
//! tables, self-closing tags, CDATA, breakout and integration points.

use crate::browser::dom::node::Ns;
use crate::shape::{all, body, check, doc};

#[test]
fn svg_names_keep_their_case_and_namespace() {
    let d = doc("<svg viewbox='0 0 1 1'><clippath/><foreignobject><p>x</p></foreignobject></svg>");
    check(&d);
    let svg = &d.nodes[all(&d, "svg")[0]];
    assert_eq!((svg.ns, svg.attr("viewBox")), (Ns::Svg, Some("0 0 1 1")));
    assert_eq!(svg.attrs[0].0, "viewBox");
    let clip = &d.nodes[all(&d, "clipPath")[0]];
    assert!(clip.children.is_empty() && clip.ns == Ns::Svg);
    let fo = all(&d, "foreignObject")[0];
    let p = &d.nodes[all(&d, "p")[0]];
    assert_eq!((p.parent, p.ns), (fo, Ns::Html));
}

#[test]
fn an_html_tag_breaks_out_of_foreign_content() {
    assert_eq!(body("<svg><g><p>x"), "<svg><g></g></svg><p>x</p>");
    assert_eq!(body("<svg><font color=red>x"), "<svg></svg><font color=\"red\">x</font>");
    let d = doc("<svg><font>x</font></svg>");
    assert_eq!(d.nodes[all(&d, "font")[0]].ns, Ns::Svg);
    assert_eq!(body("<math><mi><p>y</p></mi></math>"), "<math><mi><p>y</p></mi></math>");
}

#[test]
fn cdata_is_text_in_foreign_content_and_a_comment_elsewhere() {
    let d = doc("<svg><![CDATA[a<b]]></svg><![CDATA[c]]>");
    check(&d);
    let svg = &d.nodes[all(&d, "svg")[0]];
    assert_eq!(d.nodes[svg.children[0]].text, "a<b");
    assert_eq!(svg.children.len(), 1);
    assert!(!d.nodes.iter().any(|n| n.text.contains('c')));
}

#[test]
fn mathml_integration_points_hold_html() {
    let d = doc("<math definitionurl=u><annotation-xml encoding='text/html'><div>x</div>");
    check(&d);
    let math = &d.nodes[all(&d, "math")[0]];
    assert_eq!((math.ns, math.attrs[0].0.as_str()), (Ns::MathMl, "definitionURL"));
    let div = &d.nodes[all(&d, "div")[0]];
    assert_eq!((div.ns, div.parent), (Ns::Html, all(&d, "annotation-xml")[0]));
    let plain = doc("<math><annotation-xml><div>x</div></annotation-xml></math>");
    assert_eq!(plain.nodes[all(&plain, "div")[0]].parent, all(&plain, "body")[0]);
}

#[test]
fn self_closing_is_honoured_only_in_foreign_content() {
    assert_eq!(body("<svg><path/>x</svg>"), "<svg><path></path>x</svg>");
    assert_eq!(body("<div/>x"), "<div>x</div>");
    let d = doc("<svg><a xlink:href=u></a></svg>");
    assert_eq!(d.nodes[all(&d, "a")[0]].attrs[0].0, "xlink:href");
}
