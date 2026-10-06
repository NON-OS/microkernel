// NONOS Operating System (AGPL-3.0-or-later)
//! Script queries: every match comes back in tree order, an element query
//! walks only its subtree with :scope as the element, `#id` takes a fast
//! path that still answers like the full walk, and detached nodes are never
//! found.

use capsule_browser_proofs::browser::{css, dom};

fn anchors(n: usize) -> dom::Dom {
    let mut h = String::from("<!DOCTYPE html><html><body><p id=x>x</p><div id=box>");
    for i in 0..n {
        h.push_str(&format!("<a href=/{i} id=a{i}>{i}</a>"));
    }
    dom::parse(format!("{h}</div><a id=out>o</a></body></html>").as_bytes())
}

#[test]
fn queries_return_every_match() {
    let d = anchors(300);
    assert_eq!(css::select(&d, "a", usize::MAX).len(), 301);
    assert_eq!(css::select(&anchors(2000), "a[href]", usize::MAX).len(), 2000);
    assert_eq!(css::select(&d, "a", 7).len(), 7, "the limit still limits");
}

#[test]
fn an_element_query_walks_only_its_subtree() {
    let d = anchors(300);
    let bx = css::select(&d, "#box", 1)[0];
    let hits = css::select_in(&d, bx, "a", usize::MAX);
    assert_eq!(hits.len(), 300);
    assert!(hits.windows(2).all(|w| w[0] < w[1]), "tree order");
    assert_eq!(css::select_in(&d, bx, ":scope > a:last-child", usize::MAX).len(), 1);
    assert_eq!(css::select_in(&d, bx, "body a", usize::MAX).len(), 300, "ancestors outside count");
    assert!(css::select_in(&d, bx, ":scope", usize::MAX).is_empty(), "the root is not a candidate");
}

#[test]
fn id_queries_answer_like_the_walk() {
    let mut d = anchors(50);
    assert_eq!(css::select(&d, "#a7", usize::MAX), css::select(&d, "a#a7", usize::MAX));
    assert!(css::select(&d, "#nonexistent", usize::MAX).is_empty());
    /* Two elements with one id: the first in tree order answers. */
    let p = css::select(&d, "#x", 1)[0];
    d.set_attr(css::select(&d, "#out", 1)[0], "id", "x".into());
    assert_eq!(css::select(&d, "#x", 1), vec![p]);
    assert_eq!(css::select(&d, "#x", usize::MAX).len(), 2);
}

#[test]
fn detached_nodes_are_never_found() {
    let mut d = anchors(3);
    let a0 = css::select(&d, "#a0", 1)[0];
    d.detach(a0);
    assert!(css::select(&d, "#a0", usize::MAX).is_empty());
    assert_eq!(css::select(&d, "a", usize::MAX).len(), 3);
    let made = d.create(dom::node::NodeKind::Element, "a".into()).unwrap();
    d.set_attr(made, "id", "new".into());
    assert!(css::select(&d, "#new", usize::MAX).is_empty(), "created, never attached");
}

#[test]
fn matches_and_closest_scope_to_the_element() {
    let d = anchors(3);
    let a1 = css::select(&d, "#a1", 1)[0];
    assert!(css::matches(&d, a1, "div > a:nth-child(2)"));
    assert!(css::matches(&d, a1, ":scope"));
    assert_eq!(css::closest(&d, a1, "div"), css::select(&d, "#box", 1).first().copied());
    assert_eq!(css::closest(&d, a1, "#box > :scope"), Some(a1));
}
