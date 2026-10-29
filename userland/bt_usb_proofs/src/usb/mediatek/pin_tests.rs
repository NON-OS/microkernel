/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The package's pins: a file of another length or hash, or none at all,
//! is refused before the chip is reset or written; and the capsule's own
//! pins are linux-firmware's two files at kinds 0x26 and 0x27.

use super::build::{holding, image, pins, three};
use super::chip::Chip;
use super::fail::Fail;
use super::files::{for_chip, FILES, KIND_FIRST};
use super::model::Model;
use super::setup::setup;

fn refused(file: Vec<u8>, pinned: &[u8]) -> (Option<Fail>, Model) {
    let mut m = holding(Model::mt7922(), &file);
    let why = setup(&mut m, &pins(pinned)).err();
    (why, m)
}

#[test]
fn another_length_or_hash_is_refused_and_the_chip_left_alone() {
    let img = image(&three());
    let mut longer = img.clone();
    longer.push(0);
    let mut other = img.clone();
    other[400] ^= 1;
    for (file, what, digests) in [
        (longer, "one octet more", 0),
        (img[..img.len() - 1].to_vec(), "one less", 0),
        (other, "one bit", 1),
    ] {
        let (why, m) = refused(file, &img);
        assert_eq!(why, Some(Fail::Mismatch), "{what}");
        assert!(m.writes.is_empty() && m.pending.is_none(), "{what}: chip touched");
        assert_eq!(m.digests, digests, "{what}: a wrong length is refused unhashed");
    }
}

#[test]
fn a_package_without_the_file_is_missing_and_the_chip_left_alone() {
    let img = image(&three());
    let mut m = Model::mt7922();
    assert_eq!(setup(&mut m, &pins(&img)).err(), Some(Fail::Missing));
    assert!(m.writes.is_empty());
}

#[test]
fn the_capsules_pins_take_no_file_but_linux_firmwares() {
    assert_eq!((KIND_FIRST, FILES[0].kind, FILES[1].kind), (0x26, 0x26, 0x27));
    assert_eq!((FILES[0].len, FILES[1].len), (533_870, 517_398));
    assert_eq!(FILES[0].sha256[..4], [0x7a, 0x44, 0x0a, 0x0f], "BT_RAM_CODE_MT7961_1_2_hdr.bin");
    assert_eq!(FILES[1].sha256[..4], [0x29, 0x99, 0x42, 0xdb], "BT_RAM_CODE_MT7922_1_1_hdr.bin");
    assert_eq!(for_chip(&FILES, Chip::Mt7922).kind, 0x27);
    let img = image(&three());
    let mut m = holding(Model::mt7922(), &img);
    m.files.insert(0x27, img.clone());
    assert_eq!(setup(&mut m, &FILES).err(), Some(Fail::Mismatch), "a made-up file");
}
