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

//! The name a refusal is shown by, after `POLICY`.

use core::fmt;

use crate::{CapsuleFault, Refusal};

impl Refusal {
    pub const fn name(&self) -> &'static str {
        match self {
            Refusal::Format(_) => "format",
            Refusal::Version(_) => "version",
            Refusal::Challenge => "challenge",
            Refusal::Quote(_) => "quote",
            Refusal::Binding => "binding",
            Refusal::Signer => "signer",
            Refusal::Signature => "signature",
            Refusal::Selection => "selection",
            Refusal::PcrDigest => "pcr-digest",
            Refusal::EnrolFormat => "enrol",
            Refusal::EnrolAuthority => "enrol-authority",
            Refusal::EnrolSignature => "enrol-signature",
            Refusal::EnrolFleet => "enrol-fleet",
            Refusal::EnrolHost => "enrol-host",
            Refusal::EnrolAk => "enrol-ak",
            Refusal::EnrolHostKeys => "enrol-host-keys",
            Refusal::EnrolExpired => "enrol-expired",
            Refusal::EnrolRevoked => "enrol-revoked",
            Refusal::Pcr9 => "pcr9",
            Refusal::Pcr8 => "pcr8",
            Refusal::Pcr14(CapsuleFault::None) => "pcr14",
            Refusal::Pcr14(CapsuleFault::PathOnly) => "path-only",
            Refusal::Pcr14(CapsuleFault::Developer) => "developer",
            Refusal::Pcr14(CapsuleFault::LocalBuild) => "local-build",
            Refusal::Log(_) => "log",
            Refusal::Replay(_) => "replay",
            Refusal::Loader => "loader",
            Refusal::BootApplication => "boot-application",
            Refusal::Firmware(0) => "pcr0",
            Refusal::Firmware(1) => "pcr1",
            Refusal::Firmware(2) => "pcr2",
            Refusal::Firmware(7) => "pcr7",
            Refusal::Firmware(_) => "firmware",
            Refusal::Dma => "dma",
            Refusal::Registry => "registry",
            Refusal::RegistryIncomplete => "registry-complete",
            Refusal::CapsuleCount => "capsule-count",
            Refusal::Front => "front",
        }
    }
}

/// `POLICY (<name>)`, the line the client shows before it closes.
impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "POLICY ({})", self.name())
    }
}
