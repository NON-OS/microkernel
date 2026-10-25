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

//! NOS1 packages as nonos-pack lays them out: header, four entries, the sections, a two-signature trailer.

/// The cap pkg_query.rs reads a package under (its MAX_PACKAGE).
pub const MAX_PACKAGE: usize = 16 * 1024 * 1024;

pub const MANIFEST: &[u8] = b"manifest-bytes";
pub const CERT: &[u8] = b"id-cert-bytes";
pub const ZK: &[u8] = b"zk-trailer-proof";
const SIGS: [(u8, &[u8]); 2] = [(1, &[0xED; 64]), (2, &[0x3D; 96])];
const TABLE_END: usize = 8 + 4 * 16;

pub fn elf(n: usize) -> Vec<u8> {
    vec![0xE1; n]
}

/// Kinds 1 to 4 (manifest, elf, id_cert, zk_trailer), the elf elf_len bytes,
/// then the trailer of two tagged signatures running to the end.
pub fn package(elf_len: usize) -> Vec<u8> {
    let elf = elf(elf_len);
    let sections: [&[u8]; 4] = [MANIFEST, &elf, CERT, ZK];
    let mut p = [&b"NOS1"[..], &1u16.to_be_bytes(), &4u16.to_be_bytes()].concat();
    let mut offset = TABLE_END as u32;
    for (kind, s) in (1u16..).zip(sections) {
        let len = s.len() as u32;
        let entry = [kind.to_be_bytes(), [0; 2]].concat();
        p.extend(
            [entry, offset.to_be_bytes().to_vec(), len.to_be_bytes().to_vec(), vec![0; 4]].concat(),
        );
        offset += len;
    }
    sections.iter().for_each(|s| p.extend_from_slice(s));
    p.push(SIGS.len() as u8);
    for (tag, sig) in SIGS {
        p.push(tag);
        p.extend_from_slice(&(sig.len() as u16).to_be_bytes());
        p.extend_from_slice(sig);
    }
    p
}

/// The elf length that makes a package exactly n bytes.
pub fn elf_len_of(n: usize) -> usize {
    n - package(0).len()
}

pub fn package_of(n: usize) -> Vec<u8> {
    let p = package(elf_len_of(n));
    assert_eq!(p.len(), n);
    p
}

/// A package of MAX_PACKAGE bytes that verifies, and one byte after it. Cut
/// at MAX_PACKAGE it is a whole package again, so only its length refuses it.
pub fn past_max() -> Vec<u8> {
    let mut p = package_of(MAX_PACKAGE);
    p.push(0);
    p
}
