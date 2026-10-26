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
//! Why a receipt was refused. The first refusal ends the check, and each is
//! said by its own name, so a receipt is never "mostly" verified.

use crate::napr::read::Fault;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// The receipt's own bytes (napr/read.rs), or not one NFSR v1 record.
    Format(Fault),
    Record,
    /// Not final, a count saturated, or M overflowed.
    Unfinished,
    /// Not sealed under the vendor authority, or by a supervisor not trusted.
    Supervisor,
    /// The box word is not one capsule_linux takes, or the policy no NLXPOL01.
    Box,
    Policy,
    /// Slot 0 is not this app in this box (capsule_linux seal_slot.rs).
    Binding,
    /// The attestation document, as nsh_attest reads and checks it.
    Doc(nsh_attest::Refusal),
    /// The doc answers another receipt's challenge.
    Challenge,
    /// The doc carries a part its quote does not sign (napr/doc_form.rs).
    Unsigned,
    /// The DMA posture fails nsh_attest::steps::dma.
    Dma,
    /// The capsules do not replay to the PCR 14 the quote signed.
    Capsules,
    /// The entries do not fold to the registry root the quote signed.
    Registry,
    /// The launcher is no capsule the signed registry lists.
    Launcher,
    /// PCR 9 names no trusted release, or PCR 8 is not that release's.
    Release,
    Ak, // the doc's AK is not the one pinned
}

impl Refusal {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Format(_) => "format",
            Self::Record => "record",
            Self::Unfinished => "unfinished",
            Self::Supervisor => "supervisor",
            Self::Box => "box",
            Self::Policy => "policy",
            Self::Binding => "binding",
            Self::Doc(_) => "doc",
            Self::Challenge => "challenge",
            Self::Unsigned => "unsigned",
            Self::Dma => "dma",
            Self::Capsules => "capsules",
            Self::Registry => "registry",
            Self::Launcher => "launcher",
            Self::Release => "release",
            Self::Ak => "ak",
        }
    }
}
