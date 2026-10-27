// NONOS Operating System (AGPL-3.0-or-later)

//! Register access through the model: the L1 window opened onto a chip
//! address before each access, an address no window reaches never
//! touched, and the read-modify-write forms as `mt76_rmw` computes them.

use crate::hw::Regs;
use crate::model::rig::{chip, regs};

fn fresh() -> (Regs, crate::model::Shared) {
    let c = chip();
    (regs(&c), c)
}

#[test]
fn a_remapped_read_opens_the_window_onto_its_upper_half_first() {
    let (r, c) = fresh();
    assert_eq!(r.rr(0x70010200), 0x7922, "MT_HW_CHIPID through the window");
    let w = c.borrow().writes.clone();
    assert_eq!(w, vec![(0xfe24c, 0x7001)], "the remap register's low half, alone");
    assert_eq!(c.borrow().reg(0xfe24c) & 0xffff, 0x7001);
}

#[test]
fn the_remap_keeps_the_registers_upper_half() {
    let (r, c) = fresh();
    c.borrow_mut().regs.insert(0xfe24c, 0xabcd_0000);
    r.wr(0x18000140, 1);
    assert_eq!(c.borrow().reg(0xfe24c), 0xabcd_1800);
    assert_eq!(c.borrow().l1.get(&0x18000140), Some(&1));
}

#[test]
fn an_address_no_window_reaches_is_never_touched() {
    let (r, c) = fresh();
    assert_eq!(r.rr(0x9000_0000), u32::MAX);
    r.wr(0x9000_0000, 5);
    r.set(0x9000_0000, 1);
    assert!(c.borrow().writes.is_empty(), "nothing went to the card");
}

#[test]
fn read_modify_write_replaces_only_its_mask() {
    let (r, c) = fresh();
    c.borrow_mut().regs.insert(0x10194, 0xffff_0000);
    r.rmw_field(0x10194, 1 << 8, 1);
    assert_eq!(c.borrow().reg(0x10194), 0xffff_0100);
    r.rmw_field(0x10194, 0xfff8, 1536);
    assert_eq!(c.borrow().reg(0x10194), 0xffff_0000 | 1536 << 3);
    r.clear(0x10194, 0xffff_0000);
    assert_eq!(c.borrow().reg(0x10194), 1536 << 3);
}
