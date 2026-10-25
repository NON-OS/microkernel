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

//! A port-only claim: its capsule holds neither Dma nor Mmio, and its
//! Memory Space and Bus Master are cleared at the claim and kept off, so it
//! starts no DMA or MSI and no peer's request reaches it: it neither joins
//! nor is blocked by a group but across a shared conventional bus. Groups
//! hold both ways (H-03, `blocked_by`). Pure: kernel_proofs.

use super::member::Member;

type Bdf = (u8, u8, u8);

/// Whether a capsule holding Dma as `dma` and Mmio as `mmio` claims port-only.
pub(crate) const fn port_only(dma: bool, mmio: bool) -> bool {
    !dma && !mmio
}

/// Whether a member another capsule holds stops a claim: always across a
/// shared conventional bus, else when neither side is port-only.
pub(crate) const fn blocks(claim_port_only: bool, holder_port_only: bool, shared: bool) -> bool {
    shared || !(claim_port_only || holder_port_only)
}

/// The first of `group` that a holder `holds` and that blocks the claim.
pub(super) fn blocker(
    group: &[Member],
    holds: impl Fn(u64) -> bool,
    claim_port_only: bool,
    holder_port_only: bool,
) -> Option<&Member> {
    group.iter().find(|m| holds(m.id) && blocks(claim_port_only, holder_port_only, m.shared))
}

/// A function reaching the claim past a hop, or a held PCI device whose
/// group cannot be read (fails closed).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Blocked {
    Reaches(Bdf, Bdf),
    Unreadable(Bdf),
}

/// What, of one other capsule's `held` devices, blocks a claim of `claimed`
/// (group `group`). `peer`: a held device's place and group; None off the
/// PCI tree (no group, blocks nothing); no group when it cannot be read.
pub(super) fn blocked_by<'a>(
    claimed: u64,
    group: &[Member],
    held: &[u64],
    (claim_port_only, holder_port_only): (bool, bool),
    peer: impl Fn(u64) -> Option<(Bdf, Option<&'a [Member]>)>,
) -> Option<Blocked> {
    let holds = |id: u64| held.contains(&id);
    if let Some(m) = blocker(group, holds, claim_port_only, holder_port_only) {
        return Some(Blocked::Reaches(m.at, m.hop));
    }
    let me = |m: &&Member| m.id == claimed && blocks(claim_port_only, holder_port_only, m.shared);
    held.iter().find_map(|&e| match peer(e)? {
        (at, None) => Some(Blocked::Unreadable(at)),
        (at, Some(g)) => g.iter().find(me).map(|m| Blocked::Reaches(at, m.hop)),
    })
}
