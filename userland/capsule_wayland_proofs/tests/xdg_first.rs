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

//! A toplevel's configures, as xdg-shell orders them: none before the
//! client's initial commit, the first one answering it (with a mode asked
//! before then), and the same state told again when the window gains or
//! loses the keyboard, with wl_keyboard's enter, modifiers and leave.

use nonos_capsule_wayland_proofs::client::*;
use nonos_app_skeleton::desk;

const TOPLEVEL_CONFIGURE: u16 = 0;
const XDG_SURFACE_CONFIGURE: u16 = 0;
const FULLSCREEN: u32 = 2;
const MAXIMIZED: u32 = 1;
const ACTIVATED: u32 = 4;

/// The first configure: the toplevel's, then its xdg_surface's serial.
fn first(c: &mut Client) -> (u32, u32, Vec<u32>, u32) {
    c.req(SURFACE, 6, &[]);
    let evs = c.events();
    let top = to(&evs, TOPLEVEL);
    let xdg = to(&evs, XDG_SURFACE);
    assert_eq!(top.len(), 1, "one toplevel configure: {evs:?}");
    assert_eq!((top[0].opcode, xdg.len()), (TOPLEVEL_CONFIGURE, 1));
    assert_eq!(xdg[0].opcode, XDG_SURFACE_CONFIGURE);
    let at_top = evs.iter().position(|e| e.object == TOPLEVEL);
    let at_xdg = evs.iter().position(|e| e.object == XDG_SURFACE);
    assert!(at_top < at_xdg, "the xdg_surface configure ends the sequence");
    (top[0].word(0), top[0].word(1), top[0].array(2), xdg[0].word(0))
}

#[test]
fn nothing_is_configured_before_the_initial_commit() {
    let mut c = toplevel(2);
    c.req(TOPLEVEL, 2, &[A::S("a title")]);
    assert!(c.events().is_empty());
    let (w, h, states, _) = first(&mut c);
    // A window of its own is offered the size a guest window opens at.
    assert_eq!((w, h, states), (800, 600, vec![]));
    // A second commit is not answered again.
    c.req(SURFACE, 6, &[]);
    assert!(to(&c.events(), TOPLEVEL).is_empty());
}

#[test]
fn full_screen_asked_before_mapping_is_told_in_the_first_configure() {
    let mut c = toplevel(2);
    c.req(TOPLEVEL, 11, &[A::U(0)]);
    assert!(to(&c.events(), TOPLEVEL).is_empty(), "told only at the initial commit");
    let (w, h, states, _) = first(&mut c);
    assert_eq!(states, vec![FULLSCREEN]);
    assert!(w > 0 && h > 0, "the display's size: {w}x{h}");
}

#[test]
fn maximised_asked_and_taken_back_before_mapping_is_not_told() {
    let mut c = toplevel(2);
    c.req(TOPLEVEL, 9, &[]);
    c.req(TOPLEVEL, 10, &[]);
    let (w, h, states, _) = first(&mut c);
    assert_eq!((w, h, states), (800, 600, vec![]));
    assert_ne!(MAXIMIZED, 0);
}

#[test]
fn a_dialog_is_told_to_take_its_own_size() {
    let mut c = toplevel(6);
    // A second toplevel, whose parent is the first.
    c.req(COMPOSITOR, 0, &[A::U(50)]);
    c.req(WM_BASE, 2, &[A::U(51), A::U(50)]);
    c.req(51, 1, &[A::U(52)]);
    c.req(52, 1, &[A::U(TOPLEVEL)]);
    c.req(50, 6, &[]);
    let evs = c.events();
    let configure = to(&evs, 52).into_iter().find(|e| e.opcode == 0).expect("configured");
    assert_eq!((configure.word(0), configure.word(1)), (0, 0));
}

#[test]
fn the_size_offered_keeps_to_the_windows_limits() {
    let mut c = toplevel(2);
    c.req(TOPLEVEL, 8, &[A::I(1000), A::I(0)]);
    c.req(TOPLEVEL, 7, &[A::I(0), A::I(400)]);
    let (w, h, _, _) = first(&mut c);
    assert_eq!((w, h), (1000, 400), "at least 1000 wide, at most 400 high");
}

