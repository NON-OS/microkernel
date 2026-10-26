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
    /// The receipt's own bytes (napr/read.rs).
    Format(Fault),
    /// Not one NFSR v1 record.
    Record,
    /// The record is not final, or a count saturated, or M overflowed.
    Unfinished,
    /// Not sealed under the vendor authority, or by a supervisor not trusted.
    Supervisor,
    /// The box word is not one capsule_linux takes.
    Box,
    /// The app policy is not one NLXPOL01 policy.
    Policy,
    /// Slot 0 is not this app in this box (capsule_linux seal_slot.rs).
    Binding,
    /// The attestation document, as nsh_attest reads and checks it.
    Doc(nsh_attest::Refusal),
    /// The doc answers another challenge than this receipt's.
    Challenge,
    /// The doc carries a part its quote does not sign (napr/doc_form.rs).
    Unsigned,
    /// No IOMMU, one not enforcing, a grant around it, or a device in a
    /// domain that reaches all of memory (nsh_attest::steps::dma).
    Dma,
    /// The entries do not fold to the registry root the quote signed.
    Registry,
    /// The launcher named is no capsule the signed registry lists.
    Launcher,
    /// PCR 9 names no trusted release, or PCR 8 or 14 is not that release's.
    Release,
    /// The doc's AK is not the one pinned.
    Ak,
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
            Self::Registry => "registry",
            Self::Launcher => "launcher",
            Self::Release => "release",
            Self::Ak => "ak",
        }
    }
}
