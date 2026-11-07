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

//! Drag and drop within a guest, as wl_data_device 3 has it: a drag only
//! from a held press, the pointer taken, each surface entered with an offer
//! of the source's types and actions, the action chosen from both sides,
//! the drop or the cancel, the data read as a selection's is, and finish.

use nonos_capsule_wayland_proofs::client::*;
use nonos_capsule_wayland_proofs::input::{self, Kind};

const POINTER: u32 = 14;
const MANAGER: u32 = 30;
const DEVICE: u32 = 31;
const SOURCE: u32 = 32;
const OFFER_FIRST: u32 = 0xff00_0000;

// wl_data_device events.
const DATA_OFFER: u16 = 0;
const ENTER: u16 = 1;
const LEAVE: u16 = 2;
const MOTION: u16 = 3;
const DROP: u16 = 4;
// wl_data_source events.
const TARGET: u16 = 0;
const SEND: u16 = 1;
const CANCELLED: u16 = 2;
const DROP_PERFORMED: u16 = 3;
const FINISHED: u16 = 4;
const ACTION: u16 = 5;

/// A shown window with a pointer on its middle, a data device, and a source
/// of text offering copy and move. The client and the window's middle.
fn ready() -> (Client, (i32, i32)) {
    let mut c = Client::new();
    let globals = [
        ("wl_compositor", 6, COMPOSITOR),
        ("xdg_wm_base", 6, WM_BASE),
        ("wl_seat", 9, SEAT),
        ("wl_shm", 1, SHM),
        ("wl_data_device_manager", 3, MANAGER),
    ];
    c.bind(REGISTRY, &globals);
    c.req(SEAT, 0, &[A::U(POINTER)]);
    c.req(MANAGER, 1, &[A::U(DEVICE), A::U(SEAT)]);
    c.req(MANAGER, 0, &[A::U(SOURCE)]);
    c.req(SOURCE, 0, &[A::S("text/plain")]);
    c.req(SOURCE, 2, &[A::U(3)]);
    c.req(COMPOSITOR, 0, &[A::U(SURFACE)]);
    c.req(WM_BASE, 2, &[A::U(XDG_SURFACE), A::U(SURFACE)]);
    c.req(XDG_SURFACE, 1, &[A::U(TOPLEVEL)]);
    c.req(SURFACE, 6, &[]);
    let serial = to(&c.events(), XDG_SURFACE)[0].word(0);
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    c.buffer(SHM, POOL, BUFFER, (400, 300));
    c.req(SURFACE, 1, &[A::U(BUFFER), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    let (w, h, _) = c.s.guest.scene.win.shape.expect("shaped");
    let mid = ((w / 2) as i32, (h / 2) as i32);
    input::motion(mid.0, mid.1);
    c.events();
    (c, mid)
}

/// The left button held: the serial of its press.
fn press(c: &mut Client, at: (i32, i32)) -> u32 {
    input::post(Kind::ButtonDown, 0, 1, at, (0, 0));
    let evs = c.events();
    to(&evs, POINTER).into_iter().find(|e| e.opcode == 3).expect("pressed").word(0)
}

fn ops(evs: &[Ev], object: u32) -> Vec<u16> {
    to(evs, object).iter().map(|e| e.opcode).collect()
}

/// A drag started and moved twice over the window: the offer it entered with.
fn dragging() -> (Client, u32) {
    let (mut c, (x, y)) = ready();
    let serial = press(&mut c, (x, y));
    c.req(DEVICE, 0, &[A::U(SOURCE), A::U(SURFACE), A::U(0), A::U(serial)]);
    let evs = c.events();
    assert_eq!(ops(&evs, POINTER), vec![1, 5], "the pointer leaves, taken by the drag");
    input::motion(x + 5, y + 5);
    let evs = c.events();
    assert_eq!(ops(&evs, DEVICE), vec![DATA_OFFER, ENTER]);
    let offer = to(&evs, DEVICE)[0].word(0);
    assert_eq!(offer, OFFER_FIRST);
    // Its type, then the source's actions (offer version 3).
    assert_eq!(ops(&evs, offer), vec![0, 1]);
    assert_eq!(to(&evs, offer)[1].word(0), 3);
    let enter = to(&evs, DEVICE)[1];
    assert_eq!((enter.word(1), enter.word(4)), (SURFACE, offer));
    assert!(ops(&evs, POINTER).is_empty(), "no pointer events while dragging");
    input::motion(x + 9, y + 5);
    assert_eq!(ops(&c.events(), DEVICE), vec![MOTION]);
    (c, offer)
}

#[test]
fn a_drag_without_a_held_press_is_cancelled() {
    let (mut c, _) = ready();
    c.req(DEVICE, 0, &[A::U(SOURCE), A::U(SURFACE), A::U(0), A::U(12345)]);
    assert_eq!(ops(&c.events(), SOURCE), vec![CANCELLED]);
    assert!(!c.s.guest.scene.dnd.dragging());
}

#[test]
fn accepted_and_agreed_it_drops_is_read_and_finishes() {
    let (mut c, offer) = dragging();
    // Copy or move taken, move preferred: move, told to both.
    c.req(offer, 4, &[A::U(3), A::U(2)]);
    let evs = c.events();
    assert_eq!((ops(&evs, offer), to(&evs, offer)[0].word(0)), (vec![2], 2));
    assert_eq!((ops(&evs, SOURCE), to(&evs, SOURCE)[0].word(0)), (vec![ACTION], 2));
    c.req(offer, 0, &[A::U(1), A::S("text/plain")]);
    let evs = c.events();
    assert_eq!(ops(&evs, SOURCE), vec![TARGET]);
    let (x, y) = c.s.guest.scene.win.frame.pointer;
    input::post(Kind::ButtonUp, 0, 1, (x, y), (0, 0));
    let evs = c.events();
    assert_eq!(ops(&evs, DEVICE), vec![DROP]);
    assert_eq!(ops(&evs, SOURCE), vec![DROP_PERFORMED]);
    // The destination reads it: the source is asked to send, with the pipe.
    let mut ends = [0i32; 2];
    // SAFETY: a pipe for the receive, its write end passed with it.
    assert_eq!(unsafe { libc::pipe(ends.as_mut_ptr()) }, 0);
    c.req_fds(offer, 1, &[A::S("text/plain")], &[ends[1]]);
    let (bytes, fds) = c.s.recv(usize::MAX);
    assert_eq!(fds.len(), 1, "the source is given the descriptor");
    // SAFETY: descriptors this test holds, closed once.
    unsafe {
        libc::close(ends[0]);
        for &fd in &fds {
            libc::close(fd);
        }
    }
    assert_eq!(u16::from_le_bytes([bytes[4], bytes[5]]), SEND);
    c.req(offer, 3, &[]);
    assert_eq!(ops(&c.events(), SOURCE), vec![FINISHED]);
    assert!(c.s.guest.scene.dnd.drag.is_none());
}

#[test]
fn let_go_where_nothing_was_accepted_it_is_cancelled() {
    let (mut c, _) = dragging();
    let (x, y) = c.s.guest.scene.win.frame.pointer;
    input::post(Kind::ButtonUp, 0, 1, (x, y), (0, 0));
    let evs = c.events();
    assert_eq!(ops(&evs, DEVICE), vec![LEAVE]);
    assert_eq!(ops(&evs, SOURCE), vec![CANCELLED]);
    // The pointer is free: its next motion enters the window again.
    input::motion(x + 1, y);
    assert_eq!(ops(&c.events(), POINTER), vec![0, 2, 5]);
}

#[test]
fn over_the_frame_the_drag_leaves_and_the_source_hears_no_target() {
    let (mut c, offer) = dragging();
    c.req(offer, 0, &[A::U(1), A::S("text/plain")]);
    c.events();
    input::motion(2, 2);
    let evs = c.events();
    assert_eq!(ops(&evs, DEVICE), vec![LEAVE]);
    let target = to(&evs, SOURCE);
    assert_eq!((target[0].opcode, target[0].word(0)), (TARGET, 0), "a null type");
}

#[test]
fn the_source_destroyed_ends_the_drag() {
    let (mut c, _) = dragging();
    c.req(SOURCE, 1, &[]);
    let evs = c.events();
    assert_eq!(ops(&evs, DEVICE), vec![LEAVE]);
    assert!(c.s.guest.scene.dnd.drag.is_none());
}

/// A drop the client never finished gives way to the next drag, which
/// starts; the unfinished one's source is told it was cancelled.
#[test]
fn a_drop_never_finished_does_not_hold_off_the_next_drag() {
    let (mut c, offer) = dragging();
    c.req(offer, 4, &[A::U(3), A::U(2)]);
    c.req(offer, 0, &[A::U(1), A::S("text/plain")]);
    c.events();
    let (x, y) = c.s.guest.scene.win.frame.pointer;
    input::post(Kind::ButtonUp, 0, 1, (x, y), (0, 0));
    assert_eq!(ops(&c.events(), SOURCE), vec![DROP_PERFORMED]);
    // No finish. A new source, a new press, a new drag.
    const SECOND: u32 = 33;
    c.req(MANAGER, 0, &[A::U(SECOND)]);
    c.req(SECOND, 0, &[A::S("text/plain")]);
    input::motion(x, y);
    c.events();
    let serial = press(&mut c, (x, y));
    c.req(DEVICE, 0, &[A::U(SECOND), A::U(SURFACE), A::U(0), A::U(serial)]);
    let evs = c.events();
    assert_eq!(ops(&evs, SOURCE).last(), Some(&CANCELLED), "the unfinished one ends");
    assert!(ops(&evs, SECOND).is_empty(), "the new one is not cancelled");
    assert!(c.s.guest.scene.dnd.dragging());
}
