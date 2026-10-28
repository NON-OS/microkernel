// NONOS Operating System (AGPL-3.0-or-later)

//! The firmware package: the pinned files the capsule accepts, each read
//! whole through `MkInstallSource` and accepted only at its pinned length
//! and SHA-256; a slot the loader left empty is "missing", any other file
//! "does not match", and either stops bring-up with its own stage.

use nonos_libc::set_slot;

use crate::fw::package::{for_chip, FILES, SLOT_FIRST};
use crate::fw::pins::PINS;
use crate::fw::{fetch, Fetched};
use crate::ids::Chip;
use crate::model::images::pattern;
use crate::model::package::file;

#[test]
fn the_slots_are_the_pins_in_order_from_0x20() {
    for (i, f) in FILES.iter().enumerate() {
        assert_eq!((f.name, f.len, f.kind), (PINS[i].0, PINS[i].1, SLOT_FIRST + i as u64));
        assert_eq!(SLOT_FIRST, 0x20);
    }
    assert_eq!(FILES[4].sha256[..4], [0x7f, 0xc9, 0x07, 0x5d], "the MT7922 patch's pin");
}

#[test]
fn each_chip_takes_its_own_patch_and_ram_code() {
    let names = |c| (for_chip(c).0.name, for_chip(c).1.name);
    let want = ("WIFI_MT7922_patch_mcu_1_1_hdr.bin", "WIFI_RAM_CODE_MT7922_1.bin");
    assert_eq!(names(Chip::Mt7922), want);
    let want = ("WIFI_MT7961_patch_mcu_1_2_hdr.bin", "WIFI_RAM_CODE_MT7961_1.bin");
    assert_eq!(names(Chip::Mt7961), want);
    let want = ("WIFI_MT7961_patch_mcu_1a_2_hdr.bin", "WIFI_RAM_CODE_MT7961_1a.bin");
    assert_eq!(names(Chip::Mt7920), want);
}

#[test]
fn a_file_is_taken_only_at_its_length_and_hash() {
    let bytes = pattern(9, 70_000);
    let f = file(0x30, &bytes);
    set_slot(0x30, None);
    assert!(matches!(fetch(f), Fetched::Missing));
    set_slot(0x30, Some(bytes.clone()));
    assert!(matches!(fetch(f), Fetched::Bytes(b) if b == bytes), "read across many calls");
    set_slot(0x30, Some(bytes[..69_999].to_vec()));
    assert!(matches!(fetch(f), Fetched::Mismatch), "short");
    let mut longer = bytes.clone();
    longer.push(0);
    set_slot(0x30, Some(longer));
    assert!(matches!(fetch(f), Fetched::Mismatch), "long");
    let mut other = bytes.clone();
    other[35_000] ^= 1;
    set_slot(0x30, Some(other));
    assert!(matches!(fetch(f), Fetched::Mismatch), "one bit changed");
}

#[test]
fn the_real_pins_refuse_anything_else_in_their_slots() {
    let f = &FILES[5];
    set_slot(f.kind, Some(pattern(1, f.len)));
    assert!(matches!(fetch(f), Fetched::Mismatch));
    set_slot(f.kind, None);
    assert!(matches!(fetch(f), Fetched::Missing));
}
