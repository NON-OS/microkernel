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

//! The cursor plane, judged from the host's side: a shape is loaded once
//! into a 64 x 64 resource with the slot's pixels, each scanout told it
//! with UPDATE_CURSOR at the image's top left; pointer motion is one
//! MOVE_CURSOR and not one command on the control queue; hiding is an
//! UPDATE with no resource, sent once. A cursor queue that stops answering
//! takes the plane away and leaves the screen served, and a recovery reset
//! ends the plane with its image's pages given back.

use nonos_gfx_wire::body::caps::HW_CURSOR;
use nonos_gfx_wire::body::cursor::{NEW_SHAPE, VISIBLE};
use nonos_gfx_wire::errno::{EINVAL, ENOSYS, ETIMEDOUT};
use nonos_gfx_wire::{ops, Caps};

use super::ask::ask;
use super::cursor_kit::{at, cursor_rig, point, shaped, Said, LEN};
use super::kit::{draw, live};
use super::recover_kit::{flush, stall};
use super::rig::{rig, Rig};
use crate::constants::{VG_CMD_MOVE_CURSOR as MOVE, VG_CMD_UPDATE_CURSOR as UPDATE};
use crate::constants::{VG_CMD_RESOURCE_FLUSH, VG_CMD_TRANSFER_TO_HOST_2D};

const SHOW: u32 = VISIBLE | NEW_SHAPE;
const ARROW: u32 = 0xFF10_2030;

fn caps(r: &mut Rig) -> Caps {
    Caps::decode(&ask(r, ops::CAPS, 0, &[]).1).expect("caps")
}

fn image(r: &Rig) -> u32 {
    let plane = r.d.server.display.cursor.as_ref().expect("the plane");
    plane.image.as_ref().expect("an image").id
}

#[test]
fn a_device_without_a_cursor_queue_has_no_plane_and_refuses_cursor() {
    let mut r = rig();
    assert_eq!(caps(&mut r).flags & HW_CURSOR, 0);
    let s = super::ask::surface(&mut r, 0);
    assert_eq!(s.cursor_offset, 0, "no slot without the plane");
    assert_eq!(point(&mut r, at(SHOW, (10, 10), (0, 0))), ENOSYS);
}

#[test]
fn a_new_shape_is_loaded_whole_and_told_at_its_top_left() {
    let mut c = cursor_rig();
    assert_ne!(caps(&mut c.r).flags & HW_CURSOR, 0, "the plane offered");
    shaped(&mut c.r, ARROW);
    assert_eq!(point(&mut c.r, at(SHOW, (100, 50), (4, 2))), 0);
    let id = image(&c.r);
    let host = c.r.host.borrow();
    let res = host.res.get(&id).expect("the image on the host");
    assert_eq!((res.w, res.h), (64, 64));
    assert!(res.px.iter().all(|&p| p == ARROW), "the slot's pixels, all of them");
    let told = Said { kind: UPDATE, scanout: 0, x: 96, y: 48, resource: id, hot: (4, 2), len: LEN };
    assert_eq!(c.cursor.borrow().said, [told]);
}

#[test]
fn pointer_motion_is_one_move_and_nothing_on_the_control_queue() {
    let mut c = cursor_rig();
    shaped(&mut c.r, ARROW);
    assert_eq!(point(&mut c.r, at(SHOW, (100, 50), (4, 2))), 0);
    let steps = c.r.host.borrow().steps();
    for i in 0..60 {
        assert_eq!(point(&mut c.r, at(VISIBLE, (100 + i, 50 + i / 2), (4, 2))), 0);
    }
    assert_eq!(c.r.host.borrow().steps(), steps, "no transfer, flush or any control command");
    let said = c.cursor.borrow().said.clone();
    assert_eq!(said.len(), 61, "one command a motion");
    let last = said[60];
    let moved = Said { kind: MOVE, scanout: 0, x: 155, y: 77, resource: 0, hot: (0, 0), len: LEN };
    assert_eq!(last, moved);
    assert!(said[1..].iter().all(|s| s.kind == MOVE && s.len == LEN));
}

