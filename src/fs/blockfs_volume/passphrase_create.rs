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

//! Creating a passphrase-keyed volume: only over a header ring that has
//! never been written, so no volume under any key is formatted over.

use super::error::VolumeError;
use super::key_header::{Keyed, Sealed};
use super::key_seal::seal_key;
use super::mount_or_format::mount_or_format;
use super::opened::install;
use super::passphrase_key::stretch;
use super::plan_types::Plan;
use super::ring_blank::ring_blank;
use crate::crypto::constant_time::secure_zero;
use crate::crypto::rng::fill_random_bytes;
use crate::crypto::util::argon2::RECOMMENDED;

pub(super) fn create_keyed(passphrase: &[u8], plan: &Plan) -> Result<(), VolumeError> {
    if !ring_blank(plan.volume_base)? {
        crate::log::warn!("[DATA] a volume exists already; not creating one over it");
        return Err(VolumeError::VolumeExists);
    }
    let mut sealed =
        Sealed { params: RECOMMENDED, salt: [0; 32], nonce: [0; 12], key_and_tag: [0; 48] };
    fill_random_bytes(&mut sealed.salt);
    fill_random_bytes(&mut sealed.nonce);
    let mut key = [0u8; 32];
    fill_random_bytes(&mut key);
    let mut kek = stretch(passphrase, &sealed)?;
    let done = seal_key(&kek, &mut sealed, &key);
    secure_zero(&mut kek);
    if !done {
        secure_zero(&mut key);
        crate::log::warn!("[DATA] the volume key could not be sealed; nothing was written");
        return Err(VolumeError::Unopenable);
    }
    /*
     * The key header is written before the volume is formatted: a boot cut
     * off between the two finds a blank ring and a header, and a create or
     * an open with the same passphrase then finishes the volume.
     */
    let opened = mount_or_format(&key, plan, &Keyed::Passphrase(sealed));
    let result = opened.map(|mount| install(plan, key, mount));
    secure_zero(&mut key);
    result
}
