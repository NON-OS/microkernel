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

//! One node as bytes, the same in a body and in a journal record:
//! ino u64, parent u64, kind u8 (0 dir, 1 file), mode u32, size u64,
//! mtime u64, ctime u64, name_len u16, name, chunk count u32, chunk ids u64.

use alloc::vec::Vec;

use crate::bytes::{In, Out};
use crate::limits::MAX_FILE;
use crate::node::{chunks_for, Kind, Node};

pub fn put(o: &mut Out, ino: u64, n: &Node) {
    o.u64(ino);
    o.u64(n.parent);
    o.u8(match n.kind {
        Kind::Dir => 0,
        Kind::File => 1,
    });
    o.u32(n.mode);
    o.u64(n.size);
    o.u64(n.mtime);
    o.u64(n.ctime);
    o.u16(n.name.len() as u16);
    o.bytes(&n.name);
    o.u32(n.chunks.len() as u32);
    n.chunks.iter().for_each(|id| o.u64(*id));
}

/// One node, every field held to its range; None for anything `put` did
/// not write.
pub fn take(r: &mut In) -> Option<(u64, Node)> {
    let ino = r.u64()?;
    let parent = r.u64()?;
    let kind = match r.u8()? {
        0 => Kind::Dir,
        1 => Kind::File,
        _ => return None,
    };
    let mode = r.u32()?;
    let size = r.u64()?;
    let mtime = r.u64()?;
    let ctime = r.u64()?;
    let name_len = r.u16()? as usize;
    let name = r.take(name_len)?.to_vec();
    let chunk_count = r.u32()? as usize;
    if mode > 0o7777 || size > MAX_FILE || chunk_count != chunks_for(size) {
        return None;
    }
    if kind == Kind::Dir && size != 0 {
        return None;
    }
    let chunks = (0..chunk_count).map(|_| r.u64()).collect::<Option<Vec<u64>>>()?;
    let mut node = Node::new(kind, mode, parent, &name, mtime);
    (node.size, node.ctime, node.chunks) = (size, ctime, chunks);
    Some((ino, node))
}

/// What a commit compares to tell a changed node: the first 16 bytes of
/// the BLAKE3 of its bytes.
pub fn print(ino: u64, n: &Node) -> [u8; 16] {
    let mut o = Out(Vec::new());
    put(&mut o, ino, n);
    let mut out = [0u8; 16];
    out.copy_from_slice(&blake3::hash(&o.0).as_bytes()[..16]);
    out
}

/// A count of eight-byte numbers no longer than what is left of `r`.
pub fn ids(r: &mut In, total: usize) -> Option<Vec<u64>> {
    let count = r.u32()? as usize;
    if count > (total - r.at) / 8 {
        return None;
    }
    (0..count).map(|_| r.u64()).collect()
}
