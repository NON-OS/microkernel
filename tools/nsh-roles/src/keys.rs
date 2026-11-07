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

//! Keys, as nsh-enrol keeps them: a secret file the key signs from, and a
//! public file, the Ed25519 key then the ML-DSA-65 key, 1984 bytes. A key
//! id is SHA-256 of the public key.

use crate::error::Error;
use nsh_enrol::keys::PUBLIC_FILE_LEN;
use nsh_enrol::{files, AuthorityKey};

pub type Public = Box<[u8; PUBLIC_FILE_LEN]>;

pub fn secret(path: &str) -> Result<AuthorityKey, Error> {
    let mut b = files::read(path)?;
    let key = AuthorityKey::from_file(&b);
    nsh_enrol::keys::wipe(&mut b);
    Ok(key?)
}

pub fn public(path: &str) -> Result<Public, Error> {
    Ok(files::read_exact::<PUBLIC_FILE_LEN>(path)?)
}

pub fn id(public: &[u8]) -> [u8; 32] {
    nonos_hash::sha256(public)
}

/// SHA-512 of a public key: the subject a grant names.
pub fn subject(public: &[u8]) -> [u8; 64] {
    nonos_hash::sha512(public)
}

/// Keys in ascending key id order, as a table lists them, each once.
pub fn sorted(mut keys: Vec<Public>) -> Result<Vec<Public>, Error> {
    keys.sort_by_key(|k| id(&k[..]));
    keys.dedup_by_key(|k| id(&k[..]));
    Ok(keys)
}
