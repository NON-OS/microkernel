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

//! The pointer's shape asked of the compositor, and the pointer leaving the
//! client's surface for the window's frame.

use nonos_capsule_wayland_proofs::client::*;
use nonos_app_skeleton::desk::{self, Call};
use nonos_capsule_wayland_proofs::input;

const POINTER: u32 = 14;
const CURSOR_MANAGER: u32 = 30;
const CURSOR_DEVICE: u32 = 31;
const CURSOR_SURFACE: u32 = 32;

/// A shown window, a pointer on its middle, and a cursor shape device:
/// the client and the serial of the pointer's enter.
fn entered() -> (Client, u32) {
    let mut c = Client::new();
    let globals = [
        ("wl_compositor", 6, COMPOSITOR),
        ("xdg_wm_base", 6, WM_BASE),
        ("wl_seat", 9, SEAT),
        ("wl_shm", 1, SHM),
        ("wp_cursor_shape_manager_v1", 1, CURSOR_MANAGER),
    ];
    c.bind(REGISTRY, &globals);
    c.req(SEAT, 0, &[A::U(POINTER)]);
    c.req(CURSOR_MANAGER, 1, &[A::U(CURSOR_DEVICE), A::U(POINTER)]);
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
    input::motion((w / 2) as i32, (h / 2) as i32);
    let evs = c.events();
    let enter = to(&evs, POINTER).into_iter().find(|e| e.opcode == 0).expect("entered");
    let serial = enter.word(0);
    (c, serial)
}

/// The shapes the compositor was asked for, in order.
fn shapes() -> Vec<u8> {
    desk::with(|d| {
        d.calls
            .iter()
            .filter_map(|c| match c {
                Call::Cursor(_, s) => Some(*s),
                _ => None,
            })
            .collect()
    })
}

#[test]
fn a_resize_shape_is_the_compositors_resize_arrow_and_text_is_the_arrow() {
    let (mut c, serial) = entered();
    c.req(CURSOR_DEVICE, 1, &[A::U(serial), A::U(26)]);
    c.req(CURSOR_DEVICE, 1, &[A::U(serial), A::U(29)]);
    c.req(CURSOR_DEVICE, 1, &[A::U(serial), A::U(9)]);
    // ew resize, nwse resize, then text, which it has no drawing of.
    assert_eq!(shapes(), vec![1, 3, 0]);
    let handle = c.s.guest.scene.win.out.expect("a surface");
    assert!(
        desk::with(|d| d.calls.contains(&Call::Cursor(handle, 1))),
        "over the window's surface"
    );
}

#[test]
fn a_shape_named_with_a_stale_serial_is_not_taken() {
    let (mut c, serial) = entered();
    c.req(CURSOR_DEVICE, 1, &[A::U(serial + 1000), A::U(26)]);
    assert!(shapes().is_empty());
}

#[test]
fn a_cursor_image_of_the_clients_own_puts_the_arrow_back() {
    let (mut c, serial) = entered();
    c.req(CURSOR_DEVICE, 1, &[A::U(serial), A::U(27)]);
    c.req(COMPOSITOR, 0, &[A::U(CURSOR_SURFACE)]);
    c.req(POINTER, 0, &[A::U(serial), A::U(CURSOR_SURFACE), A::I(0), A::I(0)]);
    assert_eq!(shapes(), vec![2, 0]);
}

#[test]
fn the_pointer_onto_the_frame_leaves_the_surface_and_comes_back_with_an_enter() {
    let (mut c, serial) = entered();
    c.req(CURSOR_DEVICE, 1, &[A::U(serial), A::U(26)]);
    // The window's top left corner is its frame, and a corner it resizes by.
    input::motion(2, 2);
    let evs = c.events();
    let ops: Vec<u16> = to(&evs, POINTER).iter().map(|e| e.opcode).collect();
    assert_eq!(ops, vec![1, 5], "leave, frame");
    assert_eq!(to(&evs, POINTER)[0].word(1), SURFACE);
    assert_eq!(shapes(), vec![1, 0, 3], "the arrow put back, then the corner's diagonal");
    assert_eq!(c.s.guest.scene.pointer_on, None);
    let (w, h, _) = c.s.guest.scene.win.shape.expect("shaped");
    input::motion((w / 2) as i32, (h / 2) as i32);
    let ops: Vec<u16> = to(&c.events(), POINTER).iter().map(|e| e.opcode).collect();
    assert_eq!(ops, vec![0, 2, 5], "enter, motion, frame");
}
