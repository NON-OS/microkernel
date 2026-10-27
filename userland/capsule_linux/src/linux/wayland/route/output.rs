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

//! wl_output's release and zxdg_output_manager_v1; the scale a surface is
//! drawn at is scale.rs's. An xdg_output says where the one screen is in the
//! logical space and how large: the size wl_output gives (output.rs) over
//! the display's scale, then done (version 2). False means nothing served it.

use crate::linux::guest::Guest;

use super::super::args::Args;
use super::super::object::Object;
use super::super::ops::output::*;
use super::super::out::Event;
use super::super::present::scale_math::{display, to_logical};
use nonos_app_skeleton::runner::chrome::quarters_for;

pub fn route(
    guest: &mut Guest,
    object: Option<Object>,
    id: u32,
    op: u16,
    args: &mut Args<'_>,
) -> bool {
    use Object::*;
    match (object, op) {
        (Some(XdgOutputManager), XDG_OUTPUT_MANAGER_GET) => xdg_output(guest, args),
        (Some(Viewporter | Viewport | FractionalManager | FractionalScale), _) => {
            return super::scale::route(guest, object, id, op, args)
        }
        (Some(Output), OUTPUT_RELEASE)
        | (Some(XdgOutputManager), XDG_OUTPUT_MANAGER_DESTROY)
        | (Some(XdgOutput), XDG_OUTPUT_DESTROY) => super::super::forget::forget(guest, object, id),
        _ => return super::misc::route(guest, object, id, op, args),
    }
    true
}

/// Made only for a wl_output the client holds.
fn xdg_output(guest: &mut Guest, args: &mut Args<'_>) {
    let (Some(id), Some(output)) = (args.u32(), args.u32()) else { return };
    if guest.objects.get(output) != Some(Object::Output)
        || !guest.objects.put(id, Object::XdgOutput)
    {
        return;
    }
    let (w, h) = super::super::output::screen();
    let d = display(quarters_for(w, h));
    let (w, h) = (to_logical(w as i32, d) as u32, to_logical(h as i32, d) as u32);
    // The name and description came with zxdg_output 2.
    let v2 = guest.objects.at_least(id, 2);
    let to = &mut guest.display.to_client;
    Event::new(id, XDG_OUTPUT_LOGICAL_POSITION).u32(0).u32(0).send(to);
    Event::new(id, XDG_OUTPUT_LOGICAL_SIZE).u32(w).u32(h).send(to);
    if v2 {
        Event::new(id, XDG_OUTPUT_NAME).string(b"NONOS-1").send(to);
        Event::new(id, XDG_OUTPUT_DESCRIPTION).string(b"the NONOS display").send(to);
    }
    Event::new(id, XDG_OUTPUT_DONE).send(to);
}
