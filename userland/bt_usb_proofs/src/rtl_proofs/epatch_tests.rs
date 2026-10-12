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

//! The first patch layout: the patch whose chip id is the ROM version plus
//! one, cut before its last four octets, and every bound checked.

use super::fixture::{epatch, VERSION};
use super::rtl::epatch::patch_for;
use super::rtl::refused::Layout;

const A: &[u8] = &[0xa0; 40];
const B: &[u8] = &[0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8];
const C: &[u8] = &[0xc0; 12];

fn three() -> Vec<u8> {
    epatch(VERSION, &[(1, A), (2, B), (3, C)], 10)
}

#[test]
fn the_rom_version_plus_one_picks_the_patch() {
    let f = three();
    let p = patch_for(&f, 1).map_err(|_| ()).unwrap();
    assert_eq!(p.body, &B[..4], "its last four octets are left for the version");
    assert_eq!(p.version, VERSION);
    assert_eq!(patch_for(&f, 0).map_err(|_| ()).unwrap().body, &A[..36]);
    assert_eq!(patch_for(&f, 2).map_err(|_| ()).unwrap().body, &C[..8]);
}

#[test]
fn a_rom_with_no_patch_in_the_file_is_refused() {
    assert_eq!(patch_for(&three(), 3).err(), Some(Layout::NoPatch));
}

#[test]
fn a_file_without_the_signature_is_refused() {
    let mut f = three();
    f[0] = b'r';
    assert_eq!(patch_for(&f, 1).err(), Some(Layout::Signature));
    assert_eq!(patch_for(b"Realtech\0\0", 1).err(), Some(Layout::Signature));
}

#[test]
fn tables_longer_than_the_file_are_refused() {
    let mut f = three();
    f[12..14].copy_from_slice(&5000u16.to_le_bytes());
    assert_eq!(patch_for(&f, 1).err(), Some(Layout::Table));
}

#[test]
fn a_patch_out_of_bounds_is_refused() {
    let at_offset = 14 + 2 * 3 + 2 * 3 + 4;
    let mut inside = three();
    inside[at_offset..at_offset + 4].copy_from_slice(&0u32.to_le_bytes());
    assert_eq!(patch_for(&inside, 1).err(), Some(Layout::Bounds), "inside the tables");
    let mut long = three();
    long[14 + 6 + 2..14 + 6 + 4].copy_from_slice(&60_000u16.to_le_bytes());
    assert_eq!(patch_for(&long, 1).err(), Some(Layout::Bounds), "past the end");
    let short = epatch(VERSION, &[(2, &[1, 2, 3])], 10);
    assert_eq!(patch_for(&short, 1).err(), Some(Layout::Bounds), "shorter than the version");
}
