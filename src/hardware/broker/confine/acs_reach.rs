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

//! What a device reaches past the IOMMU, hop by hop, pure (K11).
//! kernel_proofs::acs_rule.

extern crate alloc;

use alloc::vec::Vec;

use super::acs_path::Hop;
use super::acs_rule::redirects;
use super::acs_tree::Bdf;

/// What the device reaches past the IOMMU: every hop's shared bus, and its
/// peers wherever `acs` (its words, with what it needs turned on) does not
/// redirect them.
pub(crate) fn group(
    hops: Vec<Hop>,
    mut acs: impl FnMut(Bdf, u16) -> Option<(u16, u16)>,
) -> Vec<Bdf> {
    let mut reached = Vec::new();
    for hop in hops {
        reached.extend(hop.always);
        if !hop.peers.is_empty() && !redirects(acs(hop.at, hop.needed), hop.needed) {
            reached.extend(hop.peers);
        }
    }
    reached
}
