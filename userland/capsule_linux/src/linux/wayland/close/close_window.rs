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

//! Taking a guest's window down when what made it ends (window_life.rs):
//! its toplevel or surface destroyed, its connection closed, its process
//! ended.

use core::mem;

use crate::linux::guest::Guest;

use super::super::fit::Fit;
use super::super::present::surface::say;
use super::super::scene::Scene;
use super::super::scene_pixels::Pixels;
use super::super::window_life::{take_down, takes_down, End};
use super::close_steps::Calls;

/// Take down the window `end` ends, if it ends the one shown.
pub fn close_for(scene: &mut Scene, end: End) {
    if takes_down(end, scene.shown) {
        close_window(scene);
    }
}

/// Take the shown window down: off the screen, its surface released and
/// its pixels freed, and gone from the window manager, which brings the
/// dock back if it was full screen.
pub fn close_window(scene: &mut Scene) {
    let Some(handle) = scene.out.take() else { return };
    let window = scene.window.take();
    let pixels = Some(mem::replace(&mut scene.pixels, Pixels::empty()));
    scene.at = None;
    scene.shape = None;
    scene.shown = None;
    scene.fit = Fit::default();
    let mut calls = Calls { handle, window, pixels, serial: scene.serial };
    let whole = take_down(&mut calls);
    scene.serial = calls.serial;
    if !whole {
        say(alloc::format!(
            "[WAYLAND] window of surface {handle} not wholly taken down: the compositor, the \
             kernel or the window manager refused\n"
        ));
    }
}

/// The client closed its connection: its window goes, and so does every
/// object it made, so a new connection starts from nothing.
pub fn disconnect(guest: &mut Guest) {
    close_window(&mut guest.scene);
    let subscribed = guest.scene.subscribed;
    guest.scene = Scene::new();
    guest.scene.subscribed = subscribed;
    guest.objects = super::super::Objects::new();
    guest.display = Default::default();
}
