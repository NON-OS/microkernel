// NONOS Operating System (AGPL-3.0-or-later)
//! The efuse read on the modelled chip: the wifi bank, the 8822C's own LDO
//! (0x29 bit 0, not the 8821C's 0x37 bit 7) off, every byte through the
//! control register; the cut and the RF path count from REG_SYS_CFG1; an
//! all-ones window or a controller that never answers is no board.

use super::fixtures::{blank, put};
use crate::efuse::{diag, read, EfuseDiag};
use crate::model::Chip;

fn chip(efuse: [u8; 512]) -> Chip {
    let c = Chip::new();
    *c.hw.efuse.borrow_mut() = Some(efuse.to_vec());
    c.set32(0x34, 0x8000_0300); // bank select bits set, bit 31 set
    c.mem.borrow_mut().insert(0x29, 0xFF);
    c.mem.borrow_mut().insert(0x37, 0xFF);
    c
}

fn board() -> [u8; 512] {
    let mut p = blank();
    // Block 25 (0xC8..0xD0): word 1 holds 0xCA, the front end.
    put(&mut p, 0, 25, 0b1101, &[[0x02, 0x00]]);
    p
}

#[test]
fn the_bank_and_the_8822cs_ldo_are_set_for_the_read() {
    let c = chip(board());
    c.set32(0xF0, 0x0800_3000); // RF_TYPE_ID, cut 3
    let info = read(&c).expect("a board");
    assert_eq!((info.rfe, info.cut, info.paths), (0x02, 3, 2));
    assert_eq!(c.get32(0x34) & 0x300, 0, "wifi bank");
    assert_eq!(c.mem.borrow()[&0x29] & 1, 0, "LDO25 off");
    assert_eq!(c.mem.borrow()[&0x37], 0xFF, "the 8821C's LDO byte untouched");
    assert_eq!(c.at(0x30).len(), 512, "one control write a byte");
}

#[test]
fn a_one_path_chip_is_read_as_one() {
    let c = chip(board());
    c.set32(0xF0, 0x0000_1000);
    assert_eq!(read(&c).map(|i| (i.paths, i.cut)), Some((1, 1)));
}

#[test]
fn an_all_ones_window_or_a_stalled_controller_is_no_board() {
    let _one = super::lock();
    let c = chip([0xFF; 512]);
    assert!(read(&c).is_none());
    assert_eq!(diag().flags, EfuseDiag::ALL_ONES);
    let c = chip(board());
    c.hw.efuse_stuck.set(true);
    assert!(read(&c).is_none());
    assert_eq!((diag().flags, diag().stalled_at), (EfuseDiag::ADDR_LATCHED, 1));
}
