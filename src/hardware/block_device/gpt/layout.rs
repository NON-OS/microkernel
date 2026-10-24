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

//! Where a disk's store and plan are looked for, from the table it names.
//! A table naming NONOS's partitions places them. Otherwise the fixed
//! sectors are used: on a disk with no GPT, on one whose two headers both
//! failed (the store magic still guards it), and on an older NONOS disk
//! whose table names only an ESP clear of them. A table naming another
//! system's partition over the fixed sectors refuses them: a stale store
//! magic left there must not get that partition written over.

use alloc::vec::Vec;

use super::choose::{Held, Table};
use super::entry::Entry;
use super::fixed::FIXED;
use super::places::{places, Places, NONOS_DATA, NONOS_PLAN, NONOS_STORE};

/// What the disk's GPT showed, for the log.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Shown {
    NoGpt,
    Named(Held),
    /// A GPT shown, both its headers failed.
    Failed,
}

pub(crate) enum Layout {
    /// Where to look; the partitions named, for the loader's; whether the
    /// table placed the store and plan; what the GPT showed.
    At { places: Places, entries: Vec<Entry>, named: bool, shown: Shown },
    /// The table names another system's partition over the fixed sectors.
    Crossed,
}

pub(crate) fn layout(t: Table) -> Layout {
    let (entries, shown) = match t {
        Table::Named(e, held) => (e, Shown::Named(held)),
        Table::NoGpt => (Vec::new(), Shown::NoGpt),
        Table::Unbelieved => (Vec::new(), Shown::Failed),
    };
    if let Some(places) = places(&entries) {
        return Layout::At { places, entries, named: true, shown };
    }
    if entries.iter().any(crosses_fixed) {
        return Layout::Crossed;
    }
    Layout::At { places: FIXED, entries, named: false, shown }
}

/// Whether `e`, not of NONOS's own types, shares a sector with the fixed
/// store or plan. The fixed layout names no data extent of its own: the
/// plan names the data volume, so there is nothing else to cross.
fn crosses_fixed(e: &Entry) -> bool {
    let ours = [NONOS_STORE, NONOS_PLAN, NONOS_DATA].contains(&e.type_guid);
    let crosses = |x: super::places::Extent| e.first < x.end() && e.last >= x.first;
    !ours && (crosses(FIXED.store) || crosses(FIXED.plan))
}
