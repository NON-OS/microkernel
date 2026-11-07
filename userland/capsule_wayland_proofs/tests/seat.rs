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

//! wl_seat and wl_pointer as a client bound at each version reads them: the
//! seat's name, and a wheel notch as axis_discrete (5 to 7), axis_value120
//! (8 on), with axis_relative_direction (9 on), always before its axis
//! event and inside one frame.

use nonos_capsule_wayland_proofs::client::*;
use nonos_capsule_wayland_proofs::input;

const POINTER: u32 = 14;

/// A shown window and a pointer, all bound with the seat at `version`.
fn pointer_at(version: u32) -> Client {
    let mut c = Client::new();
    c.bind(
        REGISTRY,
        &[
            ("wl_compositor", 6, COMPOSITOR),
            ("xdg_wm_base", 6, WM_BASE),
            ("wl_seat", version, SEAT),
            ("wl_shm", 1, SHM),
        ],
    );
    c.req(SEAT, 0, &[A::U(POINTER)]);
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
    c
}

/// One notch away from the person, over the window's middle: the pointer's
/// events after the motion that entered it.
fn notch(c: &mut Client) -> Vec<(u16, Vec<u32>)> {
    let (w, h, _) = c.s.guest.scene.win.shape.expect("shaped");
    let at = ((w / 2) as i32, (h / 2) as i32);
    input::motion(at.0, at.1);
    c.events();
    input::wheel(0, 1, at);
    let evs = c.events();
    let mine = to(&evs, POINTER);
    mine.iter().map(|e| (e.opcode, (0..e.body.len() / 4).map(|i| e.word(i)).collect())).collect()
}

const AXIS: u16 = 4;
const FRAME: u16 = 5;
const SOURCE: u16 = 6;
const DISCRETE: u16 = 8;
const VALUE120: u16 = 9;
const DIRECTION: u16 = 10;

#[test]
fn the_seat_says_its_name() {
    let mut c = Client::new();
    c.bind(REGISTRY, &[("wl_seat", 9, SEAT)]);
    let evs = c.events();
    let seat = to(&evs, SEAT);
    assert_eq!(seat.iter().map(|e| e.opcode).collect::<Vec<_>>(), vec![0, 1]);
    assert_eq!(&seat[1].body[4..9], b"seat0");
}

#[test]
fn a_version_5_pointer_hears_the_notch_as_axis_discrete() {
    let evs = notch(&mut pointer_at(5));
    let ops: Vec<u16> = evs.iter().map(|e| e.0).collect();
    assert_eq!(ops, vec![SOURCE, DISCRETE, AXIS, FRAME]);
    // Vertical, one notch up the page: -1, and 15 units in 24.8.
    assert_eq!(evs[1].1, vec![0, (-1i32) as u32]);
    assert_eq!(&evs[2].1[1..], &[0, (-15i32 * 256) as u32]);
}

#[test]
fn a_version_8_pointer_hears_it_as_axis_value120() {
    let evs = notch(&mut pointer_at(8));
    let ops: Vec<u16> = evs.iter().map(|e| e.0).collect();
    assert_eq!(ops, vec![SOURCE, VALUE120, AXIS, FRAME]);
    assert_eq!(evs[1].1, vec![0, (-120i32) as u32]);
}

#[test]
fn a_version_9_pointer_is_told_the_direction_too() {
    let evs = notch(&mut pointer_at(9));
    let ops: Vec<u16> = evs.iter().map(|e| e.0).collect();
    assert_eq!(ops, vec![SOURCE, VALUE120, DIRECTION, AXIS, FRAME]);
    assert_eq!(evs[2].1, vec![0, 0], "vertical, identical");
}

/// Two seats bound, each with its keyboard and pointer, as Firefox's GDK and
/// its widget layer make them: each is told every enter, key and motion.
#[test]
fn every_keyboard_and_pointer_the_client_holds_is_told() {
    let mut c = pointer_at(9);
    const SEAT2: u32 = 60;
    const KEYBOARD2: u32 = 61;
    const POINTER2: u32 = 62;
    c.bind(70, &[("wl_seat", 1, SEAT2)]);
    c.req(SEAT, 1, &[A::U(KEYBOARD)]);
    c.req(SEAT2, 1, &[A::U(KEYBOARD2)]);
    c.req(SEAT2, 0, &[A::U(POINTER2)]);
    // The window has the keyboard already: each is entered as it is made.
    let evs = c.events();
    // keymap, repeat_info (wl_keyboard 4 on), enter, modifiers.
    for (k, ops) in [(KEYBOARD, vec![0, 5, 1, 4]), (KEYBOARD2, vec![0, 1, 4])] {
        assert_eq!(to(&evs, k).iter().map(|e| e.opcode).collect::<Vec<_>>(), ops, "keyboard {k}");
    }
    let (w, h, _) = c.s.guest.scene.win.shape.expect("shaped");
    input::motion((w / 2) as i32, (h / 2) as i32);
    let evs = c.events();
    for p in [POINTER, POINTER2] {
        assert_eq!(to(&evs, p).first().map(|e| e.opcode), Some(0), "pointer {p} entered");
    }
    input::tap(b'a' as u32, 0);
    let evs = c.events();
    for k in [KEYBOARD, KEYBOARD2] {
        let ops: Vec<u16> = to(&evs, k).iter().map(|e| e.opcode).collect();
        assert_eq!(ops, vec![3, 3], "keyboard {k}: the key pressed and let go");
    }
    // One made now, while the window has the keyboard, is told so at once.
    const KEYBOARD3: u32 = 63;
    c.req(SEAT, 1, &[A::U(KEYBOARD3)]);
    let ops: Vec<u16> = to(&c.events(), KEYBOARD3).iter().map(|e| e.opcode).collect();
    assert_eq!(ops, vec![0, 5, 1, 4], "keymap, repeat_info, enter, modifiers");
}

