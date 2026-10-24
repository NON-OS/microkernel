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

//! ramfs's encrypted files (fs/ramfs/filesystem/{core,ops,ops_write}.rs)
//! when the key or nonce draw refuses: no file is stored. Two findings are
//! pinned here as the code has them (see each test).

use super::ramfs::NO;
use crate::fs::ramfs::filesystem::NonosFilesystem;
use crate::stand_in::rng;

#[test]
fn create_refuses_with_no_randomness_and_stores_nothing() {
    let fs = NonosFilesystem::new_encrypted();
    rng::refuse();
    assert_eq!(fs.create_file("/a", b"secret"), Err(NO));
    assert!(fs.files.read().is_empty());
    assert!(fs.file_keys.read().is_empty());
    assert_eq!(fs.stats.read().encryptions, 0);
}

/// Finding: with the salt drawn and the nonce refused, no file is stored
/// but the file's key stays in `file_keys`.
#[test]
fn create_refusing_at_the_nonce_stores_no_file_but_keeps_the_key() {
    let fs = NonosFilesystem::new_encrypted();
    rng::fail_from(Some(2));
    assert_eq!(fs.create_file("/a", b"secret"), Err(NO));
    assert!(fs.files.read().is_empty());
    assert_eq!(fs.file_keys.read().len(), 1);
}

#[test]
fn a_file_is_stored_sealed_under_a_drawn_nonce_and_reads_back() {
    let fs = NonosFilesystem::new_encrypted();
    rng::fail_from(None);
    fs.create_file("/a", b"secret").expect("created");
    fs.create_file("/b", b"secret").expect("created");
    let (a, b) = (fs.files.read()["/a"].data.clone(), fs.files.read()["/b"].data.clone());
    let given = rng::given();
    assert_eq!((given[1].as_slice(), given[3].as_slice()), (&a[..12], &b[..12]));
    assert!(!a.windows(6).any(|w| w == b"secret"));
    assert_ne!(a, b);
    assert_eq!(fs.read_file("/a"), Ok(b"secret".to_vec()));
    assert_eq!(rng::weak(), 0);
}

/// A write whose nonce draw refuses leaves the file's old contents whole:
/// the new contents are sealed before the old are wiped.
#[test]
fn a_write_refused_for_its_nonce_keeps_the_old_contents() {
    let fs = NonosFilesystem::new_encrypted();
    rng::fail_from(None);
    fs.create_file("/a", b"old").expect("created");
    rng::refuse();
    assert_eq!(fs.write_file("/a", b"new"), Err(NO));
    assert_eq!(fs.write_or_create("/a", b"new"), Err(NO));
    rng::fail_from(None);
    assert_eq!(fs.read_file("/a"), Ok(b"old".to_vec()));
    assert_eq!(fs.write_file("/a", b"new"), Ok(()));
    assert_eq!(fs.read_file("/a"), Ok(b"new".to_vec()));
}
