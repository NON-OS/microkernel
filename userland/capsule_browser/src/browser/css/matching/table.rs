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

use alloc::vec::Vec;

use crate::browser::dom::Dom;

use super::bloom::ancestor_bloom;
use super::positions::positions;
use super::tokens::{tokens, Tok};

/* The per-document answers a Siblings table holds, indexed by node id. */
pub(super) struct Table {
    pos: Vec<[i32; 4]>,
    prev: Vec<u32>,
    bloom: Vec<[u64; 2]>,
    toks: Vec<Tok>,
    tok_at: Vec<u32>,
    ids: Vec<u64>,
}

impl Table {
    pub fn build(dom: &Dom) -> Table {
        let (pos, prev) = positions(dom);
        let (toks, tok_at, ids) = tokens(dom);
        let bloom = ancestor_bloom(dom, &toks, &tok_at, &ids);
        Table { pos, prev, bloom, toks, tok_at, ids }
    }

    pub fn position(&self, id: usize) -> Option<(i32, i32, i32, i32)> {
        let [pos, count, pos_ty, count_ty] = *self.pos.get(id)?;
        (pos != 0).then_some((pos, count, pos_ty, count_ty))
    }

    pub fn prev(&self, id: usize) -> Option<usize> {
        let p = *self.prev.get(id)?;
        (p != u32::MAX).then_some(p as usize)
    }

    /* The element's class names as (key, byte range in its class value). */
    pub fn classes(&self, id: usize) -> &[Tok] {
        match (self.tok_at.get(id), self.tok_at.get(id + 1)) {
            (Some(&a), Some(&b)) => self.toks.get(a as usize..b as usize).unwrap_or(&[]),
            _ => &[],
        }
    }

    /* id_key of the element's id, 0 without one. */
    pub fn id_key(&self, id: usize) -> u64 {
        self.ids.get(id).copied().unwrap_or(0)
    }

    /* False when some bit the selector needs is missing from every
     * ancestor, which proves no ancestor chain can match it. A node the
     * table never reached keeps every bit set and is never rejected. */
    pub fn may_match(&self, id: usize, need: &[u64; 2]) -> bool {
        match self.bloom.get(id) {
            Some(have) => need[0] & !have[0] == 0 && need[1] & !have[1] == 0,
            None => true,
        }
    }
}
