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

//! The kernel's reader run as the kernel runs it, the model beside it, and
//! what any table the reader believes must hold.

use super::disk::Disk;
use super::model::{self, Part};
use crate::kernel_gpt::{entries, header, mbr, places, Entry};

/// The reader as the kernel runs it: sector 0, then LBA 1, then the array.
pub fn kernel(d: &Disk) -> Option<Vec<Part>> {
    if !mbr::protects(&d.mbr) {
        return None;
    }
    let h = header::parse(&d.header, d.sectors)?;
    let array = &d.array[..h.array_sectors() as usize * 512];
    let found = entries(&h, array)?;
    Some(found.iter().map(|e| (e.type_guid, e.unique, e.first, e.last)).collect())
}

pub fn model(d: &Disk) -> Option<Vec<Part>> {
    model::protects(&d.mbr).then(|| model::table(d)).flatten()
}

/// What any table the reader believes holds, lie or none.
pub fn held(d: &Disk, parts: Option<&[Part]>, seed: u64) {
    let (Some(parts), Some((first, last, ..))) = (parts, model::header(d)) else { return };
    assert!(parts.iter().all(|p| first <= p.2 && p.2 <= p.3 && p.3 <= last), "seed {seed}");
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
