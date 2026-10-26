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

use crate::syscall::{call_raw, N_MK_IPC_RECV_REQ};

/// What `mk_ipc_recv_req` tells a server of a request beside its bytes,
/// abi/wire.toml [ipc.recv_meta_v1]: 24 bytes, little-endian.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct IpcReqMeta {
    /// IPC_REQ_META_V1.
    pub version: u16,
    /// 0.
    pub reserved: u16,
    /// The caller's pid, 0 for a kernel sender.
    pub sender: u32,
    /// When the kernel queued the request, in `mk_uptime_ms` milliseconds.
    pub enqueued_ms: u64,
    /// The id `mk_ipc_reply_to` answers the request by; 0 when no reply is
    /// owed. Per server, from 1, never reused while the server lives.
    pub request: u64,
}

/// The meta version this libc reads.
pub const IPC_REQ_META_V1: u16 = 1;

const _: () = assert!(core::mem::size_of::<IpcReqMeta>() == 24);

// Dequeue one request, as `mk_ipc_recv_from_at`, and write its meta to
// `meta`. A server that takes several requests before it answers them
// answers each with `mk_ipc_reply_to(meta.request, ...)`, which reaches
// that request's caller and no other.
#[no_mangle]
pub extern "C" fn mk_ipc_recv_req(
    endpoint: u64,
    buf: *mut u8,
    len: usize,
    timeout_ms: u64,
    meta: *mut IpcReqMeta,
) -> i64 {
    let meta_len = core::mem::size_of::<IpcReqMeta>() as u64;
    call_raw(
        N_MK_IPC_RECV_REQ,
        [endpoint, buf as u64, len as u64, timeout_ms, meta as u64, meta_len],
    )
}
