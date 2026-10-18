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
use crate::constants::VG_FORMAT_B8G8R8A8_UNORM;
use crate::driver::Driver;
use crate::protocol::{
    Request, E_BUSY, E_DEVICE, GET_PRIMARY_SURFACE_RESP_LEN, HDR_LEN, STATUS_LEN,
};
use crate::server::respond;
use nonos_libc::mk_surface_share;
use nonos_libc::surface_registry::SURFACE_SHARE_WRITE;
pub fn handle(driver: &Driver, sender_pid: u32, req: &Request, tx: &mut [u8]) {
    let Some(primary) = driver.primary.as_ref() else {
        respond::status(sender_pid, req, E_DEVICE, tx);
        return;
    };
    if !driver.resources.update(primary.resource_id, |r| {
        if r.owner_pid == 0 {
            r.owner_pid = sender_pid;
        }
    }) {
        respond::status(sender_pid, req, E_DEVICE, tx);
        return;
    }
    let Some(resource) = driver.resources.lookup(primary.resource_id) else {
        respond::status(sender_pid, req, E_DEVICE, tx);
        return;
    };
    if resource.owner_pid != sender_pid {
        respond::status(sender_pid, req, E_BUSY, tx);
        return;
    }
    // The compositor draws the screen into it: it alone maps it, writable,
    // by the handle this share answers.
    let handle = mk_surface_share(primary.sid, sender_pid, SURFACE_SHARE_WRITE);
    if handle <= 0 {
        respond::status(sender_pid, req, E_DEVICE, tx);
        return;
    }
    let off = HDR_LEN + STATUS_LEN;
    tx[off..off + 8].copy_from_slice(&(handle as u64).to_le_bytes());
    tx[off + 8..off + 12].copy_from_slice(&primary.resource_id.to_le_bytes());
    tx[off + 12..off + 16].copy_from_slice(&primary.width.to_le_bytes());
    tx[off + 16..off + 20].copy_from_slice(&primary.height.to_le_bytes());
    tx[off + 20..off + 24].copy_from_slice(&primary.stride.to_le_bytes());
    tx[off + 24..off + 28].copy_from_slice(&VG_FORMAT_B8G8R8A8_UNORM.to_le_bytes());
    tx[off + 28..off + 32].fill(0);
    respond::payload(sender_pid, req, GET_PRIMARY_SURFACE_RESP_LEN, tx);
}