#[test]
fn a_decoration_made_before_mapping_waits_for_the_first_configure() {
    let mut c = toplevel(2);
    const DECO_MANAGER: u32 = 30;
    const DECO: u32 = 31;
    c.bind(40, &[("zxdg_decoration_manager_v1", 1, DECO_MANAGER)]);
    c.events();
    c.req(DECO_MANAGER, 1, &[A::U(DECO), A::U(TOPLEVEL)]);
    let evs = c.events();
    assert_eq!(to(&evs, DECO).len(), 1, "its mode is said");
    assert!(to(&evs, XDG_SURFACE).is_empty(), "and no sequence ends before the commit");
    first(&mut c);
}

/// The window shown: the first configure acked and a buffer committed.
fn shown() -> Client {
    let mut c = toplevel(2);
    let (_, _, _, serial) = first(&mut c);
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    c.buffer(SHM, POOL, BUFFER, (320, 200));
    c.req(SURFACE, 1, &[A::U(BUFFER), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c
}

#[test]
fn the_window_manager_focusing_the_window_enters_the_keyboard_and_activates_it() {
    let mut c = shown();
    // Opened, the window is focused (present/adopt.rs), and the window
    // manager says so to its owner, which the next read takes in.
    let evs = c.events();
    let kb = to(&evs, KEYBOARD);
    assert_eq!(kb.iter().map(|e| e.opcode).collect::<Vec<_>>(), vec![1, 4], "enter, modifiers");
    assert_eq!(kb[0].word(1), SURFACE);
    assert_eq!(&kb[1].body[4..], &[0u8; 16], "nothing held");
    let top = to(&evs, TOPLEVEL);
    assert_eq!(top.last().expect("configured again").array(2), vec![ACTIVATED]);
    assert!(c.s.guest.scene.win.focused);

    // Another window chosen: left, and no longer activated.
    let window = c.s.guest.scene.win.window.expect("the window is open");
    desk::keys_notice(window, false);
    let evs = c.events();
    assert_eq!(to(&evs, KEYBOARD).iter().map(|e| e.opcode).collect::<Vec<_>>(), vec![2]);
    let top = to(&evs, TOPLEVEL);
    assert_eq!(top.len(), 1);
    // Its size now, said again, and no states.
    assert_eq!((top[0].word(0), top[0].word(1), top[0].array(2)), (320, 200, vec![]));
    assert!(!c.s.guest.scene.win.focused);
}

#[test]
fn a_notice_not_from_the_window_manager_changes_nothing() {
    let mut c = shown();
    c.events();
    let window = c.s.guest.scene.win.window.expect("the window is open");
    desk::keys_notice(window, false);
    // The same frame, sent by the router's pid instead.
    let forged = nonos_libc::world::with(|w| w.inbox.pop_back()).expect("queued").1;
    nonos_libc::world::post(desk::INPUT_ROUTER + 100, forged);
    assert!(to(&c.events(), KEYBOARD).is_empty());
    assert!(c.s.guest.scene.win.focused);
}

const RELEASE: u16 = 0;

#[test]
fn a_buffer_is_released_once_by_the_commit_that_applies_it() {
    let mut c = shown();
    let released = |evs: &[Ev]| to(evs, BUFFER).iter().filter(|e| e.opcode == RELEASE).count();
    assert_eq!(released(&c.events()), 1, "the commit that showed it");
    // A commit with no new attach, as one that sets a region is.
    c.req(SURFACE, 6, &[]);
    assert_eq!(released(&c.events()), 0, "no second release");
    c.req(SURFACE, 1, &[A::U(BUFFER), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    assert_eq!(released(&c.events()), 1);
}

#[test]
fn a_null_buffer_unmaps_the_window_and_the_next_commit_is_initial_again() {
    let mut c = shown();
    c.events();
    let window = c.s.guest.scene.win.window.expect("open");
    c.req(SURFACE, 1, &[A::U(0), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    assert!(desk::with(|d| d.calls.contains(&nonos_app_skeleton::desk::Call::Close(window))));
    assert!(c.s.guest.scene.win.shown.is_none());
    // Mapped again: an initial commit, configured, then a buffer.
    let (w, h, _, serial) = first(&mut c);
    assert_eq!((w, h), (800, 600));
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    c.req(SURFACE, 1, &[A::U(BUFFER), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    assert_eq!(c.s.guest.scene.win.shown.map(|w| w.surface), Some(SURFACE), "shown again");
}
