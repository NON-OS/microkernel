// NONOS Operating System (AGPL-3.0-or-later)
//! The rail's lists as painted: hovering or pressing a control changes pixels
//! only inside the box a click there lands on, the selection washes only the
//! active session's row, and a scrolled column is the same picture moved up.

use nonos_app_skeleton::InputKind::{ButtonDown, PointerAbs};
use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::widgets::rect::contains;
use nonos_toolkit::widgets::{Pointer, Rect};

use crate::rail_boxes::{changed, col, drawn, ev, frame, mid};
use crate::rail_column::Column;
use crate::rail_ptr::feed;
use crate::rail_spot::Spot;
use crate::strip_boxes::serial;

fn screen(off: u32, b: Rect) -> Rect {
    Rect { y: b.y.saturating_sub(off), h: (b.y + b.h).saturating_sub(off.max(b.y)), ..b }
}

#[test]
fn hover_and_press_light_only_the_box_a_click_there_lands_on() {
    let _g = serial();
    for (q, scroll) in [(4, 0), (4, 90), (8, 0)] {
        set_scale(q);
        let c = col(scroll);
        let rest = frame(&c, &Pointer::new(), 0, c.rect.h);
        for (spot, b) in drawn(&c).into_iter().filter(|&(_, b)| c.shows(b)) {
            let at = mid(&c, b);
            if c.local(at.0, at.1).is_none() {
                continue;
            }
            for (kind, how) in [(PointerAbs, "hover"), (ButtonDown, "press")] {
                let mut ptr = Pointer::new();
                feed(&mut ptr, &c, &ev(kind, at));
                let lit = changed(&rest, &frame(&c, &ptr, 0, c.rect.h), c.rect.w);
                assert!(!lit.is_empty(), "{q} {scroll} {spot:?} {how} draws nothing");
                let inside = |&(x, y): &(i32, i32)| contains(screen(c.off, b), x, y);
                assert!(lit.iter().all(inside), "{q} {scroll} {spot:?} {how}");
                assert_eq!(c.spot(at.0, at.1), Some(spot));
            }
        }
    }
}

#[test]
fn the_selection_washes_only_the_active_sessions_row() {
    let _g = serial();
    set_scale(4);
    let c = col(0);
    let row = |i| drawn(&c).into_iter().find(|&(s, _)| s == Spot::Session(i)).unwrap().1;
    let (a, b) = (frame(&c, &Pointer::new(), 0, c.rect.h), frame(&c, &Pointer::new(), 2, c.rect.h));
    let lit = changed(&a, &b, c.rect.w);
    assert!(lit.iter().any(|&(x, y)| contains(row(0), x, y)));
    assert!(lit.iter().any(|&(x, y)| contains(row(2), x, y)));
    assert!(lit.iter().all(|&(x, y)| contains(row(0), x, y) || contains(row(2), x, y)));
}

#[test]
fn a_scrolled_column_is_the_same_picture_moved_up() {
    let _g = serial();
    set_scale(4);
    let c = col(90);
    let top = Column { off: 0, rect: Rect { h: c.rect.h + c.off, ..c.rect }, ..c };
    assert_eq!(c.off, 90);
    let w = c.rect.w;
    let tall = frame(&top, &Pointer::new(), 0, c.rect.h + c.off);
    let moved = frame(&c, &Pointer::new(), 0, c.rect.h);
    let skip = (w * c.off) as usize;
    assert_eq!(&tall[skip..], &moved[..], "the column translates, it does not relayout");
}
