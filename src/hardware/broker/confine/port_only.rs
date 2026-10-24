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
//! starts no DMA or MSI and no peer's memory write or PCIe request reaches
//! it. It neither joins nor is blocked by an isolation group, but across a
//! bridge to a shared conventional bus, whose masters drive I/O cycles. The
//! relation holds both ways (H-03, `blocked_by`), each side's group read
//! from its own ACS at every hop. Pure: kernel_proofs::confine_posture
//! port_only_tests, acs_group symmetric_tests.

use super::member::Member;

/// Bus, device, function.
type Bdf = (u8, u8, u8);

/// Whether a capsule holding Dma as `dma` and Mmio as `mmio` claims port-only.
pub(crate) const fn port_only(dma: bool, mmio: bool) -> bool {
    !dma && !mmio
}

/// Whether a group member held by another capsule stops a claim: always
/// across a shared conventional bus (`shared`), else only when neither the
/// claim nor the holder is port-only.
pub(crate) const fn blocks(claim_port_only: bool, holder_port_only: bool, shared: bool) -> bool {
    shared || !(claim_port_only || holder_port_only)
}

/// The first of `group` held by a capsule that `holds` it, port-only as
/// `holder_port_only` says, that blocks a claim port-only as
/// `claim_port_only` says.
pub(super) fn blocker(
    group: &[Member],
    holds: impl Fn(u64) -> bool,
    claim_port_only: bool,
    holder_port_only: bool,
) -> Option<&Member> {
    group.iter().find(|m| holds(m.id) && blocks(claim_port_only, holder_port_only, m.shared))
}

/// What, of the devices `held` by one other capsule, blocks a claim of
/// `claimed` (its group `group`; `peer` a held device's place and group):
/// the function a refusal names and the hop it reaches the claim past.
pub(super) fn blocked_by<'a>(
    claimed: u64,
    group: &[Member],
    held: &[u64],
    (claim_port_only, holder_port_only): (bool, bool),
    peer: impl Fn(u64) -> Option<(Bdf, &'a [Member])>,
) -> Option<(Bdf, Bdf)> {
    let holds = |id: u64| held.contains(&id);
    if let Some(m) = blocker(group, holds, claim_port_only, holder_port_only) {
        return Some((m.at, m.hop));
    }
    let me = |m: &&Member| m.id == claimed && blocks(claim_port_only, holder_port_only, m.shared);
    held.iter().find_map(|&e| peer(e).and_then(|(at, g)| g.iter().find(me).map(|m| (at, m.hop))))
}
