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

//! Taking a guest's windows down when what made them ends (window_life.rs):
//! a toplevel or surface destroyed takes its own window, a connection
//! closed or a process ended takes them all.

use core::mem;

use crate::linux::guest::Guest;

use super::super::present::surface::say;
use super::super::scene::Scene;
use super::super::window_front;
use super::super::window_life::{take_down, takes_down, End};
use super::close_steps::Calls;

/// Take down the window `end` ends; the guest's other windows stay, and
/// the latest of them is the one acted on next.
pub fn close_for(guest: &mut Guest, end: End) {
    if !window_front::select(guest, |w| takes_down(end, w.shown)) {
        return;
    }
    // The keyboard and the pointer leave it first, if they are on it.
    super::super::input::focus::leave(guest);
    let pointed = guest.scene.pointer_on.filter(|&s| window_front::on_win(&guest.scene, s));
    if pointed.is_some() {
        super::super::input::off::pointer_off(guest);
    }
    // A guest buffer the window kept to draw goes back: it never will.
    if let Some(buffer) = super::super::present::owed::kept(&guest.scene.win) {
        super::super::present::owed::release(guest, buffer);
    }
    // So does the one its layer drew, as the layer goes.
    if let Some(held) = guest.scene.win.content.take() {
        super::super::present::owed::release(guest, held.buffer);
    }
    close_shown(&mut guest.scene);
    if let Some(next) = guest.scene.behind.pop() {
        guest.scene.win = next;
    }
}

/// Take every window of the guest down: its connection or process ended.
pub fn close_window(scene: &mut Scene) {
    loop {
        close_shown(scene);
        match scene.behind.pop() {
            Some(next) => scene.win = next,
            None => return,
        }
    }
}

/// Take `Scene.win` down: its layer off the screen, its buffers released
/// and their pixels freed, and gone from the window manager, which brings the
/// dock back if it was full screen.
fn close_shown(scene: &mut Scene) {
    let Some(handle) = scene.win.out.take() else {
        scene.win = Default::default();
        return;
    };
    let window = scene.win.window.take();
    let bufs = Some(mem::take(&mut scene.win.bufs));
    let quarters = scene.win.frame.quarters;
    scene.win = Default::default();
    scene.win.frame.quarters = quarters;
    let mut calls = Calls { handle, window, bufs, serial: scene.serial };
    let taken = take_down(&mut calls);
    scene.serial = calls.serial;
    if !taken.whole() {
        let who: alloc::vec::Vec<&str> = taken.refused().collect();
        say(alloc::format!(
            "[WAYLAND] window of surface {handle} not wholly taken down: {} refused\n",
            who.join(", ")
        ));
    }
}

/// The client closed its connection: its window goes, and so does every
/// object it made, so a new connection starts from nothing.
pub fn disconnect(guest: &mut Guest) {
    super::super::clip::desk_read::stop(guest);
    close_window(&mut guest.scene);
    // The descriptors passed and kept (unix/passed.rs) go with the connection.
    let queued = core::mem::take(&mut guest.display.fds);
    let pools: alloc::vec::Vec<u32> = guest.scene.pools.iter().map(|p| p.fd).collect();
    queued.into_iter().chain(pools).for_each(|fd| crate::linux::unix::passed::drop(guest, fd));
    let subscribed = guest.scene.subscribed;
    guest.scene = Scene::new();
    guest.scene.subscribed = subscribed;
    guest.objects = super::super::Objects::new();
    guest.display = Default::default();
}
