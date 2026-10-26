// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


//! An app's ask for a rect: never moving an app that does not ask, clipped
//! to the work area, acted on only when it changes, given back when it
//! stops, and asked again only when the display had no answer.

use super::super::rect_ask::{Move, RectAsk};
use super::super::work_rect::{resolve, WorkRect};

const WORK: (u32, u32, u32, u32) = (0, 58, 1920, 922);
const HOME: (u32, u32, u32, u32) = (337, 151, 1246, 736);
const DROP: WorkRect = WorkRect { x: 0, y: 0, w: 1000, h: 450 };

#[test]
fn an_app_that_does_not_ask_is_never_moved_and_costs_nothing() {
    let mut ask = RectAsk::new();
    for _ in 0..3 {
        let mv = ask.step(None, HOME, || panic!("the display is not asked"));
        assert_eq!(mv, Move::Stay);
    }
}

#[test]
fn an_ask_is_placed_once_and_given_back() {
    let mut ask = RectAsk::new();
    assert_eq!(ask.step(Some(DROP), HOME, || Some(WORK)), Move::Place((0, 58, 1920, 414)));
    let again = ask.step(Some(DROP), (0, 58, 1920, 414), || panic!("not asked twice"));
    assert_eq!(again, Move::Stay);
    assert_eq!(ask.step(None, (0, 58, 1920, 414), || None), Move::Restore(HOME));
    assert_eq!(ask.step(None, HOME, || None), Move::Stay);
}

#[test]
fn a_changed_ask_still_gives_back_the_first_rect() {
    let mut ask = RectAsk::new();
    ask.step(Some(DROP), HOME, || Some(WORK));
    let taller = WorkRect { h: 700, ..DROP };
    assert!(matches!(ask.step(Some(taller), (0, 58, 1920, 414), || Some(WORK)), Move::Place(_)));
    assert_eq!(ask.step(None, (0, 58, 1920, 645), || None), Move::Restore(HOME));
}

#[test]
fn a_rect_is_clipped_to_the_work_area() {
    let r = resolve(WorkRect { x: 900, y: 900, w: 500, h: 500 }, WORK);
    assert_eq!(r, (960, 519, 960, 461), "inside the work area, not past it");
    let big = resolve(WorkRect { x: 0, y: 0, w: 4000, h: 4000 }, WORK);
    assert_eq!(big, WORK);
    assert_eq!(resolve(WorkRect { x: 0, y: 0, w: 0, h: 0 }, WORK).2, 1, "never zero wide");
}

#[test]
fn no_display_answer_is_asked_again_next_frame() {
    let mut ask = RectAsk::new();
    assert_eq!(ask.step(Some(DROP), HOME, || None), Move::Stay);
    assert_eq!(ask.step(Some(DROP), HOME, || Some(WORK)), Move::Place((0, 58, 1920, 414)));
}
