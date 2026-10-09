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

//! The display socket: connect, send, send with a descriptor, receive.

use crate::sys::{call, READ, WRITE};

const SOCKET: u64 = 41;
const CONNECT: u64 = 42;
const SENDMSG: u64 = 46;
const AF_UNIX: u64 = 1;
const SOCK_STREAM: u64 = 1;

pub struct Conn {
    fd: u64,
    rx: Vec<u8>,
}

impl Conn {
    pub fn connect(path: &[u8]) -> Option<Conn> {
        let fd = call(SOCKET, [AF_UNIX, SOCK_STREAM, 0, 0, 0, 0]);
        let mut addr = [0u8; 110];
        addr[..2].copy_from_slice(&(AF_UNIX as u16).to_le_bytes());
        addr[2..2 + path.len()].copy_from_slice(path);
        let rc = call(CONNECT, [fd as u64, addr.as_ptr() as u64, 110, 0, 0, 0]);
        (fd >= 0 && rc == 0).then(|| Conn { fd: fd as u64, rx: Vec::new() })
    }

    pub fn send(&self, bytes: &[u8]) -> bool {
        call(WRITE, [self.fd, bytes.as_ptr() as u64, bytes.len() as u64, 0, 0, 0]) as usize
            == bytes.len()
    }

    // SCM_RIGHTS: cmsg_len 20 (header 16 + one int), level and type 1, padded to 24.
    pub fn send_fd(&self, bytes: &[u8], fd: i32) -> bool {
        let iov = [bytes.as_ptr() as u64, bytes.len() as u64];
        let mut cmsg = [0u8; 24];
        cmsg[..8].copy_from_slice(&20u64.to_le_bytes());
        cmsg[8..12].copy_from_slice(&1i32.to_le_bytes());
        cmsg[12..16].copy_from_slice(&1i32.to_le_bytes());
        cmsg[16..20].copy_from_slice(&fd.to_le_bytes());
        let hdr = [0u64, 0, iov.as_ptr() as u64, 1, cmsg.as_ptr() as u64, 24, 0];
        call(SENDMSG, [self.fd, hdr.as_ptr() as u64, 0, 0, 0, 0]) as usize == bytes.len()
    }

    /// The next whole event, reading more when the buffer holds only part.
    pub fn event(&mut self) -> Option<(u32, u16, Vec<u8>)> {
        loop {
            if let Some((object, opcode, body, size)) = super::msg::split(&self.rx) {
                let out = (object, opcode, body.to_vec());
                self.rx.drain(..size);
                return Some(out);
            }
            let mut buf = [0u8; 4096];
            let n = call(READ, [self.fd, buf.as_mut_ptr() as u64, 4096, 0, 0, 0]);
            if n <= 0 {
                return None;
            }
            self.rx.extend_from_slice(&buf[..n as usize]);
        }
    }
}
