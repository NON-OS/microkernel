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

//! Starting from the volume: the newest whole generation (its body with
//! each journal record applied), what it said to remove, and whatever a
//! save cut short left past it.

use alloc::vec;
use alloc::vec::Vec;

use super::{read_full, Store};
use crate::errno::{Errno, ENOENT, EUCLEAN};
use crate::host::Objects;
use crate::limits::CHUNK;
use crate::manifest::{apply, finish, parse, print, Decoded, Slot, FIRST_ID, SLOT_MAX};
use crate::tree::Tree;

enum Found {
    Absent,
    /// There, but not whole: a cut slot write, or an older slot whose body
    /// a later commit removed.
    Broken,
    Whole(Slot, Decoded),
}

/// The committed tree, or a fresh one when neither slot was ever written.
/// A slot that cannot be read (EIO) fails the start rather than falling
/// back past it: falling back would remove the newer generation's objects.
pub fn load(o: &mut impl Objects, now: u64) -> Result<(Tree, Store), Errno> {
    let (a, b) = (found(o, 0)?, found(o, 1)?);
    let pick = match (a, b) {
        (Found::Absent, Found::Absent) => None,
        (Found::Whole(sa, da), Found::Whole(sb, db)) => {
            Some(if sa.generation > sb.generation { (sa, da) } else { (sb, db) })
        }
        (Found::Whole(s, d), _) | (_, Found::Whole(s, d)) => Some((s, d)),
        _ => return Err(EUCLEAN),
    };
    let Some((slot, decoded)) = pick else {
        return Ok((Tree::fresh(now), sweep(o, Store::empty(FIRST_ID))?));
    };
    let linked = decoded.tree.nodes.iter().filter(|(_, n)| n.linked);
    let store = Store {
        generation: slot.generation,
        next_id: decoded.next_id,
        base_len: slot.body_len,
        base_hash: slot.body_hash,
        journal_bytes: slot.journal.iter().map(|r| r.len as u64).sum(),
        journal: slot.journal,
        parts: slot.parts,
        prints: linked.map(|(i, n)| (*i, print(*i, n))).collect(),
        pending: decoded.freed,
    };
    Ok((decoded.tree, sweep(o, store)?))
}

/// Removes what the manifest named, then every object at or past its
/// next number, which only a save cut before its slot was written leaves.
fn sweep(o: &mut impl Objects, mut store: Store) -> Result<Store, Errno> {
    store.remove_pending(o);
    loop {
        match o.size(store.next_id) {
            Ok(_) => o.remove(store.next_id)?,
            Err(ENOENT) => return Ok(store),
            Err(e) => return Err(e),
        }
        store.next_id += 1;
    }
}

fn found(o: &mut impl Objects, id: u64) -> Result<Found, Errno> {
    let size = match o.size(id) {
        Ok(size) => size,
        Err(ENOENT) => return Ok(Found::Absent),
        Err(e) => return Err(e),
    };
    if size > SLOT_MAX as u64 {
        return Ok(Found::Broken);
    }
    let mut raw = vec![0u8; size as usize];
    let got = read_full(o, id, 0, &mut raw)?;
    let Some(slot) = Slot::decode(&raw[..got]) else {
        return Ok(Found::Broken);
    };
    let mut body = Vec::new();
    body.try_reserve_exact(slot.body_len as usize).map_err(|_| crate::errno::ENOMEM)?;
    for (i, part) in slot.parts.iter().enumerate() {
        let want = (slot.body_len as usize - i * CHUNK).min(CHUNK);
        let at = body.len();
        body.resize(at + want, 0);
        match read_full(o, *part, 0, &mut body[at..]) {
            Ok(n) if n == want => {}
            Ok(_) | Err(ENOENT) => return Ok(Found::Broken),
            Err(e) => return Err(e),
        }
    }
    if blake3::hash(&body).as_bytes() != &slot.body_hash {
        return Ok(Found::Broken);
    }
    let Some(mut parsed) = parse(&body) else {
        return Ok(Found::Broken);
    };
    drop(body);
    for r in &slot.journal {
        let mut rec = vec![0u8; r.len as usize];
        match read_full(o, r.id, 0, &mut rec) {
            Ok(n) if n == rec.len() => {}
            Ok(_) | Err(ENOENT) => return Ok(Found::Broken),
            Err(e) => return Err(e),
        }
        if blake3::hash(&rec).as_bytes() != &r.hash || apply(&mut parsed, &rec).is_none() {
            return Ok(Found::Broken);
        }
    }
    Ok(finish(parsed).map_or(Found::Broken, |d| Found::Whole(slot, d)))
}
