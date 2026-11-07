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

//! Reading a reply's body on the client side.

use alloc::vec::Vec;

use crate::bytes::In;
use crate::errno::{Errno, EBADMSG};
use crate::node::{Attr, Kind};

/// A listing: names with their numbers and kinds, the next cookie, and
/// whether there are no more.
pub struct Listed {
    pub entries: Vec<(u64, Kind, Vec<u8>)>,
    pub next: u64,
    pub end: bool,
}

pub struct Totals {
    pub quota: u64,
    pub used: u64,
    pub nodes: u64,
    pub max_nodes: u64,
    pub chunk: u32,
}

fn kind(b: u8) -> Result<Kind, Errno> {
    match b {
        0 => Ok(Kind::Dir),
        1 => Ok(Kind::File),
        _ => Err(EBADMSG),
    }
}

pub fn attr(body: &[u8]) -> Result<Attr, Errno> {
    let mut r = In::new(body);
    let mut a = || -> Option<Attr> {
        Some(Attr {
            ino: r.u64()?,
            kind: kind(r.u8()?).ok()?,
            mode: r.u32()?,
            size: r.u64()?,
            mtime: r.u64()?,
            ctime: r.u64()?,
            nlink: r.u32()?,
        })
    };
    a().ok_or(EBADMSG)
}

pub fn listed(body: &[u8]) -> Result<Listed, Errno> {
    let mut r = In::new(body);
    let mut go = || -> Option<Listed> {
        let next = r.u64()?;
        let end = r.u8()? != 0;
        let count = r.u32()? as usize;
        let mut entries = Vec::new();
        for _ in 0..count.min(crate::limits::LIST_MAX) {
            let ino = r.u64()?;
            let kind = kind(r.u8()?).ok()?;
            let len = r.u8()? as usize;
            entries.push((ino, kind, r.take(len)?.to_vec()));
        }
        (entries.len() == count && r.done()).then_some(Listed { entries, next, end })
    };
    go().ok_or(EBADMSG)
}

pub fn totals(body: &[u8]) -> Result<Totals, Errno> {
    let mut r = In::new(body);
    let mut go = || -> Option<Totals> {
        Some(Totals {
            quota: r.u64()?,
            used: r.u64()?,
            nodes: r.u64()?,
            max_nodes: r.u64()?,
            chunk: r.u32()?,
        })
    };
    go().ok_or(EBADMSG)
}
