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

//! The loader knows its own disk by the GPT entry that carries the GUID its
//! volume's Hard Drive node names, as the kernel does, and not only by the
//! device path. Headers and entries as UEFI 2.10, 5.3 lays them out.

use crate::loader::gpt_entry::{entries, lists, ENTRY_BYTES_MAX};

fn header(lba: u64, count: u32, size: u32) -> Vec<u8> {
    let mut h = vec![0u8; 512];
    h[..8].copy_from_slice(b"EFI PART");
    h[72..80].copy_from_slice(&lba.to_le_bytes());
    h[80..84].copy_from_slice(&count.to_le_bytes());
    h[84..88].copy_from_slice(&size.to_le_bytes());
    h
}

fn table(guids: &[[u8; 16]]) -> Vec<u8> {
    let mut t = vec![0u8; 128 * guids.len()];
    for (i, g) in guids.iter().enumerate() {
        t[128 * i + 16..128 * i + 32].copy_from_slice(g);
    }
    t
}

#[test]
fn the_entries_are_where_the_header_says() {
    assert_eq!(entries(&header(2, 128, 128)), Some((2, 128, 128)), "every install's table");
    assert_eq!(entries(&header(1024, 4, 256)), Some((1024, 4, 256)));
}

#[test]
fn a_header_this_does_not_read_is_none() {
    let mut not_gpt = header(2, 128, 128);
    not_gpt[0] = b'X';
    assert_eq!(entries(&not_gpt), None, "no signature");
    assert_eq!(entries(&header(2, 128, 100)), None, "entries under 128 bytes");
    assert_eq!(entries(&header(2, 0, 128)), None, "no entries");
    assert_eq!(entries(&header(1, 128, 128)), None, "entries over the header");
    let past = (ENTRY_BYTES_MAX / 128 + 1) as u32;
    assert_eq!(entries(&header(2, past, 128)), None, "more than is read");
    assert_eq!(entries(&header(2, u32::MAX, u32::MAX)), None, "a size that overflows");
    assert_eq!(entries(&[0u8; 64]), None, "short");
}

#[test]
fn a_disk_lists_the_partition_by_its_own_guid() {
    let (stick, other) = ([7u8; 16], [9u8; 16]);
    assert!(lists(&table(&[other, stick]), 128, &stick), "the second entry");
    assert!(!lists(&table(&[other]), 128, &stick), "the installed disk");
    // An unused slot is all zero, and a zero GUID names nothing.
    assert!(!lists(&table(&[[0; 16]]), 128, &[0; 16]));
    // The type GUID, bytes 0..16, is not the partition's own.
    let mut t = table(&[other]);
    t[..16].copy_from_slice(&stick);
    assert!(!lists(&t, 128, &stick));
}
