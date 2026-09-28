// NONOS Operating System (AGPL-3.0-or-later)
//! The positional pseudo-classes answer from a table built once per pass
//! over the document. `select` reads that table and `matches` still walks
//! the parent's children, so the two agreeing on every element, for every
//! positional selector, is the proof the table says what the walk said.

use crate::browser::css::{matches, select};
use crate::browser::dom;

const PAGE: &str = "<body><ul><li>a</li>text<li>b<p>x</p><p>y</p><span>z</span><p>w</p></li>\
<!-- c --><li>c</li><div>d</div><li>e</li></ul><div><span>s</span></div><p>only</p>\
<table><tr><td>1</td><td>2</td><th>3</th><td>4</td></tr></table></body>";

const SELECTORS: &[&str] = &[
    "li:first-child",
    "li:last-child",
    ":only-child",
    "p:first-of-type",
    "p:last-of-type",
    "td:nth-child(2n+1)",
    "li:nth-child(odd)",
    ":nth-child(3)",
    "li:not(:first-child)",
    "ul > :nth-child(2) p:last-of-type",
    "*:first-child *:last-child",
];

#[test]
fn the_table_and_the_walk_agree_on_every_element() {
    let d = dom::parse(PAGE.as_bytes());
    for sel in SELECTORS {
        let from_table = select(&d, sel, usize::MAX);
        let from_walk: Vec<usize> = (0..d.nodes.len()).filter(|&id| matches(&d, id, sel)).collect();
        assert_eq!(from_table, from_walk, "{sel}");
        assert!(!from_walk.is_empty(), "{sel} matched nothing, so it proves nothing");
    }
}

#[test]
fn nth_child_counts_elements_not_text_or_comments() {
    let d = dom::parse(PAGE.as_bytes());
    let texts: Vec<&str> = select(&d, "li:nth-child(odd)", usize::MAX)
        .into_iter()
        .map(|id| d.nodes[d.nodes[id].children[0]].text.as_str())
        .collect();
    /* a, b, c, then the div, then e: the text node and the comment take no place. */
    assert_eq!(texts, ["a", "c", "e"]);
}
