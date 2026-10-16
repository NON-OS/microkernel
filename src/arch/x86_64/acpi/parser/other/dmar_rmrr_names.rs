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

use super::dmar_rmrr::Rmrr;
use super::dmar_scope::{scope_covers, Cover, Scope, UnitScope, MAX_SCOPES, SCOPE_ENDPOINT};

/// Yes when one of `r`'s scopes names `bdf` in `segment`, No when none does,
/// Unknown when a scope could not be resolved or some were dropped.
pub fn rmrr_names<F>(r: &Rmrr, segment: u16, bdf: (u8, u8, u8), bridge_buses: &F) -> Cover
where
    F: Fn(u8, u8, u8) -> Option<(u8, u8)>,
{
    let mut scopes = [Scope::EMPTY; MAX_SCOPES];
    let n = (r.count as usize).min(MAX_SCOPES).min(r.scopes.len());
    scopes[..n].copy_from_slice(&r.scopes[..n]);
    let (segment_of, truncated) = (r.segment, r.truncated);
    let unit =
        UnitScope { include_all: false, segment: segment_of, scopes, count: n as u8, truncated };
    scope_covers(&unit, segment, bdf, bridge_buses)
}

/// Whether `r` names one endpoint alone (L63). A region several devices
/// share, or a bridge's whole subtree, is theirs together: mapped for one
/// claimant, it lets that device rewrite what a sibling, unclaimed and still
/// in the identity domain, is driven by.
pub fn rmrr_sole(r: &Rmrr) -> bool {
    r.count == 1 && !r.truncated && r.scopes[0].kind == SCOPE_ENDPOINT
}
