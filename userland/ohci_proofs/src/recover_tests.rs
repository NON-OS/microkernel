// NONOS Operating System (AGPL-3.0-or-later)
//! After a timeout, a halt or a slot's end, each ED is left so the next
//! transfer on it runs: taken back, emptied, and on the toggle it should.

use crate::facts::ed::ED_K;
use crate::facts::regs::INT_WDH;
use crate::model::{addressed, bulk_ready, call, rig, Device, BUS};
use crate::sched::layout::bulk_ed;
use crate::wire::ops::*;
use crate::wire::E_IO;

#[test]
fn a_bulk_in_that_times_out_is_taken_back_and_the_next_runs() {
    let (mut cx, w) = rig(vec![Some(Device::full(64)), None, None]);
    let slot = addressed(&mut cx, 1);
    bulk_ready(&mut cx, slot);
    assert_eq!(call(&mut cx, OP_BULK_IN, &[slot, 0, 64, 0]).status, E_IO, "nothing comes");
    w.borrow_mut().devs[0].as_mut().unwrap().send(&[4; 20]);
    let r = call(&mut cx, OP_BULK_IN, &[slot, 0, 64, 0]);
    assert_eq!((r.status, &r.data[4..]), (0, &[4u8; 20][..]));
    let w = w.borrow();
    assert_eq!(w.devs[0].as_ref().unwrap().toggle_errors, 0, "the NAKed packet moved no toggle");
    assert_eq!(w.int_status & INT_WDH, 0, "every done head acknowledged");
}

#[test]
fn a_full_4_kib_in_ends_on_its_last_packet() {
    let (mut cx, w) = rig(vec![Some(Device::full(64)), None, None]);
    let slot = addressed(&mut cx, 1);
    bulk_ready(&mut cx, slot);
    let msg: Vec<u8> = (0..4096).map(|i| (i * 7) as u8).collect();
    w.borrow_mut().devs[0].as_mut().unwrap().send(&msg);
    let r = call(&mut cx, OP_BULK_IN, &[slot, 0, 0, 0x10]);
    assert_eq!((r.status, &r.data[..2], &r.data[4..]), (0, &[0u8, 0x10][..], &msg[..]));
}

#[test]
fn a_slot_given_back_retires_its_bulk_eds_and_the_next_device_starts_at_data0() {
    let (mut cx, w) = rig(vec![Some(Device::full(64)), Some(Device::full(64)), None]);
    let slot = addressed(&mut cx, 1);
    bulk_ready(&mut cx, slot);
    assert_eq!(call(&mut cx, OP_BULK_OUT, &[slot, 0, 3, 0, 1, 2, 3]).status, 0);
    assert_eq!(call(&mut cx, OP_DISABLE_SLOT, &[slot]).status, 0);
    for i in [0, 1] {
        assert_eq!(w.borrow().rd32(BUS + bulk_ed(i) as u32) & ED_K, ED_K, "ED {i} skipped");
    }
    let again = addressed(&mut cx, 2);
    assert_eq!(again, slot, "the same slot, so the same EDs");
    bulk_ready(&mut cx, again);
    assert_eq!(call(&mut cx, OP_BULK_OUT, &[again, 0, 2, 0, 9, 9]).status, 0);
    let w = w.borrow();
    assert_eq!(
        (&w.devs[1].as_ref().unwrap().bulk_out[..], w.devs[1].as_ref().unwrap().toggle_errors),
        (&[9u8, 9][..], 0)
    );
}
