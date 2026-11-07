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

//! The body: every linked node, then the objects to remove.
//!
//! ```text
//! 0   magic "NUFB"   4   version u16   6   pad u16
//! 8   next_ino u64   16  next_id u64   24  node count u32
//! per node: node_codec.rs
//! freed count u32, freed ids u64 each
//! ```

use alloc::vec::Vec;

use super::{node_codec, BODY_MAGIC, VERSION};
use crate::bytes::Out;
use crate::tree::Tree;

/// Where next_id is, so a commit can set it once it knows how many objects
/// the body itself takes.
pub const NEXT_ID_AT: usize = 16;

/// The body of `tree`, with `freed` the objects whoever loads it removes.
/// An unlinked node is left out: it is kept only while open.
pub fn encode(tree: &Tree, next_id: u64, freed: &[u64]) -> Vec<u8> {
    let mut o = Out(Vec::new());
    o.bytes(&BODY_MAGIC);
    o.u16(VERSION);
    o.u16(0);
    o.u64(tree.next_ino);
    o.u64(next_id);
    let linked = tree.nodes.iter().filter(|(_, n)| n.linked);
    o.u32(linked.clone().count() as u32);
    for (ino, n) in linked {
        node_codec::put(&mut o, *ino, n);
    }
    o.u32(freed.len() as u32);
    freed.iter().for_each(|id| o.u64(*id));
    o.0
}

/// The body of a first start's tree: the root and both empty folders.
pub fn first_body() -> Vec<u8> {
    encode(&Tree::fresh(0), super::FIRST_ID, &[])
}
