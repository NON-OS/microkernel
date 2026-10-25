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

//! ramfs's file key and nonce (fs/ramfs/filesystem/{key,crypto}.rs): each
//! drawn only from the seeded RNG, refused with `IoError("no randomness")`.

use core::sync::atomic::{AtomicU64, Ordering};

use crate::fs::ramfs::error::FsError;
use crate::fs::ramfs::filesystem::crypto::generate_nonce;
use crate::fs::ramfs::filesystem::key::{derive_key, FileKey};
use crate::stand_in::rng;

pub(super) const NO: FsError = FsError::IoError("no randomness");

#[test]
fn a_file_key_refuses_with_no_randomness() {
    rng::refuse();
    assert_eq!(FileKey::new("/a").err(), Some(NO));
    assert_eq!(rng::weak(), 0);
}

#[test]
fn a_file_key_is_derived_from_a_drawn_salt() {
    rng::fail_from(None);
    let a = FileKey::new("/a").expect("key");
    let b = FileKey::new("/a").expect("key");
    assert_eq!(rng::given(), [a.salt.to_vec(), b.salt.to_vec()]);
    assert_eq!(a.key, derive_key("/a", &a.salt));
    assert_ne!(a.salt, b.salt);
    assert_ne!(a.key, b.key);
}

/// The count moves even when the draw refuses: it counts attempts, and no
/// nonce is made from it.
#[test]
fn the_nonce_refuses_with_no_randomness() {
    let count = AtomicU64::new(0);
    rng::refuse();
    assert_eq!(generate_nonce(&count), Err(NO));
    assert_eq!(count.load(Ordering::SeqCst), 1);
    assert_eq!(rng::weak(), 0);
}

#[test]
fn each_nonce_is_drawn_fresh() {
    let count = AtomicU64::new(0);
    rng::fail_from(None);
    let a = generate_nonce(&count).expect("nonce");
    let b = generate_nonce(&count).expect("nonce");
    assert_eq!(rng::given(), [a.to_vec(), b.to_vec()]);
    assert_ne!(a, b);
}
