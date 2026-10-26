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

//! What reply-twice's receive saw when it found none of its own requests
//! (reply_twice.rs): the call's return, and the sender, request id and
//! first byte of a message it passed over, so a boot log names the cause.

use nonos_libc::ipc::IpcReqMeta;

use crate::line::marker;

/// One receive that was not the awaited request: its return `rc`, and
/// for a message, what it carried.
pub fn seen(rc: i64, first: u8, meta: &IpcReqMeta) {
    if rc < 0 {
        return marker(
            b"[ATTACK-NOTE] reply-twice: MkIpcRecvReq errno -",
            rc.unsigned_abs(),
            b"\n",
        );
    }
    marker(b"[ATTACK-NOTE] reply-twice: passed over a message of ", rc as u64, b" bytes");
    marker(b" first byte ", u64::from(first), b"");
    marker(b" sender ", u64::from(meta.sender), b"");
    marker(b" request ", meta.request, b"\n");
}
