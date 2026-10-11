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

//! GEOMETRY: the soldered part is always there, fixed and writable.

use crate::emmc::{Opened, SECTOR_SIZE};
use crate::protocol::{encode_geometry, Request, GEOMETRY_LEN, MEDIA_PRESENT};
use crate::server::reply::body_reply;

pub fn serve(o: &mut Opened, req: &Request, tx: &mut [u8]) {
    let mut out = [0u8; GEOMETRY_LEN];
    let max_transfer = o.disk.max_sectors() * SECTOR_SIZE as u32;
    encode_geometry(
        &mut out,
        o.disk.capacity_sectors(),
        SECTOR_SIZE as u32,
        max_transfer,
        MEDIA_PRESENT,
    );
    body_reply(tx, req, &out);
}
