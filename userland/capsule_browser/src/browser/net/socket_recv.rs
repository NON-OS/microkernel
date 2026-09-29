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

use alloc::vec;

use super::ask::{ask, Fault};
use super::constants::{OP_RECV, SOCKETS_MAGIC};
use super::recv_kind::Recv;

const RECV_TIMEOUT_MS: u64 = 200;

/// The most one read asks for: what one reply from net.sockets can carry.
const RECV_CHUNK: usize = 32 * 1024;

/// Read into `out` from `handle`: first what an earlier read left over,
/// then one exchange with net.sockets, whose surplus is kept for next time.
///
/// net.sockets answers an empty socket with a status rather than zero bytes,
/// so a status is `Empty`; only a reply that never came is `Lost`.
pub fn socket_recv(sockets_port: u32, handle: u32, out: &mut [u8]) -> Recv {
    if super::mixnet::is_on() {
        return match super::mixnet::recv(out) {
            Ok(n) if n > 0 => Recv::Bytes(n),
            _ => Recv::Empty,
        };
    }
    let held = super::recv_pending::take(handle, out);
    if held > 0 {
        return Recv::Bytes(held);
    }
    let mut body = [0u8; 12];
    body[0..4].copy_from_slice(&handle.to_le_bytes());
    body[4..8].copy_from_slice(&super::recv_seq::current(handle).to_le_bytes());
    body[8..12].copy_from_slice(&(RECV_CHUNK as u32).to_le_bytes());
    let mut rx = vec![0u8; RECV_CHUNK + 20];
    let n = match ask(sockets_port, SOCKETS_MAGIC, OP_RECV, &body, &mut rx, RECV_TIMEOUT_MS) {
        Ok(n) => n,
        Err(Fault::Lost) => return Recv::Lost,
        Err(Fault::Status(_) | Fault::Garbled) => return Recv::Empty,
    };
    let payload = u32::from_le_bytes([rx[16], rx[17], rx[18], rx[19]]) as usize;
    let got = &rx[20..20 + payload.min(n - 20)];
    if got.is_empty() {
        return Recv::Empty;
    }
    super::recv_seq::answered(handle);
    let copy_len = got.len().min(out.len());
    out[..copy_len].copy_from_slice(&got[..copy_len]);
    super::recv_pending::put(handle, &got[copy_len..]);
    Recv::Bytes(copy_len)
}
