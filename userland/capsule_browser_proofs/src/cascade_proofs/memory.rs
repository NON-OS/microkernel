// NONOS Operating System (AGPL-3.0-or-later)
//! Per-node memory of the cascade output: a text node owns no style (it
//! reads its parent's), and the pseudo-elements and grid data are kept for
//! the elements that have them, the rest costing nothing.

use alloc::format;
use alloc::string::String;

use crate::browser::css::{collect_css, compute};
use crate::browser::dom::{self, node::NodeKind};

#[test]
fn a_text_node_reads_its_parents_style() {
    let d = dom::parse(b"<p id=a style=\"color:#123456\">text <b>bold</b></p>");
    let s = compute(&d, &collect_css(&d));
    for (i, n) in d.nodes.iter().enumerate().filter(|(_, n)| n.kind == NodeKind::Text) {
        assert!(
            core::ptr::eq(&s.styles[i], &s.styles[n.parent]),
            "node {i} shares its parent's style"
        );
    }
}

#[test]
fn pseudos_and_grids_cost_nothing_where_absent() {
    let mut html =
        String::from("<style>.x::before{content:'*'}.g{display:grid}.m{grid-area:main}</style>");
    for i in 0..2000 {
        html.push_str(&format!("<p>{i}</p>"));
    }
    html.push_str("<p class=x id=last>x</p><div class=g><p class=m>g</p></div>");
    let d = dom::parse(html.as_bytes());
    let s = compute(&d, &collect_css(&d));
    let with: usize = (0..d.nodes.len()).filter(|&i| !s.pseudos[i].is_empty()).count();
    assert_eq!(with, 1, "one element has a ::before");
    let last = d.nodes.iter().position(|n| n.attr("id") == Some("last")).expect("last");
    assert_eq!(s.pseudos[last][0].text.as_deref(), Some("*"));
    assert_eq!(s.grids.iter().filter(|g| g.is_some()).count(), 1);
    assert!(core::mem::size_of_val(&s.grids[0]) <= 8, "a grid slot is one pointer");
}
