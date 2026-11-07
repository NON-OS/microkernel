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

//! A journal record: what one commit changed since the body or the record
//! before it, so a commit writes what changed, not the whole tree.
//!
//! ```text
//! 0 magic "NUFJ"  4 version u16  6 pad u16  8 next_ino u64  16 next_id u64
//! 24 node count u32, nodes (node_codec.rs), in rising order
//! removed count u32, inos u64 each, rising
//! freed count u32, ids u64 each
//! ```

use alloc::vec::Vec;

use super::decode::Parsed;
use super::{node_codec, VERSION};
use crate::bytes::{In, Out};
use crate::limits::MAX_NODES;
use crate::node::Node;

pub(crate) const JOURNAL_MAGIC: [u8; 4] = *b"NUFJ";

/// Where next_id is, set once the record knows its own number.
pub const RECORD_NEXT_ID_AT: usize = 16;

pub fn record<'a>(
    next_ino: u64,
    changed: impl Iterator<Item = (u64, &'a Node)>,
    removed: &[u64],
    freed: &[u64],
) -> Vec<u8> {
    let mut o = Out(Vec::new());
    o.bytes(&JOURNAL_MAGIC);
    o.u16(VERSION);
    o.u16(0);
    o.u64(next_ino);
    o.u64(0);
    let at = o.0.len();
    o.u32(0);
    let mut count = 0u32;
    for (ino, n) in changed {
        node_codec::put(&mut o, ino, n);
        count += 1;
    }
    o.0[at..at + 4].copy_from_slice(&count.to_le_bytes());
    o.u32(removed.len() as u32);
    removed.iter().for_each(|ino| o.u64(*ino));
    o.u32(freed.len() as u32);
    freed.iter().for_each(|id| o.u64(*id));
    o.0
}

/// Applies a record to `p`; None for anything `record` did not write.
pub fn apply(p: &mut Parsed, bytes: &[u8]) -> Option<()> {
    let mut r = In::new(bytes);
    if r.take(4)? != JOURNAL_MAGIC || r.u16()? != VERSION || r.u16()? != 0 {
        return None;
    }
    p.next_ino = r.u64()?;
    p.next_id = r.u64()?;
    let count = r.u32()? as usize;
    if count > MAX_NODES {
        return None;
    }
    let mut last = None;
    for _ in 0..count {
        let (ino, node) = node_codec::take(&mut r)?;
        if last.is_some_and(|l| l >= ino) {
            return None;
        }
        last = Some(ino);
        p.nodes.insert(ino, node);
    }
    let removed = node_codec::ids(&mut r, bytes.len())?;
    if removed.windows(2).any(|w| w[0] >= w[1]) {
        return None;
    }
    for ino in removed {
        p.nodes.remove(&ino)?;
    }
    p.freed = node_codec::ids(&mut r, bytes.len())?;
    r.done().then_some(())
}
