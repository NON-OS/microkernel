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

//! One description of everything a host sends and a client holds. The
//! default is a host that passes every check; a test changes one field.

use nsh_attest::Release;

use super::attest::Quote;
use super::doc::Parts;
use super::enrol::Cert;
use super::tcg_log::Log;

pub type Entry = [u8; 45];

pub struct Spec {
    pub version: u32,
    pub th: [u8; 64],
    pub host: Vec<u8>,
    pub now_ms: u64,
    pub log: Log,
    /// The PCR values the document carries and the quote signs.
    pub pcr: Vec<(u8, [u8; 32])>,
    /// Values the quote's digest covers in place of the document's.
    pub quoted: Vec<(u8, [u8; 32])>,
    pub challenge: Option<[u8; 32]>,
    /// `iommu_vendor`, `iommu_enforcing`, `unconfined_grants`,
    /// `identity_devices`.
    pub dma: (u8, bool, u32, u32),
    /// A posture to bind into `extraData` in place of the document's.
    pub bound_dma: Option<(u8, bool, u32, u32)>,
    pub quote: fn(&mut Quote),
    pub after_sign: fn(&mut Vec<u8>),
    pub sig: fn([u8; 32], [u8; 32]) -> Vec<u8>,
    pub signing_key: u8,
    pub doc_key: u8,
    pub doc: fn(&mut Parts),
    pub bytes: fn(&mut Vec<u8>),
    pub entries: Vec<Entry>,
    pub root: Option<[u8; 32]>,
    pub cert: fn(&mut Cert),
    pub cert_signer: u8,
    pub cert_bytes: fn(&mut Vec<u8>),
    pub releases: Vec<Release>,
    pub revoke: bool,
    /// Boot applications the policy admits in PCR 4 besides the loader.
    pub boot_apps: Vec<[u8; 32]>,
    /// The keys that sign `HOST_PROOF`.
    pub host_key: u8,
}
