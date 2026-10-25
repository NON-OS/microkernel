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

//! The key store (security/crypto/key_management/{store,ops,rotation}.rs):
//! its master key, new keys and rotated keys are drawn only from the seeded
//! RNG; a refused draw adds no key and leaves the old one in use. The store
//! is one global: the first test to take its turn proves that it will not
//! start with no randomness, then starts it.

use std::sync::{Mutex, MutexGuard};

use crate::security::crypto::key_management::{
    generate_key, init, key_count, use_key, KeyError, KeyType, KeyUsage,
};
use crate::stand_in::rng;

static STARTED: Mutex<bool> = Mutex::new(false);

pub(super) fn turn() -> MutexGuard<'static, bool> {
    let mut started = STARTED.lock().unwrap_or_else(|p| p.into_inner());
    if !*started {
        rng::refuse();
        assert_eq!(init(), Err(KeyError::CryptoError));
        assert_eq!(new_key(), Err(KeyError::NotInitialized));
        rng::fail_from(None);
        assert_eq!(init(), Ok(()));
        *started = true;
    }
    started
}

pub(super) fn new_key() -> Result<u64, KeyError> {
    generate_key("k".into(), KeyType::ChaCha20, KeyUsage::signing(), 9)
}

#[test]
fn a_key_is_not_made_with_no_randomness() {
    let _turn = turn();
    let count = key_count();
    rng::refuse();
    assert_eq!(new_key(), Err(KeyError::CryptoError));
    assert_eq!(key_count(), count);
    assert_eq!(rng::weak(), 0);
}

#[test]
fn each_key_is_drawn_fresh() {
    let _turn = turn();
    rng::fail_from(None);
    let (a, b) = (new_key().expect("key"), new_key().expect("key"));
    let given = rng::given();
    assert_eq!((use_key(a, 9), use_key(b, 9)), (Ok(given[0].clone()), Ok(given[1].clone())));
    assert_ne!(given[0], given[1]);
}
