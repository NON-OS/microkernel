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

//! The enrolment certificate, `nsh-enrol/1`: the fleet's authority says
//! that this TPM's attestation key, these host keys, this host name and
//! these firmware PCRs and boot applications belong together, until
//! `not_after_ms`.

pub mod boot;

use crate::keys::{HybridSig, DIGEST_LEN, KEY_ID_LEN};
use crate::WireError;

pub const ENROL_MAGIC: [u8; 4] = *b"NSHE";
pub const ENROL_VERSION: u16 = 1;
pub const FLEET_ID_LEN: usize = 16;
/// The quote's P-256 attestation key, x then y.
pub const AK_PUBLIC_LEN: usize = 64;
/// PCRs 0, 1, 2 and 7, SHA-256 bank, in that order.
pub const FIRMWARE_PCRS: usize = 4;
pub const PCR_LEN: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Enrol<'a> {
    pub fleet_id: &'a [u8; FLEET_ID_LEN],
    pub host: &'a [u8],
    pub ak_public: &'a [u8; AK_PUBLIC_LEN],
    pub host_keys: &'a [u8; DIGEST_LEN],
    pub pcrs: [&'a [u8; PCR_LEN]; FIRMWARE_PCRS],
    /// The boot applications, whole digests end to end (`boot`).
    pub boot_apps: &'a [u8],
    pub not_after_ms: u64,
    pub authority: &'a [u8; KEY_ID_LEN],
    pub sig: HybridSig<'a>,
    /// The bytes the authority signed.
    pub signed: &'a [u8],
}

/// A fleet id of all zeros names no fleet, and is refused.
pub fn fleet(id: &[u8; FLEET_ID_LEN]) -> Result<&[u8; FLEET_ID_LEN], WireError> {
    id.iter().any(|b| *b != 0).then_some(id).ok_or(WireError::Value)
}
