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

//! The client side: a request for each op, and its reply read back, over
//! whatever carries messages (`Transport`). capsule_linux and native apps
//! both ask through this.

use alloc::vec;
use alloc::vec::Vec;

use super::header::{Header, HEADER};
use super::{op, MESSAGE_MAX};
use crate::bytes::In;
use crate::errno::{Errno, EBADMSG, EIO};
use crate::node::Attr;

pub use super::client_read::{Listed, Totals};

/// One request out and its reply back, into `reply`; the reply's length.
pub trait Transport {
    fn call(&mut self, request: &[u8], reply: &mut [u8]) -> Result<usize, Errno>;
}

pub struct Client<T: Transport> {
    pub transport: T,
    next: u32,
    reply: Vec<u8>,
}

impl<T: Transport> Client<T> {
    pub fn new(transport: T) -> Client<T> {
        Client { transport, next: 0, reply: vec![0u8; MESSAGE_MAX] }
    }

    /// Sends `op` with `fields` then `tail`, and returns the reply's body
    /// after its status, or the errno it carried.
    pub fn ask(&mut self, op: u16, fields: &[u8], tail: &[u8]) -> Result<&[u8], Errno> {
        self.next = self.next.wrapping_add(1);
        let id = self.next;
        let body_len = (fields.len() + tail.len()) as u32;
        let mut msg = Vec::with_capacity(HEADER + body_len as usize);
        msg.resize(HEADER, 0);
        let mut head = [0u8; HEADER];
        Header { op, id, body_len }.write(&mut head);
        msg[..HEADER].copy_from_slice(&head);
        msg.extend_from_slice(fields);
        msg.extend_from_slice(tail);
        let n = self.transport.call(&msg, &mut self.reply)?;
        let (h, body) = Header::read(self.reply.get(..n).ok_or(EIO)?).ok_or(EBADMSG)?;
        if h.op != op || h.id != id {
            return Err(EBADMSG);
        }
        let mut r = In::new(body);
        let status = r.u32().ok_or(EBADMSG)? as i32;
        if status != 0 {
            return Err(status.checked_neg().filter(|e| *e > 0).ok_or(EBADMSG)?);
        }
        Ok(&body[4..])
    }

    pub fn stat(&mut self, path: &[u8]) -> Result<Attr, Errno> {
        super::client_read::attr(self.ask(op::STAT, &[], path)?)
    }

    /// A handle and what it opened. `flags` are wire::flags.
    pub fn open(&mut self, path: &[u8], flags: u32, mode: u32) -> Result<(u64, Attr), Errno> {
        let body = self.ask(op::OPEN, &cat(&[&flags.to_le_bytes(), &mode.to_le_bytes()]), path)?;
        let mut r = In::new(body);
        let handle = r.u64().ok_or(EBADMSG)?;
        Ok((handle, super::client_read::attr(&body[8..])?))
    }

    pub fn close(&mut self, handle: u64) -> Result<(), Errno> {
        self.ask(op::CLOSE, &handle.to_le_bytes(), &[]).map(drop)
    }

    /// Up to `len` bytes into `buf`; how many came, 0 at the end.
    pub fn read(&mut self, handle: u64, offset: u64, buf: &mut [u8]) -> Result<usize, Errno> {
        let len = buf.len().min(crate::limits::IO_MAX) as u32;
        let f = cat(&[&handle.to_le_bytes(), &offset.to_le_bytes(), &len.to_le_bytes()]);
        let body = self.ask(op::READ, &f, &[])?;
        let n = body.len().min(buf.len());
        buf[..n].copy_from_slice(&body[..n]);
        Ok(n)
    }

    /// Writes up to IO_MAX of `data`; what landed and the size after.
    pub fn write(&mut self, handle: u64, offset: u64, data: &[u8]) -> Result<(usize, u64), Errno> {
        let data = &data[..data.len().min(crate::limits::IO_MAX)];
        let body =
            self.ask(op::WRITE, &cat(&[&handle.to_le_bytes(), &offset.to_le_bytes()]), data)?;
        let mut r = In::new(body);
        let n = r.u32().ok_or(EBADMSG)? as usize;
        Ok((n, r.u64().ok_or(EBADMSG)?))
    }

    pub fn fstat(&mut self, handle: u64) -> Result<Attr, Errno> {
        super::client_read::attr(self.ask(op::FSTAT, &handle.to_le_bytes(), &[])?)
    }

    pub fn ftruncate(&mut self, handle: u64, size: u64) -> Result<(), Errno> {
        self.ask(op::FTRUNCATE, &cat(&[&handle.to_le_bytes(), &size.to_le_bytes()]), &[]).map(drop)
    }

    pub fn fsync(&mut self, handle: u64) -> Result<(), Errno> {
        self.ask(op::FSYNC, &handle.to_le_bytes(), &[]).map(drop)
    }
}

/// Fields laid end to end.
pub(super) fn cat(parts: &[&[u8]]) -> Vec<u8> {
    parts.concat()
}
