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

//! The PPTT's structures one by one after the header, each length at least
//! four, inside the table and enough for its type; then every reference
//! checked (`check`).

use super::super::le::u8_at;
use super::super::sdt::{revision, table, TableError, HEADER};
use super::cache::{cache, Cache, CACHE_MIN};
use super::check::references;
use super::structs::{node, ProcessorNode, NODE_MIN};
use alloc::vec::Vec;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Pptt {
    pub nodes: Vec<ProcessorNode>,
    pub caches: Vec<Cache>,
}

pub fn pptt(t: &[u8]) -> Result<Pptt, TableError> {
    let t = table(t, b"PPTT")?;
    let (rev, mut at, mut p) = (revision(t), HEADER, Pptt::default());
    let mut others = Vec::new();
    while at < t.len() {
        let bad = TableError::Structure(at);
        let len = u8_at(t, at + 1).ok_or(bad)? as usize;
        let s = t.get(at..at + len).filter(|_| len >= 4).ok_or(bad)?;
        let offset = at as u32;
        match s[0] {
            0 if len >= NODE_MIN => p.nodes.push(node(s, offset).ok_or(bad)?),
            1 if len >= CACHE_MIN => p.caches.push(cache(s, offset, rev).ok_or(bad)?),
            0 | 1 => return Err(bad),
            _ => others.push(offset),
        }
        at += len;
    }
    references(&p, &others)?;
    Ok(p)
}
