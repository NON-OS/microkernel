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

//! Positions, buttons, the wheel and motion posted from the queue; the
//! bound on one look; a pointer lost and brought back; the console lines.

use nonos_libc::{
    backdoor_calls, input_abs_scale, logged, posted, InputEvent, INPUT_ABS_MAX,
    INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP, INPUT_KIND_POINTER_ABS, INPUT_KIND_POINTER_REL,
    INPUT_KIND_WHEEL,
};

use super::util::up;
use crate::constants::{DISABLE, ENABLE, PACKETS_PER_LOOK};
use crate::say;
use crate::serve::{step, Look};

fn ev(kind: u16, code: u32, at: (i32, i32), delta: (i32, i32)) -> InputEvent {
    InputEvent {
        kind,
        flags: 0,
        code,
        x: at.0,
        y: at.1,
        delta_x: delta.0,
        delta_y: delta.1,
        timestamp_ns: 0,
    }
}

fn abs(x: i32, y: i32) -> InputEvent {
    let at = (input_abs_scale(x, 0, 0xFFFF), input_abs_scale(y, 0, 0xFFFF));
    ev(INPUT_KIND_POINTER_ABS, 0, at, (0, 0))
}

#[test]
fn a_position_is_scaled_and_posted_once() {
    let (mut d, h) = up();
    h.mouse(0x4000, 0x2000, 0, 0);
    h.mouse(0x4000, 0x2000, 0, 0);
    assert_eq!(step(&mut d), Look::Idle, "both read, the queue empty");
    assert_eq!(posted(), [abs(0x8000, 0x4000)]);
    h.mouse(0x7FFF, 0, 0, 0);
    step(&mut d);
    assert_eq!(posted(), [abs(0xFFFE, 0)]);
}

#[test]
fn buttons_and_the_wheel() {
    let (mut d, h) = up();
    h.mouse(10, 10, 1, 0);
    h.mouse(10, 10, 1 | 2, 0);
    h.mouse(10, 10, 4, 0);
    h.mouse(10, 10, 0, -1);
    h.mouse(10, 10, 0, 2);
    step(&mut d);
    let e = posted();
    assert_eq!(
        e[1..],
        [
            ev(INPUT_KIND_BUTTON_DOWN, 1, (0, 0), (0, 0)),
            ev(INPUT_KIND_BUTTON_DOWN, 2, (0, 0), (0, 0)),
            ev(INPUT_KIND_BUTTON_UP, 1, (0, 0), (0, 0)),
            ev(INPUT_KIND_BUTTON_UP, 2, (0, 0), (0, 0)),
            ev(INPUT_KIND_BUTTON_DOWN, 3, (0, 0), (0, 0)),
            ev(INPUT_KIND_WHEEL, 0, (0, 0), (0, 1)),
            ev(INPUT_KIND_BUTTON_UP, 3, (0, 0), (0, 0)),
            ev(INPUT_KIND_WHEEL, 0, (0, 0), (0, -2)),
        ]
    );
}

#[test]
fn motion_while_the_host_has_the_pointer() {
    let (mut d, h) = up();
    h.raw(&[0x0001_0020, 5, 3, 0]);
    h.raw(&[0x0001_0000, 0, 0, 0]);
    step(&mut d);
    assert_eq!(
        posted(),
        [
            ev(INPUT_KIND_POINTER_REL, 0, (0, 0), (5, -3)),
            ev(INPUT_KIND_BUTTON_DOWN, 1, (0, 0), (0, 0)),
            ev(INPUT_KIND_BUTTON_UP, 1, (0, 0), (0, 0)),
        ]
    );
}

#[test]
fn a_position_after_motion_is_posted_again() {
    let (mut d, h) = up();
    h.mouse(0x4000, 0x2000, 0, 0);
    h.raw(&[0x0001_0000, 4, 0, 0]);
    h.mouse(0x4000, 0x2000, 0, 0);
    step(&mut d);
    let p = abs(0x8000, 0x4000);
    assert_eq!(posted(), [p, ev(INPUT_KIND_POINTER_REL, 0, (0, 0), (4, 0)), p]);
}

#[test]
fn a_position_past_the_grid_is_its_edge() {
    let (mut d, h) = up();
    h.raw(&[0, 0x1_0000, u32::MAX, 0]);
    step(&mut d);
    assert_eq!(posted(), [ev(INPUT_KIND_POINTER_ABS, 0, (INPUT_ABS_MAX, 0), (0, 0))]);
}

#[test]
fn one_look_takes_a_bounded_number_of_packets() {
    let (mut d, h) = up();
    for i in 0..40 {
        h.mouse(i * 0x100, i * 0x100, 0, 0);
    }
    assert_eq!(step(&mut d), Look::Read);
    assert_eq!(h.0.borrow().queue.len(), (40 - PACKETS_PER_LOOK) * 4);
    assert_eq!(posted().len(), PACKETS_PER_LOOK);
}

#[test]
fn an_empty_queue_is_one_status_call() {
    let (mut d, _h) = up();
    assert_eq!(step(&mut d), Look::Idle);
    assert_eq!(backdoor_calls(), [(40, 0)]);
    assert!(posted().is_empty());
}

#[test]
fn a_broken_queue_loses_the_pointer_and_lets_its_buttons_go() {
    let (mut d, h) = up();
    h.mouse(1, 1, 1, 0);
    step(&mut d);
    let _ = (posted(), backdoor_calls());
    h.raw(&[0, 1, 2]);
    assert_eq!(step(&mut d), Look::Lost);
    assert_eq!(posted(), [ev(INPUT_KIND_BUTTON_UP, 1, (0, 0), (0, 0))]);
    assert!(logged().iter().any(|l| l == "driver.vmmouse: pointer lost, enabling it again\n"));
    assert_eq!(backdoor_calls(), [(40, 0), (41, DISABLE)], "no data read past a broken queue");
    assert!(!h.0.borrow().absolute);
    assert_eq!(step(&mut d), Look::Idle, "back on the next look");
    assert!(h.0.borrow().absolute);
    assert!(logged().iter().any(|l| l == "driver.vmmouse: pointer back\n"));
    let _ = backdoor_calls();
    assert_eq!(step(&mut d), Look::Idle);
    assert_eq!(backdoor_calls(), [(40, 0)], "back for good: not enabled again");
}

#[test]
fn an_error_the_host_flags_loses_the_pointer() {
    let (mut d, h) = up();
    h.0.borrow_mut().status = 0xFFFF;
    assert_eq!(step(&mut d), Look::Lost);
    assert!(d.lost);
}

#[test]
fn a_pointer_that_will_not_come_back_is_left_lost() {
    let (mut d, h) = up();
    h.0.borrow_mut().status = 0xFFFF;
    step(&mut d);
    h.0.borrow_mut().dead = true;
    let _ = backdoor_calls();
    assert_eq!(step(&mut d), Look::Lost);
    assert_eq!(step(&mut d), Look::Lost);
    let once = [(41, DISABLE), (41, ENABLE), (40, 0)];
    assert_eq!(backdoor_calls(), [once, once].concat(), "only enabling tried, no packet read");
    assert!(d.lost);
}

#[test]
fn the_console_names_the_version_alone() {
    say::up(6);
    assert_eq!(logged(), ["driver.vmmouse: up, backdoor version 6, absolute pointer on\n"]);
    say::up(4_294_967_295);
    assert_eq!(
        logged()[1],
        "driver.vmmouse: up, backdoor version 4294967295, absolute pointer on\n"
    );
}
