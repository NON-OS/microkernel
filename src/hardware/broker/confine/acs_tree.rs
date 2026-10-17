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

//! The enumerated functions as a tree of buses, pure. kernel_proofs::
//! acs_rule.

extern crate alloc;

use alloc::vec::Vec;

/// Bus, device, function.
pub(crate) type Bdf = (u8, u8, u8);

/// One enumerated function; a bridge's secondary and subordinate bus;
/// whether its secondary is conventional PCI, where devices reach each
/// other directly; and the requester id DMA from below it carries upstream,
/// when that is not the device's own: a PCIe-to-PCI bridge's is its
/// secondary bus, device 0, function 0; a legacy PCI bridge's is its own.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Func {
    pub at: Bdf,
    pub buses: Option<(u8, u8)>,
    pub conventional: bool,
    pub alias: Option<Bdf>,
}

/// The functions `pick` chooses, and every function under those that are
/// bridges.
pub(crate) fn with_below(all: &[Func], pick: impl Fn(&Func) -> bool) -> Vec<Bdf> {
    let chosen = all.iter().filter(|f| pick(f));
    let ranges: Vec<(u8, u8)> = chosen.clone().filter_map(|f| f.buses).collect();
    let under = all.iter().filter(|f| ranges.iter().any(|&(s, e)| (s..=e).contains(&f.at.0)));
    chosen.chain(under).map(|f| f.at).collect()
}
