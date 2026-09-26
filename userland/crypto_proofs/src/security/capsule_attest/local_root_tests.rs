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

//! A trailer minted the way `local_build::sign` mints one, checked by the
//! verifier an enrolled root is sent to. The tree and the encoder are the
//! kernel's own files; only the context is restated here, and a mismatch
//! would fail the first test.

use super::against_pedersen::verify;
use crate::crypto::zk_kernel::{prove_enrolled, PedersenCommitment};

#[path = "../../../../../src/security/local_build/tree.rs"]
mod tree;

#[path = "../../../../../src/security/local_build/trailer.rs"]
mod mint;

pub(super) const ELF: &[u8] = b"\x7fELF an installed program";

fn ctx(elf: &[u8], caps: u64) -> [u8; 48] {
    let mut c = [0u8; 48];
    c[..32].copy_from_slice(blake3::hash(elf).as_bytes());
    c[32..40].copy_from_slice(&caps.to_be_bytes());
    c[40..48].copy_from_slice(&super::layout::POLICY_EPOCH.to_be_bytes());
    c
}

/// An identity from two fixed secrets, its root, and a trailer it minted.
pub(super) fn minted(secret: u8, elf: &[u8], caps: u64) -> ([u8; 32], alloc::vec::Vec<u8>) {
    let (x, r) = ([secret; 32], [secret ^ 0x5a; 32]);
    let root = tree::root_for(&PedersenCommitment::commit(&x, &r).commitment);
    let proof = prove_enrolled(&x, &r, 0, &tree::empty_siblings(), &root, &ctx(elf, caps))
        .expect("proof");
    (root, mint::encode(&proof).expect("trailer"))
}

#[test]
fn a_local_trailer_verifies_for_exactly_what_it_was_minted_for() {
    let (root, trailer) = minted(7, ELF, 0);
    assert_eq!(trailer.len(), mint::TRAILER_LEN);
    let got = verify(&trailer, ELF, 0, &root).map_err(|e| e.as_str());
    assert_eq!(got, Ok(*blake3::hash(ELF).as_bytes()));
}
