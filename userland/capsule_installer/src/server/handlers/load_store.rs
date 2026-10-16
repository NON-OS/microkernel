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

mod layout;

use alloc::vec::Vec;

use crate::protocol::{encode_response, Request, EINVAL};
use layout::{Layout, HEAD_LEN};
use nonos_libc::{mk_capsule_load, CapsuleLoadRequest};

// on_behalf_of below is intentionally left 0 (unattributed): this handler is
// the direct VFS-store load path, not the broker-attributed spawn path.
pub fn load_store(req: Request<'_>) -> Vec<u8> {
    let p = req.payload;
    let Some(l) = Layout::parse(p) else {
        return encode_response(req.seq, EINVAL, &[]);
    };
    let load = CapsuleLoadRequest {
        elf_ptr: p[HEAD_LEN..l.after_elf].as_ptr() as u64,
        cert_ptr: p[l.after_elf..l.after_cert].as_ptr() as u64,
        manifest_ptr: p[l.after_cert..l.after_manifest].as_ptr() as u64,
        trailer_ptr: p[l.after_manifest..l.after_trailer].as_ptr() as u64,
        requested_caps: u64::from_le_bytes([p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7]]),
        args_ptr: arg_ptr(p, l.after_trailer, l.total),
        elf_len: l.elf_len as u32,
        cert_len: l.cert_len as u32,
        manifest_len: l.manifest_len as u32,
        trailer_len: l.trailer_len as u32,
        args_len: l.args_len as u32,
        on_behalf_of: 0,
    };
    let rc = mk_capsule_load(&load as *const CapsuleLoadRequest);
    if rc < 0 {
        return encode_response(req.seq, rc as i32, &[]);
    }
    encode_response(req.seq, 0, &(rc as u32).to_le_bytes())
}

fn arg_ptr(p: &[u8], start: usize, total: usize) -> u64 {
    if start == total {
        0
    } else {
        p[start..total].as_ptr() as u64
    }
}
