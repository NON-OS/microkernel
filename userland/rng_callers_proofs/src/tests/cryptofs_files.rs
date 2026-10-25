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

//! cryptofs's files with the RNG answering and refusing part way: a salt
//! and a nonce drawn per file, and round trips.

use super::cryptofs::{entry, next_inode, turn};
use crate::fs::cryptofs::crypto::derive_key;
use crate::fs::cryptofs::{
    create_encrypted_file, create_ephemeral_file, get_cryptofs, read_encrypted, CryptoFsError,
};
use crate::stand_in::rng;

#[test]
fn new_files_take_distinct_drawn_salts() {
    let _turn = turn();
    rng::fail_from(None);
    let a = create_encrypted_file(0, "/f/a", &[]).expect("made");
    let b = create_encrypted_file(0, "/f/b", &[]).expect("made");
    assert_ne!(a, b);
    let ((ka, sa, _), (kb, sb, _)) = (entry("/f/a"), entry("/f/b"));
    assert_eq!(rng::given(), [sa.to_vec(), sb.to_vec()]);
    assert_eq!(ka, derive_key("/f/a", &sa));
    assert_ne!((ka, sa), (kb, sb));
}

/// The nonce is drawn first, before any inode is taken; the salt after.
#[test]
fn an_ephemeral_file_refused_at_either_draw_is_not_made() {
    let _turn = turn();
    for (from, spent) in [(1, 0), (2, 1)] {
        let before = next_inode();
        rng::fail_from(Some(from));
        let made = create_ephemeral_file("/f/eph", b"secret");
        assert_eq!(made, Err(CryptoFsError::RngFailed));
        assert!(!get_cryptofs().expect("cryptofs").exists("/f/eph"));
        assert_eq!(next_inode(), before + spent);
    }
}

#[test]
fn an_ephemeral_file_is_sealed_under_drawn_bytes_and_reads_back() {
    let _turn = turn();
    rng::fail_from(None);
    create_ephemeral_file("/f/rt", b"secret").expect("made");
    let (_, salt, sealed) = entry("/f/rt");
    assert_eq!(rng::given(), [sealed[..12].to_vec(), salt.to_vec()]);
    assert!(!sealed.windows(6).any(|w| w == b"secret"));
    assert_eq!(read_encrypted("/f/rt"), Ok(b"secret".to_vec()));
}
