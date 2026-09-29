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
