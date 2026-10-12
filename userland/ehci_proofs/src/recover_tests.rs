// NONOS Operating System (AGPL-3.0-or-later)
//! After a timeout or a slot's end each QH is left so the next transfer on
//! it runs: taken from the ring, the doorbell answered, emptied, linked
//! back, on the toggle it should.

use crate::facts::qh::QH_TOKEN;
use crate::model::{addressed, bulk_ready, call, high, rig, BUS};
use crate::sched::layout::{bulk_ring, qh, RING};
use crate::wire::ops::*;
use crate::wire::E_IO;

#[test]
fn a_bulk_in_that_times_out_is_taken_back_and_the_next_runs() {
    let (mut cx, w) = rig(vec![high(), None, None]);
    let slot = addressed(&mut cx, 1);
    bulk_ready(&mut cx, slot);
    assert_eq!(call(&mut cx, OP_BULK_IN, &[slot, 0, 0, 2]).status, E_IO, "nothing comes");
    w.borrow_mut().devs[0].as_mut().unwrap().send(&[4; 700]);
    let r = call(&mut cx, OP_BULK_IN, &[slot, 0, 0, 4]);
    assert_eq!((r.status, &r.data[4..]), (0, &[4u8; 700][..]));
    let w = w.borrow();
    assert_eq!(w.devs[0].as_ref().unwrap().toggle_errors, 0, "the NAKed packet moved no toggle");
    let mut at = BUS + qh(0) as u32;
    for _ in 0..RING {
        at = w.rd32(at) & !0x1F;
    }
    assert_eq!(at, BUS + qh(0) as u32, "the ring whole again");
}

#[test]
fn a_full_4_kib_in_ends_on_its_last_packet() {
    let (mut cx, w) = rig(vec![high(), None, None]);
    let slot = addressed(&mut cx, 1);
    bulk_ready(&mut cx, slot);
    let msg: Vec<u8> = (0..4096).map(|i| (i * 7) as u8).collect();
    w.borrow_mut().devs[0].as_mut().unwrap().send(&msg);
    let r = call(&mut cx, OP_BULK_IN, &[slot, 0, 0, 0x10]);
    assert_eq!((r.status, &r.data[..2], &r.data[4..]), (0, &[0u8, 0x10][..], &msg[..]));
}

#[test]
fn a_slot_given_back_takes_its_bulk_qhs_back_and_the_next_device_starts_at_data0() {
    let (mut cx, w) = rig(vec![high(), high(), None]);
    let slot = addressed(&mut cx, 1);
    bulk_ready(&mut cx, slot);
    assert_eq!(call(&mut cx, OP_BULK_OUT, &[slot, 0, 3, 0, 1, 2, 3]).status, 0);
    assert_eq!(call(&mut cx, OP_DISABLE_SLOT, &[slot]).status, 0);
    let tok = w.borrow().rd32(BUS + (qh(bulk_ring(slot, false)) + QH_TOKEN) as u32);
    assert_eq!(tok, 0, "emptied, DATA0 next");
    let again = addressed(&mut cx, 2);
    assert_eq!(again, slot, "the same slot, so the same QHs");
    bulk_ready(&mut cx, again);
    assert_eq!(call(&mut cx, OP_BULK_OUT, &[again, 0, 2, 0, 9, 9]).status, 0);
    let w = w.borrow();
    let d = w.devs[1].as_ref().unwrap();
    assert_eq!((&d.bulk_out[..], d.toggle_errors), (&[9u8, 9][..], 0));
}
