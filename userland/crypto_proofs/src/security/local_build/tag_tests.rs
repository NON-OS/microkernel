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

//! The local trailer as the kernel writes and reads it: one shape, a tag that
//! changes with every input it binds, and nothing else accepted.

use super::trailer::{context, decode, encode, tag, MAGIC, TRAILER_LEN};

const KEY: [u8; 32] = [0x11; 32];
const OTHER_KEY: [u8; 32] = [0x22; 32];
const ROOT: [u8; 32] = [0x33; 32];
const ELF: &[u8] = b"\x7fELF a capsule built on this machine";

#[test]
fn a_trailer_reads_back_as_it_was_written() {
    let t = tag(&KEY, &context(blake3::hash(ELF).as_bytes(), 0));
    let bytes = encode(&ROOT, &t);
    assert_eq!(bytes.len(), TRAILER_LEN);
    assert!(bytes.starts_with(MAGIC));
    assert_eq!(decode(&bytes), Some((ROOT, *t.as_bytes())));
}

#[test]
fn any_other_shape_is_refused() {
    let bytes = encode(&ROOT, &tag(&KEY, &context(blake3::hash(ELF).as_bytes(), 0)));
    assert_eq!(decode(&bytes[..TRAILER_LEN - 1]), None);
    let mut long = bytes.clone();
    long.push(0);
    assert_eq!(decode(&long), None);
    let mut wrong_magic = bytes.clone();
    wrong_magic[0] ^= 1;
    assert_eq!(decode(&wrong_magic), None);
    assert_eq!(decode(&[]), None);
}

#[test]
fn the_context_is_the_path_leafs_layout() {
    let caps = 0x0102_0304_0506_0708u64;
    let ctx = context(blake3::hash(ELF).as_bytes(), caps);
    assert_eq!(&ctx[..32], blake3::hash(ELF).as_bytes());
    assert_eq!(&ctx[32..40], &caps.to_be_bytes());
    assert_eq!(&ctx[40..48], &super::super::capsule_attest::layout::POLICY_EPOCH.to_be_bytes());
}

#[test]
fn the_tag_binds_the_bytes_the_capabilities_and_the_key() {
    let base = tag(&KEY, &context(blake3::hash(ELF).as_bytes(), 0));
    assert_eq!(base, tag(&KEY, &context(blake3::hash(ELF).as_bytes(), 0)));
    assert_ne!(base, tag(&KEY, &context(blake3::hash(b"\x7fELF another capsule").as_bytes(), 0)));
    assert_ne!(base, tag(&KEY, &context(blake3::hash(ELF).as_bytes(), 1)));
    assert_ne!(base, tag(&OTHER_KEY, &context(blake3::hash(ELF).as_bytes(), 0)));
}

#[test]
fn the_tag_is_not_an_unkeyed_hash_of_the_context() {
    let ctx = context(blake3::hash(ELF).as_bytes(), 0);
    let t = tag(&KEY, &ctx);
    assert_ne!(t, blake3::hash(&ctx));
    assert_ne!(t, blake3::keyed_hash(&KEY, &ctx));
}
