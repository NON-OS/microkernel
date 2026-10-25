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

//! One capsule's PCR 14 event, as bytes: the 65-byte body the kernel
//! extends, with its kind from `kind.rs`; the digest of that body under each
//! bank the firmware log declares; and the `TCG_PCR_EVENT2` a verifier
//! replays. Pure, so a proof
//! can hold the bytes to what a client reads.

use alloc::vec::Vec;

/// Outside every value TCG PC Client PFP 1.06 r52, Table 27 assigns, in the
/// space it leaves to application and OS use.
pub const EV_NONOS_CAPSULE: u32 = 0x4E4F_4E01;
pub const PCR_CAPSULE: u32 = 14;
pub const BODY_LEN: usize = 65;

pub use super::algs::{ALG_SHA1, ALG_SHA256, ALG_SHA384};

pub fn body(cert: &[u8; 32], verifier: &[u8; 32], kind: u8) -> [u8; BODY_LEN] {
    let mut b = [0u8; BODY_LEN];
    b[..32].copy_from_slice(cert);
    b[32..64].copy_from_slice(verifier);
    b[64] = kind;
    b
}

pub use super::algs::digest;

/// Each bank with its digest of the body.
pub type Digests = Vec<(u16, Vec<u8>)>;

/// The event as the log carries it, little-endian, and the digests the TPM
/// is given, bank by bank in the same order.
pub fn event(banks: &[u16], body: &[u8; BODY_LEN]) -> Option<(Vec<u8>, Digests)> {
    let digests: Digests =
        banks.iter().map(|a| digest(*a, body).map(|d| (*a, d))).collect::<Option<_>>()?;
    let mut e = PCR_CAPSULE.to_le_bytes().to_vec();
    e.extend_from_slice(&EV_NONOS_CAPSULE.to_le_bytes());
    e.extend_from_slice(&(digests.len() as u32).to_le_bytes());
    for (alg, d) in &digests {
        e.extend_from_slice(&alg.to_le_bytes());
        e.extend_from_slice(d);
    }
    e.extend_from_slice(&(BODY_LEN as u32).to_le_bytes());
    e.extend_from_slice(body);
    Some((e, digests))
}
