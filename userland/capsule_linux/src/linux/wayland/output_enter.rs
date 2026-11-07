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

//! wl_surface.enter: which outputs a surface is on, which a client sizes
//! and scales its window for. A window shown is on the one display, so its
//! surface is told it entered every wl_output the client holds, once, when
//! it first comes to be shown, and again for an output bound after. A
//! window kept behind the shown one stays on it (window_front/).

use alloc::vec::Vec;

use crate::linux::guest::Guest;

use super::object::Object;
use super::out::Event;

const SURFACE_ENTER: u16 = 0;
const SURFACE_LEAVE: u16 = 1;

/// After a present: a surface shown for the first time enters the outputs.
pub fn shown(guest: &mut Guest) {
    let Some(surface) = guest.scene.win.shown.map(|w| w.surface) else { return };
    if guest.scene.on_output.contains(&surface) {
        return;
    }
    guest.scene.on_output.push(surface);
    let outputs: Vec<u32> = guest.objects.all(Object::Output).collect();
    for output in outputs {
        enter(guest, surface, output);
    }
}

/// An output bound while a surface is on the display.
pub fn bound(guest: &mut Guest, output: u32) {
    for surface in guest.scene.on_output.clone() {
        enter(guest, surface, output);
    }
}

/// A surface no longer shown leaves every output, and enters them again
/// when it is shown again.
pub fn left(guest: &mut Guest, surface: u32) {
    if !guest.scene.on_output.contains(&surface) {
        return;
    }
    guest.scene.on_output.retain(|&s| s != surface);
    if guest.objects.get(surface) != Some(Object::Surface) {
        return;
    }
    let outputs: Vec<u32> = guest.objects.all(Object::Output).collect();
    let to = &mut guest.display.to_client;
    outputs.into_iter().for_each(|o| Event::new(surface, SURFACE_LEAVE).u32(o).send(to));
}

fn enter(guest: &mut Guest, surface: u32, output: u32) {
    // A surface the client has destroyed is told nothing.
    if guest.objects.get(surface) == Some(Object::Surface) {
        Event::new(surface, SURFACE_ENTER).u32(output).send(&mut guest.display.to_client);
    }
}
