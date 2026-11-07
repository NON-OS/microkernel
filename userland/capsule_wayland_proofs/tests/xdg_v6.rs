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

//! What xdg_wm_base 3 to 6 adds, as this server serves it: a toplevel's
//! wm_capabilities and configure_bounds ahead of its configure, said only to
//! one made at a version that reads them; set_minimized taken to the window
//! manager; a popup repositioned, told so with the client's token.

use nonos_app_skeleton::desk::{self, Call};
use nonos_capsule_wayland_proofs::client::*;

const BOUNDS: u16 = 2;
const CAPABILITIES: u16 = 3;

#[test]
fn a_version_6_toplevel_hears_its_capabilities_and_bounds_before_its_configure() {
    let mut c = toplevel(6);
    c.req(SURFACE, 6, &[]);
    let evs = c.events();
    let ops: Vec<u16> = to(&evs, TOPLEVEL).iter().map(|e| e.opcode).collect();
    assert_eq!(ops, vec![CAPABILITIES, BOUNDS, 0], "capabilities, bounds, configure");
    let top = to(&evs, TOPLEVEL);
    // Maximise, full screen, minimise; no window menu.
    assert_eq!(top[0].array(0), vec![2, 3, 4]);
    let (w, h) = (top[1].word(0), top[1].word(1));
    let (dw, dh) = desk::with(|d| (d.width, d.height));
    assert!(w > 0 && h > 0 && w <= dw && h < dh, "inside the display, below the bar: {w}x{h}");
    // Every later configure says the bounds again; capabilities only once.
    let window = shown_after(&mut c, &evs);
    desk::keys_notice(window, false);
    let ops: Vec<u16> = to(&c.events(), TOPLEVEL).iter().map(|e| e.opcode).collect();
    assert_eq!(ops, vec![BOUNDS, 0]);
}

#[test]
fn an_older_toplevel_is_sent_nothing_it_cannot_read() {
    let mut c = toplevel(3);
    c.req(SURFACE, 6, &[]);
    let ops: Vec<u16> = to(&c.events(), TOPLEVEL).iter().map(|e| e.opcode).collect();
    assert_eq!(ops, vec![0]);
}

