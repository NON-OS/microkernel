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

//! The owners, secret and store fillers the keyring owner tests share.

use crate::store::{KeyType, Store, StoreError};

pub(super) const A: u32 = 70;
pub(super) const B: u32 = 71;
pub(super) const SECRET: [u8; 32] = [0x11; 32];

pub(super) fn put(s: &mut Store, owner: u32) -> Result<u32, StoreError> {
    s.store(KeyType::Symmetric, &SECRET, owner, 0, 0)
}

/// Store keys for `owner` until the store refuses it.
pub(super) fn fill(s: &mut Store, owner: u32) -> Vec<u32> {
    let mut ids = Vec::new();
    while let Ok(id) = put(s, owner) {
        ids.push(id);
    }
    ids
}

/// Fill the whole store, one owner after another from `first`.
pub(super) fn fill_all(s: &mut Store, first: u32) -> Vec<(u32, Vec<u32>)> {
    let mut owners = Vec::new();
    let mut owner = first;
    loop {
        let ids = fill(s, owner);
        if ids.is_empty() {
            return owners;
        }
        owners.push((owner, ids));
        owner += 1;
    }
}

pub(super) fn kept(s: &mut Store, owner: u32, id: u32) -> bool {
    matches!(s.retrieve(id, owner), Ok(bytes) if bytes == SECRET)
}
