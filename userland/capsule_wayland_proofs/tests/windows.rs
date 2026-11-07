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

//! A guest's second toplevel, a dialog, in a window of its own beside its
//! parent: its own compositor layer and window manager window, drawn on its
//! own, pointer events going to the window the router names, keys to the
//! one the window manager gave the keyboard; closed, its layer and window
//! go and the parent's stay as they were (window_front/).

use nonos_app_skeleton::desk::{self, Call};
use nonos_capsule_wayland_proofs::client::*;
use nonos_capsule_wayland_proofs::host::dump::surface_pixels;
use nonos_capsule_wayland_proofs::input;

const DIALOG_SURFACE: u32 = 30;
const DIALOG_XDG: u32 = 31;
const DIALOG: u32 = 32;
const DIALOG_POOL: u32 = 33;
const DIALOG_BUFFER: u32 = 34;
const POOL2: u32 = 35;
const BUFFER2: u32 = 36;
const POINTER: u32 = 37;
const BUFFER_RELEASE: u16 = 0;
const KEYBOARD_ENTER: u16 = 1;
const POINTER_ENTER: u16 = 0;

const BLUE: u32 = 0xFF20_40A0;
const RED: u32 = 0xFFC0_2020;
const GREEN: u32 = 0xFF20_A040;

type Shown = (u32, u32, Vec<u32>);

/// The NONOS surface and window manager window of `surface`'s window.
fn window_of(c: &Client, surface: u32) -> (u64, u32) {
    let scene = &c.s.guest.scene;
    let mut all = std::iter::once(&scene.win).chain(scene.behind.iter());
    let w = all.find(|w| w.shown.is_some_and(|s| s.surface == surface));
    let w = w.expect("a window of its own");
    (w.out.expect("on a surface"), w.window.expect("a window manager window"))
}

/// Whether `surface`'s window has the keyboard.
fn window_focused(c: &Client, surface: u32) -> bool {
    let scene = &c.s.guest.scene;
    let mut all = std::iter::once(&scene.win).chain(scene.behind.iter());
    all.any(|w| w.focused && w.shown.is_some_and(|s| s.surface == surface))
}

/// `surface`'s window's pixels as shown.
fn pixels_of(c: &Client, surface: u32) -> Shown {
    surface_pixels(window_of(c, surface).0).expect("its pixels")
}

fn count(px: &[u32], argb: u32) -> usize {
    px.iter().filter(|&&p| p == argb).count()
}

/// The layers the compositor holds, by handle.
fn layers() -> Vec<u64> {
    desk::with(|d| d.layers.iter().map(|l| l.surface_handle).collect())
}