/// A client that has left a mebibyte unread is sent no pointer motion, and
/// the inbox is drained still, so a window manager notice is never stranded.
#[test]
fn input_waits_while_the_client_reads_nothing() {
    use nonos_capsule_wayland_proofs::linux::wayland::pump;
    let mut c = toplevel(6);
    c.req(SEAT, 0, &[A::U(40)]);
    c.req(SURFACE, 6, &[]);
    let serial = to(&c.events(), XDG_SURFACE)[0].word(0);
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    c.buffer(SHM, POOL, BUFFER, (200, 200));
    c.req(SURFACE, 1, &[A::U(BUFFER), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    c.s.guest.display.to_client.resize(1 << 20, 0);
    let held = c.s.guest.display.to_client.len();
    for x in 0..50 {
        nonos_capsule_wayland_proofs::input::motion(50 + x, 60);
        pump(&mut c.s.guest);
    }
    assert_eq!(c.s.guest.display.to_client.len(), held, "no motion queued");
    // The window manager taking the keyboard away is read and acted on.
    let window = c.s.guest.scene.win.window.expect("open");
    nonos_app_skeleton::desk::keys_notice(window, false);
    pump(&mut c.s.guest);
    assert!(!c.s.guest.scene.win.focused, "the notice was read while backlogged");
    c.s.guest.display.to_client.clear();
    nonos_capsule_wayland_proofs::input::motion(70, 60);
    pump(&mut c.s.guest);
    assert!(!c.s.guest.display.to_client.is_empty(), "read, the motion comes");
}

/// A client is told not to repeat keys itself (repeat_info rate 0): it timed
/// a repeat from when a press reached it, so a release a slow turn later
/// typed a tapped key four times (foot, qemu-tcg: `eccccho`). A driver's
/// repeat of a held key is told as a release and a press; a held Shift is
/// not repeated at all.
#[test]
fn a_held_key_repeats_as_the_driver_does_and_the_client_does_not() {
    use nonos_capsule_wayland_proofs::input::{post, Kind};
    let mut c = pointer_at(9);
    c.req(SEAT, 1, &[A::U(KEYBOARD)]);
    let evs = c.events();
    let info = to(&evs, KEYBOARD).into_iter().find(|e| e.opcode == 5).expect("repeat_info");
    assert_eq!((info.word(0), info.word(1)), (0, 600), "rate 0: the client repeats nothing");
    let a = b'a' as u32;
    post(Kind::KeyDown, 0, a, (0, 0), (0, 0));
    let first = to(&c.events(), KEYBOARD).iter().filter(|e| e.opcode == 3).map(|e| e.word(3)).collect::<Vec<_>>();
    assert_eq!(first, vec![1], "pressed");
    // Two repeats from the driver, then the release.
    post(Kind::KeyDown, 0, a, (0, 0), (0, 0));
    post(Kind::KeyDown, 0, a, (0, 0), (0, 0));
    post(Kind::KeyUp, 0, a, (0, 0), (0, 0));
    let states: Vec<u32> =
        to(&c.events(), KEYBOARD).iter().filter(|e| e.opcode == 3).map(|e| e.word(3)).collect();
    assert_eq!(states, vec![0, 1, 0, 1, 0], "each repeat one more key typed, then let go");
    // Shift held: its repeats are dropped.
    let shift = 0x1003; // left shift, NONOS code (evdev_keys.rs)
    post(Kind::KeyDown, 0, shift, (0, 0), (0, 0));
    post(Kind::KeyDown, 0, shift, (0, 0), (0, 0));
    post(Kind::KeyUp, 0, shift, (0, 0), (0, 0));
    let states: Vec<u32> =
        to(&c.events(), KEYBOARD).iter().filter(|e| e.opcode == 3).map(|e| e.word(3)).collect();
    assert_eq!(states, vec![1, 0], "pressed once, let go once");
}
