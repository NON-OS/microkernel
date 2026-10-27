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

//! A selection told to every device of the guest: a new offer, its mime
//! types, then the selection, as wl_data_device and the primary selection
//! device say it. Pure, so the host proofs hold it.

use alloc::vec::Vec;

use super::state::Clip;
use super::which::Which;
use super::{Event, Objects};

/// Where the ids a server makes start (WL_SERVER_ID_START).
pub const SERVER_START: u32 = 0xff00_0000;

/// The lowest server id the client does not hold. libwayland-client takes
/// a new server id only at or below the end of its table of them, which
/// the lowest free one always is.
pub fn server_id(objects: &Objects) -> u32 {
    (SERVER_START..=u32::MAX).find(|&id| objects.get(id).is_none()).unwrap_or(SERVER_START)
}

/// Every device of `which` is given a new offer of `source`, or told there
/// is no selection when `source` is None.
pub fn offer_all(
    objects: &mut Objects,
    clip: &mut Clip,
    which: Which,
    source: Option<u32>,
    out: &mut Vec<u8>,
) {
    let devices: Vec<u32> = objects.all(which.device()).collect();
    for device in devices {
        let offer = source.and_then(|s| offer_one(objects, clip, (which, device), s, out));
        Event::new(device, which.device_events().1).u32(offer.unwrap_or(0)).send(out);
    }
}

/// One offer, made at the device's version; None, and no offer, when the
/// client's table is full.
fn offer_one(
    objects: &mut Objects,
    clip: &mut Clip,
    (which, device): (Which, u32),
    source: u32,
    out: &mut Vec<u8>,
) -> Option<u32> {
    let id = server_id(objects);
    if !objects.bound(id, which.offer(), objects.version(device)) {
        return None;
    }
    clip.offers.push((id, source));
    Event::new(device, which.device_events().0).u32(id).send(out);
    for mime in clip.mimes_of(source) {
        Event::new(id, which.offer_event()).string(&mime).send(out);
    }
    Some(id)
}
