// NONOS Operating System (AGPL-3.0-or-later)
//! The strip's pointer: a move inside one control repaints nothing and one
//! onto the next repaints, a press acts on the control under it and lights
//! only that one, and leaving the strip unlights everything once.

use nonos_app_skeleton::InputEvent;
use nonos_app_skeleton::InputKind::{self, ButtonDown, ButtonUp, PointerAbs};
use nonos_toolkit::widgets::{Rect, State};

use crate::strip_ptr::StripPointer;
use crate::strip_spot::{Spot, Step};
use crate::strip_boxes::{drawn, serial, SETS, SIZES};

fn feed(sp: &mut StripPointer, kind: InputKind, (x, y): (i32, i32)) -> Step {
    let (flags, code, delta_x, delta_y, timestamp_ns) = (0, 0, 0, 0, 0);
    sp.feed(SETS[1], 0, &InputEvent { kind, flags, code, x, y, delta_x, delta_y, timestamp_ns })
}

fn mid(r: Rect) -> (i32, i32) {
    ((r.x + r.w / 2) as i32, (r.y + r.h / 2) as i32)
}

#[test]
fn a_move_inside_a_control_is_idle_and_one_onto_the_next_repaints() {
    let _g = serial();
    let mut sp = StripPointer { size: SIZES[0], ..Default::default() };
    for pair in drawn(SETS[1], 0, SIZES[0]).windows(2) {
        let ((a, ra), (b, rb)) = (pair[0], pair[1]);
        feed(&mut sp, PointerAbs, (ra.x as i32, ra.y as i32 + 2));
        assert_eq!(feed(&mut sp, PointerAbs, mid(ra)), Step::Idle, "{a:?}");
        assert_eq!(feed(&mut sp, PointerAbs, mid(rb)), Step::Repaint, "{b:?}");
        assert!(sp.look(rb).hovered && sp.look(ra) == State::default());
    }
}

#[test]
fn a_press_acts_on_its_control_and_lights_only_it() {
    let _g = serial();
    let mut sp = StripPointer { size: SIZES[0], ..Default::default() };
    let all = drawn(SETS[1], 0, SIZES[0]);
    for &(want, r) in &all {
        assert_eq!(feed(&mut sp, ButtonDown, mid(r)), Step::Act(want));
        assert!(all.iter().all(|&(s, o)| sp.look(o).pressed == (s == want)), "{want:?}");
        assert_eq!(feed(&mut sp, ButtonUp, mid(r)), Step::Repaint);
    }
    assert_eq!(feed(&mut sp, ButtonDown, (-5, -5)), Step::Repaint);
}

#[test]
fn leaving_the_strip_unlights_it_once() {
    let _g = serial();
    let mut sp = StripPointer { size: SIZES[0], ..Default::default() };
    let (_, r) = drawn(SETS[1], 0, SIZES[0])[0];
    feed(&mut sp, PointerAbs, mid(r));
    assert!(sp.leave(SETS[1], 0));
    assert!(sp.spot(SETS[1], 0).is_none() && sp.look(r) == State::default());
    assert!(!sp.leave(SETS[1], 0));
    assert!(matches!(drawn(SETS[1], 0, SIZES[0])[0].0, Spot::Tool(0)));
}
