/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Which controllers are MediaTek's, which chip each says it is, and the
//! reset each is given before its download.

use super::build::{holding, image, pins, three};
use super::chip::{chip, Chip};
use super::fail::Fail;
use super::ids::ours;
use super::model::Model;
use super::reg::{EP_RST_OPT, SUBSYS_RST, UDMA_INT_STA_BT, UDMA_INT_STA_BT1};
use super::setup::setup;

#[test]
fn every_mt7921_and_mt7922_id_is_ours_and_no_other() {
    let foxconn = [0xe0c8, 0xe0cd, 0xe0e0, 0xe0f2, 0xe0d8, 0xe0d9, 0xe0e2, 0xe0e4, 0xe0f1];
    let azurewave = [0x3563, 0x3564, 0x3567, 0x3578, 0x3583, 0x3606, 0x3585, 0x3568, 0x3605];
    let more = [(0x0489, 0xe0f5), (0x0489, 0xe0f6), (0x0489, 0xe102), (0x04ca, 0x3802)];
    let rest = [(0x04ca, 0x3804), (0x04ca, 0x38e4), (0x13d3, 0x3607), (0x13d3, 0x3614)];
    let ids = foxconn.map(|p| (0x0489, p)).into_iter().chain(azurewave.map(|p| (0x13d3, p)));
    let last = [(0x13d3, 0x3615), (0x35f5, 0x7922), (0x0e8d, 0x0608), (0x0e8d, 0x0616)];
    for (v, p) in ids.chain(more).chain(rest).chain(last) {
        assert!(ours(v, p), "{v:04x}:{p:04x}");
    }
    let mt7925 = [(0x0489, 0xe113), (0x0489, 0xe118), (0x13d3, 0x3602), (0x13d3, 0x3608)];
    for (v, p) in mt7925.into_iter().chain([(0x0bda, 0x8771), (0x8087, 0x0026), (0x35f5, 0x7925)]) {
        assert!(!ours(v, p), "{v:04x}:{p:04x}");
    }
}

#[test]
fn the_chip_names_its_patch_and_others_are_refused() {
    assert_eq!(chip(0x7961, 0x8a01, 0), Ok(Chip::Mt7961));
    assert_eq!(chip(0x7961, 0x8a01, 0x80), Err(Fail::Chip), "the 1a flavour: no file");
    assert_eq!(chip(0x7961, 0x8a00, 0), Err(Fail::Chip), "BT_RAM_CODE_MT7961_1_1: no file");
    assert_eq!(chip(0x7922, 0x8a00, 0x80), Ok(Chip::Mt7922), "flavour is the MT7961's alone");
    assert_eq!(chip(0x7922, 0x8a01, 0), Err(Fail::Chip));
    for id in [0x7925, 0x7663, 0x7668, 0] {
        assert_eq!(chip(id, 0, 0), Err(Fail::Chip));
    }
    let mut m = Model::chip(0x7925, 0, 0);
    assert_eq!(setup(&mut m, &pins(&[])).err(), Some(Fail::Chip));
    assert!(m.writes.is_empty(), "an MT7925 is not reset");
}

#[test]
fn each_chip_is_reset_its_own_way_before_the_patch() {
    let img = image(&three());
    let mut m = holding(Model::mt7921(), &img);
    setup(&mut m, &pins(&img)).expect("set up");
    let mt7921 = [(EP_RST_OPT, 0x0001_0001), (SUBSYS_RST, 1), (UDMA_INT_STA_BT, 0xff)];
    let then = [(UDMA_INT_STA_BT1, 0xff), (SUBSYS_RST, 0), (EP_RST_OPT, 0x0001_0001)];
    assert_eq!(m.writes, [&mt7921[..], &then[..]].concat());
    let mut m = holding(Model::mt7922(), &img);
    setup(&mut m, &pins(&img)).expect("set up");
    let mt7922 = [(SUBSYS_RST, 0x2020), (EP_RST_OPT, 0x0001_0001), (SUBSYS_RST, 0x2021)];
    let then = [(UDMA_INT_STA_BT, 0xff), (EP_RST_OPT, 0x0001_0001)];
    assert_eq!(m.writes, [&mt7922[..], &then[..]].concat());
    assert_eq!((m.broken, m.func_on), (None, true));
}

#[test]
fn the_log_names_the_chip_and_the_failure_and_nothing_of_the_device() {
    assert_eq!((Chip::Mt7961.name(), Chip::Mt7922.name()), (&b"MT7921"[..], &b"MT7922"[..]));
    let all = [Fail::Usb, Fail::Chip, Fail::Missing, Fail::Mismatch, Fail::Image, Fail::Silent];
    for f in all.into_iter().chain([Fail::Answer, Fail::Status, Fail::Busy, Fail::Reset]) {
        let w = f.words();
        assert!(!w.is_empty() && w.iter().all(|c| c.is_ascii_graphic() || *c == b' '), "{f:?}");
    }
}
