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

//! Writing the whole tree again: its body into new objects, then the slot
//! that names them with an empty journal, then removing the old body and
//! records with everything else the tree no longer needs.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use super::{slot_of, Store};
use crate::errno::Errno;
use crate::host::Objects;
use crate::limits::CHUNK;
use crate::manifest::{encode, Slot, NEXT_ID_AT};
use crate::tree::Tree;

pub(super) fn checkpoint(
    o: &mut impl Objects,
    tree: &Tree,
    store: &mut Store,
    orphans: &[u64],
    prints: BTreeMap<u64, [u8; 16]>,
) -> Result<(), Errno> {
    let old: Vec<u64> =
        store.parts.iter().copied().chain(store.journal.iter().map(|r| r.id)).collect();
    let mut freed = store.pending.clone();
    freed.extend_from_slice(orphans);
    freed.extend_from_slice(&old);
    let mut body = encode(tree, 0, &freed);
    let first = store.next_id;
    let count = body.len().div_ceil(CHUNK) as u64;
    // The body's own objects are numbered before it says what is next, so
    // the start-up sweep never takes them for a cut save's.
    body[NEXT_ID_AT..NEXT_ID_AT + 8].copy_from_slice(&(first + count).to_le_bytes());
    let mut parts = Vec::new();
    let written = body.chunks(CHUNK).try_for_each(|part| {
        parts.push(store.append(o, part)?);
        Ok(())
    });
    let slot = Slot {
        generation: store.generation + 1,
        body_len: body.len() as u64,
        body_hash: crate::manifest::body_hash(&body),
        parts: parts.clone(),
        journal: Vec::new(),
    };
    if let Err(e) = written.and_then(|()| o.put(slot_of(slot.generation), &slot.encode())) {
        store.pending.extend_from_slice(&parts);
        return Err(e);
    }
    store.generation = slot.generation;
    store.parts = parts;
    (store.base_len, store.base_hash) = (slot.body_len, slot.body_hash);
    store.journal.clear();
    store.journal_bytes = 0;
    store.prints = prints;
    store.pending.extend_from_slice(&old);
    store.remove_pending(o);
    Ok(())
}
