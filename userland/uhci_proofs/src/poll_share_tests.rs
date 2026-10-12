// NONOS Operating System (AGPL-3.0-or-later)
//! The polled chain has one place: a second slot waits for the first's.

use crate::model::{addressed, bulk_ready, call, rig, Device};
use crate::wire::ops::OP_BULK_IN_POLL;
use crate::wire::{E_AGAIN, E_BUSY};

#[test]
fn a_second_slot_is_busy_until_the_first_slots_chain_is_taken() {
    let (mut cx, w) = rig([Some(Device::full(64)), Some(Device::full(64))]);
    let (a, b) = (addressed(&mut cx, 1), addressed(&mut cx, 2));
    bulk_ready(&mut cx, a);
    bulk_ready(&mut cx, b);
    let poll = |cx: &mut crate::model::Cx, s: u8| call(cx, OP_BULK_IN_POLL, &[s, 0, 0, 2]).status;
    assert_eq!(poll(&mut cx, a), E_AGAIN);
    assert_eq!(poll(&mut cx, b), E_BUSY, "a's chain is armed");
    assert_eq!(poll(&mut cx, a), E_AGAIN, "a still polls");
    w.borrow_mut().devs[0].as_mut().unwrap().send(&[1; 9]);
    crate::env::Clock::idle(&cx.clock);
    assert_eq!(poll(&mut cx, a), 0, "a takes its data and its chain");
    assert_eq!(poll(&mut cx, b), E_AGAIN, "b arms now");
    assert_eq!(poll(&mut cx, a), E_BUSY);
    let w = w.borrow();
    assert!(w.devs.iter().all(|d| d.as_ref().unwrap().toggle_errors == 0));
}