/// Ack the first configure in `evs`, commit a buffer: the window's id.
fn shown_after(c: &mut Client, evs: &[Ev]) -> u32 {
    let serial = to(evs, XDG_SURFACE)[0].word(0);
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    c.buffer(SHM, POOL, BUFFER, (400, 300));
    c.req(SURFACE, 1, &[A::U(BUFFER), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    c.s.guest.scene.win.window.expect("the window is open")
}

#[test]
fn set_minimized_minimises_the_window() {
    let mut c = toplevel(6);
    c.req(SURFACE, 6, &[]);
    let evs = c.events();
    let window = shown_after(&mut c, &evs);
    c.req(TOPLEVEL, 13, &[]);
    assert!(desk::with(|d| d.calls.contains(&Call::Minimize(window))));
}

const POINTER: u32 = 14;
const POSITIONER: u32 = 40;
const POSITIONER2: u32 = 41;
const POPUP_SURFACE: u32 = 42;
const POPUP_XDG: u32 = 43;
const POPUP: u32 = 44;

/// A positioner of a 100 by 50 popup below the 1 by 1 rect at (x, y).
fn positioner(c: &mut Client, id: u32, (x, y): (i32, i32)) {
    c.req(WM_BASE, 1, &[A::U(id)]);
    c.req(id, 1, &[A::I(100), A::I(50)]);
    c.req(id, 2, &[A::I(x), A::I(y), A::I(1), A::I(1)]);
    // Anchored at the rect's bottom right, growing down and right.
    c.req(id, 3, &[A::U(8)]);
    c.req(id, 4, &[A::U(8)]);
}

#[test]
fn a_repositioned_popup_is_told_its_token_then_its_new_place() {
    let mut c = toplevel(6);
    c.req(SEAT, 0, &[A::U(POINTER)]);
    c.req(SURFACE, 6, &[]);
    let evs = c.events();
    shown_after(&mut c, &evs);
    // A press on the window, whose serial the grab names.
    // The router gives a point on the window, its frame included.
    let (w, h, _) = c.s.guest.scene.win.shape.expect("shaped");
    nonos_capsule_wayland_proofs::input::click(1, ((w / 2) as i32, (h / 2) as i32));
    let evs = c.events();
    let press = evs.into_iter().find(|e| e.object == POINTER && e.opcode == 3 && e.word(3) == 1);
    let serial = press.expect("the press reached the client").word(0);

    positioner(&mut c, POSITIONER, (20, 30));
    c.req(COMPOSITOR, 0, &[A::U(POPUP_SURFACE)]);
    c.req(WM_BASE, 2, &[A::U(POPUP_XDG), A::U(POPUP_SURFACE)]);
    c.req(POPUP_XDG, 2, &[A::U(POPUP), A::U(XDG_SURFACE), A::U(POSITIONER)]);
    c.req(POPUP, 1, &[A::U(SEAT), A::U(serial)]);
    let evs = c.events();
    let placed = to(&evs, POPUP);
    assert_eq!(placed.len(), 1, "{evs:?}");
    assert_eq!((0..4).map(|i| placed[0].word(i)).collect::<Vec<_>>(), vec![21, 31, 100, 50]);

    positioner(&mut c, POSITIONER2, (60, 70));
    c.req(POPUP, 2, &[A::U(POSITIONER2), A::U(77)]);
    let evs = c.events();
    let ev = to(&evs, POPUP);
    assert_eq!(
        ev.iter().map(|e| e.opcode).collect::<Vec<_>>(),
        vec![2, 0],
        "repositioned, configure"
    );
    assert_eq!(ev[0].word(0), 77);
    assert_eq!((0..4).map(|i| ev[1].word(i)).collect::<Vec<_>>(), vec![61, 71, 100, 50]);
    assert_eq!(to(&evs, POPUP_XDG).len(), 1, "the sequence ends with the xdg_surface's configure");
}

#[test]
fn a_popup_with_no_grab_is_configured_at_its_first_commit_and_reached_by_the_pointer() {
    let mut c = toplevel(6);
    c.req(SEAT, 0, &[A::U(POINTER)]);
    c.req(SURFACE, 6, &[]);
    let evs = c.events();
    shown_after(&mut c, &evs);
    positioner(&mut c, POSITIONER, (20, 30));
    c.req(COMPOSITOR, 0, &[A::U(POPUP_SURFACE)]);
    c.req(WM_BASE, 2, &[A::U(POPUP_XDG), A::U(POPUP_SURFACE)]);
    c.req(POPUP_XDG, 2, &[A::U(POPUP), A::U(XDG_SURFACE), A::U(POSITIONER)]);
    assert!(to(&c.events(), POPUP).is_empty(), "nothing before its commit");
    c.req(POPUP_SURFACE, 6, &[]);
    let evs = c.events();
    let placed = to(&evs, POPUP);
    assert_eq!(
        placed.iter().map(|e| e.opcode).collect::<Vec<_>>(),
        vec![0],
        "configured, not ended"
    );
    assert_eq!((0..4).map(|i| placed[0].word(i)).collect::<Vec<_>>(), vec![21, 31, 100, 50]);
    let serial = to(&evs, POPUP_XDG)[0].word(0);
    c.req(POPUP_XDG, 4, &[A::U(serial)]);
    c.painted(SHM, 50, 51, (100, 50), 0xFFFF_0000);
    c.req(POPUP_SURFACE, 1, &[A::U(51), A::I(0), A::I(0)]);
    c.req(POPUP_SURFACE, 2, &[A::I(0), A::I(0), A::I(100), A::I(50)]);
    c.req(POPUP_SURFACE, 6, &[]);
    c.events();
    // In a window of its own, all of it on its own surface and layer; the
    // window under it untouched.
    let popup = &c.s.guest.scene.popup_windows[0];
    let handle = popup.out.expect("its own surface");
    let (_, _, px) =
        nonos_capsule_wayland_proofs::host::dump::surface_pixels(handle).expect("shown");
    assert_eq!(px.iter().filter(|&&p| p & 0xFF_FFFF == 0xFF_0000).count(), 100 * 50);
    let layers = nonos_app_skeleton::desk::with(|d| d.layers.len());
    assert_eq!(layers, 2, "the window's and the popup's");
    let (_, _, under) =
        nonos_capsule_wayland_proofs::host::dump::pixels(&c.s.guest).expect("shown");
    assert_eq!(under.iter().filter(|&&p| p & 0xFF_FFFF == 0xFF_0000).count(), 0);
    // The keyboard stays on the window; the pointer over it enters it.
    let keyboard_on = c.s.guest.scene.keyboard_on;
    let on_window = c.s.guest.scene.popups.under(40, 50);
    assert_eq!(on_window.map(|u| u.0), Some(POPUP_SURFACE));
    assert_eq!(c.s.guest.scene.keyboard_on, keyboard_on);
    // Destroyed by its client: gone, with no popup_done.
    c.req(POPUP, 0, &[]);
    assert!(to(&c.events(), POPUP).is_empty());
    assert!(c.s.guest.scene.popups.list.is_empty());
}

/// A shown window with a pointer at (x, y) on it, its left button held: the
/// press's serial.
fn pressed_at(c: &mut Client, (x, y): (i32, i32)) -> u32 {
    use nonos_capsule_wayland_proofs::input::{post, Kind};
    nonos_capsule_wayland_proofs::input::motion(x, y);
    c.events();
    post(Kind::ButtonDown, 0, 1, (x, y), (0, 0));
    let evs = c.events();
    to(&evs, POINTER).into_iter().find(|e| e.opcode == 3).expect("pressed").word(0)
}

fn moved(window: u32) -> Vec<(u32, u32)> {
    desk::with(|d| {
        d.calls
            .iter()
            .filter_map(|c| match c {
                Call::Move(w, x, y) if *w == window => Some((*x, *y)),
                _ => None,
            })
            .collect()
    })
}

#[test]
fn a_move_asked_from_a_held_press_drags_the_window() {
    let mut c = toplevel(6);
    c.req(SEAT, 0, &[A::U(POINTER)]);
    c.req(SURFACE, 6, &[]);
    let evs = c.events();
    let window = shown_after(&mut c, &evs);
    let (w, h, _) = c.s.guest.scene.win.shape.expect("shaped");
    let (x, y) = ((w / 2) as i32, (h / 2) as i32);
    let serial = pressed_at(&mut c, (x, y));
    let at = c.s.guest.scene.win.at.expect("placed");
    c.req(TOPLEVEL, 5, &[A::U(SEAT), A::U(serial)]);
    let evs = c.events();
    assert_eq!(to(&evs, POINTER).iter().map(|e| e.opcode).collect::<Vec<_>>(), vec![1, 5], "left");
    nonos_capsule_wayland_proofs::input::motion(x + 40, y + 30);
    c.events();
    assert_eq!(moved(window).last(), Some(&(at.0 + 40, at.1 + 30)));
    // The router keeps a held press's motion in the frame it was armed in:
    // further on, the window is where the pointer went, not that plus the
    // move it has made already.
    nonos_capsule_wayland_proofs::input::motion(x + 60, y + 50);
    c.events();
    assert_eq!(moved(window).last(), Some(&(at.0 + 60, at.1 + 50)), "not run ahead");
    // Let go: the move ends, and the next motion is the client's again.
    nonos_capsule_wayland_proofs::input::post(
        nonos_capsule_wayland_proofs::input::Kind::ButtonUp,
        0,
        1,
        (x, y),
        (0, 0),
    );
    c.events();
    nonos_capsule_wayland_proofs::input::motion(x, y);
    assert_eq!(to(&c.events(), POINTER).first().map(|e| e.opcode), Some(0), "entered again");
}

/// The window manager's word that the window lost the keyboard can reach
/// the client after the press that a title-bar drag starts from (the notice
/// is offered again while the inbox is full, wm focus/owed_keys.rs). The
/// popups end with the focus, but the press still held is the one the move
/// names, and the window is dragged.
#[test]
fn a_move_from_a_press_still_held_survives_the_keyboard_leaving() {
    let mut c = toplevel(6);
    c.req(SEAT, 0, &[A::U(POINTER)]);
    c.req(SURFACE, 6, &[]);
    let evs = c.events();
    let window = shown_after(&mut c, &evs);
    desk::keys_notice(window, true);
    let (w, h, _) = c.s.guest.scene.win.shape.expect("shaped");
    let (x, y) = ((w / 2) as i32, (h / 2) as i32);
    let serial = pressed_at(&mut c, (x, y));
    assert!(c.s.guest.scene.win.focused, "focused before the notice");
    desk::keys_notice(window, false);
    nonos_capsule_wayland_proofs::input::motion(x, y);
    c.events();
    assert!(!c.s.guest.scene.win.focused, "the keyboard left");
    let at = c.s.guest.scene.win.at.expect("placed");
    c.req(TOPLEVEL, 5, &[A::U(SEAT), A::U(serial)]);
    c.events();
    nonos_capsule_wayland_proofs::input::motion(x + 40, y + 30);
    c.events();
    assert_eq!(moved(window).last(), Some(&(at.0 + 40, at.1 + 30)), "dragged");
}

/// The press kept through the keyboard leaving names nothing once it is let
/// go while the guest has no keyboard: a menu grabbing with it would open
/// over the window that has the keyboard now.
#[test]
fn a_press_kept_through_the_keyboard_leaving_grabs_nothing_once_let_go() {
    use nonos_capsule_wayland_proofs::input::{post, Kind};
    let mut c = toplevel(6);
    c.req(SEAT, 0, &[A::U(POINTER)]);
    c.req(SURFACE, 6, &[]);
    let evs = c.events();
    let window = shown_after(&mut c, &evs);
    desk::keys_notice(window, true);
    let (w, h, _) = c.s.guest.scene.win.shape.expect("shaped");
    let (x, y) = ((w / 2) as i32, (h / 2) as i32);
    let serial = pressed_at(&mut c, (x, y));
    desk::keys_notice(window, false);
    nonos_capsule_wayland_proofs::input::motion(x, y);
    post(Kind::ButtonUp, 0, 1, (x, y), (0, 0));
    c.events();
    positioner(&mut c, POSITIONER, (20, 30));
    c.req(COMPOSITOR, 0, &[A::U(POPUP_SURFACE)]);
    c.req(WM_BASE, 2, &[A::U(POPUP_XDG), A::U(POPUP_SURFACE)]);
    c.req(POPUP_XDG, 2, &[A::U(POPUP), A::U(XDG_SURFACE), A::U(POSITIONER)]);
    c.req(POPUP, 1, &[A::U(SEAT), A::U(serial)]);
    let evs = c.events();
    assert_eq!(to(&evs, POPUP).iter().map(|e| e.opcode).collect::<Vec<_>>(), vec![1], "done");
    assert!(c.s.guest.scene.popups.list.is_empty());
}

#[test]
fn a_move_with_a_stale_serial_moves_nothing() {
    let mut c = toplevel(6);
    c.req(SEAT, 0, &[A::U(POINTER)]);
    c.req(SURFACE, 6, &[]);
    let evs = c.events();
    let window = shown_after(&mut c, &evs);
    let (w, h, _) = c.s.guest.scene.win.shape.expect("shaped");
    let serial = pressed_at(&mut c, ((w / 2) as i32, (h / 2) as i32));
    c.req(TOPLEVEL, 5, &[A::U(SEAT), A::U(serial + 100)]);
    nonos_capsule_wayland_proofs::input::motion((w / 2) as i32 + 40, (h / 2) as i32);
    c.events();
    assert!(moved(window).is_empty());
    assert!(c.s.guest.scene.win.frame.drag.is_none());
}

#[test]
fn a_grabbing_popup_takes_the_keyboard_at_once_and_gives_it_back() {
    let mut c = toplevel(6);
    c.req(SEAT, 0, &[A::U(POINTER)]);
    c.req(SURFACE, 6, &[]);
    let evs = c.events();
    shown_after(&mut c, &evs);
    let (w, h, _) = c.s.guest.scene.win.shape.expect("shaped");
    let serial = pressed_at(&mut c, ((w / 2) as i32, (h / 2) as i32));
    positioner(&mut c, POSITIONER, (20, 30));
    c.req(COMPOSITOR, 0, &[A::U(POPUP_SURFACE)]);
    c.req(WM_BASE, 2, &[A::U(POPUP_XDG), A::U(POPUP_SURFACE)]);
    c.req(POPUP_XDG, 2, &[A::U(POPUP), A::U(XDG_SURFACE), A::U(POSITIONER)]);
    c.req(POPUP, 1, &[A::U(SEAT), A::U(serial)]);
    let evs = c.events();
    let kb: Vec<(u16, u32)> = to(&evs, KEYBOARD).iter().map(|e| (e.opcode, e.word(1))).collect();
    // leave the window, enter the popup, then the modifiers.
    assert_eq!(kb[..2], [(2, SURFACE), (1, POPUP_SURFACE)]);
    c.req(POPUP, 0, &[]);
    let evs = c.events();
    let kb: Vec<(u16, u32)> = to(&evs, KEYBOARD).iter().map(|e| (e.opcode, e.word(1))).collect();
    assert_eq!(kb.iter().find(|k| k.0 == 1), Some(&(1, SURFACE)), "back on the window");
}

#[test]
fn a_press_on_the_window_outside_the_menu_dismisses_it_and_reaches_the_window() {
    use nonos_capsule_wayland_proofs::input::{post, Kind};
    let mut c = toplevel(6);
    c.req(SEAT, 0, &[A::U(POINTER)]);
    c.req(SURFACE, 6, &[]);
    let evs = c.events();
    shown_after(&mut c, &evs);
    let (w, h, _) = c.s.guest.scene.win.shape.expect("shaped");
    let at = ((w / 2) as i32, (h / 2) as i32);
    let serial = pressed_at(&mut c, at);
    post(Kind::ButtonUp, 0, 1, at, (0, 0));
    positioner(&mut c, POSITIONER, (20, 30));
    c.req(COMPOSITOR, 0, &[A::U(POPUP_SURFACE)]);
    c.req(WM_BASE, 2, &[A::U(POPUP_XDG), A::U(POPUP_SURFACE)]);
    c.req(POPUP_XDG, 2, &[A::U(POPUP), A::U(XDG_SURFACE), A::U(POSITIONER)]);
    c.req(POPUP, 1, &[A::U(SEAT), A::U(serial)]);
    c.events();
    // Pressed and let go on the window, away from the menu: the menu ends
    // first, then the window has the press and its release, as a click on
    // another menubar item opens that menu with one click.
    post(Kind::ButtonDown, 0, 1, at, (0, 0));
    post(Kind::ButtonUp, 0, 1, at, (0, 0));
    let evs = c.events();
    assert_eq!(to(&evs, POPUP).iter().map(|e| e.opcode).collect::<Vec<_>>(), vec![1], "popup_done");
    let done = evs.iter().position(|e| e.object == POPUP).expect("popup_done");
    let buttons: Vec<(usize, u32)> = evs
        .iter()
        .enumerate()
        .filter(|(_, e)| e.object == POINTER && e.opcode == 3)
        .map(|(i, e)| (i, e.word(3)))
        .collect();
    assert_eq!(buttons.iter().map(|b| b.1).collect::<Vec<_>>(), vec![1, 0], "pressed, released");
    assert!(buttons[0].0 > done, "after the menu ended");
    assert!(c.s.guest.scene.popups.top().is_none());
}

#[test]
fn a_press_off_the_window_s_content_dismisses_the_menu_and_reaches_nothing() {
    use nonos_capsule_wayland_proofs::input::{post, Kind};
    let mut c = toplevel(6);
    c.req(SEAT, 0, &[A::U(POINTER)]);
    c.req(SURFACE, 6, &[]);
    let evs = c.events();
    shown_after(&mut c, &evs);
    let (w, h, _) = c.s.guest.scene.win.shape.expect("shaped");
    let at = ((w / 2) as i32, (h / 2) as i32);
    let serial = pressed_at(&mut c, at);
    post(Kind::ButtonUp, 0, 1, at, (0, 0));
    positioner(&mut c, POSITIONER, (20, 30));
    c.req(COMPOSITOR, 0, &[A::U(POPUP_SURFACE)]);
    c.req(WM_BASE, 2, &[A::U(POPUP_XDG), A::U(POPUP_SURFACE)]);
    c.req(POPUP_XDG, 2, &[A::U(POPUP), A::U(XDG_SURFACE), A::U(POSITIONER)]);
    c.req(POPUP, 1, &[A::U(SEAT), A::U(serial)]);
    c.events();
    // Off every surface of the client: the pointer left it.
    let off = (w as i32 + 4000, h as i32 + 4000);
    nonos_capsule_wayland_proofs::input::motion(off.0, off.1);
    c.events();
    assert_eq!(c.s.guest.scene.pointer_on, None);
    post(Kind::ButtonDown, 0, 1, off, (0, 0));
    post(Kind::ButtonUp, 0, 1, off, (0, 0));
    let evs = c.events();
    assert_eq!(to(&evs, POPUP).iter().map(|e| e.opcode).collect::<Vec<_>>(), vec![1], "popup_done");
    assert!(to(&evs, POINTER).iter().all(|e| e.opcode != 3), "no button reached the client");
    assert!(!c.s.guest.scene.popups.dismissed, "its release was taken too");
}
