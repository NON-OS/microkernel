// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! The patch for each ROM version, chosen by chip id = ROM version + 1,
//! and the file refused when it is not the RTL8821C's.

use nonos_bt_rtl::epatch::select;
use nonos_bt_rtl::image::image;
use nonos_bt_rtl::project::{project, PROJECT_8821C};
use nonos_bt_rtl::RtlError;

use crate::firmware::{CONFIG, FW};
use crate::fixture::{expected, FW_VERSION, PATCHES};

#[test]
fn each_rom_version_gets_its_own_patch() {
    for (rom, at, len) in PATCHES {
        let e = select(FW, rom).unwrap();
        assert_eq!(e.fw_version, FW_VERSION);
        assert_eq!(e.patch, &FW[at..at + len], "ROM {rom} got another patch");
        assert_eq!(image(FW, CONFIG, rom).unwrap(), expected(rom));
    }
}

#[test]
fn the_image_ends_with_the_version_then_the_config() {
    let img = image(FW, CONFIG, 1).unwrap();
    let (body, cfg) = img.split_at(img.len() - CONFIG.len());
    assert_eq!(cfg, CONFIG);
    assert_eq!(body[body.len() - 4..], FW_VERSION.to_le_bytes());
    assert_eq!(body.len(), 0x8864, "the patch keeps its length");
}

#[test]
fn a_rom_version_with_no_patch_is_refused() {
    for rom in [2, 3, 0x7f, 0xff] {
        assert_eq!(select(FW, rom), Err(RtlError::NoPatchFor(rom)));
    }
}

#[test]
fn the_file_is_for_the_rtl8821c() {
    assert_eq!(project(FW), Ok(PROJECT_8821C));
    let mut other = FW.to_vec();
    let at = other.len() - 4 - 3;
    other[at] = 13; // the RTL8822C's project
    assert_eq!(select(&other, 1), Err(RtlError::Project(13)));
}

#[test]
fn a_file_that_is_not_an_epatch_is_refused() {
    let mut fw = FW.to_vec();
    fw[..8].copy_from_slice(b"Realtec_");
    assert_eq!(select(&fw, 1), Err(RtlError::Signature));
    let mut fw = FW.to_vec();
    let n = fw.len();
    fw[n - 1] ^= 0xff;
    assert_eq!(select(&fw, 1), Err(RtlError::Extension));
}

#[test]
fn an_extension_with_an_empty_instruction_or_no_project_is_refused() {
    let n = FW.len();
    let tail = |op: u8, len: u8| [&FW[..n - 7], &[0xaa, len, op], &FW[n - 4..]].concat();
    assert_eq!(project(&tail(5, 0)), Err(RtlError::NoProject));
    assert_eq!(project(&tail(0xff, 1)), Err(RtlError::NoProject));
}