#[test]
fn a_shape_changed_is_loaded_again_into_the_same_resource_and_told_again() {
    let mut c = cursor_rig();
    shaped(&mut c.r, ARROW);
    assert_eq!(point(&mut c.r, at(SHOW, (10, 10), (0, 0))), 0);
    let id = image(&c.r);
    shaped(&mut c.r, 0xFF00_00FF);
    let from = c.r.host.borrow().steps() as usize;
    assert_eq!(point(&mut c.r, at(SHOW, (10, 10), (7, 7))), 0);
    assert_eq!(image(&c.r), id, "the resource is reused, not made again");
    let host = c.r.host.borrow();
    let kinds: Vec<u32> = host.log[from..].iter().map(|c| c.kind).collect();
    assert_eq!(kinds, [VG_CMD_TRANSFER_TO_HOST_2D], "one transfer of the image, no flush");
    assert!(host.res[&id].px.iter().all(|&p| p == 0xFF00_00FF));
    let last = *c.cursor.borrow().said.last().unwrap();
    assert_eq!((last.kind, last.resource, last.hot, last.x), (UPDATE, id, (7, 7), 3));
}

#[test]
fn hiding_is_told_once_and_showing_again_tells_the_shape() {
    let mut c = cursor_rig();
    shaped(&mut c.r, ARROW);
    assert_eq!(point(&mut c.r, at(SHOW, (30, 30), (1, 1))), 0);
    assert_eq!(point(&mut c.r, at(0, (31, 31), (1, 1))), 0);
    assert_eq!(point(&mut c.r, at(0, (32, 32), (1, 1))), 0);
    assert_eq!(point(&mut c.r, at(VISIBLE, (33, 33), (1, 1))), 0);
    let id = image(&c.r);
    let said: Vec<(u32, u32)> =
        c.cursor.borrow().said.iter().map(|s| (s.kind, s.resource)).collect();
    assert_eq!(said, [(UPDATE, id), (UPDATE, 0), (UPDATE, id)], "hidden once, shown again");
}

#[test]
fn a_pointer_near_the_corner_puts_the_image_left_of_and_above_the_screen() {
    let mut c = cursor_rig();
    shaped(&mut c.r, ARROW);
    assert_eq!(point(&mut c.r, at(SHOW, (2, 1), (5, 5))), 0);
    let s = c.cursor.borrow().said[0];
    assert_eq!((s.x, s.y), (-3, -4));
}

#[test]
fn a_visible_cursor_before_any_shape_is_refused_and_nothing_is_sent() {
    let mut c = cursor_rig();
    shaped(&mut c.r, ARROW);
    assert_eq!(point(&mut c.r, at(VISIBLE, (10, 10), (0, 0))), EINVAL);
    assert!(c.cursor.borrow().said.is_empty());
}

#[test]
fn a_cursor_queue_that_stops_answering_takes_the_plane_and_leaves_the_screen_served() {
    let mut c = cursor_rig();
    let s = draw(&mut c.r, 0, 0x0011_2233);
    shaped(&mut c.r, ARROW);
    assert_eq!(point(&mut c.r, at(SHOW, (10, 10), (0, 0))), 0);
    c.cursor.borrow_mut().dead = true;
    assert_eq!(point(&mut c.r, at(VISIBLE, (11, 11), (0, 0))), ETIMEDOUT);
    assert_eq!(c.cursor.borrow().said.len(), 2);
    assert_eq!(caps(&mut c.r).flags & HW_CURSOR, 0, "the plane is no longer offered");
    assert_ne!(point(&mut c.r, at(VISIBLE, (12, 12), (0, 0))), 0);
    assert_eq!(c.cursor.borrow().said.len(), 2, "nothing staged over the chain it holds");
    let from = c.r.host.borrow().steps() as usize;
    assert_eq!(flush(&mut c.r, &s), 0, "the control queue serves on");
    let host = c.r.host.borrow();
    assert!(host.log[from..].iter().any(|c| c.landed && c.kind == VG_CMD_RESOURCE_FLUSH));
}

#[test]
fn a_recovery_reset_ends_the_plane_and_gives_its_image_back() {
    let mut c = cursor_rig();
    let s = draw(&mut c.r, 0, 0x0011_2233);
    shaped(&mut c.r, ARROW);
    assert_eq!(point(&mut c.r, at(SHOW, (10, 10), (0, 0))), 0);
    let with_image = live();
    assert!(!stall(&mut c.r, &s, 1), "recovered, not down");
    assert!(c.r.d.server.display.cursor.is_none(), "the reset ended the cursor queue");
    assert_eq!(live().len() + 1, with_image.len(), "the image's pages given back, no more");
    assert_eq!(caps(&mut c.r).flags & HW_CURSOR, 0);
    assert_eq!(point(&mut c.r, at(VISIBLE, (11, 11), (0, 0))), ENOSYS);
    assert_eq!(flush(&mut c.r, &s), 0, "served again");
}
