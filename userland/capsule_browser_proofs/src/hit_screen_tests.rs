// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! Clicks hit what is painted. A fixed header paints at the top of the
//! viewport whatever the scroll; hit tests ran in document space, so after
//! scrolling a click on the header found the content under it instead.

use crate::browser::layout::hit_screen::frag_screen_y;
use crate::render::render;

const PAGE: &str = "<html><head><style>body{margin:0}\
    .top{position:fixed;top:0;left:0;width:100%;height:40px;background:#123}\
    .top a{display:block;height:40px}\
    .body{padding-top:40px}\
    .body a{display:block;height:40px}\
    </style></head><body>\
    <div class=top><a href=/nav>nav</a></div>\
    <div class=body><a href=/one>one</a><div style=height:3000px></div>\
    <a href=/two>two</a></div></body></html>";

#[test]
fn a_fixed_header_is_hit_where_it_is_painted() {
    let doc = render(PAGE, 800);
    assert_eq!(doc.link_at(10, 10, 0), Some("/nav"));
    assert_eq!(doc.link_at(10, 10, 500), Some("/nav"));
    assert_eq!(doc.link_at(10, 50, 0), Some("/one"));
    assert_eq!(doc.link_at(10, 50, 500), None);
    let two = doc.frags.iter().find(|f| f.href.as_deref() == Some("/two")).expect("link");
    let y = two.y - 3000 + 5;
    assert_eq!(doc.link_at(10, y, 3000), Some("/two"));
    assert!(doc.hit_node(10, 10, 500).is_some());
}

#[test]
fn screen_rows_for_fixed_sticky_and_flowing_boxes() {
    assert_eq!(frag_screen_y(100, true, None, 500), 100);
    assert_eq!(frag_screen_y(700, false, None, 500), 200);
    assert_eq!(frag_screen_y(300, false, Some((300, 0)), 100), 200);
    assert_eq!(frag_screen_y(300, false, Some((300, 0)), 500), 0);
    assert_eq!(frag_screen_y(i32::MAX, false, None, -5), i32::MAX);
}
