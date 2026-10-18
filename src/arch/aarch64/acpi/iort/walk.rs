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

//! The IORT's nodes: the count and the array's offset from the table's
//! fixed part, each node's 16-bit length inside the table, its mappings
//! inside the node, and every mapping's output naming a node's start.

use super::super::le::{u16_at, u32_at, u8_at};
use super::super::sdt::{table, TableError, HEADER as SDT};
use super::mapping::{Mapping, LEN};
use super::node::{Node, HEADER};
use super::node_data::kind;
use alloc::vec::Vec;

const FIXED: usize = SDT + 12;

pub fn iort(t: &[u8]) -> Result<Vec<Node>, TableError> {
    let t = table(t, b"IORT")?;
    let s = TableError::Short;
    let (count, mut at) = (u32_at(t, SDT).ok_or(s)?, u32_at(t, SDT + 4).ok_or(s)? as usize);
    if count > 0 && at < FIXED {
        return Err(TableError::Structure(at));
    }
    let mut nodes = Vec::new();
    for _ in 0..count {
        let bad = TableError::Structure(at);
        let len = u16_at(t, at + 1).ok_or(bad)? as usize;
        let n = t.get(at..at.checked_add(len).ok_or(bad)?).filter(|_| len >= HEADER).ok_or(bad)?;
        let (ty, revision) = (n[0], u8_at(n, 3).ok_or(bad)?);
        let (maps, map_at) =
            (u32_at(n, 8).ok_or(bad)? as usize, u32_at(n, 12).ok_or(bad)? as usize);
        let end = maps.checked_mul(LEN).and_then(|b| b.checked_add(map_at)).ok_or(bad)?;
        if maps > 0 && (map_at < HEADER || end > len) {
            return Err(bad);
        }
        let mappings = (0..maps).map(|i| Mapping::read(&n[map_at + i * LEN..]).ok_or(bad));
        let mappings = mappings.collect::<Result<Vec<_>, _>>()?;
        let kind = kind(n, ty, revision).ok_or(bad)?;
        let id = u32_at(n, 4).ok_or(bad)?;
        nodes.push(Node { offset: at as u32, revision, id, kind, mappings });
        at += len;
    }
    for n in &nodes {
        let known = |m: &Mapping| nodes.binary_search_by_key(&m.output_ref, |x| x.offset).is_ok();
        if !n.mappings.iter().all(known) {
            return Err(TableError::Reference(n.offset as usize));
        }
    }
    Ok(nodes)
}
