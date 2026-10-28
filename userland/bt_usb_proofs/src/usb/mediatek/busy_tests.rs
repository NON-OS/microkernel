/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Waits that end: the patch semaphore held through 20 looks 100 ms apart,
//! a WMT answer that never comes, a reset that never finishes. Each stops
//! the setup before an octet of the patch is sent.

use super::build::{holding, image, pins, three};
use super::fail::Fail;
use super::load::download;
use super::model::Model;
use super::setup::setup;
use super::wmt::{answer, OP_FUNC_CTRL, OP_PATCH};

#[test]
fn a_semaphore_held_through_twenty_looks_is_a_timeout() {
    let mut m = Model::mt7922();
    m.held = 20;
    assert_eq!(download(&mut m, &image(&three())).err(), Some(Fail::Busy));
    assert!(m.blocks.is_empty());
    assert!(m.clock_ms >= 2000, "20 looks, 100 ms apart");
}

#[test]
fn a_semaphore_given_up_in_time_lets_the_patch_go() {
    let mut m = Model::mt7922();
    m.held = 19;
    assert!(download(&mut m, &image(&three())).is_ok());
    assert_eq!((m.broken, m.patched.len()), (None, 2));
}

#[test]
fn a_status_with_no_meaning_stops_the_download() {
    let mut m = Model::mt7922();
    m.odd = Some(3);
    assert_eq!(download(&mut m, &image(&three())).err(), Some(Fail::Status));
    assert!(m.blocks.is_empty());
}

#[test]
fn a_chip_that_never_answers_stops_after_10_s() {
    let mut m = Model::mt7922();
    m.answer_after = None;
    assert_eq!(download(&mut m, &image(&three())).err(), Some(Fail::Silent));
    assert!((10_000..11_000).contains(&m.clock_ms));
    let mut slow = Model::mt7922();
    slow.answer_after = Some(9_000);
    assert!(download(&mut slow, &image(&three())).is_ok(), "9 s is still an answer");
}

#[test]
fn an_answer_to_another_op_or_cut_short_is_refused() {
    let ok = [0xe4, 5, 2, OP_PATCH, 1, 0, 2];
    assert_eq!(answer(OP_PATCH, &ok), Some(2));
    assert_eq!(answer(OP_FUNC_CTRL, &ok), None, "another op");
    assert_eq!(answer(OP_PATCH, &[0x0e, 5, 2, OP_PATCH, 1, 0, 2]), None, "not 0xe4");
    assert_eq!(answer(OP_PATCH, &ok[..6]), None, "shorter than its length");
    assert_eq!(answer(OP_PATCH, &[0xe4, 4, 2, OP_PATCH, 1, 0, 2]), None, "no flag within it");
}

#[test]
fn a_reset_that_never_finishes_stops_before_the_download() {
    let img = image(&three());
    let mut m = holding(Model::mt7921(), &img);
    m.reset_takes = None;
    assert_eq!(setup(&mut m, &pins(&img)).err(), Some(Fail::Reset));
    assert!(m.blocks.is_empty() && m.patched.is_empty() && !m.func_on);
    assert!(m.clock_ms >= 1000, "looked at for 1 s");
}
