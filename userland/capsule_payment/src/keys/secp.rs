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

//! The meter's secp256k1 key across boots. Channels are bound to its address,
//! so the key must come back: a meter that quietly made a new one would leave
//! every open channel's deposit to a key nobody holds. A sealed key that does
//! not open is refused, never replaced.

use nonos_libc::crypto_random;
use nonos_payment_proto::{EAGAIN, EIO, ENOPERSIST};
use nonos_seal::NONCE_LEN;
use nonos_vault::OVERHEAD;

use crate::disk::{self, Loaded, KEY_PATH};
use crate::keyring;
use crate::wipe::wipe;

const RECORD: &[u8] = b"meter.secp256k1";
const BLOB_LEN: usize = 32 + OVERHEAD;

/// The key's id in the keyring, from the sealed copy or, on a machine that
/// never had one, a new key sealed and kept before it is used.
pub fn restore(root: &[u8; 32]) -> Result<u32, i32> {
    match disk::load(KEY_PATH, BLOB_LEN) {
        Loaded::Silent => Err(EAGAIN),
        Loaded::Bytes(blob) => {
            let mut secret = [0u8; 32];
            nonos_vault::open(root, RECORD, &blob, &mut secret).map_err(|_| EIO)?;
            let id = keyring::import(&secret);
            wipe(&mut secret);
            id
        }
        Loaded::Absent if !disk::kept() => Err(ENOPERSIST),
        Loaded::Absent => fresh(root),
    }
}

fn fresh(root: &[u8; 32]) -> Result<u32, i32> {
    let id = keyring::generate()?;
    let mut nonce = [0u8; NONCE_LEN];
    if crypto_random(nonce.as_mut_ptr(), NONCE_LEN) < NONCE_LEN as i64 {
        return Err(EAGAIN);
    }
    let mut secret = keyring::export(id)?;
    let mut blob = [0u8; BLOB_LEN];
    let sealed = nonos_vault::seal(root, RECORD, &secret, &nonce, &mut blob);
    wipe(&mut secret);
    sealed.map_err(|_| EAGAIN)?;
    disk::save(KEY_PATH, &blob)?;
    Ok(id)
}
