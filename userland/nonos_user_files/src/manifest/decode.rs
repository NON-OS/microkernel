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

//! A body read back, every field held to its range; journal records are
//! applied to it (journal.rs), and `finish` then holds the whole to being
//! a tree.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use super::{node_codec, BODY_MAGIC, VERSION};
use crate::bytes::In;
use crate::limits::MAX_NODES;
use crate::node::Node;
use crate::tree::Tree;

pub struct Decoded {
    pub tree: Tree,
    pub next_id: u64,
    pub freed: Vec<u64>,
}

/// A body's nodes and numbers, before they are held to being a tree.
pub struct Parsed {
    pub nodes: BTreeMap<u64, Node>,
    pub next_ino: u64,
    pub next_id: u64,
    pub freed: Vec<u64>,
}

/// The tree a body holds, or None for any body `encode` did not write or
/// that is not a tree.
pub fn decode(body: &[u8]) -> Option<Decoded> {
    finish(parse(body)?)
}

pub fn parse(body: &[u8]) -> Option<Parsed> {
    let mut r = In::new(body);
    if r.take(4)? != BODY_MAGIC || r.u16()? != VERSION || r.u16()? != 0 {
        return None;
    }
    let next_ino = r.u64()?;
    let next_id = r.u64()?;
    let count = r.u32()? as usize;
    if count > MAX_NODES {
        return None;
    }
    let mut nodes = BTreeMap::new();
    for _ in 0..count {
        let (ino, node) = node_codec::take(&mut r)?;
        // In rising order, as encode writes them: one tree, one body.
        if nodes.last_key_value().is_some_and(|(last, _)| *last >= ino) {
            return None;
        }
        nodes.insert(ino, node);
    }
    let freed = node_codec::ids(&mut r, body.len())?;
    r.done().then_some(Parsed { nodes, next_ino, next_id, freed })
}

/// The tree, once checked.
pub fn finish(p: Parsed) -> Option<Decoded> {
    if p.nodes.len() > MAX_NODES {
        return None;
    }
    let mut tree = Tree { nodes: p.nodes, next_ino: p.next_ino };
    super::check::tree(&mut tree, p.next_id, &p.freed)?;
    Some(Decoded { tree, next_id: p.next_id, freed: p.freed })
}
