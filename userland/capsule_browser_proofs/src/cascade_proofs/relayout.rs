// NONOS Operating System (AGPL-3.0-or-later)
//! Restyle cost: the kept styles stand while the document fingerprint,
//! the CSS and the viewport do; a typed value restyles only when a
//! selector reads values, and a flipped @media verdict restyles.

use alloc::string::String;

use crate::browser::css::{collect_css, CssCache};
use crate::browser::dom::{self, Dom};
use crate::browser::dom_print::dom_print;

const VP: (u32, u32) = (800, 600);

/* One restyle as the relayout pass runs it; whether it recomputed, and
 * how many times the CSS text had to be produced. */
fn restyle(cache: &mut Option<CssCache>, d: &Dom, vp: (u32, u32)) -> (bool, u32) {
    let mut built = 0;
    let mut css = || {
        built += 1;
        collect_css(d)
    };
    let text = CssCache::restyle(cache, d, &mut css, vp, dom_print(d, ""), (true, None));
    (text.is_some(), built)
}

#[test]
fn an_unchanged_document_does_not_restyle() {
    let mut d = dom::parse(b"<style>p{color:red}</style><p id=a>one</p><input id=f value=x>");
    let mut cache = None;
    assert_eq!(restyle(&mut cache, &d, VP), (true, 1), "the first pass styles");
    assert_eq!(
        restyle(&mut cache, &d, VP),
        (false, 0),
        "same print: nothing rebuilt, no CSS text made"
    );
    assert_eq!(
        restyle(&mut cache, &d, (820, 600)),
        (false, 0),
        "a width no @media reads keeps the styles"
    );
    let f = d.nodes.iter().position(|n| n.attr("id") == Some("f")).expect("field");
    d.nodes[f].attrs.retain(|a| a.0 != "value");
    d.nodes[f].attrs.push((String::from("value"), String::from("typed")));
    assert_eq!(restyle(&mut cache, &d, (820, 600)), (false, 0), "no selector reads a value");
    let t = d.nodes.iter().position(|n| n.text == "one").expect("text");
    d.nodes[t].text = String::from("two");
    assert_eq!(restyle(&mut cache, &d, (820, 600)), (true, 1), "changed text restyles");
}

#[test]
fn a_value_selector_restyles_on_typing_and_media_flips_restyle() {
    let src = b"<style>input[value=go]{color:red}@media (max-width:500px){p{color:blue}}</style><input id=f value=x><p>p</p>";
    let mut d = dom::parse(src);
    let mut cache = None;
    restyle(&mut cache, &d, VP);
    let f = d.nodes.iter().position(|n| n.attr("id") == Some("f")).expect("field");
    d.nodes[f].attrs.retain(|a| a.0 != "value");
    d.nodes[f].attrs.push((String::from("value"), String::from("go")));
    assert!(restyle(&mut cache, &d, VP).0, "[value] is read, so typing restyles");
    assert!(restyle(&mut cache, &d, (400, 600)).0, "the @media verdict flips at 400px");
}
