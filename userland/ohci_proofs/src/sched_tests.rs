// NONOS Operating System (AGPL-3.0-or-later)
//! The ED and TD words and the lists, pure over the model's memory.

use crate::facts::ed::{ED_K, ED_S, HEAD_C};
use crate::facts::td::*;
use crate::model::{rig, BUS};
use crate::sched::ed::{control, Target};
use crate::sched::layout::*;
use crate::sched::td::{cc, moved, write, Td};

#[test]
fn an_ed_word_carries_address_endpoint_speed_skip_and_packet_size() {
    let t = Target { addr: 5, ep: 2, low_speed: true };
    assert_eq!(control(t, 64, false), 5 | 2 << 7 | ED_S | 64 << 16);
    assert_eq!(control(Target { low_speed: false, ..t }, 8, true) & (ED_S | ED_K), ED_K);
}

#[test]
fn every_ed_starts_skipped_and_empty_on_its_first_place_and_the_lists_link() {
    let (_cx, w) = rig(vec![None, None, None]);
    let w = w.borrow();
    for i in 0..INTR_MAX {
        let ed = BUS + intr_ed(i) as u32;
        assert_eq!(w.rd32(ed) & ED_K, ED_K);
        assert_eq!(
            (w.rd32(ed + 4), w.rd32(ed + 8)),
            (BUS + intr_td(i, 0) as u32, BUS + intr_td(i, 0) as u32)
        );
        let next = if i + 1 < INTR_MAX { BUS + intr_ed(i + 1) as u32 } else { 0 };
        assert_eq!(w.rd32(ed + 12), next);
    }
    for i in 0..BULK_EDS {
        let ed = BUS + bulk_ed(i) as u32;
        assert_eq!(w.rd32(ed + 8) & HEAD_C, 0, "DATA0 first");
        let next = if i + 1 < BULK_EDS { BUS + bulk_ed(i + 1) as u32 } else { 0 };
        assert_eq!(w.rd32(ed + 12), next);
    }
}

#[test]
fn a_td_is_written_unretired_and_reads_back_what_it_moved() {
    let (cx, w) = rig(vec![None, None, None]);
    let td = Td { dp: DP_IN, toggle: Some(true), round: true, buf: BULK_BUF, len: 100 };
    write(&cx.mem, BULK_TD, td, BULK_TD + 16);
    {
        let w = w.borrow();
        let ctl = w.rd32(BUS + BULK_TD as u32);
        assert_eq!(ctl & (TD_R | 3 << 19 | 3 << 24), TD_R | DP_IN << 19 | TD_T_DATA1);
        assert_eq!(ctl >> 28, CC_NOT_ACCESSED);
        assert_eq!(w.rd32(BUS + BULK_TD as u32 + 12), BUS + (BULK_BUF + 99) as u32);
    }
    assert_eq!(cc(&cx.mem, BULK_TD), CC_NOT_ACCESSED);
    w.borrow_mut().wr32(BUS + BULK_TD as u32 + 4, BUS + (BULK_BUF + 36) as u32);
    assert_eq!(moved(&cx.mem, BULK_TD, BULK_BUF, 100), 36, "short: up to CBP");
    w.borrow_mut().wr32(BUS + BULK_TD as u32 + 4, 0);
    assert_eq!(moved(&cx.mem, BULK_TD, BULK_BUF, 100), 100, "CBP 0: all of it");
    let none = Td { dp: DP_IN, toggle: None, round: false, buf: 0, len: 0 };
    write(&cx.mem, CTRL_TD, none, CTRL_TD + 16);
    assert_eq!((w.borrow().rd32(BUS + CTRL_TD as u32 + 4), moved(&cx.mem, CTRL_TD, 0, 0)), (0, 0));
}
