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

//! One round trip to net.anon's handle front.
//!
//! Streams there are keyed by the caller's pid and a stream id, so every
//! stream belongs to this capsule and a guest can only reach one through a
//! descriptor this capsule gave it.

use alloc::{vec, vec::Vec};

use nonos_libc::mk_ipc_call;

use super::anon_ops::HDR_LEN;
use super::anon_wire::{arrived, reply, request};

/// The status and body of the reply to `op`, with room for `want` bytes of
/// body; None when nothing came back or what came is not that reply.
pub fn call(port: u32, op: u16, body: &[u8], want: usize) -> Option<(u16, Vec<u8>)> {
    /* Nothing reaches net.anon from a family that holds a model. */
    if crate::linux::file::models::held() {
        return None;
    }
    let tx = request(op, body);
    let mut rx = vec![0u8; HDR_LEN + want];
    let n = mk_ipc_call(port as u64, tx.as_ptr(), tx.len(), rx.as_mut_ptr(), rx.len());
    let n = arrived(n, rx.len())?;
    let (status, out) = reply(rx.get(..n)?, op)?;
    Some((status, out.to_vec()))
}
