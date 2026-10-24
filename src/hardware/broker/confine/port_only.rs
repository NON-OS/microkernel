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

//! A port-only claim (H-02): its capsule holds neither Dma nor Mmio. With
//! Memory Space and Bus Master cleared at the claim (claim.rs) and kept off
//! (pci/command_rule.rs) the function starts no DMA and no MSI, and no
//! peer's memory write reaches it; PCIe peers send it no I/O requests. So a
//! port-only claim neither joins an isolation group nor is blocked by one,
//! but for a member across a bridge to a shared conventional bus, whose bus
//! masters drive I/O cycles. Pure: kernel_proofs::confine_posture port_only_tests.

use super::member::Member;

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
