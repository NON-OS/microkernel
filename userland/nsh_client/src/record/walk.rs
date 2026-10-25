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

//! The record's chain: `c(0) = H("nsh/1 record\0" || header)`, then
//! `c(n+1) = H(c(n) || entry n)`. Walking the entries recomputes it and
//! holds every checkpoint to the value after the entry it names.

use super::ckpt::Mark;
use super::CkptRefusal;
use crate::ct::eq;
use nonos_hash::Sha512;
use nsh_wire::{Entry, EntryBody};

pub(crate) struct Walked {
    pub chain: [u8; 64],
    pub count: u32,
    /// The first `BOX` entry, if any.
    pub first_box: Option<u32>,
}

fn link(parts: &[&[u8]]) -> [u8; 64] {
    let mut h = Sha512::new();
    parts.iter().for_each(|p| h.update(p));
    h.finalize()
}

pub(crate) fn walk<'a>(
    header: &[u8],
    entries: impl Iterator<Item = Entry<'a>>,
    marks: &mut [Mark],
) -> Result<Walked, CkptRefusal> {
    marks.sort_unstable_by_key(|m| m.seq);
    let mut w = Walked { chain: link(&[b"nsh/1 record\0", header]), count: 0, first_box: None };
    let mut next = 0;
    for e in entries {
        w.chain = link(&[&w.chain, e.bytes]);
        if matches!(e.body, EntryBody::Box(_)) && w.first_box.is_none() {
            w.first_box = Some(e.seq);
        }
        while let Some(m) = marks.get(next).filter(|m| m.seq == e.seq) {
            if !eq(&m.chain, &w.chain) {
                return Err(CkptRefusal::Chain);
            }
            next += 1;
        }
        w.count += 1;
    }
    match next == marks.len() {
        true => Ok(w),
        false => Err(CkptRefusal::Beyond),
    }
}
