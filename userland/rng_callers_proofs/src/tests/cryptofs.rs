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

//! cryptofs's nonce and its new files' salts (fs/cryptofs/{crypto,ops}.rs):
//! drawn only from the seeded RNG, refused with `RngFailed`. The file
//! system is one global; these tests take turns on it, each with its paths.

use std::sync::{Mutex, MutexGuard};

use crate::fs::cryptofs::{
    create_encrypted_file, crypto::generate_nonce, get_cryptofs, init_cryptofs, CryptoFsError,
};
use crate::stand_in::rng;

static TURN: Mutex<()> = Mutex::new(());

pub(super) fn turn() -> MutexGuard<'static, ()> {
    init_cryptofs(64, 4096).expect("cryptofs");
    TURN.lock().unwrap_or_else(|p| p.into_inner())
}

pub(super) fn next_inode() -> u64 {
    let fs = get_cryptofs().expect("cryptofs");
    let inode = fs.inner.read().next_inode.load(core::sync::atomic::Ordering::SeqCst);
    inode
}

/// The key, salt and sealed bytes kept for `path`.
pub(super) fn entry(path: &str) -> ([u8; 32], [u8; 16], Vec<u8>) {
    let inner = get_cryptofs().expect("cryptofs").inner.read();
    let e = &inner.files[path];
    (e.key, e.salt, e.encrypted.clone())
}

#[test]
fn the_nonce_refuses_with_no_randomness() {
    rng::refuse();
    assert_eq!(generate_nonce(), Err(CryptoFsError::RngFailed));
    assert_eq!(rng::weak(), 0);
}

#[test]
fn each_nonce_is_drawn_fresh() {
    rng::fail_from(None);
    let (a, b) = (generate_nonce().expect("nonce"), generate_nonce().expect("nonce"));
    assert_eq!(rng::given(), [a.to_vec(), b.to_vec()]);
    assert_ne!(a, b);
}

/// Finding: the inode number is taken before the salt is drawn, so a
/// refused create spends one. Nothing else is kept.
#[test]
fn a_new_file_refuses_with_no_randomness_and_is_not_made() {
    let _turn = turn();
    let before = next_inode();
    rng::refuse();
    assert_eq!(create_encrypted_file(0, "/r/new", &[]), Err(CryptoFsError::RngFailed));
    assert!(!get_cryptofs().expect("cryptofs").exists("/r/new"));
    assert_eq!(next_inode(), before + 1);
    assert_eq!(rng::weak(), 0);
}
