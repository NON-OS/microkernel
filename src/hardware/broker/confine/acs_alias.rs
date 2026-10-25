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

//! The requester ids a device's DMA may carry besides its own, pure (K11).
//! A bridge to conventional PCI forwards what it takes from below under an
//! id of its own, so the IOMMU sees that id; each one on the way up adds
//! one. Every alias joins the device's domain, or its DMA is denied, or
//! worse, lands in whatever domain holds the alias. kernel_proofs::acs_group.

extern crate alloc;

use alloc::vec::Vec;

use super::acs_tree::{Bdf, Func};

/// Bridges deeper than this are taken for a loop.
const MAX_HOPS: usize = 32;

/// Each alias of `at`, nearest first, without repeats.
pub(crate) fn aliases(at: Bdf, all: &[Func]) -> Vec<Bdf> {
    let mut found: Vec<Bdf> = Vec::new();
    let mut below = at;
    for _ in 0..MAX_HOPS {
        let Some(bridge) = all.iter().find(|f| f.buses.is_some_and(|(s, _)| s == below.0)) else {
            break;
        };
        if let Some(alias) = bridge.alias.filter(|a| *a != at && !found.contains(a)) {
            found.push(alias);
        }
        below = bridge.at;
    }
    found
}
