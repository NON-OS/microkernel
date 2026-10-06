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

//! Store images laid out the two ways a disk gets one, written here from the
//! format `nonos_disk_map` documents rather than by either writer, so a
//! proof does not take a writer's word for what a store looks like.
//!
//! The installer puts the first payload past a table of `MAX_ENTRIES`
//! (`TOC_SPAN`), where vfs appends. `tools/nonos-store-pack`, which packs
//! the live USB stick and the QEMU disk, puts it right after the table the
//! entries need.

use nonos_disk_map::{digest16, SECTOR_SIZE, STORE_BASE_LBA, TOC_SPAN};
use nonos_libc::disk;

pub const HEADER_LEN: usize = 32;
pub const ENTRY_LEN: usize = 128;
pub const NAME_LEN: usize = 96;
/// The store's first byte on the disk.
pub const BASE: u64 = STORE_BASE_LBA * SECTOR_SIZE as u64;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Layout {
    Installer,
    Packer,
}

pub fn span(bytes: usize) -> usize {
    bytes.div_ceil(SECTOR_SIZE) * SECTOR_SIZE
}

/// The container for `files`, from `STORE_BASE_LBA` on, with digests.
pub fn image(files: &[(&str, &[u8])], layout: Layout) -> Vec<u8> {
    let table = span(HEADER_LEN + ENTRY_LEN * files.len());
    let mut out = vec![0u8; if layout == Layout::Installer { TOC_SPAN } else { table }];
    out[..8].copy_from_slice(b"NONOSTR1");
    out[8..12].copy_from_slice(&1u32.to_le_bytes());
    out[12..16].copy_from_slice(&(files.len() as u32).to_le_bytes());
    for (i, (name, data)) in files.iter().enumerate() {
        let offset = BASE + out.len() as u64;
        let e = HEADER_LEN + ENTRY_LEN * i;
        out[e..e + name.len()].copy_from_slice(name.as_bytes());
        out[e + NAME_LEN..e + NAME_LEN + 8].copy_from_slice(&offset.to_le_bytes());
        out[e + NAME_LEN + 8..e + NAME_LEN + 16]
            .copy_from_slice(&(data.len() as u64).to_le_bytes());
        out[e + NAME_LEN + 16..e + ENTRY_LEN].copy_from_slice(&digest16(data));
        out.extend_from_slice(data);
        out.resize(span(out.len()), 0);
    }
    out
}

/// A fresh disk holding `image` at the store's sectors.
pub fn install(image: &[u8]) {
    disk::reset();
    disk::put(STORE_BASE_LBA, image);
}

/// Distinct bytes for file `seed`, `len` long.
pub fn bytes(seed: u8, len: usize) -> Vec<u8> {
    (0..len)
        .map(|i| (i as u32).wrapping_mul(2_654_435_761).wrapping_add(seed as u32) as u8)
        .collect()
}

/// Three files shaped like what a store keeps: a record, a program across
/// many read chunks, and an empty file.
pub fn three() -> Vec<(String, Vec<u8>)> {
    vec![
        (String::from("/nonos/setup/answers"), bytes(1, 300)),
        (String::from("/capsules/demo.elf"), bytes(2, 100_000)),
        (String::from("/nonos/empty"), Vec::new()),
    ]
}

pub fn refs(files: &[(String, Vec<u8>)]) -> Vec<(&str, &[u8])> {
    files.iter().map(|(n, d)| (n.as_str(), d.as_slice())).collect()
}

/// Where entry `i`'s field at `at` (within the 128-byte entry) sits in an
/// image.
pub fn field(i: usize, at: usize) -> usize {
    HEADER_LEN + ENTRY_LEN * i + at
}

pub fn offset_of(img: &[u8], i: usize) -> u64 {
    let f = field(i, NAME_LEN);
    u64::from_le_bytes(img[f..f + 8].try_into().unwrap())
}

pub fn set_offset(img: &mut [u8], i: usize, offset: u64) {
    let f = field(i, NAME_LEN);
    img[f..f + 8].copy_from_slice(&offset.to_le_bytes());
}

pub fn set_len(img: &mut [u8], i: usize, len: u64) {
    let f = field(i, NAME_LEN + 8);
    img[f..f + 8].copy_from_slice(&len.to_le_bytes());
}

/// Entry `i`'s name field, NUL padded.
pub fn set_name(img: &mut [u8], i: usize, name: &[u8]) {
    let f = field(i, 0);
    img[f..f + NAME_LEN].fill(0);
    img[f..f + name.len()].copy_from_slice(name);
}

/// Every file of `files` but the one at `skip`.
pub fn without(files: &[(String, Vec<u8>)], skip: usize) -> Vec<(String, Vec<u8>)> {
    files.iter().enumerate().filter(|(i, _)| *i != skip).map(|(_, f)| f.clone()).collect()
}
