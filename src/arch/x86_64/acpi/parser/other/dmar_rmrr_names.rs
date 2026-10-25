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

//! Whether an RMRR names a device (L60). Its scopes are read as a remapping
//! unit's are, an endpoint by its path and a bridge with every bus below it,
//! so a claimant's domain maps exactly the regions firmware said that device
//! keeps doing DMA to. Pure, so the host proofs hold it.

use super::dmar_rmrr::{Rmrr, MAX_RMRR_SCOPES};
use super::dmar_scope::{scope_covers, Cover, UnitScope, SCOPE_ENDPOINT};

/// Yes when one of `r`'s scopes names `bdf` in `segment`, No when none does,
/// Unknown when a scope could not be resolved or some were dropped.
pub fn rmrr_names<F>(r: &Rmrr, segment: u16, bdf: (u8, u8, u8), bridge_buses: &F) -> Cover
where
    F: Fn(u8, u8, u8) -> Option<(u8, u8)>,
{
    // Its own size, not a remapping unit's 255 scopes, on the claim's stack.
    let count = r.count.min(MAX_RMRR_SCOPES as u8);
    let (scopes, truncated) = (r.scopes, r.truncated);
    let unit = UnitScope::<MAX_RMRR_SCOPES> {
        include_all: false,
        segment: r.segment,
        scopes,
        count,
        truncated,
    };
    scope_covers(&unit, segment, bdf, bridge_buses)
}

/// Whether `r` names one endpoint alone (L63). A region several devices
/// share, or a bridge's whole subtree, is theirs together: mapped for one
/// claimant, it lets that device rewrite what a sibling, unclaimed and still
/// in the identity domain, is driven by. A device off PCI counts as one more.
pub fn rmrr_sole(r: &Rmrr) -> bool {
    r.count == 1 && !r.truncated && !r.foreign && r.scopes[0].kind == SCOPE_ENDPOINT
}

/// Whether `r`'s memory is the device's alone (L65): every RMRR it overlaps,
/// itself included, names that one endpoint and no other device. Firmware
/// may list one region twice, once per device that uses it, and on another
/// segment: memory is one for every segment (L67).
pub fn rmrr_own<F>(all: &[Rmrr], r: &Rmrr, segment: u16, bdf: (u8, u8, u8), bb: &F) -> bool
where
    F: Fn(u8, u8, u8) -> Option<(u8, u8)>,
{
    let meets = |o: &&Rmrr| o.base <= r.limit && r.base <= o.limit;
    all.iter().filter(meets).all(|o| rmrr_sole(o) && rmrr_names(o, segment, bdf, bb) == Cover::Yes)
}

/// Whether `r` touches memory an RMRR left out names (L67), `[first, last]`.
pub fn rmrr_barred_by(barred: &[(u64, u64)], r: &Rmrr) -> bool {
    barred.iter().any(|&(first, last)| first <= r.limit && r.base <= last)
}
