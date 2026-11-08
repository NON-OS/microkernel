// NONOS Operating System (AGPL-3.0-or-later)
//! The rail's column: the lists stack with each plus inside its heading, the
//! telemetry steps by the heights the scroll is clamped against, and the
//! offset is clamped at both ends.

use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::widgets::rect::contains;

use crate::rail_boxes::RAIL;
use crate::rail_geom::{disk_h, net_h, sys_h, telemetry_h};
use crate::rail_nav::{head_h, nav, nav_h, row_h, title};
use crate::rail_scroll::{clamp, column, content_h, telemetry_top, RailFit};
use crate::rail_text::{gap, lh, pad};
use crate::strip_boxes::serial;

fn fit(sessions: u32, projects: u32) -> RailFit {
    RailFit { sessions, projects, telemetry: true }
}

#[test]
fn the_lists_stack_and_nav_height_tracks_the_last_one() {
    let _g = serial();
    for q in [4, 8] {
        set_scale(q);
        for (s, p) in [(0, 0), (1, 0), (4, 3), (11, 4)] {
            let n = nav(RAIL.w, s, p);
            assert_eq!((n.s_head.h, n.p_head.h), (head_h(), head_h()));
            let tops = (n.s_head.y + n.s_head.h, n.s_list.y + n.s_list.h + gap());
            assert_eq!((n.s_list.y, n.p_head.y, n.p_list.y), (tops.0, tops.1, tops.1 + head_h()));
            assert_eq!((n.s_list.h, n.p_list.h), (s * row_h(), p * row_h()));
            assert_eq!(n.p_list.y + n.p_list.h, nav_h(s, p), "{q} {s} {p}");
        }
    }
}

#[test]
fn each_plus_sits_in_its_heading_clear_of_the_title() {
    let _g = serial();
    for q in [4, 8] {
        set_scale(q);
        let n = nav(RAIL.w, 4, 3);
        for (head, plus) in [(n.s_head, n.s_plus), (n.p_head, n.p_plus)] {
            assert!(contains(head, plus.x as i32, plus.y as i32) && plus.w > 0);
            assert!(plus.x + plus.w <= head.x + head.w && plus.y + plus.h <= head.y + head.h);
            let t = title(head, plus);
            assert!(t.w > 0 && t.x + t.w < plus.x, "{q}");
        }
    }
}

#[test]
fn the_telemetry_steps_by_the_heights_the_scroll_is_clamped_against() {
    let _g = serial();
    set_scale(4);
    assert_eq!(net_h(), head_h() + lh() * 4);
    assert_eq!(disk_h(), head_h() + lh() * 2);
    assert!(sys_h() > head_h() + lh() * 5);
    assert_eq!(telemetry_h(), sys_h() + net_h() + disk_h() + gap() * 2 + pad());
    let f = fit(4, 3);
    assert_eq!(content_h(f), nav_h(4, 3) + gap() + telemetry_h());
    assert_eq!(content_h(RailFit { telemetry: false, ..f }), nav_h(4, 3));
    assert_eq!(telemetry_top(f), nav_h(4, 3) + gap());
}

#[test]
fn the_offset_is_clamped_at_both_ends() {
    let _g = serial();
    set_scale(4);
    let (f, content) = (fit(30, 20), content_h(fit(30, 20)));
    assert!(content > RAIL.h, "the fixture must overflow the rail");
    assert_eq!(clamp(0, f, RAIL.h), 0);
    assert_eq!(clamp(u32::MAX, f, RAIL.h), content - RAIL.h);
    assert_eq!(column(RAIL, u32::MAX, f).off, content - RAIL.h);
    assert_eq!(clamp(500, fit(1, 0), 4000), 0, "a short column never scrolls");
}
