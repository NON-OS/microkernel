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

//! The kernel's reader run as the kernel runs it, its answer in the rule's
//! terms, and what any table it names must hold.

use super::disk::Disk;
use super::model::{header, Part};
use super::rule::Named;
use crate::kernel_gpt::header::Header;
use crate::kernel_gpt::{choose, places, Entry, Held, Table};

pub fn kernel(d: &Disk) -> Named {
    let read = |h: &Header| Ok::<_, ()>(d.read(h.entries_lba, h.array_sectors() as usize * 512));
    let parts =
        |e: Vec<Entry>| e.iter().map(|e| (e.type_guid, e.unique, e.first, e.last)).collect();
    match choose((&d.mbr, &d.heads[0], &d.heads[1]), d.sectors, read).unwrap() {
        Table::NoGpt => Named::NoGpt,
        Table::Unbelieved => Named::Unbelieved,
        Table::Named(e, Held::Primary) => Named::Primary(parts(e)),
        Table::Named(e, Held::Backup) => Named::Backup(parts(e)),
        Table::Named(e, Held::Disagreeing) => Named::Disagreeing(parts(e)),
    }
}

/// Whatever a believed table names lies in its header's usable range,
/// shares no sector, and never puts the store and the plan together.
pub fn held(d: &Disk, named: &Named, seed: u64) {
    let (w, parts) = match named {
        Named::Primary(p) | Named::Disagreeing(p) => (0, p),
        Named::Backup(p) => (1, p),
        _ => return,
    };
    let (first, last, ..) = header(d, w).expect("a table names only under a believed header");
    assert!(parts.iter().all(|p: &Part| first <= p.2 && p.2 <= p.3 && p.3 <= last), "seed {seed}");
    let kept: Vec<_> = parts
        .iter()
        .map(|p| Entry { type_guid: p.0, unique: p.1, first: p.2, last: p.3 })
        .collect();
    if let Some(p) = places(&kept) {
        let (s, l) = (p.store, p.plan);
        assert!(
            s.end() <= l.first || l.end() <= s.first,
            "seed {seed}: store and plan share a sector"
        );
    }
}
