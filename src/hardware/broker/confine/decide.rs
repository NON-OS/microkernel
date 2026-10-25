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

//! What a claim does with a device's DMA (plan/50 P2, fail closed); pure,
//! proven in kernel_proofs::confine_posture. A device a unit in service
//! translates gets its capsule's own domain. A PCI device no unit confines
//! would reach all memory once its driver set Bus Master: refused. A device
//! with no PCI requester id (PS/2, an ACPI I2C client) is no bus master: its
//! claim goes ahead and no DMA grant is made for it. The one way past is the
//! owner's development opt-in, which lib.rs refuses in a release; the
//! posture line states it, and unconfined grants are counted into the quote.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Verdict {
    /// Into the capsule's own domain; grants are IOVAs there.
    Confine,
    /// The claim goes ahead; no DMA grant is ever made for it.
    NoDma,
    /// The claim goes ahead unconfined, under the opt-in only.
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
pub(crate) const fn verdict(
    requester: bool,
    covered: bool,
    domain: Result<(), bool>,
    opt_in: bool,
    production: bool,
) -> Verdict {
    let open = opted_in(opt_in, production);
    match (requester, covered, domain) {
        (false, _, _) if open => Verdict::Unconfined,
        (false, _, _) => Verdict::NoDma,
        (true, true, Ok(())) => Verdict::Confine,
        // A unit in service that could not give the device a domain.
        (true, true, Err(false)) => Verdict::Refuse,
        _ if open => Verdict::Unconfined,
        _ => Verdict::Refuse,
    }
}

/// Whether a DMA grant may hand a device its physical address: never, but
/// under the opt-in.
/// A device a unit confines can still write 0xFEEx_xxxx: claimed only if
/// interrupts are remapped (K3), or under the opt-in.
pub(crate) const fn interrupts_confined(remapping: bool, opt_in: bool, production: bool) -> bool {
    remapping || opted_in(opt_in, production)
}

pub(crate) const fn physical_dma() -> bool {
    crate::memory::iommu::dev_unconfined::holds()
}
