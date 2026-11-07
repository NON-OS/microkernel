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

//! A client of `net.sockets` (the socket multiplexer capsule). Every call is
//! one blocking IPC round trip in net.sockets' own v1 frame: a 20-byte
//! header (magic, version, op, errno, request id, payload length) and a
//! body. This capsule opens a stream socket, binds and listens, then accepts
//! children and reads and writes them. nsh.front is the first in-guest
//! listener, so this exercises net.sockets' OP_LISTEN/OP_ACCEPT path that
//! only its own tests have reached before.

use alloc::vec;
use alloc::vec::Vec;
use nonos_libc::{mk_ipc_call, mk_service_lookup};

use crate::accepted::accepted;
pub use crate::accepted::Accept;
use crate::read::{read_is, Read};

const MAGIC: u32 = 0x4E53_4B54;
const VERSION: u16 = 1;
const HDR: usize = 20;
const OP_SOCKET: u16 = 2;
const OP_BIND: u16 = 3;
const OP_LISTEN: u16 = 4;
const OP_ACCEPT: u16 = 5;
const OP_SEND: u16 = 7;
const OP_RECV: u16 = 8;
const OP_CLOSE: u16 = 9;
const OP_POLL: u16 = 13;
const E_OK: u16 = 0;
const FAMILY_V4: u16 = 4;
const KIND_STREAM: u16 = 1;
/// The most one read drains, matching net.sockets' own ceiling.
pub const RECV_MAX: usize = 32 * 1024;
/// The most one write carries; net.sockets buffers a request of header plus
/// this, well under its own limit, and the loop sends longer output in parts.
const SEND_MAX: usize = 16 * 1024;

/// A connected net.sockets endpoint, keyed by its service port.
pub struct Net {
    port: u64,
    rid: u32,
    resp: Vec<u8>,
}

/// The result of a read: bytes, nothing waiting, or the socket is gone.
pub enum Recv {
    Data(Vec<u8>),
    Idle,
    Gone,
}

impl Net {
    /// Resolve net.sockets by name, or `None` until it is up.
    pub fn resolve() -> Option<Net> {
        let name = b"net.sockets";
        let (mut port, mut pid) = (0u32, 0u32);
        if mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid) != 0 || port == 0 {
            return None;
        }
        Some(Net { port: port as u64, rid: 1, resp: vec![0u8; HDR + RECV_MAX + 64] })
    }

    /// One op: returns (errno, payload offset, payload length) in `self.resp`.
    fn op(&mut self, op: u16, body: &[u8]) -> Option<(u16, usize, usize)> {
        let mut req = vec![0u8; HDR + body.len()];
        req[0..4].copy_from_slice(&MAGIC.to_le_bytes());
        req[4..6].copy_from_slice(&VERSION.to_le_bytes());
        req[6..8].copy_from_slice(&op.to_le_bytes());
        req[12..16].copy_from_slice(&self.rid.to_le_bytes());
        req[16..20].copy_from_slice(&(body.len() as u32).to_le_bytes());
        req[HDR..].copy_from_slice(body);
        self.rid = self.rid.wrapping_add(1).max(1);
        let n = mk_ipc_call(
            self.port,
            req.as_ptr(),
            req.len(),
            self.resp.as_mut_ptr(),
            self.resp.len(),
        );
        if n < HDR as i64 {
            return None;
        }
        let r = crate::reply::written(&self.resp, n)?;
        if le32(r, 0) != MAGIC || le16(r, 4) != VERSION {
            return None;
        }
        let plen = usize::try_from(le32(r, 16)).ok()?;
        crate::reply::payload(r, HDR, plen)?;
        Some((le16(r, 8), HDR, plen))
    }

    /// Open a stream (TCP) socket; the handle, or `None`.
    pub fn socket_stream(&mut self) -> Option<u32> {
        let mut body = [0u8; 4];
        body[0..2].copy_from_slice(&FAMILY_V4.to_le_bytes());
        body[2..4].copy_from_slice(&KIND_STREAM.to_le_bytes());
        match self.op(OP_SOCKET, &body) {
            Some((E_OK, off, plen)) if plen >= 4 => Some(le32(&self.resp[off..], 0)),
            _ => None,
        }
    }

    /// Bind the socket to `ip:port`.
    pub fn bind(&mut self, handle: u32, ip: [u8; 4], port: u16) -> bool {
        let mut body = [0u8; 10];
        body[0..4].copy_from_slice(&handle.to_le_bytes());
        body[4..8].copy_from_slice(&ip);
        body[8..10].copy_from_slice(&port.to_le_bytes());
        matches!(self.op(OP_BIND, &body), Some((E_OK, _, _)))
    }

    /// Put a bound stream socket into the listening state.
    pub fn listen(&mut self, handle: u32) -> bool {
        matches!(self.op(OP_LISTEN, &handle.to_le_bytes()), Some((E_OK, _, _)))
    }

    /// Take one pending connection, if any.
    pub fn accept(&mut self, handle: u32) -> Accept {
        let answer = self
            .op(OP_ACCEPT, &handle.to_le_bytes())
            .map(|(errno, off, plen)| (errno, (plen >= 4).then(|| le32(&self.resp[off..], 0))));
        accepted(answer)
    }

    /// Read up to `RECV_MAX` from a child; `seq` numbers the read so a
    /// retried one is replayed rather than draining fresh bytes. An empty
    /// read asks the connection's POLL whether the client is quiet or gone
    /// (read.rs).
    pub fn recv(&mut self, handle: u32, seq: u32) -> Recv {
        let mut body = [0u8; 12];
        body[0..4].copy_from_slice(&handle.to_le_bytes());
        body[4..8].copy_from_slice(&seq.to_le_bytes());
        body[8..12].copy_from_slice(&(RECV_MAX as u32).to_le_bytes());
        let answer = self.op(OP_RECV, &body);
        let read = read_is(answer.map(|(errno, _, plen)| (errno, plen)), || self.poll(handle));
        match (read, answer) {
            (Read::Data, Some((_, off, plen))) => Recv::Data(self.resp[off..off + plen].to_vec()),
            (Read::Idle, _) => Recv::Idle,
            _ => Recv::Gone,
        }
    }

    /// The connection's POLL answer: its errno and, when the answer held
    /// one, its readiness bits.
    fn poll(&mut self, handle: u32) -> Option<(u16, Option<u8>)> {
        let (errno, off, plen) = self.op(OP_POLL, &handle.to_le_bytes())?;
        Some((errno, (plen >= 1).then(|| self.resp[off])))
    }

    /// Write every byte of `data`, in `SEND_MAX` parts; `false` on any error.
    pub fn send(&mut self, handle: u32, data: &[u8]) -> bool {
        for chunk in data.chunks(SEND_MAX) {
            let mut body = vec![0u8; 4 + chunk.len()];
            body[0..4].copy_from_slice(&handle.to_le_bytes());
            body[4..].copy_from_slice(chunk);
            if !matches!(self.op(OP_SEND, &body), Some((E_OK, _, _))) {
                return false;
            }
        }
        true
    }

    /// Close a socket handle; errors are ignored, the handle is done either way.
    pub fn close(&mut self, handle: u32) {
        let _ = self.op(OP_CLOSE, &handle.to_le_bytes());
    }
}

fn le16(buf: &[u8], off: usize) -> u16 {
    u16::from_le_bytes([buf[off], buf[off + 1]])
}

fn le32(buf: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([buf[off], buf[off + 1], buf[off + 2], buf[off + 3]])
}
