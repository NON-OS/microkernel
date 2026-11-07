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

//! The window gaining or losing the keyboard, as the window manager says it
//! to the window's owner (capsule_wm focus/notice.rs, abi/wire.toml
//! [wm.notify_v1] kind 4), on every edge however focus moved. The client is
//! told as a compositor tells it: wl_keyboard.enter or leave, its toplevel
//! configured with or without `activated`, and its popups' grabs ended when
//! it loses focus. Only the window manager's own pid is believed.

use nonos_app_skeleton::clients::wm::popup_done;
use nonos_app_skeleton::discover::from_wm;
use nonos_app_skeleton::runner::inbox::keys::keys_for;

use crate::linux::guest::Guest;

use super::super::object::Object;
use super::super::out::Event;
use super::enter::keyboard_on;
use super::leave::KEYBOARD_LEAVE;

/// A frame from the inbox that was not routed input: the window manager's
/// keyboard notice for this window, when it is one.
pub fn notice(guest: &mut Guest, frame: &[u8], sender: u32) {
    if !from_wm(sender) {
        return;
    }
    // The window manager closed a popup in a window of its own: its parent
    // went, moved or was minimised, or a press fell outside its grab.
    if let Some(window) = popup_done(frame) {
        let popups = &guest.scene.popup_windows;
        if let Some(id) = popups.iter().find(|w| w.window == window).map(|w| w.popup) {
            super::super::popup::ends::end(guest, id);
        }
        return;
    }
    for window in super::super::window_front::windows(&guest.scene) {
        if let Some(has) = keys_for(frame, window) {
            super::super::window_front::select_window(guest, window);
            keys(guest, has);
        }
    }
}

/// The window has the keyboard now, or no longer has it.
pub fn keys(guest: &mut Guest, has: bool) {
    if guest.scene.win.focused == has {
        return;
    }
    guest.scene.win.focused = has;
    match has {
        true => {
            let _ = keyboard_on(guest);
        }
        false => {
            // Input given before is no claim on what the user copies next.
            guest.scene.clip.input_ms = 0;
            // The popups end with the focus, but a press still held is the
            // one a title-bar move names: the window manager's word that the
            // keyboard left can come after it, and the move was refused.
            let held = guest.scene.dnd.pressed.then_some(guest.scene.popups.last_press).flatten();
            super::super::popup::ends::end_all(guest);
            guest.scene.popups.last_press = held;
            guest.scene.dnd.kept = held.is_some();
            super::super::dnd::end::cancel(guest);
            leave(guest);
        }
    }
    // Its frame is drawn as focused or not with the frame the toplevel draws
    // for this state (its paint key holds focus, present/frame/fill.rs).
    super::super::toplevel::again(guest);
}

/// A popup took its grab on the window that has the keyboard: the keyboard
/// goes to it now, as a compositor moves it, not at the next key.
pub fn follow(guest: &mut Guest) {
    if guest.scene.win.focused {
        let _ = keyboard_on(guest);
    }
}

/// Every keyboard leaves the surface it is on, every key let go with it.
/// Only when it is on this window: another of the guest's windows may have
/// been told it has the keyboard first.
pub fn leave(guest: &mut Guest) {
    let on =
        guest.scene.keyboard_on.filter(|&s| super::super::window_front::on_win(&guest.scene, s));
    if let Some(on) = on {
        leave_surface(guest, on);
    }
}

/// Popups the keyboard may be on ended while the window has no focus: when
/// it was on one of their `surfaces`, it leaves that one, so the client is
/// not left holding the keyboard on a surface that is gone; nothing enters
/// in its place, since the window has no keyboard to give.
pub fn popups_gone(guest: &mut Guest, surfaces: &[u32]) {
    if let Some(on) = guest.scene.keyboard_on.filter(|s| surfaces.contains(s)) {
        leave_surface(guest, on);
    }
}

fn leave_surface(guest: &mut Guest, on: u32) {
    guest.scene.keyboard_on = None;
    guest.scene.held = super::super::held_keys::HeldKeys::new();
    if guest.objects.get(on) == Some(Object::Surface) {
        let serial = guest.scene.next_serial();
        for keyboard in super::devices::keyboards(guest) {
            let ev = Event::new(keyboard, KEYBOARD_LEAVE).u32(serial).u32(on);
            ev.send(&mut guest.display.to_client);
        }
    }
}
