// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! Scrolling against the real viewport. It used a constant 680 px (a 760 px
//! window shows 619, leaving 61 px unreachable), and End added i32::MAX to
//! the offset, which wrapped negative and jumped to the top.

use crate::browser::omnibox::parts::page_step;
use crate::browser::omnibox::{scroll_to, ScrollAct};

const VIEW: u32 = 619;
const PAGE: u32 = 5000;

#[test]
fn end_reaches_the_bottom_and_stays() {
    let end = scroll_to(100, PAGE, VIEW, ScrollAct::End);
    assert_eq!(end, PAGE - VIEW);
    assert_eq!(scroll_to(end, PAGE, VIEW, ScrollAct::Line(1)), end);
    assert_eq!(scroll_to(end, PAGE, VIEW, ScrollAct::Page(1)), end);
}

#[test]
fn a_page_keeps_a_tenth_of_the_viewport() {
    assert_eq!(page_step(VIEW), 558);
    assert_eq!(scroll_to(0, PAGE, VIEW, ScrollAct::Page(1)), 558);
    assert_eq!(scroll_to(558, PAGE, VIEW, ScrollAct::Page(-1)), 0);
    assert_eq!(page_step(30), 40, "never less than a line");
}

#[test]
fn extreme_wheel_deltas_clamp_without_overflow() {
    assert_eq!(scroll_to(10, PAGE, VIEW, ScrollAct::Wheel(i32::MIN)), PAGE - VIEW);
    assert_eq!(scroll_to(10, PAGE, VIEW, ScrollAct::Wheel(i32::MAX)), 0);
    assert_eq!(scroll_to(0, PAGE, VIEW, ScrollAct::Wheel(-1)), 60);
    assert_eq!(scroll_to(u32::MAX, PAGE, VIEW, ScrollAct::Line(i32::MAX)), PAGE - VIEW);
}

#[test]
fn a_short_page_does_not_scroll() {
    assert_eq!(scroll_to(0, 300, VIEW, ScrollAct::End), 0);
    assert_eq!(scroll_to(0, 300, VIEW, ScrollAct::Page(1)), 0);
}

#[test]
fn home_and_anchor_positions() {
    assert_eq!(scroll_to(900, PAGE, VIEW, ScrollAct::Home), 0);
    assert_eq!(scroll_to(0, PAGE, VIEW, ScrollAct::To(1200)), 1200);
    assert_eq!(scroll_to(0, PAGE, VIEW, ScrollAct::To(i64::MAX)), PAGE - VIEW);
    assert_eq!(scroll_to(0, PAGE, VIEW, ScrollAct::To(-5)), 0);
}

/* A page's own scrollTo and scrollIntoView (dom::script_scroll): held to
 * what the page can scroll once it is laid out, kept as asked before. */
mod script_scroll {
    use crate::browser::dom::parse;
    use crate::browser::dom::script_scroll::Block;
    use crate::browser::dom::Dom;

    fn laid(html: &str) -> Dom {
        let mut dom = parse(html.as_bytes());
        dom.viewport = (800, 600);
        dom.content_h = 3000;
        /* Every node 100 px tall, the n-th at 200 * n. */
        let n = dom.nodes.len();
        dom.record_rects((1..n).map(|i| (i, 0, 200 * i as i32, 800, 100)));
        dom
    }

    #[test]
    fn a_scroll_is_held_to_the_page() {
        let mut dom = laid("<p>a</p>");
        assert_eq!(dom.script_scroll(500), 500);
        assert_eq!(dom.scroll_y, 500);
        assert_eq!(dom.script_scroll(10_000), 2400, "the bottom meets the window's");
        assert_eq!(dom.script_scroll(-5), 0);
        let mut fresh = parse(b"<p>a</p>");
        assert_eq!(fresh.script_scroll(900), 900, "before a layout the wish is kept");
    }

    #[test]
    fn into_view_puts_the_element_where_block_says() {
        let mut dom = laid("<div id=a></div>");
        let id = dom.nodes.iter().position(|n| n.attr("id") == Some("a")).expect("the div");
        let top = 200 * id as i64;
        assert_eq!(dom.into_view_y(id, Block::Start), Some(top));
        assert_eq!(dom.into_view_y(id, Block::End), Some(top + 100 - 600));
        assert_eq!(dom.into_view_y(id, Block::Center), Some(top + 50 - 300));
        dom.scroll_y = (top - 100) as u32;
        assert_eq!(dom.into_view_y(id, Block::Nearest), Some(top - 100), "already in view");
        dom.scroll_y = (top + 50) as u32;
        assert_eq!(dom.into_view_y(id, Block::Nearest), Some(top), "above: its top to the top");
        dom.scroll_y = 0;
        assert_eq!(dom.into_view_y(id, Block::Nearest), Some(top - 500), "below: bottom to bottom");
        dom.rects[id] = [0; 4];
        assert_eq!(dom.into_view_y(id, Block::Start), None, "one not laid out moves nothing");
        assert_eq!(Block::from_code(2), Block::End);
        assert_eq!(Block::from_code(9), Block::Start);
    }
}
