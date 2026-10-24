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

use alloc::vec::Vec;
use core::str;

use nonos_libc::mk_debug;

use super::installer_gate::require_installer;
use super::path::normalize;
use super::persist_gate::may_persist;
use super::util::{map_blk_err, split_caller};
use crate::protocol::{encode_response, Request, EACCES, EINVAL, MAX_PATH_BYTES, OP_STORE_REMOVE};

const REFUSED: &[u8] = b"[VFS] refused store remove: amnesic boot\n";

pub fn store_remove(req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    if let Err(s) = require_installer(sender_pid) {
        return encode_response(OP_STORE_REMOVE, req.flags, req.request_id, s, &[]);
    }
    let (_pid, rest) = match split_caller(req.payload, sender_pid) {
        Ok(v) => v,
        Err(s) => return encode_response(OP_STORE_REMOVE, req.flags, req.request_id, s, &[]),
    };
    if rest.is_empty() {
        return encode_response(OP_STORE_REMOVE, req.flags, req.request_id, EINVAL, &[]);
    }
    let len = rest[0] as usize;
    if len == 0 || len > MAX_PATH_BYTES as usize || rest.len() < 1 + len {
        return encode_response(OP_STORE_REMOVE, req.flags, req.request_id, EINVAL, &[]);
    }
    let path = match str::from_utf8(&rest[1..1 + len]) {
        Ok(s) => s,
        Err(_) => return encode_response(OP_STORE_REMOVE, req.flags, req.request_id, EINVAL, &[]),
    };
    let Some(path) = normalize(path) else {
        return encode_response(OP_STORE_REMOVE, req.flags, req.request_id, EINVAL, &[]);
    };
    /* A boot that keeps nothing leaves the disk as it found it. */
    let status = if !may_persist() {
        let _ = mk_debug(REFUSED.as_ptr(), REFUSED.len());
        EACCES
    } else {
        crate::blk::store_remove::remove(&path).map_or_else(map_blk_err, |()| 0)
    };
    encode_response(OP_STORE_REMOVE, req.flags, req.request_id, status, &[])
}
