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

//! A request read from its bytes, each field held to its range. This is
//! what a client controls; nothing past it trusts a length it gave.

use super::header::Header;
use super::op;
use crate::bytes::In;
use crate::errno::{Errno, EBADMSG, ENOSYS};

#[derive(Debug, PartialEq, Eq)]
pub enum Request<'a> {
    Stat { path: &'a [u8] },
    Open { flags: u32, mode: u32, path: &'a [u8] },
    Close { handle: u64 },
    Read { handle: u64, offset: u64, len: u32 },
    Write { handle: u64, offset: u64, data: &'a [u8] },
    Fstat { handle: u64 },
    Ftruncate { handle: u64, size: u64 },
    Fsync { handle: u64 },
    Truncate { size: u64, path: &'a [u8] },
    Rename { flags: u32, from: &'a [u8], to: &'a [u8] },
    Unlink { path: &'a [u8] },
    Mkdir { mode: u32, path: &'a [u8] },
    Rmdir { path: &'a [u8] },
    List { cookie: u64, max: u32, path: &'a [u8] },
    Setattr { mask: u32, mode: u32, mtime: u64, path: &'a [u8] },
    Statfs,
}

impl<'a> Request<'a> {
    /// The request in `msg`, or the header and why it is refused: EBADMSG
    /// for fields that do not fit, ENOSYS for an op this wire lacks. None
    /// when there is no header to answer.
    pub fn read(msg: &'a [u8]) -> Option<(Header, Result<Request<'a>, Errno>)> {
        let (h, body) = Header::read(msg)?;
        Some((h, fields(h.op, body)))
    }
}

fn fields(op: u16, body: &[u8]) -> Result<Request<'_>, Errno> {
    let mut r = In::new(body);
    let bad = EBADMSG;
    let rest = |r: &In<'_>| &body[r.at..];
    let req = match op {
        op::STAT => Request::Stat { path: body },
        op::OPEN => {
            Request::Open { flags: r.u32().ok_or(bad)?, mode: r.u32().ok_or(bad)?, path: rest(&r) }
        }
        op::CLOSE => Request::Close { handle: r.u64().ok_or(bad)? },
        op::READ => Request::Read {
            handle: r.u64().ok_or(bad)?,
            offset: r.u64().ok_or(bad)?,
            len: r.u32().ok_or(bad)?,
        },
        op::WRITE => Request::Write {
            handle: r.u64().ok_or(bad)?,
            offset: r.u64().ok_or(bad)?,
            data: rest(&r),
        },
        op::FSTAT => Request::Fstat { handle: r.u64().ok_or(bad)? },
        op::FTRUNCATE => {
            Request::Ftruncate { handle: r.u64().ok_or(bad)?, size: r.u64().ok_or(bad)? }
        }
        op::FSYNC => Request::Fsync { handle: r.u64().ok_or(bad)? },
        op::TRUNCATE => Request::Truncate { size: r.u64().ok_or(bad)?, path: rest(&r) },
        op::RENAME => {
            let flags = r.u32().ok_or(bad)?;
            let len = r.u16().ok_or(bad)? as usize;
            let from = r.take(len).ok_or(bad)?;
            Request::Rename { flags, from, to: rest(&r) }
        }
        op::UNLINK => Request::Unlink { path: body },
        op::MKDIR => Request::Mkdir { mode: r.u32().ok_or(bad)?, path: rest(&r) },
        op::RMDIR => Request::Rmdir { path: body },
        op::LIST => {
            Request::List { cookie: r.u64().ok_or(bad)?, max: r.u32().ok_or(bad)?, path: rest(&r) }
        }
        op::SETATTR => Request::Setattr {
            mask: r.u32().ok_or(bad)?,
            mode: r.u32().ok_or(bad)?,
            mtime: r.u64().ok_or(bad)?,
            path: rest(&r),
        },
        op::STATFS if body.is_empty() => Request::Statfs,
        op::STATFS => return Err(bad),
        _ => return Err(ENOSYS),
    };
    // A fixed-size request with bytes past its fields was not written by a client of this wire.
    let fixed = matches!(op, op::CLOSE | op::READ | op::FSTAT | op::FTRUNCATE | op::FSYNC);
    if fixed && !r.done() {
        return Err(bad);
    }
    Ok(req)
}
