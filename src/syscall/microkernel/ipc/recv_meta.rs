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

//! MkIpcRecvReq's request meta, v1: what the server learns of a request
//! beside its bytes. 24 bytes, little-endian, as abi/wire.toml
//! [ipc.recv_meta_v1] pins it. Pure, so the host holds the layout to the
//! wire file (kernel_proofs::ipc_reply_token).

/// The only meta length MkIpcRecvReq accepts.
pub(crate) const META_V1_LEN: usize = 24;
/// The version a v1 meta carries in its first two bytes.
pub(crate) const META_V1: u16 = 1;

/// Whether a caller's `meta_len` is the v1 length: a caller built for any
/// other version is refused (EINVAL), never half-written.
pub(crate) fn meta_len_ok(meta_len: u64) -> bool {
    meta_len == META_V1_LEN as u64
}

/// The meta for a request from `sender` (0 for a kernel sender), queued
/// at `enqueued_ms` (MkTimeMonotonic ms), known to the server as
/// `request` (0 when no reply is owed).
pub(crate) fn meta_v1(sender: u32, enqueued_ms: u64, request: u64) -> [u8; META_V1_LEN] {
    let mut out = [0u8; META_V1_LEN];
    out[0..2].copy_from_slice(&META_V1.to_le_bytes());
    out[4..8].copy_from_slice(&sender.to_le_bytes());
    out[8..16].copy_from_slice(&enqueued_ms.to_le_bytes());
    out[16..24].copy_from_slice(&request.to_le_bytes());
    out
}
