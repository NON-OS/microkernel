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

//! GET_PRIMARY_SURFACE: the shadow's handle and the mode, laid out as
//! driver.virtio_gpu0 lays out its primary surface.

use nonos_libc::SURFACE_FORMAT_ARGB8888;

use crate::gfx::RESOURCE_ID;
use crate::protocol::{Request, GET_PRIMARY_SURFACE_RESP_LEN, HDR_LEN, STATUS_LEN};
use crate::server::respond;
use crate::setup::Driver;

pub fn primary(d: &mut Driver, sender: u32, req: &Request, tx: &mut [u8]) {
    if let Err(e) = d.screen.take(sender) {
        return respond::status(sender, req, e, tx);
    }
    let m = d.mode;
    let words = [m.width, m.height, m.stride, SURFACE_FORMAT_ARGB8888, 0];
    let out = &mut tx[HDR_LEN + STATUS_LEN..][..GET_PRIMARY_SURFACE_RESP_LEN];
    out[..8].copy_from_slice(&d.shadow.handle.to_le_bytes());
    out[8..12].copy_from_slice(&RESOURCE_ID.to_le_bytes());
    for (i, w) in words.iter().enumerate() {
        out[12 + i * 4..16 + i * 4].copy_from_slice(&w.to_le_bytes());
    }
    respond::payload(sender, req, GET_PRIMARY_SURFACE_RESP_LEN, tx)
}
