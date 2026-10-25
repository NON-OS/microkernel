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

//! Whether ACS keeps a device's peers from it, pure. A device reaches
//! a peer without the IOMMU when they share a device, a bus, or a port that
//! forwards between them, unless that port redirects upstream with ACS.
//! What it reaches is its isolation group, and one capsule holds a group at
//! a time (join.rs); acs_reach.rs walks it. kernel_proofs::acs_rule.

/// ACS capability and control bits (PCIe 6.0, 7.7.11): Source Validation,
/// P2P Request Redirect, P2P Completion Redirect, Upstream Forwarding.
pub(crate) const SV: u16 = 1 << 0;
pub(crate) const RR: u16 = 1 << 2;
pub(crate) const CR: u16 = 1 << 3;
pub(crate) const UF: u16 = 1 << 4;
/// What a downstream or root port must redirect.
pub(crate) const PORT: u16 = SV | RR | CR | UF;
/// What a function of a multi-function device must redirect.
pub(crate) const FUNCTION: u16 = RR | CR;

/// The control word with what the capability offers of `needed` turned on.
pub(crate) const fn enabling(cap: u16, ctrl: u16, needed: u16) -> u16 {
    ctrl | (needed & cap)
}

/*
 * Whether the capability and control words, if any, redirect `needed`. A
 * bit the capability does not offer is hard-wired: a port that forwards no
 * peer traffic need not implement its redirect (PCIe 6.0, 6.12.1.1).
 * A function with no ACS capability redirects nothing.
 */
pub(crate) fn redirects(acs: Option<(u16, u16)>, needed: u16) -> bool {
    acs.is_some_and(|(cap, ctrl)| ctrl & (needed & cap) == needed & cap)
}
