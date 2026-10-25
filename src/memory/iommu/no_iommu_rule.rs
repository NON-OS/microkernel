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
//! When a boot takes the no-IOMMU posture. Pure, and proven on the host
//! (kernel_proofs::confine_posture).
//!
//! Many machines leave the firmware's VT-d or AMD-Vi switch off. With no
//! remapping unit in service a device reaches all of memory whatever the
//! kernel does, so refusing every claim only leaves the machine at its splash.
//! A build that carries the posture lets those claims go ahead and says so in
//! every place a person or a verifier looks. A Hardened or Air-Gapped boot
//! never takes it, and a build without the feature has no way to.

/// Why no unit is in service, as the posture line names it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Reason {
    /// Firmware described no remapping hardware at all.
    NoTable,
    /// Firmware described a unit and none came into service.
    UnitDown,
}

impl Reason {
    pub(crate) const fn name(self) -> &'static [u8] {
        match self {
            Self::NoTable => b"no-remapping-hardware-described",
            Self::UnitDown => b"unit-not-in-service",
        }
    }
}

/// Whether this boot takes the posture: built in, no unit in service, and a
/// boot that does not refuse it.
pub(crate) const fn enters(built: bool, unit_in_service: bool, refusing_boot: bool) -> bool {
    built && !unit_in_service && !refusing_boot
}

/// The boot menu postures that refuse it: Hardened and Air-Gapped promise
/// more than a machine with no IOMMU can give, so their claims stay refused.
pub(crate) const fn refusing_boot(hardened: bool, air_gapped: bool) -> bool {
    hardened || air_gapped
}

/// The reason, from whether firmware described any remapping hardware.
pub(crate) const fn reason(described: bool) -> Reason {
    if described {
        Reason::UnitDown
    } else {
        Reason::NoTable
    }
}
