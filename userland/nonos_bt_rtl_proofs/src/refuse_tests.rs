// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! A controller that answers wrongly stops the load at once, and the model
//! itself refuses what a real ROM would.

use nonos_bt_hci::HciError;
use nonos_bt_rtl::{prepare, RtlError, DOWNLOAD};

use crate::firmware::{CONFIG, FW};
use crate::fixture::{chip, rom, HP};

#[test]
fn a_rom_version_of_other_than_one_octet_stops_it() {
    for reply in [vec![], vec![1, 0]] {
        let mut h = rom(1);
        chip(&mut h).rom_reply = reply;
        assert_eq!(prepare(&mut h, HP, FW, CONFIG), Err(RtlError::RomReply));
        assert_eq!(chip(&mut h).opcodes(), [0x1001, 0xfc6d]);
    }
}

#[test]
fn a_refused_fragment_ends_the_download_there() {
    let mut h = rom(1);
    chip(&mut h).refuse_at = Some(5);
    let e = prepare(&mut h, HP, FW, CONFIG);
    assert_eq!(e, Err(RtlError::Hci(HciError::Status(DOWNLOAD, 0x01))));
    assert_eq!(chip(&mut h).indices(), [0, 1, 2, 3, 4, 5]);
    assert_eq!(chip(&mut h).running, None);
}

#[test]
fn a_download_reply_of_other_than_one_octet_stops_it() {
    let mut h = rom(1);
    chip(&mut h).download_reply = Some(vec![0, 0]);
    assert_eq!(prepare(&mut h, HP, FW, CONFIG), Err(RtlError::DownloadReply));
    assert_eq!(chip(&mut h).indices(), [0], "the download went on");
}

#[test]
fn the_model_refuses_a_fragment_out_of_order() {
    let mut h = rom(1);
    let skipped = [&[1u8][..], &[0u8; 252]].concat();
    assert_eq!(h.command(DOWNLOAD, &skipped), Err(HciError::Status(DOWNLOAD, 0x12)));
    assert_eq!(chip(&mut h).broken, Some("a fragment out of order"));
}

#[test]
fn the_model_refuses_a_short_fragment_before_the_last() {
    let mut h = rom(1);
    let short = [&[0u8][..], &[0u8; 100]].concat();
    assert_eq!(h.command(DOWNLOAD, &short), Err(HciError::Status(DOWNLOAD, 0x12)));
}

#[test]
fn the_model_runs_no_image_it_does_not_accept() {
    let mut h = rom(1);
    let wrong = [&[0x80u8][..], &[0u8; 10]].concat();
    assert_eq!(h.command(DOWNLOAD, &wrong), Err(HciError::Status(DOWNLOAD, 0x12)));
    assert_eq!(chip(&mut h).running, None);
}
