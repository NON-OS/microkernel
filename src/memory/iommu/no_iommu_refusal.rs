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

//! A boot that may not go on with no IOMMU in service (the owner's O-IOMMU
//! ruling): Hardened and Air-Gapped from the boot menu, and an AgentProof
//! build, promise that a device reaches only what it was granted. With no unit
//! confining device DMA that promise cannot be kept, so the boot stops before
//! any driver starts, with one line saying why, rather than going on to a
//! machine whose every claim is refused. Pure, so the host proofs mount it
//! (kernel_proofs::confine_posture).

const AIR_GAPPED: &str = "air-gapped needs an IOMMU to confine device DMA; this machine has none";
const HARDENED: &str = "hardened needs an IOMMU to confine device DMA; this machine has none";
const AGENTPROOF: &str = "AgentProof needs an IOMMU to confine device DMA; this machine has none";

/// The promise that stops the boot.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Needs {
    AirGapped,
    Hardened,
    AgentProof,
}

impl Needs {
    /// The line on the panel and on serial.
    pub(crate) const fn line(self) -> &'static str {
        match self {
            Self::AirGapped => AIR_GAPPED,
            Self::Hardened => HARDENED,
            Self::AgentProof => AGENTPROOF,
        }
    }
}

/// Which promise stops this boot: none while a unit is in service. An
/// Air-Gapped boot is also hardened, so it is named first.
pub(crate) const fn stops(
    unit_in_service: bool,
    agentproof_build: bool,
    hardened: bool,
    air_gapped: bool,
) -> Option<Needs> {
    if unit_in_service {
        None
    } else if air_gapped {
        Some(Needs::AirGapped)
    } else if hardened {
        Some(Needs::Hardened)
    } else if agentproof_build {
        Some(Needs::AgentProof)
    } else {
        None
    }
}
