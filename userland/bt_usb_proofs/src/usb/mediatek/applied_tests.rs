/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! A patch already applied: a section the chip says it holds is left, not
//! sent again, and the function is still turned on.

use super::build::{holding, image, octets, pins, three};
use super::model::Model;
use super::setup::setup;

#[test]
fn a_patch_the_chip_holds_is_not_sent_again() {
    let img = image(&three());
    let mut m = holding(Model::mt7922(), &img);
    m.patched.insert(0x0090_0000, vec![0xaa]);
    m.patched.insert(0x0091_8000, vec![0xbb]);
    let up = setup(&mut m, &pins(&img)).expect("set up");
    assert_eq!(m.broken, None);
    assert!(m.blocks.is_empty(), "no octet sent");
    assert_eq!((up.sent.sent, up.sent.applied), (0, 2));
    assert_eq!(m.patched[&0x0090_0000], [0xaa], "what the chip held is kept");
    assert!(m.func_on);
}

#[test]
fn only_the_sections_not_held_are_sent() {
    let img = image(&three());
    let mut m = holding(Model::mt7921(), &img);
    m.patched.insert(0x0090_0000, vec![0xaa]);
    let up = setup(&mut m, &pins(&img)).expect("set up");
    assert_eq!((m.broken, up.sent.sent, up.sent.applied), (None, 1, 1));
    assert_eq!(m.blocks, [(1, 250)]);
    assert_eq!(m.patched[&0x0091_8000], octets(250, 2));
}
