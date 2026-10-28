/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The download's phase flags: each section after its look, in blocks of
//! at most 250, flagged 1 for the first, 3 for the last and 2 between, and
//! every octet reaching the chip at its section's address.

use super::build::{holding, image, octets, pins, three, Part};
use super::chip::Chip;
use super::load::{download, phase, FIRST, LAST, MIDDLE};
use super::model::Model;
use super::setup::setup;

#[test]
fn a_block_is_flagged_by_where_it_falls() {
    assert_eq!(phase(true, 500), FIRST);
    assert_eq!(phase(true, 0), FIRST, "a section of one block");
    assert_eq!(phase(false, 1), MIDDLE);
    assert_eq!(phase(false, 0), LAST);
    assert_eq!((FIRST, MIDDLE, LAST), (1, 2, 3));
}

#[test]
fn each_section_goes_in_blocks_of_250_flagged_first_middle_last() {
    let img = image(&three());
    let mut m = Model::mt7922();
    let sent = download(&mut m, &img).expect("downloaded");
    assert_eq!(m.broken, None);
    assert_eq!(m.blocks, [(1, 250), (2, 250), (3, 200), (1, 250)]);
    assert_eq!((sent.sent, sent.applied), (2, 0));
    assert_eq!(m.patched[&0x0090_0000], octets(700, 1));
    assert_eq!(m.patched[&0x0091_8000], octets(250, 2));
    assert!(!m.patched.contains_key(&0), "a section of nothing is not looked at");
}

#[test]
fn a_section_of_whole_blocks_ends_on_a_last() {
    let img = image(&[Part { addr: 0x0090_0000, data: octets(500, 9) }]);
    let mut m = Model::mt7921();
    download(&mut m, &img).expect("downloaded");
    assert_eq!((m.broken, &m.blocks[..]), (None, &[(1, 250), (3, 250)][..]));
}

#[test]
fn the_whole_setup_resets_downloads_and_turns_bluetooth_on() {
    let img = image(&three());
    let mut m = holding(Model::mt7922(), &img);
    let up = setup(&mut m, &pins(&img)).expect("set up");
    assert_eq!((m.broken, up.chip, up.sent.sent, m.func_on), (None, Chip::Mt7922, 2, true));
    assert_eq!(m.patched.len(), 2);
    assert!(m.clock_ms >= 200, "the reset's and the activation's waits");
}

#[test]
fn a_malformed_image_sends_nothing() {
    let img = image(&three());
    let mut m = Model::mt7922();
    assert!(download(&mut m, &img[..img.len() - 1]).is_err());
    assert!(m.blocks.is_empty() && m.pending.is_none());
}
