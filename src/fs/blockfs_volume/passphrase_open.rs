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

//! Opening a passphrase-keyed volume. The sealed volume key is opened
//! first; a passphrase that does not open it is refused with nothing read
//! from the volume and nothing written anywhere.

use super::error::VolumeError;
use super::key_header::Keyed;
use super::key_header_io::read_key_header;
use super::key_seal::unseal_key;
use super::mount_or_format::mount_or_format;
use super::opened::install;
use super::passphrase_key::stretch;
use super::plan_types::Plan;
use crate::crypto::constant_time::secure_zero;

pub(super) fn open_keyed(passphrase: &[u8], plan: &Plan) -> Result<(), VolumeError> {
    let sealed = match read_key_header()? {
        Some(Keyed::Passphrase(sealed)) => sealed,
        _ => return Err(VolumeError::NotPassphraseKeyed),
    };
    let mut kek = stretch(passphrase, &sealed)?;
    let unsealed = unseal_key(&kek, &sealed);
    secure_zero(&mut kek);
    let Some(mut key) = unsealed else {
        crate::log::warn!("[DATA] the passphrase does not open the volume key");
        return Err(VolumeError::WrongPassphrase);
    };
    /*
     * The right passphrase over a blank ring is a create cut off before
     * it formatted; `mount_or_format` finishes it. Over any other ring it
     * only mounts.
     */
    let opened = mount_or_format(&key, plan, &Keyed::Passphrase(sealed));
    let result = opened.map(|mount| install(plan, key, mount));
    secure_zero(&mut key);
    result
}
