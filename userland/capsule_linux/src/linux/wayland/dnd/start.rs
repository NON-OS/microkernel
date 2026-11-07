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

//! wl_data_device.start_drag. Taken only with the serial of the latest
//! press, while a button is still held, from the surface the pointer is on;
//! otherwise the source is cancelled at once and nothing is dragged. Taken,
//! the pointer leaves the client's surface, as the drag holds it; the drag
//! enters what it is over at the next motion (over.rs).

use crate::linux::guest::Guest;

use super::super::args::Args;
use super::super::object::Object;
use super::super::ops::data::SOURCE_CANCELLED;
use super::super::out::Event;
use super::state::Drag;

pub fn start(guest: &mut Guest, args: &mut Args<'_>) {
    // The icon is the client's surface, drawn by it and not shown here.
    let (source, origin, _icon) = (args.u32().unwrap_or(0), args.u32().unwrap_or(0), args.u32());
    let serial = args.u32().unwrap_or(0);
    let source = (guest.objects.get(source) == Some(Object::DataSource)).then_some(source);
    let scene = &guest.scene;
    let held = scene.dnd.pressed && scene.popups.last_press == Some(serial);
    if !held || scene.pointer_on != Some(origin) || scene.dnd.dragging() {
        if let Some(source) = source {
            Event::new(source, SOURCE_CANCELLED).send(&mut guest.display.to_client);
        }
        return;
    }
    // A drop the client never finished gives way: kept, every drag after
    // it was cancelled at once.
    if guest.scene.dnd.drag.is_some() {
        super::end::cancel(guest);
    }
    guest.scene.dnd.drag = Some(Drag {
        source,
        over: None,
        offers: alloc::vec::Vec::new(),
        accepted: false,
        dest: (0, 0),
        action: super::action::NONE,
        dropped: false,
    });
    super::super::input::off::pointer_off(guest);
}
