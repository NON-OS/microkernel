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

//! What a policy holds.

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use nsh_wire::keys::{ED_PUBLIC_LEN, KEY_ID_LEN, ML_PUBLIC_LEN};

/// One accepted release; PCR 8 is pinned from its first boot. No
/// value may be zero. No rollback index: the document carries none, and
/// the loader enforces it against the TPM before this kernel can run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Release {
    pub version: String,
    pub pcr8: [u8; 32],
    pub pcr9: [u8; 32],
    /// The loader's Authenticode SHA-256, which PCR 4 must hold.
    pub loader: [u8; 32],
    /// The measurement of the one capsule holding Network: `nsh.front`.
    pub front: [u8; 32],
}

/// A fleet enrolment authority's hybrid key, and its key id: SHA-256 of
/// the Ed25519 key, then the ML-DSA-65 key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Authority {
    pub ed: [u8; ED_PUBLIC_LEN],
    pub ml: Box<[u8; ML_PUBLIC_LEN]>,
    pub(crate) id: [u8; KEY_ID_LEN],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Policy {
    pub(crate) fleet_id: [u8; 16],
    pub(crate) authorities: Vec<Authority>,
    pub(crate) releases: Vec<Release>,
    pub(crate) revoked: Vec<[u8; 64]>,
}

/// Why a policy was refused before it was used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyError {
    /// The fleet id is all zeros.
    Fleet,
    /// No authority, more than `MAX_AUTHORITIES`, or one listed twice.
    Authorities,
    /// No release, or more than `MAX_RELEASES`.
    Releases,
    /// A release with a zero PCR 8 or 9, loader or front measurement.
    ZeroValue,
    /// Two releases with the same PCR 9: a kernel would match both.
    SharedPcr9,
    /// More than `MAX_REVOKED` revoked enrolments.
    Revoked,
}
