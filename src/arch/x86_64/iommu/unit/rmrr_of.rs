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

//! The reserved regions a claimed device keeps doing DMA to (L60), from the
//! RMRRs DMAR lists, as `[base, end)`.

use crate::arch::x86_64::acpi::parser::other::dmar_rmrr::Rmrr;
use crate::arch::x86_64::acpi::parser::other::{rmrr_barred, rmrr_barred_by, rmrr_names};
use crate::arch::x86_64::acpi::parser::other::{rmrr_own, rmrr_regions, Cover};

/// Regions an RMRR names the device alone in; whether one might name it and
/// could not be resolved; and whether one names it with other devices (L63).
/// Neither of those is mapped: too little only denies the device, too much
/// lets it reach another's.
pub struct RmrrOf {
    pub ranges: heapless::Vec<(u64, u64), 16>,
    pub unresolved: bool,
    pub shared: bool,
}

pub fn rmrr_of(segment: u16, bdf: (u8, u8, u8)) -> RmrrOf {
    let mut out = RmrrOf { ranges: heapless::Vec::new(), unresolved: false, shared: false };
    let (all, bb) = (rmrr_regions(), &super::coverage::bridge_buses);
    // An RMRR left out may name the same memory for another device (L67).
    let (barred, incomplete) = rmrr_barred();
    let free = |r: &Rmrr| !incomplete && !rmrr_barred_by(&barred, r);
    for r in all.iter() {
        match rmrr_names(r, segment, bdf, bb) {
            Cover::Yes if !(free(r) && rmrr_own(&all, r, segment, bdf, bb)) => out.shared = true,
            Cover::Yes => {
                let range = r.limit.checked_add(1).map(|end| (r.base, end));
                if range.and_then(|range| out.ranges.push(range).ok()).is_none() {
                    out.unresolved = true;
                }
            }
            Cover::Unknown => out.unresolved = true,
            Cover::No => {}
        }
    }
    out
}
