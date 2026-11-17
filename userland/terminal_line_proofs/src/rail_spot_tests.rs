// NONOS Operating System (AGPL-3.0-or-later)
//! The rail's rows and their hit test, ported from the host harness the
//! toolkit geometry retired: rows tile, a point off the lists or past the
//! count hits nothing, the plus buttons win over the rows, and a pinned path
//! is named by its last component.

use nonos_toolkit::design::scale::set_scale;

use crate::rail_boxes::RAIL;
use crate::rail_nav::{nav, row_h};
use crate::rail_scroll::{column, RailFit};
use crate::rail_spot::{base_name, row, spot, Spot};
use crate::strip_boxes::serial;

#[test]
fn rows_tile_without_overlap_or_gap() {
    let _g = serial();
    set_scale(4);
    let l = nav(RAIL.w, 9, 0).s_list;
    for i in 0..9 {
        let a = row(l, i);
        assert_eq!((a.x, a.w, a.h, a.y), (l.x, l.w, row_h(), l.y + i * row_h()), "row {i}");
    }
}

#[test]
fn a_point_off_the_lists_or_past_the_count_hits_no_row() {
    let _g = serial();
    set_scale(4);
    let n = nav(RAIL.w, 4, 2);
    let (l, at) = (n.s_list, |x: u32, y: u32| spot(&n, 4, 2, x as i32, y as i32));
    assert_eq!(at(l.x + l.w, l.y + 1), None);
    assert_eq!(at(l.x + 1, n.p_list.y + 2 * row_h()), None);
    assert_eq!(spot(&n, 2, 2, 1, (row(l, 2).y + 1) as i32), None);
    assert_eq!(at(1, row(l, 2).y + 1), Some(Spot::Session(2)));
    assert_eq!(at(1, row(n.p_list, 1).y + 1), Some(Spot::Project(1)));
    let (s, p) = (n.s_plus, n.p_plus);
    assert_eq!(at(s.x + s.w / 2, s.y + 1), Some(Spot::NewSession));
    assert_eq!(at(p.x + p.w / 2, p.y + 1), Some(Spot::AddProject));
}

#[test]
fn base_name_takes_the_last_component() {
    assert_eq!(base_name("/home/user/src"), "src");
    assert_eq!(base_name("/"), "/");
    assert_eq!(base_name(""), "");
    assert_eq!(base_name("bare"), "bare");
    assert_eq!(base_name("/trailing/"), "/trailing/");
}

#[test]
fn the_column_shows_and_takes_exactly_what_touches_the_rail() {
    let _g = serial();
    set_scale(4);
    let c = column(RAIL, 100, RailFit { sessions: 30, projects: 20, telemetry: true });
    let b = |y| nonos_toolkit::widgets::Rect { x: 0, y, w: 10, h: 20 };
    assert!(!c.shows(b(80)) && c.shows(b(81)));
    assert!(c.shows(b(100 + RAIL.h - 1)) && !c.shows(b(100 + RAIL.h)));
    let (x, y) = (RAIL.x as i32, RAIL.y as i32);
    assert_eq!(c.local(x, y), Some((0, 100)));
    assert_eq!(c.local(x, y - 1), None);
    assert_eq!(c.local(x + RAIL.w as i32, y), None);
    assert_eq!(c.local(x, y + RAIL.h as i32), None);
}
