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

//! A key's rotation (security/crypto/key_management/rotation.rs): the new
//! key is drawn only from the seeded RNG; refused, the old one stays.

use super::keys::{new_key, turn};
use crate::security::crypto::key_management::KeyError;
use crate::security::crypto::key_management::{get_key_info, key_count, rotate_key, use_key};
use crate::stand_in::rng;

#[test]
fn a_rotation_refused_keeps_the_old_key_in_use() {
    let _turn = turn();
    rng::fail_from(None);
    let old = new_key().expect("key");
    let material = use_key(old, 9);
    let count = key_count();
    rng::refuse();
    assert_eq!(rotate_key(old, 9), Err(KeyError::CryptoError));
    assert_eq!((key_count(), use_key(old, 9)), (count, material.clone()));
    rng::fail_from(None);
    let new = rotate_key(old, 9).expect("rotated");
    assert_eq!(get_key_info(old).map(|i| i.active), Ok(false));
    assert_eq!(use_key(new, 9), Ok(rng::given()[0].clone()));
    assert_ne!(use_key(new, 9), material);
}
