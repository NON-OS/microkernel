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
use crate::device::cmd;
use crate::driver::Driver;
use crate::protocol::{
    le_u32, Request, CREATE_RESOURCE_REQ_LEN, E_BUSY, E_DEVICE, E_INVAL, E_NOMEM, HDR_LEN,
    STATUS_LEN,
};
use crate::server::respond;
use crate::state::Resource;
pub fn handle(driver: &Driver, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if body.len() != CREATE_RESOURCE_REQ_LEN {
        respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    let fields = (le_u32(body, 0), le_u32(body, 4), le_u32(body, 8), le_u32(body, 12));
    let (Some(requested_id), Some(format), Some(width), Some(height)) = fields else {
        respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    if width == 0 || height == 0 || format != VG_FORMAT_B8G8R8A8_UNORM {
        respond::status(sender_pid, req, E_INVAL, tx);
        return;
    }
    // An id the table holds is the primary's, the blank's or one in use.
    if requested_id != 0 && driver.resources.lookup(requested_id).is_some() {
        respond::status(sender_pid, req, E_BUSY, tx);
        return;
    }
    let resource_id = if requested_id == 0 { driver.resources.alloc_id() } else { requested_id };
    let fence_id = driver.fences.issue();
    let q = &driver.control_queue;
    if cmd::create_resource_2d(q, fence_id, resource_id, format, width, height).is_err() {
        respond::status(sender_pid, req, E_DEVICE, tx);
        return;
    }
    let r = Resource {
        resource_id,
        owner_pid: sender_pid,
        width,
        height,
        format,
        backing_addr: 0,
        backing_len: 0,
        in_use: true,
    };
    if driver.resources.insert(r).is_err() {
        respond::status(sender_pid, req, E_NOMEM, tx);
        return;
    }
    let body_off = HDR_LEN + STATUS_LEN;
    tx[body_off..body_off + 4].copy_from_slice(&resource_id.to_le_bytes());
    respond::payload(sender_pid, req, 4, tx);
}
