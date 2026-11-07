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

//! A client's subsurface made and left empty, as foot's scrollback
//! indicator and overlay are until they are needed, lays nothing on its
//! window: the window's commits are shown in part, as a window with no
//! subsurface's are. Once it shows pixels the window is drawn whole with
//! it laid on; taken down, the window is drawn whole once more, clearing
//! it, and in part again after.

use nonos_app_skeleton::desk;
use nonos_capsule_wayland_proofs::client::*;
use nonos_capsule_wayland_proofs::host::dump::pixels;
use nonos_capsule_wayland_proofs::linux::call::set_clock;
use nonos_capsule_wayland_proofs::linux::guest_reads;

const W: i32 = 320;
const H: i32 = 200;
const SUBCOMPOSITOR: u32 = 30;
const CHILD: u32 = 50;
const SUB: u32 = 60;
const GET_SUBSURFACE: u16 = 1;
const SET_DESYNC: u16 = 5;
const CHILD_BUFFER: u32 = 111;
const CHILD_PX: u32 = 0xFF00_C060;
const WHOLE: u64 = (W * H * 4) as u64;

/// A framed toplevel shown twice whole from buffer 101, copied, with
/// buffers 101 and 103 painted, and an empty desynchronized subsurface.
fn window() -> Client {
    nonos_libc::world::with(|w| w.allow_pins = false);
    set_clock(1_000);
    desk::with(|d| (d.width, d.height) = (W as u32 + 256, H as u32 + 256));
    let mut c = toplevel(6);
    c.bind(40, &[("wl_subcompositor", 1, SUBCOMPOSITOR)]);
    c.req(TOPLEVEL, 2, &[A::S("empty sub")]);
    c.req(SURFACE, 6, &[]);
    let serial = to(&c.events(), XDG_SURFACE).last().map(|e| e.word(0)).expect("configured");
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    for n in 0..2 {
        c.painted(SHM, 100 + 2 * n, 101 + 2 * n, (W, H), 0xFF20_40A0 + n);
    }
    c.painted(SHM, 110, CHILD_BUFFER, (16, 16), CHILD_PX);
    commit(&mut c, 101, (0, 0, W, H));
    commit(&mut c, 101, (0, 0, W, H));
    c.req(COMPOSITOR, 0, &[A::U(CHILD)]);
    c.req(SUBCOMPOSITOR, GET_SUBSURFACE, &[A::U(SUB), A::U(CHILD), A::U(SURFACE)]);
    c.req(SUB, SET_DESYNC, &[]);
    c.events();
    c
}

fn commit(c: &mut Client, buffer: u32, b: (i32, i32, i32, i32)) {
    c.req(SURFACE, 1, &[A::U(buffer), A::I(0), A::I(0)]);
    c.req(SURFACE, 9, &[A::I(b.0), A::I(b.1), A::I(b.2), A::I(b.3)]);
    c.req(SURFACE, 6, &[]);
    c.events();
}

/// The bytes read out of the guest for a commit of `buffer` with `b` said.
fn read_for(c: &mut Client, buffer: u32, b: (i32, i32, i32, i32)) -> u64 {
    guest_reads::take();
    commit(c, buffer, b);
    guest_reads::take().bytes
}

fn child(c: &mut Client, buffer: Option<u32>) {
    c.req(CHILD, 1, &[A::U(buffer.unwrap_or(0)), A::I(0), A::I(0)]);
    c.req(CHILD, 9, &[A::I(0), A::I(0), A::I(16), A::I(16)]);
    c.req(CHILD, 6, &[]);
    c.events();
}

fn child_shown(c: &Client) -> usize {
    let (_, _, px) = pixels(&c.s.guest).expect("the window's pixels");
    px.iter().filter(|&&p| p == CHILD_PX).count()
}

#[test]
fn an_empty_subsurface_leaves_its_window_shown_in_part() {
    let mut c = window();
    let read = read_for(&mut c, 103, (10, 10, 40, 20));
    assert!(read < WHOLE / 8, "the box read, not the window: {read}");
    assert!(c.s.guest.scene.win.base.is_empty(), "no pixels kept for a layer that shows nothing");
    let read = read_for(&mut c, 101, (100, 50, 8, 8));
    assert!(read < WHOLE / 8, "and again: {read}");
    assert!(!c.s.guest.objects.failed());
}

#[test]
fn a_subsurface_shown_then_taken_down_is_laid_then_cleared() {
    let mut c = window();
    child(&mut c, Some(CHILD_BUFFER));
    assert_eq!(child_shown(&c), 16 * 16, "laid on the window at once");
    let read = read_for(&mut c, 103, (10, 10, 40, 20));
    assert!(read >= WHOLE, "a window with a layer on it is drawn whole: {read}");
    assert_eq!(child_shown(&c), 16 * 16, "still laid on it");
    child(&mut c, None);
    assert_eq!(child_shown(&c), 0, "taken down, its pixels are cleared");
    let read = read_for(&mut c, 101, (10, 10, 40, 20));
    assert!(read >= WHOLE, "the commit after draws the window whole once: {read}");
    assert_eq!(child_shown(&c), 0);
    let read = read_for(&mut c, 103, (10, 10, 40, 20));
    assert!(read < WHOLE / 8, "then in part again: {read}");
    assert_eq!(child_shown(&c), 0);
    assert!(!c.s.guest.objects.failed());
}

/// The handles the compositor was told to draw over the window's layer,
/// 0 for none.
fn contents() -> Vec<u64> {
    let calls = desk::with(|d| d.calls.clone());
    calls
        .iter()
        .filter_map(|c| match c {
            desk::Call::Content(_, handle, _, _, _) => Some(*handle),
            _ => None,
        })
        .collect()
}

#[test]
fn a_pinned_window_whose_subsurface_shows_goes_back_to_its_own_buffers_with_it_laid() {
    let mut c = window();
    nonos_libc::world::with(|w| w.allow_pins = true);
    desk::with(|d| d.calls.clear());
    let read = read_for(&mut c, 103, (10, 10, 40, 20));
    assert_eq!(read, 0, "an empty subsurface leaves the window to its pin");
    assert!(contents().last().is_some_and(|&h| h != 0), "shown from its pin: {:?}", contents());
    child(&mut c, Some(CHILD_BUFFER));
    assert_eq!(contents().last(), Some(&0), "the pin taken off the layer: {:?}", contents());
    assert!(c.s.guest.scene.win.content.is_none());
    assert_eq!(child_shown(&c), 16 * 16, "the subsurface laid on the window's own pixels");
    assert!(!c.s.guest.objects.failed());
}

#[test]
fn an_empty_subsurface_committing_nothing_leaves_the_window_on_its_pin() {
    let mut c = window();
    nonos_libc::world::with(|w| w.allow_pins = true);
    desk::with(|d| d.calls.clear());
    commit(&mut c, 103, (10, 10, 40, 20));
    for _ in 0..4 {
        child(&mut c, None);
    }
    let read = read_for(&mut c, 101, (10, 10, 40, 20));
    assert_eq!(read, 0, "still shown from its pin: {read}");
    assert!(contents().iter().all(|&h| h != 0), "never taken off the layer: {:?}", contents());
    assert!(c.s.guest.scene.win.content.is_some());
    assert!(!c.s.guest.objects.failed());
}
