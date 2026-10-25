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

mod artifacts;
mod own_caps;

use alloc::vec::Vec;

use nonos_libc::{mk_capsule_load, mk_getpid, CapsuleLoadRequest};

use crate::protocol::{encode_response, Request, EINVAL};
use artifacts::Artifacts;

// Request payload: requested_caps(8) | name_len(1) | name | args.
//
// The caller hands us a store name, not the bytes: we read
// /capsules/<name>.{elf,nonos_id_cert.bin,manifest.bin,zk_trailer.bin} and pass
// them to the load syscall, which runs the full trust chain. A requested_caps
// of 0 asks for the manifest's own set (own_caps.rs); the kernel refuses any
// request beyond the manifest. `sender_pid` is the kernel-attested sender
// (`mk_ipc_recv_from`), never a payload field. It becomes the child's parent,
// so the kernel bounds the child by the sender's capabilities, not this
// installer's, and the sender can `mk_wait`/`mk_kill` the child it asked for.
pub fn load_by_name(req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    let p = req.payload;
    if p.len() < 9 {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let caps = u64::from_le_bytes([p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7]]);
    let name_len = p[8] as usize;
    let Some(name_end) = 9usize.checked_add(name_len) else {
        return encode_response(req.seq, EINVAL, &[]);
    };
    if p.len() < name_end {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let name = &p[9..name_end];
    let args = &p[name_end..];
    if !valid_name(name) {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let Some(artifacts) = Artifacts::read(mk_getpid(), name) else {
        return encode_response(req.seq, EINVAL, &[]);
    };
    let Some(caps) = own_caps::asked(caps, &artifacts.manifest) else {
        return encode_response(req.seq, EINVAL, &[]);
    };
    let load = artifacts.request(caps, args, sender_pid);
    // The blobs in `artifacts` stay owned by this stack frame until the
    // syscall returns, so the kernel's copy reads live memory.
    let rc = mk_capsule_load(&load as *const CapsuleLoadRequest);
    if rc < 0 {
        return encode_response(req.seq, rc as i32, &[]);
    }
    encode_response(req.seq, 0, &(rc as u32).to_le_bytes())
}

pub(super) fn valid_name(name: &[u8]) -> bool {
    !name.is_empty()
        && name.len() <= 64
        && name.iter().all(|&b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
}
