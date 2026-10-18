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

//! GEOMETRY: what is in the slot now. A soldered part is always present,
//! fixed and writable; a card reader's slot says whether it holds a card
//! and whether the card's switch lets it be written.

use crate::emmc::{Opened, SECTOR_SIZE};
use crate::protocol::{
    encode_geometry, Request, GEOMETRY_LEN, MEDIA_PRESENT, READ_ONLY, REMOVABLE,
};
use crate::server::reply::body_reply;

pub fn serve(o: &mut Opened, req: &Request, tx: &mut [u8]) {
    let m = o.slot.describe();
    let flags = if m.present { MEDIA_PRESENT } else { 0 }
        | if m.removable { REMOVABLE } else { 0 }
        | if m.read_only { READ_ONLY } else { 0 };
    let max_transfer = o.slot.max_sectors() * SECTOR_SIZE as u32;
    let mut out = [0u8; GEOMETRY_LEN];
    encode_geometry(&mut out, m.sectors, SECTOR_SIZE as u32, max_transfer, flags);
    body_reply(tx, req, &out);
}
