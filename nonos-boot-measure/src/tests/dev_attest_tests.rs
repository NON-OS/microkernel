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

//! A development image's loader trailer is the path alone. It is held to the
//! same context and root as a v4 trailer: the right loader in the right slot
//! passes, anything else is refused. Without the feature, as in every
//! release, the same path is refused.

use nonos_attest_path::{boot_context, digest_to_bytes, leaf, pad_leaf, Kind, Poseidon, Tree};

use crate::gate::{membership, BootError, BOOT_EPOCH, DEPTH};

fn enrolled(measurement: &[u8; 32]) -> ([u8; 32], Vec<u8>) {
    let h = Poseidon::new();
    let mut leaves: Vec<_> = (0..1u32 << DEPTH).map(|i| pad_leaf(&h, &[7; 32], i)).collect();
    leaves[0] = leaf(&h, Kind::Bootloader, &boot_context(measurement, BOOT_EPOCH)).unwrap();
    let tree = Tree::commit(&h, &leaves, DEPTH).unwrap();
    (digest_to_bytes(&tree.root().unwrap()), tree.trailer(0).unwrap())
}

#[cfg(feature = "dev-attest")]
#[test]
fn the_enrolled_loader_passes_on_its_path() {
    let m = [0x11; 32];
    let (root, path) = enrolled(&m);
    assert_eq!(membership(&root, &m, &path), Ok(()));
}

#[cfg(feature = "dev-attest")]
#[test]
fn another_loader_is_refused() {
    let (root, path) = enrolled(&[0x11; 32]);
    assert_eq!(membership(&root, &[0x12; 32], &path), Err(BootError::Path));
}

#[cfg(feature = "dev-attest")]
#[test]
fn another_root_is_refused() {
    let m = [0x11; 32];
    let (_, path) = enrolled(&m);
    let (other, _) = enrolled(&[0x22; 32]);
    assert_eq!(membership(&other, &m, &path), Err(BootError::Path));
}

#[cfg(feature = "dev-attest")]
#[test]
fn a_flipped_byte_in_the_path_is_refused() {
    let m = [0x11; 32];
    let (root, mut path) = enrolled(&m);
    let last = path.len() - 1;
    path[last] ^= 1;
    assert_ne!(membership(&root, &m, &path), Ok(()));
}

#[cfg(not(feature = "dev-attest"))]
#[test]
fn a_release_refuses_the_path_alone() {
    let m = [0x11; 32];
    let (root, path) = enrolled(&m);
    assert_eq!(membership(&root, &m, &path), Err(BootError::Trailer));
}
