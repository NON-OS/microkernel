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

//! cryptofs's writes and key rotation (fs/cryptofs/ops.rs): with a draw
//! refused, the file keeps its key, salt and contents; with the draws
//! answered, it is sealed again, under a key from the new salt.

use super::cryptofs::{entry, turn};
use crate::fs::cryptofs::crypto::derive_key;
use crate::fs::cryptofs::{
    create_ephemeral_file, read_encrypted, rotate_file_key, write_encrypted, CryptoFsError,
};
use crate::stand_in::rng;

#[test]
fn a_rotation_refused_at_either_draw_changes_nothing() {
    let _turn = turn();
    rng::fail_from(None);
    create_ephemeral_file("/k/a", b"secret").expect("made");
    let before = entry("/k/a");
    for (from, only) in rng::cases(2) {
        rng::fail_at(Some(from), only);
        assert_eq!(rotate_file_key("/k/a"), Err(CryptoFsError::RngFailed));
        assert_eq!(entry("/k/a"), before);
    }
    assert_eq!(read_encrypted("/k/a"), Ok(b"secret".to_vec()));
}

#[test]
fn a_rotation_seals_again_under_a_key_from_a_drawn_salt() {
    let _turn = turn();
    rng::fail_from(None);
    create_ephemeral_file("/k/b", b"secret").expect("made");
    let (old_key, old_salt, _) = entry("/k/b");
    rng::fail_from(None);
    rotate_file_key("/k/b").expect("rotated");
    let (key, salt, sealed) = entry("/k/b");
    assert_eq!(rng::given(), [sealed[..12].to_vec(), salt.to_vec()]);
    assert_ne!((key, salt), (old_key, old_salt));
    assert_eq!(key, derive_key("/k/b", &salt));
    assert_eq!(read_encrypted("/k/b"), Ok(b"secret".to_vec()));
    assert_eq!(rng::weak(), 0);
}

#[test]
fn a_write_refused_for_its_nonce_leaves_the_old_contents() {
    let _turn = turn();
    rng::fail_from(None);
    create_ephemeral_file("/f/w", b"old").expect("made");
    rng::refuse();
    assert_eq!(write_encrypted("/f/w", b"new"), Err(CryptoFsError::RngFailed));
    assert_eq!(read_encrypted("/f/w"), Ok(b"old".to_vec()));
    rng::fail_from(None);
    write_encrypted("/f/w", b"new").expect("written");
    assert_eq!(read_encrypted("/f/w"), Ok(b"new".to_vec()));
    assert_eq!(rng::weak(), 0);
}
