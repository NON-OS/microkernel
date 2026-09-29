// NONOS Operating System (AGPL-3.0-or-later)
//! Intrinsic widths are measured once per box per layout: nesting flex
//! boxes deeper adds a fixed number of text measurements per level, where
//! measuring each subtree again at every level above it grew with the
//! square of the depth. Deep nests of this package's boxes lay out within
//! the device's 2 MiB stack.

use crate::browser::layout::boxmodel::contexts::inline_word::MEASURES;
use crate::render::render_at;

fn nest(open: &str, close: &str, leaf: &str, n: usize) -> String {
    let mut s = String::from("<body>");
    (0..n).for_each(|_| s.push_str(open));
    s.push_str(leaf);
    (0..n).for_each(|_| s.push_str(close));
    s
}

fn measures(html: &str) -> usize {
    MEASURES.with(|m| m.set(0));
    let doc = render_at(html, (1336, 800));
    assert!(!doc.frags.is_empty());
    MEASURES.with(|m| m.get())
}

/* Twice the depth, at most twice the measurements (linear), not four. */
#[test]
fn nested_flex_measures_grow_linearly() {
    let leaf = "<span>alpha beta gamma delta</span><span>epsilon zeta</span>";
    let at = |d| {
        measures(&nest(&["<div style=display:flex><div>", leaf].concat(), "</div></div>", "", d))
    };
    let (d8, d16) = (at(8), at(16));
    assert!(d16 <= 2 * d8, "depth 8: {d8} measures, depth 16: {d16}");
}

/* Deep nests through the new paths stay on the stack. */
#[test]
fn deep_inline_flex_grid_and_span_nests_lay_out() {
    for (open, close) in [
        ("<span style='display:inline-flex;padding:0 2px'>", "</span>"),
        ("<div style='display:grid;grid-template-columns:auto 1fr'>", "</div>"),
        ("<span style='padding:0 1px;background:#eee'>a ", "</span>"),
        ("<div style='display:flex;flex-direction:column;align-items:center'>", "</div>"),
        ("<span dir=rtl style='direction:rtl'>\u{5d0} ", "</span>"),
    ] {
        let html = nest(open, close, "x", 450);
        assert!(!render_at(&html, (800, 600)).frags.is_empty(), "{open}");
    }
}
