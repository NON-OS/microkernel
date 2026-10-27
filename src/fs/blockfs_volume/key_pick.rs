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

//! Which key opens the machine volume, by what its key header says. Way 1's
//! key, derived under PCR 9 alone, is new with every kernel, so every update
//! left the volume unreadable. A volume formatted now has a random key wrapped
//! (way 3, key_wrap.rs) under a key the TPM gives only a kernel the release
//! approved on this machine's firmware (tpm/device_secret/approved_key.rs): a
//! signed update keeps it, a firmware change does not, and a new bootloader
//! needs a handover (handover/). Way 1 opens as it did.

use super::error::VolumeError;
use super::key_header::{Keyed, Sealed};
use super::key_wrap::{unwrap, wrap};
use crate::crypto::constant_time::secure_zero;
use crate::crypto::rng::fill_random_bytes_secure;
use crate::security::tpm::device_secret::{approved_key, from_boot};
use crate::security::tpm::machine_key::derive_for_kernel;

const V1_LABEL: &[u8] = b"blockfs.data.v1"; // way 1's key
const V2_LABEL: &[u8] = b"blockfs.data.v2"; // way 3's wrapping key

/// Way 1's key, for a volume formatted under it.
pub(super) fn legacy() -> Result<[u8; 32], VolumeError> {
    derive_for_kernel(V1_LABEL).map_err(|e| {
        crate::log::warn!("[DATA] no machine key ({:?}); the data volume stays closed", e);
        VolumeError::MachineKey(e)
    })
}

/// A key and header for a blank volume: a random key wrapped under the
/// approved key, or, on a boot with no approval, way 1, said once.
pub(super) fn fresh() -> Result<([u8; 32], Keyed), VolumeError> {
    let Some(mut kek) = kek() else {
        crate::log::warn!("[DATA] no approved key: this volume will not outlive an update");
        return Ok((legacy()?, Keyed::Tpm));
    };
    let (mut key, mut nonce) = ([0u8; 32], [0u8; 12]);
    let drawn = fill_random_bytes_secure(&mut key).and(fill_random_bytes_secure(&mut nonce));
    let wrapped = drawn.ok().and_then(|_| wrap(&kek, &key, nonce));
    secure_zero(&mut kek);
    Ok((key, Keyed::Approved(wrapped.ok_or(VolumeError::Unopenable)?)))
}

/// The wrapping key, when this boot's chain is one the release approved.
pub(super) fn kek() -> Option<[u8; 32]> {
    from_boot().and_then(|a| approved_key(&a, V2_LABEL).ok())
}

/// Whether this boot's own key opens `wrapped` to `key`.
pub(super) fn opens_to(wrapped: &Sealed, key: &[u8; 32]) -> bool {
    let Some(mut kek) = kek() else {
        return false;
    };
    let mut opened = unwrap(&kek, wrapped);
    secure_zero(&mut kek);
    let same = opened.as_ref() == Some(key);
    if let Some(k) = opened.as_mut() {
        secure_zero(k);
    }
    same
}