/// The main window, 400 by 300 of blue, shown and focused.
fn main_window() -> Client {
    let mut c = toplevel(6);
    c.req(SEAT, 0, &[A::U(POINTER)]);
    c.req(TOPLEVEL, 2, &[A::S("main")]);
    c.req(SURFACE, 6, &[]);
    let serial = to(&c.events(), XDG_SURFACE)[0].word(0);
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    c.painted(SHM, POOL, BUFFER, (400, 300), BLUE);
    c.req(SURFACE, 1, &[A::U(BUFFER), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    assert!(c.s.guest.scene.win.focused);
    c
}

/// A dialog of the main window, `w` by `h` of `argb`, configured and drawn.
fn dialog(c: &mut Client, (w, h): (i32, i32), argb: u32) -> Vec<Ev> {
    c.req(COMPOSITOR, 0, &[A::U(DIALOG_SURFACE)]);
    c.req(WM_BASE, 2, &[A::U(DIALOG_XDG), A::U(DIALOG_SURFACE)]);
    c.req(DIALOG_XDG, 1, &[A::U(DIALOG)]);
    c.req(DIALOG, 1, &[A::U(TOPLEVEL)]);
    c.req(DIALOG, 2, &[A::S("dialog")]);
    map(c, (w, h), argb)
}

/// The dialog's initial commit, its configure acked, and a buffer drawn.
fn map(c: &mut Client, (w, h): (i32, i32), argb: u32) -> Vec<Ev> {
    c.req(DIALOG_SURFACE, 6, &[]);
    let serial = to(&c.events(), DIALOG_XDG)[0].word(0);
    c.req(DIALOG_XDG, 4, &[A::U(serial)]);
    if c.s.guest.scene.buffers.iter().all(|b| b.id != DIALOG_BUFFER) {
        c.painted(SHM, DIALOG_POOL, DIALOG_BUFFER, (w, h), argb);
    }
    c.req(DIALOG_SURFACE, 1, &[A::U(DIALOG_BUFFER), A::I(0), A::I(0)]);
    c.req(DIALOG_SURFACE, 6, &[]);
    c.events()
}

fn entered(evs: &[Ev], device: u32, op: u16) -> Vec<u32> {
    let enters = evs.iter().filter(|e| e.object == device && e.opcode == op);
    enters.map(|e| e.word(1)).collect()
}

/// The dialog's toplevel, xdg_surface and surface destroyed.
fn close_dialog(c: &mut Client) -> Vec<Ev> {
    c.req(DIALOG, 0, &[]);
    c.req(DIALOG_XDG, 0, &[]);
    c.req(DIALOG_SURFACE, 0, &[]);
    c.events()
}

#[test]
fn a_dialog_opens_in_a_window_of_its_own_and_closes_leaving_its_parent_as_it_was() {
    let mut c = main_window();
    let (main, main_id) = window_of(&c, SURFACE);
    let before = pixels_of(&c, SURFACE);
    assert_eq!(count(&before.2, BLUE), 400 * 300);

    let evs = dialog(&mut c, (200, 100), RED);
    // Opened, the window manager gives it the keyboard.
    assert_eq!(entered(&evs, KEYBOARD, KEYBOARD_ENTER), vec![DIALOG_SURFACE]);
    let (dialog, dialog_id) = window_of(&c, DIALOG_SURFACE);
    assert_ne!(dialog_id, main_id, "a window manager window of its own");
    assert_eq!(layers(), vec![main, dialog], "both on screen, the dialog on top");
    assert_eq!(count(&pixels_of(&c, DIALOG_SURFACE).2, RED), 200 * 100);
    assert_eq!(pixels_of(&c, SURFACE), before, "the main window untouched");
    assert_eq!(c.s.guest.scene.win.frame.title, b"dialog");

    // The window manager told the main window it lost the keyboard: it
    // stays on the dialog.
    assert!(!window_focused(&c, SURFACE) && window_focused(&c, DIALOG_SURFACE));

    // Closed, the keyboard leaves it and the window manager gives it back
    // to the main window.
    let evs = close_dialog(&mut c);
    const KEYBOARD_LEAVE: u16 = 2;
    assert_eq!(entered(&evs, KEYBOARD, KEYBOARD_LEAVE), vec![DIALOG_SURFACE], "left first");
    assert_eq!(layers(), vec![main], "the dialog's layer alone goes");
    desk::with(|d| {
        assert!(d.calls.contains(&Call::RemoveLayer(dialog)));
        assert!(d.calls.contains(&Call::Close(dialog_id)));
        assert!(!d.calls.contains(&Call::Close(main_id)));
    });
    let evs = [evs, c.events()].concat();
    assert_eq!(entered(&evs, KEYBOARD, KEYBOARD_ENTER), vec![SURFACE]);
    assert_eq!(c.s.guest.scene.keyboard_on, Some(SURFACE));
    assert!(window_focused(&c, SURFACE));
    assert_eq!(pixels_of(&c, SURFACE), before, "byte for byte");
    assert_eq!(c.s.guest.scene.win.frame.title, b"main");
}

#[test]
fn a_window_draws_on_its_own_while_its_dialog_is_up() {
    let mut c = main_window();
    dialog(&mut c, (200, 100), RED);
    let dialog_px = pixels_of(&c, DIALOG_SURFACE);
    c.painted(SHM, POOL2, BUFFER2, (400, 300), GREEN);
    c.req(SURFACE, 1, &[A::U(BUFFER2), A::I(0), A::I(0)]);
    c.req(SURFACE, 2, &[A::I(0), A::I(0), A::I(400), A::I(300)]);
    c.req(SURFACE, 6, &[]);
    let evs = c.events();
    assert_eq!(count(&pixels_of(&c, SURFACE).2, GREEN), 400 * 300, "shown at once");
    assert_eq!(pixels_of(&c, DIALOG_SURFACE), dialog_px, "the dialog untouched");
    let released = to(&evs, BUFFER2);
    assert_eq!(released.iter().map(|e| e.opcode).collect::<Vec<_>>(), vec![BUFFER_RELEASE]);
}

#[test]
fn a_pointer_event_goes_to_the_window_the_router_names() {
    let mut c = main_window();
    dialog(&mut c, (200, 100), RED);
    let (_, main_id) = window_of(&c, SURFACE);
    let (_, dialog_id) = window_of(&c, DIALOG_SURFACE);
    for (id, surface) in [(dialog_id, DIALOG_SURFACE), (main_id, SURFACE)] {
        input::on_window(Some(id));
        // Well inside the content, frame included.
        input::motion(100, 80);
        let evs = c.events();
        assert_eq!(entered(&evs, POINTER, POINTER_ENTER), vec![surface], "{evs:?}");
    }
    input::on_window(None);
}

#[test]
fn a_dialog_unmapped_goes_from_the_screen_and_comes_back_when_mapped() {
    let mut c = main_window();
    let (main, _) = window_of(&c, SURFACE);
    let before = pixels_of(&c, SURFACE);
    dialog(&mut c, (200, 100), RED);
    let (dialog, dialog_id) = window_of(&c, DIALOG_SURFACE);
    // Qt hides a dialog it will show again with a null buffer.
    c.req(DIALOG_SURFACE, 1, &[A::U(0), A::I(0), A::I(0)]);
    c.req(DIALOG_SURFACE, 6, &[]);
    c.events();
    assert_eq!(layers(), vec![main]);
    assert!(desk::with(|d| d.calls.contains(&Call::Close(dialog_id))));
    assert_eq!(pixels_of(&c, SURFACE), before);

    map(&mut c, (200, 100), RED);
    let (again, _) = window_of(&c, DIALOG_SURFACE);
    assert_ne!(again, dialog, "a new surface");
    assert_eq!(layers(), vec![main, again]);
    assert_eq!(count(&pixels_of(&c, DIALOG_SURFACE).2, RED), 200 * 100);
}

#[test]
fn the_main_window_destroyed_under_its_dialog_goes_and_the_dialog_stays() {
    let mut c = main_window();
    dialog(&mut c, (200, 100), RED);
    let (dialog, _) = window_of(&c, DIALOG_SURFACE);
    let with_dialog = pixels_of(&c, DIALOG_SURFACE);
    c.req(TOPLEVEL, 0, &[]);
    c.req(XDG_SURFACE, 0, &[]);
    c.req(SURFACE, 0, &[]);
    c.events();
    assert_eq!(layers(), vec![dialog]);
    assert!(c.s.guest.scene.behind.is_empty());
    assert_eq!(pixels_of(&c, DIALOG_SURFACE), with_dialog);
    close_dialog(&mut c);
    assert!(layers().is_empty(), "the last window gone, the guest has none");
    assert!(c.s.guest.scene.win.out.is_none());
}

/// The main window asks to be maximized from its own button after its
/// dialog committed last: the ask is the main window's, which is told its
/// maximized size; the dialog's window is left as it was.
#[test]
fn a_maximize_asked_on_one_window_after_another_committed_goes_to_it() {
    let mut c = main_window();
    dialog(&mut c, (200, 100), RED);
    let dialog_fit = |c: &Client| {
        let scene = &c.s.guest.scene;
        let mut all = std::iter::once(&scene.win).chain(scene.behind.iter());
        let w = all.find(|w| w.shown.is_some_and(|s| s.surface == DIALOG_SURFACE));
        w.expect("the dialog's window").fit.heading()
    };
    let before = dialog_fit(&c);
    c.req(TOPLEVEL, 9, &[]);
    let evs = c.events();
    let configure = to(&evs, TOPLEVEL).into_iter().find(|e| e.opcode == 0);
    let configure = configure.expect("the main window configured");
    assert!(configure.array(2).contains(&1), "told maximized");
    assert!(to(&evs, DIALOG).iter().all(|e| e.opcode != 0), "the dialog told nothing");
    assert_eq!(dialog_fit(&c), before, "the dialog's window as it was");
}

/// set_minimized on the main window after its dialog committed last
/// minimizes the main window's window manager window.
#[test]
fn a_minimize_asked_on_one_window_after_another_committed_goes_to_it() {
    let mut c = main_window();
    let (_, main_id) = window_of(&c, SURFACE);
    dialog(&mut c, (200, 100), RED);
    c.req(TOPLEVEL, 13, &[]);
    c.events();
    desk::with(|d| assert!(d.calls.contains(&Call::Minimize(main_id)), "{:?}", d.calls));
}

/// A second toplevel configured but not drawn yet asks to be maximized:
/// the main window, the one shown, is neither told nor filled.
#[test]
fn a_maximize_asked_before_a_window_is_drawn_leaves_the_shown_window_alone() {
    let mut c = main_window();
    let before = c.s.guest.scene.win.fit.heading();
    c.req(COMPOSITOR, 0, &[A::U(DIALOG_SURFACE)]);
    c.req(WM_BASE, 2, &[A::U(DIALOG_XDG), A::U(DIALOG_SURFACE)]);
    c.req(DIALOG_XDG, 1, &[A::U(DIALOG)]);
    c.req(DIALOG_SURFACE, 6, &[]);
    let serial = to(&c.events(), DIALOG_XDG)[0].word(0);
    c.req(DIALOG_XDG, 4, &[A::U(serial)]);
    c.req(DIALOG, 9, &[]);
    let evs = c.events();
    assert!(to(&evs, TOPLEVEL).iter().all(|e| e.opcode != 0), "the main window told nothing");
    assert_eq!(c.s.guest.scene.win.fit.heading(), before, "the main window not filled");
    assert_eq!(c.s.guest.scene.win.fit_of, None, "no ask of the dialog's on its fit");
}

/// The ask is kept, and asked once the dialog's own window is shown: the
/// dialog is told its maximized size, the main window nothing.
#[test]
fn a_maximize_asked_before_a_window_is_drawn_is_asked_once_it_is_shown() {
    let mut c = main_window();
    c.req(COMPOSITOR, 0, &[A::U(DIALOG_SURFACE)]);
    c.req(WM_BASE, 2, &[A::U(DIALOG_XDG), A::U(DIALOG_SURFACE)]);
    c.req(DIALOG_XDG, 1, &[A::U(DIALOG)]);
    c.req(DIALOG_SURFACE, 6, &[]);
    let serial = to(&c.events(), DIALOG_XDG)[0].word(0);
    c.req(DIALOG_XDG, 4, &[A::U(serial)]);
    c.req(DIALOG, 9, &[]);
    c.events();
    c.painted(SHM, DIALOG_POOL, DIALOG_BUFFER, (200, 100), RED);
    c.req(DIALOG_SURFACE, 1, &[A::U(DIALOG_BUFFER), A::I(0), A::I(0)]);
    c.req(DIALOG_SURFACE, 6, &[]);
    let evs = c.events();
    assert!(maximized(&evs, DIALOG), "the dialog told maximized");
    assert!(!maximized(&evs, TOPLEVEL), "the main window not");
}

/// Taken back before it is shown, nothing is asked.
#[test]
fn a_maximize_taken_back_before_a_window_is_drawn_is_not_asked() {
    let mut c = main_window();
    c.req(COMPOSITOR, 0, &[A::U(DIALOG_SURFACE)]);
    c.req(WM_BASE, 2, &[A::U(DIALOG_XDG), A::U(DIALOG_SURFACE)]);
    c.req(DIALOG_XDG, 1, &[A::U(DIALOG)]);
    c.req(DIALOG_SURFACE, 6, &[]);
    let serial = to(&c.events(), DIALOG_XDG)[0].word(0);
    c.req(DIALOG_XDG, 4, &[A::U(serial)]);
    let before = c.s.guest.scene.win.fit.heading();
    c.req(DIALOG, 9, &[]);
    c.req(DIALOG, 10, &[]);
    c.events();
    assert_eq!(c.s.guest.scene.win.fit.heading(), before, "the main window not restored");
    c.painted(SHM, DIALOG_POOL, DIALOG_BUFFER, (200, 100), RED);
    c.req(DIALOG_SURFACE, 1, &[A::U(DIALOG_BUFFER), A::I(0), A::I(0)]);
    c.req(DIALOG_SURFACE, 6, &[]);
    let evs = c.events();
    assert!(!maximized(&evs, DIALOG), "nothing asked");
}

/// Whether `toplevel` was told the maximized state in `evs`.
fn maximized(evs: &[Ev], toplevel: u32) -> bool {
    to(evs, toplevel).iter().any(|e| e.opcode == 0 && e.array(2).contains(&1))
}

/// Asked before its first configure while another window is shown, the
/// dialog is still configured, so it can draw; once shown it is maximized.
#[test]
fn a_maximize_asked_before_the_first_configure_still_configures_the_dialog() {
    let mut c = main_window();
    c.req(COMPOSITOR, 0, &[A::U(DIALOG_SURFACE)]);
    c.req(WM_BASE, 2, &[A::U(DIALOG_XDG), A::U(DIALOG_SURFACE)]);
    c.req(DIALOG_XDG, 1, &[A::U(DIALOG)]);
    c.req(DIALOG, 9, &[]);
    c.req(DIALOG_SURFACE, 6, &[]);
    let evs = c.events();
    let told = to(&evs, DIALOG_XDG);
    assert!(!told.is_empty(), "the dialog configured");
    assert!(!maximized(&evs, TOPLEVEL), "the main window not");
    c.req(DIALOG_XDG, 4, &[A::U(told[0].word(0))]);
    c.painted(SHM, DIALOG_POOL, DIALOG_BUFFER, (200, 100), RED);
    c.req(DIALOG_SURFACE, 1, &[A::U(DIALOG_BUFFER), A::I(0), A::I(0)]);
    c.req(DIALOG_SURFACE, 6, &[]);
    assert!(maximized(&c.events(), DIALOG), "the dialog told maximized once shown");
}
