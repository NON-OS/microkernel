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

//! A record withdrawn or replaced leaves its old bytes nowhere on the disk:
//! the extent it left is zeroed once the table no longer points at it.

use nonos_disk_map::STORE_BASE_LBA;
use nonos_libc::disk;

use super::blk::store_withdraw::withdraw;
use super::blk::store_write::append;
use super::fixture::{image, install, refs, three, Layout};
use super::run::{digests, load_counting};

/// Every sector of the store the fixtures can reach.
const SPAN: usize = 4096;

/// Bytes that repeat nowhere, unlike the fixtures' period-256 pattern, so a
/// match on the disk can only be this record.
fn secret(seed: u64, len: usize) -> Vec<u8> {
    let mut x = seed | 1;
    (0..len)
        .map(|_| {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x as u8
        })
        .collect()
}

fn on_disk(needle: &[u8]) -> bool {
    disk::get(STORE_BASE_LBA, SPAN).windows(needle.len()).any(|w| w == needle)
}

#[test]
fn a_withdrawn_record_leaves_none_of_its_bytes_on_the_disk() {
    for layout in [Layout::Installer, Layout::Packer] {
        let mut files = three();
        let secret = secret(77, 512);
        files[1] = (String::from("/nonos/consent/token"), secret.clone());
        install(&image(&refs(&files), layout));
        assert!(on_disk(&secret), "{layout:?}: the fixture holds it");
        assert_eq!(withdraw("/nonos/consent/token", &[0u8; 512]), Ok(()));
        assert!(!on_disk(&secret), "{layout:?}: the old extent still holds the record");
    }
}

#[test]
fn a_replaced_record_leaves_none_of_its_old_bytes_on_the_disk() {
    let mut files = three();
    let (old, new) = (secret(5, 3000), secret(6, 3000));
    files[0] = (String::from("/nonos/wallet/vault"), old.clone());
    install(&image(&refs(&files), Layout::Packer));
    assert_eq!(append("/nonos/wallet/vault", &new), Ok(()));
    assert!(!on_disk(&old[..64]), "the old vault is still on the disk");
    files[0].1 = new;
    assert_eq!(load_counting().map(|(f, r)| (digests(&f), r)), Ok((digests(&files), 0)));
}
