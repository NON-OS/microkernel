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

//! The requests on a guest's sources, devices and offers that touch its
//! selections. The tables that name them are data_device.rs and
//! route/select.rs.

use crate::linux::guest::Guest;

use super::super::args::Args;
use super::super::ops::{data::*, select::*};
use super::desk::{copy, fallback};
use super::which::Which;
use super::Object;

/// A source names one more mime type it offers.
pub fn mime(guest: &mut Guest, source: u32, args: &mut Args<'_>) {
    if let Some(mime) = args.string() {
        guest.scene.clip.add_mime(source, mime);
    }
}

/// set_selection(source, serial). The serial is not checked: what a guest
/// sets is its own selection, and the desktop is reached only from the
/// window with focus (desk.rs).
pub fn set(guest: &mut Guest, which: Which, args: &mut Args<'_>) {
    let source = args.u32().unwrap_or(0);
    let (to, clip) = (&mut guest.display.to_client, &mut guest.scene.clip);
    let new = super::set::set(&mut guest.objects, clip, (which, source), fallback(which), to);
    if which == Which::Data {
        copy(guest, new);
    }
}

/// A source destroyed, which may have been the selection, and forgotten.
pub fn source_gone(guest: &mut Guest, which: Which, id: u32) {
    let (to, clip) = (&mut guest.display.to_client, &mut guest.scene.clip);
    super::gone::source_gone(&mut guest.objects, clip, (which, id), fallback(which), to);
    super::super::forget::forget(guest, Some(which.source()), id);
}

/// An offer's requests. False means nothing served it.
pub fn offer(guest: &mut Guest, object: Option<Object>, id: u32, op: u16, args: &mut Args) -> bool {
    use Object::{DataOffer, PrimaryOffer};
    match (object, op) {
        (Some(DataOffer), OFFER_RECEIVE) => super::take::receive(guest, Which::Data, id, args),
        (Some(PrimaryOffer), PRIMARY_OFFER_RECEIVE) => {
            super::take::receive(guest, Which::Primary, id, args)
        }
        (Some(DataOffer), OFFER_DESTROY) | (Some(PrimaryOffer), PRIMARY_OFFER_DESTROY) => {
            super::gone::offer_gone(&mut guest.objects, &mut guest.scene.clip, id)
        }
        // Only a drag's offer is accepted or finished, and no drag is made.
        (Some(DataOffer), OFFER_ACCEPT | OFFER_FINISH | OFFER_SET_ACTIONS) => {}
        _ => return false,
    }
    true
}
