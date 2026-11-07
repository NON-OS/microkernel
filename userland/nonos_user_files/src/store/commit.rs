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

//! A commit: a journal record of what changed since the last one, or,
//! when the journal has grown past the body or JOURNAL_MAX records, the
//! whole tree again (checkpoint.rs). Either way the slot not holding the
//! newest generation is written last, so until it is whole the newest
//! generation is the one before, with every object it names still there.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use super::{slot_of, Store};
use crate::errno::Errno;
use crate::host::Objects;
use crate::limits::CHUNK;
use crate::manifest::{print, record, Record, Slot, JOURNAL_MAX, RECORD_NEXT_ID_AT};
use crate::tree::Tree;

/// The journal may grow to the body's size, or this, before the body is
/// written again: a small tree is not rewritten for every record.
const JOURNAL_FLOOR: u64 = 256 << 10;

/// Commits `tree`. `orphans` are the objects of files unlinked while
/// open: not in this tree, needed until their last close, and named for
/// removal so a restart drops them. The caller has written every changed
/// chunk first.
pub fn commit(
    o: &mut impl Objects,
    tree: &Tree,
    store: &mut Store,
    orphans: &[u64],
) -> Result<(), Errno> {
    let linked = tree.nodes.iter().filter(|(_, n)| n.linked);
    let prints: BTreeMap<u64, [u8; 16]> = linked.map(|(i, n)| (*i, print(*i, n))).collect();
    if store.generation > 0 && store.journal.len() < JOURNAL_MAX {
        let changed = prints.iter().filter(|(i, p)| store.prints.get(i) != Some(p));
        let nodes = changed.filter_map(|(i, _)| tree.nodes.get(i).map(|n| (*i, n)));
        let removed: Vec<u64> =
            store.prints.keys().filter(|i| !prints.contains_key(i)).copied().collect();
        let mut freed = store.pending.clone();
        freed.extend_from_slice(orphans);
        let rec = record(tree.next_ino, nodes, &removed, &freed);
        let room = store.base_len.max(JOURNAL_FLOOR);
        if rec.len() <= CHUNK && store.journal_bytes + rec.len() as u64 <= room {
            return delta(o, store, rec, prints);
        }
    }
    super::checkpoint::checkpoint(o, tree, store, orphans, prints)
}

fn delta(
    o: &mut impl Objects,
    store: &mut Store,
    mut rec: Vec<u8>,
    prints: BTreeMap<u64, [u8; 16]>,
) -> Result<(), Errno> {
    let id = store.next_id;
    // The record's own object is numbered before it says what is next.
    rec[RECORD_NEXT_ID_AT..RECORD_NEXT_ID_AT + 8].copy_from_slice(&(id + 1).to_le_bytes());
    let hash = *blake3::hash(&rec).as_bytes();
    store.append(o, &rec)?;
    let mut journal = store.journal.clone();
    journal.push(Record { id, len: rec.len() as u32, hash });
    let slot = Slot {
        generation: store.generation + 1,
        body_len: store.base_len,
        body_hash: store.base_hash,
        parts: store.parts.clone(),
        journal,
    };
    if let Err(e) = o.put(slot_of(slot.generation), &slot.encode()) {
        store.pending.push(id);
        return Err(e);
    }
    store.generation = slot.generation;
    store.journal = slot.journal;
    store.journal_bytes += rec.len() as u64;
    store.prints = prints;
    store.remove_pending(o);
    Ok(())
}
