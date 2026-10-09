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

extern crate alloc;

use alloc::vec::Vec;

use super::error::LocalBuildError;
use super::identity::with_identity;
use super::trailer::{context, decode, encode, tag};

/// Tag `elf` so this machine may run it holding `granted_caps`.
pub fn sign(elf: &[u8], granted_caps: u64) -> Result<Vec<u8>, LocalBuildError> {
    /*
     * A tag made here admits a capsule holding what it names, so it names
     * nothing beyond what every process inherits. Minting LocalSign, or any
     * scarce right, would let a signer hand out authority it cannot be asked
     * to justify.
     */
    if granted_caps & !crate::process::core::AMBIENT_CAPS != 0 {
        return Err(LocalBuildError::ScarceCapability);
    }
    let digest = crate::security::capsule_attest::measure::measure(elf);
    let ctx = context(&digest, granted_caps);
    with_identity(|id| encode(&id.root, &tag(&id.key, &ctx))).ok_or(LocalBuildError::NoIdentity)
}

/// Whether `trailer` is this machine's tag for the image measured as `digest`,
/// holding `granted_caps`, under `root`. Returns the measurement when it is.
///
/// Refused unless `root` is this machine's own: a root enrolled from
/// elsewhere names a key this kernel does not hold, so no tag here can be
/// checked against it.
pub fn verify(trailer: &[u8], digest: &[u8; 32], granted_caps: u64, root: &[u8; 32]) -> Option<[u8; 32]> {
    let (carried_root, carried_tag) = decode(trailer)?;
    if carried_root != *root {
        return None;
    }
    let ctx = context(digest, granted_caps);
    let ok = with_identity(|id| {
        /* blake3::Hash compares in constant time. */
        id.root == *root && tag(&id.key, &ctx) == blake3::Hash::from(carried_tag)
    })?;
    if !ok {
        return None;
    }
    let mut measurement = [0u8; 32];
    measurement.copy_from_slice(&ctx[..32]);
    Some(measurement)
}
