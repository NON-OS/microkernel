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

use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL};

use super::recv_from_deliver::Outs;
use super::recv_from_drain::drain;
use super::recv_meta::{meta_len_ok, META_V1_LEN};
use super::recv_open::{buffer, inbox};

/// `MkIpcRecvReq`: MkIpcRecvFrom's receive, with the request's meta v1
/// (recv_meta.rs) written to `meta_ptr`: the sender, when it was queued,
/// and the id the server answers it by with MkIpcReplyTo, 0 when no reply
/// is owed. EINVAL unless `meta_len` is the v1 length, so a caller built
/// for another version is refused rather than half-written; EFAULT when
/// the meta is not writable.
pub fn sys_ipc_recv_req(
    endpoint: u64,
    buf: u64,
    len: usize,
    timeout_ms: u64,
    meta_ptr: u64,
    meta_len: u64,
) -> i64 {
    if !meta_len_ok(meta_len) {
        return ERRNO_INVAL;
    }
    if let Err(e) = buffer(buf, len) {
        return e;
    }
    if crate::usercopy::validate_user_write(meta_ptr, META_V1_LEN).is_err() {
        return ERRNO_FAULT;
    }
    match inbox(endpoint) {
        Ok(name) => drain((buf, len), timeout_ms, Outs::Meta(meta_ptr), &name),
        Err(e) => e,
    }
}
