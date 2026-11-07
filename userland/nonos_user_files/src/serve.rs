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

//! One request in, one reply out.

use alloc::vec::Vec;

use crate::broker::Broker;
use crate::errno::{Errno, EACCES, EBADMSG};
use crate::host::{Host, Objects};
use crate::limits::CHUNK;
use crate::wire::{attr_bytes, Reply, Request};

/// The reply to `msg` from `sender`. A sender the host does not let use
/// the broker is refused with EACCES before its request is read further
/// than its header; a message with no header is answered EBADMSG under op
/// and id 0.
pub fn serve<O: Objects>(b: &mut Broker<O>, host: &impl Host, sender: u32, msg: &[u8]) -> Vec<u8> {
    let Some((h, req)) = Request::read(msg) else {
        return Reply::fail(0, 0, EBADMSG).0;
    };
    if !host.may_use(sender) {
        return Reply::fail(h.op, h.id, EACCES).0;
    }
    b.clock(host.mono_ns());
    let mut reply = Reply::ok(h.op, h.id);
    let done = req.and_then(|req| answer(b, sender, host.now_ns(), req, &mut reply));
    match done {
        Ok(()) => reply.0,
        Err(e) => Reply::fail(h.op, h.id, e).0,
    }
}

fn answer<O: Objects>(
    b: &mut Broker<O>,
    who: u32,
    now: u64,
    req: Request,
    out: &mut Reply,
) -> Result<(), Errno> {
    match req {
        Request::Stat { path } => {
            out.put(&attr_bytes(&b.stat(path, now)?));
        }
        Request::Open { flags, mode, path } => {
            let (handle, attr) = b.open(who, path, flags, mode, now)?;
            out.put(&handle.to_le_bytes()).put(&attr_bytes(&attr));
        }
        Request::Close { handle } => b.close(who, handle, now)?,
        Request::Read { handle, offset, len } => {
            out.put(&b.read(who, handle, offset, len as usize, now)?);
        }
        Request::Write { handle, offset, data } => {
            let (n, size) = b.write(who, handle, offset, data, now)?;
            out.put(&(n as u32).to_le_bytes()).put(&size.to_le_bytes());
        }
        Request::Fstat { handle } => {
            out.put(&attr_bytes(&b.fstat(who, handle, now)?));
        }
        Request::Ftruncate { handle, size } => b.ftruncate(who, handle, size, now)?,
        Request::Fsync { handle } => b.fsync(who, handle, now)?,
        Request::Truncate { size, path } => b.truncate(path, size, now)?,
        Request::Rename { flags, from, to } => b.rename(from, to, flags, now)?,
        Request::Unlink { path } => b.unlink(path, now)?,
        Request::Mkdir { mode, path } => {
            out.put(&attr_bytes(&b.mkdir(path, mode, now)?));
        }
        Request::Rmdir { path } => b.rmdir(path, now)?,
        Request::List { cookie, max, path } => list(b, path, cookie, max, now, out)?,
        Request::Setattr { mask, mode, mtime, path } => {
            out.put(&attr_bytes(&b.setattr(path, mask, mode, mtime, now)?));
        }
        Request::Statfs => {
            let t = b.statfs(now)?;
            for v in [t.quota, t.used, t.nodes, t.max_nodes] {
                out.put(&v.to_le_bytes());
            }
            out.put(&(CHUNK as u32).to_le_bytes());
        }
    }
    Ok(())
}

fn list<O: Objects>(
    b: &mut Broker<O>,
    path: &[u8],
    cookie: u64,
    max: u32,
    now: u64,
    out: &mut Reply,
) -> Result<(), Errno> {
    let (entries, next, end) = b.list(path, cookie, max as usize, now)?;
    out.put(&next.to_le_bytes()).put(&[u8::from(end)]).put(&(entries.len() as u32).to_le_bytes());
    for e in entries {
        // A name is at most NAME_MAX (255) bytes, so its length fits a byte.
        out.put(&e.ino.to_le_bytes())
            .put(&[crate::wire::kind_byte(e.kind), e.name.len() as u8])
            .put(&e.name);
    }
    Ok(())
}
