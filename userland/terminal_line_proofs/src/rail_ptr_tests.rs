// NONOS Operating System (AGPL-3.0-or-later)
//! The rail's pointer: a point resolves to the control drawn under it at any
//! scroll, a move inside one control is idle and one onto the next repaints, a
//! press acts on its control and lights only it, and leaving unlights once.

use nonos_app_skeleton::InputKind::{ButtonDown, ButtonUp, PointerAbs};
use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::widgets::{Pointer, State};

use crate::rail_boxes::{col, drawn, ev, mid};
use crate::rail_ptr::{feed, leave, Step};
use crate::strip_boxes::serial;

#[test]
fn every_control_answers_where_it_is_drawn_at_every_scroll() {
    let _g = serial();
    for q in [4, 8] {
        set_scale(q);
        for scroll in [0, 37, 120, u32::MAX] {
            let c = col(scroll);
            assert!(scroll == 0 || c.off > 0, "the fixture must scroll");
            for (want, b) in drawn(&c).into_iter().filter(|&(_, b)| c.shows(b)) {
                let (x, y) = mid(&c, b);
                if c.local(x, y).is_some() {
                    assert_eq!(c.spot(x, y), Some(want), "{q} {scroll} {want:?}");
                }
            }
        }
    }
}

#[test]
fn a_move_inside_a_control_is_idle_and_one_onto_the_next_repaints() {
    let _g = serial();
    set_scale(4);
    let (c, mut ptr) = (col(0), Pointer::new());
    let all: Vec<_> = drawn(&c).into_iter().filter(|&(_, b)| c.shows(b)).collect();
    for pair in all.windows(2) {
        let ((a, ra), (b, rb)) = (pair[0], pair[1]);
        let (x, y) = mid(&c, ra);
        feed(&mut ptr, &c, &ev(PointerAbs, (x, y + 1)));
        assert_eq!(feed(&mut ptr, &c, &ev(PointerAbs, (x, y))), Step::Idle, "{a:?}");
        assert_eq!(feed(&mut ptr, &c, &ev(PointerAbs, mid(&c, rb))), Step::Repaint, "{b:?}");
        assert!(c.look(&ptr, rb).hovered && c.look(&ptr, ra) == State::default());
    }
}

#[test]
fn a_press_acts_on_its_control_and_lights_only_it() {
    let _g = serial();
    set_scale(4);
    let (c, mut ptr) = (col(60), Pointer::new());
    let all: Vec<_> = drawn(&c).into_iter().filter(|&(_, b)| c.shows(b)).collect();
    for &(want, b) in &all {
        let at = mid(&c, b);
        if c.local(at.0, at.1).is_none() {
            continue;
        }
        assert_eq!(feed(&mut ptr, &c, &ev(ButtonDown, at)), Step::Act(want));
        assert!(all.iter().all(|&(s, o)| c.look(&ptr, o).pressed == (s == want)), "{want:?}");
        assert_eq!(feed(&mut ptr, &c, &ev(ButtonUp, at)), Step::Repaint);
    }
}

#[test]
fn leaving_the_rail_unlights_it_once() {
    let _g = serial();
    set_scale(4);
    let (c, mut ptr) = (col(0), Pointer::new());
    let (_, b) = drawn(&c)[2];
    feed(&mut ptr, &c, &ev(PointerAbs, mid(&c, b)));
    assert!(c.look(&ptr, b).hovered && !leave(&mut Pointer::new(), &c) && leave(&mut ptr, &c));
    assert!(c.lit(&ptr).is_none() && c.look(&ptr, b) == State::default());
    assert!(!leave(&mut ptr, &c));
}
