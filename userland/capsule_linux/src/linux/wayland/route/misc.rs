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

//! xdg_activation_v1 and wp_single_pixel_buffer_manager_v1. A guest has
//! one window, and which window has focus is the person's to say, so an
//! activation changes nothing; a token is still issued, done on its commit,
//! so a client waiting for one goes on. A single-pixel buffer is a real
//! wl_buffer, so its id is good, but only pixels from shared memory are
//! shown: a commit of one shows nothing and releases it. False means
//! nothing served it.

use alloc::format;

use crate::linux::guest::Guest;

use super::super::args::Args;
use super::super::object::Object;
use super::super::ops::misc::*;
use super::super::out::Event;
use super::rest::make;

pub fn route(
    guest: &mut Guest,
    object: Option<Object>,
    id: u32,
    op: u16,
    args: &mut Args<'_>,
) -> bool {
    use Object::*;
    match (object, op) {
        (Some(Activation), ACTIVATION_GET_TOKEN) => make(guest, args, ActivationToken),
        (Some(Activation), ACTIVATION_ACTIVATE) => {}
        (Some(ActivationToken), TOKEN_SET_FIRST..=TOKEN_SET_LAST) => {}
        (Some(ActivationToken), TOKEN_COMMIT) => {
            let token = format!("nonos-{id}");
            Event::new(id, TOKEN_DONE).string(token.as_bytes()).send(&mut guest.display.to_client);
        }
        (Some(SinglePixel), SINGLE_PIXEL_CREATE_BUFFER) => make(guest, args, Buffer),
        (Some(Activation), ACTIVATION_DESTROY)
        | (Some(ActivationToken), TOKEN_DESTROY)
        | (Some(SinglePixel), SINGLE_PIXEL_DESTROY) => {
            super::super::forget::forget(guest, object, id)
        }
        _ => return super::select::route(guest, object, id, op, args),
    }
    true
}
