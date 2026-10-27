// NONOS Operating System (AGPL-3.0-or-later)

//! The MAC's bring-up sequence on the model, in `__mt7921_init_hardware`
//! and `__mt7921_start` order, each command where Linux sends it, and the
//! receive path's MDP and DMA lengths with translation and group 5 off.

use super::download_tests::run;
use super::mcu_tests::card;
use crate::init::run as init;
use crate::model::images::{patch_7922, ram_7922};

#[test]
fn the_mac_comes_up_in_linuxs_order() {
    let (mut k, c) = card();
    run(&mut k, &patch_7922(), &ram_7922()).unwrap();
    let before = c.borrow().mcu.seen.len();
    init(&mut k.mcu, &k.regs, &mut k.dma).unwrap();
    let ch = c.borrow();
    let order: Vec<(u16, u8)> = ch.mcu.seen[before..].iter().map(|s| (s.cid, s.ext)).collect();
    assert_eq!(order, [(0x8a, 0), (0xed, 0x21), (0xed, 0x46), (0x0f, 0), (0xed, 0x4e)]);
    let efuse = &ch.mcu.seen[before + 1].body;
    assert_eq!(efuse[..4], [0, 1, 0, 0], "EE_MODE_EFUSE, EE_FORMAT_WHOLE");
    let mac = &ch.mcu.seen[before + 2].body;
    assert_eq!(mac[..4], [1, 0, 0, 0], "band 0 enabled");
}

#[test]
fn header_translation_and_group_5_stay_off() {
    let (mut k, c) = card();
    c.borrow_mut().regs.insert(0xf000, 1 << 19 | 1 << 15 | 1);
    c.borrow_mut().regs.insert(0x21e00, 1 << 23);
    run(&mut k, &patch_7922(), &ram_7922()).unwrap();
    init(&mut k.mcu, &k.regs, &mut k.dma).unwrap();
    let ch = c.borrow();
    assert_eq!(ch.reg(0xf000), 1, "MT_MDP_DCR0: no RX_HDR_TRANS, no DAMSDU");
    assert_eq!(ch.reg(0xf004) & 0xfff8, 1536 << 3, "MT_MDP_DCR1 max length");
    assert_eq!(ch.reg(0x21e00), 1536 << 3, "MT_DMA_DCR0: length, RXD_G5_EN off");
}

#[test]
fn a_firmware_that_will_not_answer_stops_the_mac_at_its_first_step() {
    let (mut k, c) = card();
    run(&mut k, &patch_7922(), &ram_7922()).unwrap();
    c.borrow_mut().faults.silent = true;
    assert_eq!(init(&mut k.mcu, &k.regs, &mut k.dma).err(), Some(crate::init::InitStep::Caps));
}
