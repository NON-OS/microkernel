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

//! What a claim does with a device's DMA; pure,
//! proven in kernel_proofs::confine_posture. A device a unit translates gets
//! its capsule's own domain; any other PCI device would reach all memory once
//! its driver set Bus Master: refused. A device with no PCI requester id, or
//! claimed port-only (port_only.rs), is no bus master: the claim goes ahead,
//! no DMA grant is made for it. Two ways past: the development
//! opt-in, never in a release, and the no-IOMMU posture, taken only when no
//! unit is in service at all (memory/iommu/no_iommu_rule.rs). The posture
//! line states either.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Verdict {
    /// Into the capsule's own domain; grants are IOVAs there.
    Confine,
    /// The claim goes ahead; no DMA grant is ever made for it.
    NoDma,
    /// The claim goes ahead unconfined, under the opt-in or the posture.
    Unconfined,
    Refuse,
}

/// Whether the opt-in holds: set, and never in a production build.
pub(crate) const fn opted_in(opt_in: bool, production: bool) -> bool {
    opt_in && !production
}

/// The claim's verdict. `requester`: the device has a PCI requester id.
/// `covered`: a unit in service translates it. `domain`: a domain was had
/// for it, Ok, or Err(no_unit) where no_unit says no unit is in service.
/// `port_only`: the claiming capsule holds neither Dma nor Mmio.
/// `posture`: the no-IOMMU posture holds on this boot.
pub(crate) const fn verdict(
    requester: bool,
    covered: bool,
    domain: Result<(), bool>,
    port_only: bool,
    (opt_in, production): (bool, bool),
    posture: bool,
) -> Verdict {
    let open = opted_in(opt_in, production);
    match (requester, covered, domain) {
        (true, true, Ok(())) => Verdict::Confine,
        // Nothing for a unit to confine, so nothing for the opt-in to open.
        _ if port_only => Verdict::NoDma,
        (false, _, _) if open => Verdict::Unconfined,
        (false, _, _) => Verdict::NoDma,
        // A unit in service that could not give the device a domain.
        (true, true, Err(false)) => Verdict::Refuse,
        _ if open => Verdict::Unconfined,
        // No unit in service anywhere: the device reaches all memory anyway.
        (true, _, Err(true)) if posture => Verdict::Unconfined,
        _ => Verdict::Refuse,
    }
}
