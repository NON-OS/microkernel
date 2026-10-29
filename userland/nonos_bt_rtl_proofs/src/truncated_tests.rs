// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! A patch file cut short is never sent: not cut anywhere, not with its
//! tail kept and its middle gone, not with tables that run past its end.

use nonos_bt_rtl::epatch::select;
use nonos_bt_rtl::{prepare, RtlError};

use crate::firmware::{CONFIG, FW};
use crate::fixture::{chip, rom, HP};

#[test]
fn a_file_cut_anywhere_is_refused() {
    for cut in [0, 8, 20, 21, 0x30, 0x4c44, 0x9000, FW.len() - 4, FW.len() - 1] {
        assert!(select(&FW[..cut], 1).is_err(), "a file cut at {cut:#x} was taken");
    }
}

#[test]
fn a_patch_cut_with_the_tail_kept_is_refused() {
    let n = FW.len();
    let cut = [&FW[..0x9000], &FW[n - 8..]].concat();
    assert_eq!(select(&cut, 1), Err(RtlError::Truncated), "ROM 1's patch ends at 0xd524");
    // ROM 0's patch ends at 0x4c44, inside what is left: it is still whole.
    assert_eq!(select(&cut, 0).unwrap().patch, &FW[0x30..0x4c44]);
}

#[test]
fn tables_that_run_past_the_end_are_refused() {
    let mut fw = FW.to_vec();
    fw[12..14].copy_from_slice(&0xffffu16.to_le_bytes());
    assert_eq!(select(&fw, 1), Err(RtlError::Truncated));
}

#[test]
fn a_patch_shorter_than_its_version_is_refused() {
    let mut fw = FW.to_vec();
    // ROM 1's length, the second of the two, set to 3.
    fw[20..22].copy_from_slice(&3u16.to_le_bytes());
    assert_eq!(select(&fw, 1), Err(RtlError::Truncated));
}

#[test]
fn a_cut_file_sends_no_fragment() {
    let mut h = rom(1);
    let cut = [&FW[..0x9000], &FW[FW.len() - 8..]].concat();
    assert_eq!(prepare(&mut h, HP, &cut, CONFIG), Err(RtlError::Truncated));
    assert_eq!(chip(&mut h).opcodes(), [0x1001, 0xfc6d], "a fragment went out");
    assert_eq!(chip(&mut h).running, None);
}
