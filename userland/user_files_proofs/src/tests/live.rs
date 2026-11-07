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

//! Which objects the newest whole generation on a volume needs, read the
//! way a start reads it, through the shipped manifest codec: the body,
//! each journal record applied, and every chunk the tree names.

use std::collections::BTreeSet;

use nonos_user_files::limits::CHUNK;
use nonos_user_files::manifest::{apply, finish, parse, Slot};

use crate::doubles::Volume;

/// The slots there are, the newest generation's body parts, and every
/// chunk its tree names.
pub fn needed(v: &Volume) -> BTreeSet<u64> {
    let newest = [0u64, 1]
        .iter()
        .filter_map(|id| v.objects.get(id).and_then(|raw| Slot::decode(raw)))
        .filter(|s| {
            s.parts.iter().chain(s.journal.iter().map(|r| &r.id)).all(|p| v.objects.contains_key(p))
        })
        .max_by_key(|s| s.generation)
        .expect("a whole slot");
    let body: Vec<u8> = newest.parts.iter().flat_map(|p| v.objects[p].clone()).collect();
    assert_eq!(body.len(), newest.body_len as usize);
    assert!(newest.parts.len() == body.len().div_ceil(CHUNK));
    let mut p = parse(&body).expect("the newest body parses");
    for r in &newest.journal {
        apply(&mut p, &v.objects[&r.id]).expect("each record applies");
    }
    let d = finish(p).expect("the newest tree checks");
    let mut out: BTreeSet<u64> =
        [0u64, 1].into_iter().filter(|id| v.objects.contains_key(id)).collect();
    out.extend(newest.parts.iter().copied());
    out.extend(newest.journal.iter().map(|r| r.id));
    out.extend(d.tree.nodes.values().flat_map(|n| n.chunks.iter().copied()).filter(|id| *id != 0));
    out
}

/// Asserts the volume holds exactly what its newest generation needs.
pub fn no_strays(v: &Volume) {
    let held: BTreeSet<u64> = v.objects.keys().copied().collect();
    assert_eq!(held, needed(v), "objects past what the tree needs");
}
