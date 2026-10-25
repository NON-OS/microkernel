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
//! and scales its window for. The shown window is on the one display, so
//! its surface is told it entered every wl_output the client holds, once,
//! when it comes to be shown, and again for an output bound after.

use alloc::vec::Vec;

use crate::linux::guest::Guest;

use super::object::Object;
use super::out::Event;

const SURFACE_ENTER: u16 = 0;

/// After a present: a surface shown for the first time enters the outputs.
pub fn shown(guest: &mut Guest) {
    let Some(surface) = guest.scene.shown.map(|w| w.surface) else { return };
    if guest.scene.on_output == Some(surface) {
        return;
    }
    guest.scene.on_output = Some(surface);
    let outputs: Vec<u32> = guest.objects.all(Object::Output).collect();
    for output in outputs {
        enter(guest, surface, output);
    }
}

/// An output bound while a surface is on the display.
pub fn bound(guest: &mut Guest, output: u32) {
    if let Some(surface) = guest.scene.on_output {
        enter(guest, surface, output);
    }
}

fn enter(guest: &mut Guest, surface: u32, output: u32) {
    // A surface the client has destroyed is told nothing.
    if guest.objects.get(surface) == Some(Object::Surface) {
        Event::new(surface, SURFACE_ENTER).u32(output).send(&mut guest.display.to_client);
    }
}
