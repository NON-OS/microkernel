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
//! What a verifier is told to trust, by its own operator: the releases whose
//! kernels it accepts, the supervisors whose records it accepts, and the AK
//! it pinned on first use. v0 checks no EK certificate chain, so a receipt
//! never says the TPM's maker vouched for the key (letter agent-02, gap 6).

use alloc::string::String;
use alloc::vec::Vec;

/// One accepted release, by nsh_attest's rule: PCR 9 names the admitted
/// kernel and so picks the release; PCR 8 and 14 must then be its own.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Release {
    pub version: String,
    pub pcr8: [u8; 32],
    pub pcr9: [u8; 32],
    pub pcr14: [u8; 32],
}

/// A supervisor whose sealed records are taken: the Linux personality's
/// measurement and the capability mask its role runs with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Supervisor {
    pub meas: [u8; 32],
    pub caps: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trust {
    pub releases: Vec<Release>,
    pub supervisors: Vec<Supervisor>,
    /// The attestation key, as the doc carries it, pinned on first use.
    pub ak: [u8; 64],
}
