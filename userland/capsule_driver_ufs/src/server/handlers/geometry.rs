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

//! GEOMETRY: a UFS logical unit is soldered, so always present and fixed.
//! A write-protected one is only found out by the write it refuses (-30).

use crate::platform::Opened;
use crate::protocol::{encode_geometry, Request, GEOMETRY_LEN, MEDIA_PRESENT};
use crate::server::reply;
use crate::ufs::disk::Unit;

pub fn serve(o: &Opened, u: &Unit, req: &Request, tx: &mut [u8]) {
    let max_transfer = o.ufs.max_blocks(u) * u.block_len;
    let mut out = [0u8; GEOMETRY_LEN];
    encode_geometry(&mut out, u.blocks, u.block_len, max_transfer, MEDIA_PRESENT);
    reply::body(tx, req, &out);
}
