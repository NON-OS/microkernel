// NONOS Operating System (AGPL-3.0-or-later)
//! A second bulk pair (OP_CONFIGURE_BULK2, 0x23): UAS's data pipes beside
//! its command and status pipes, each on QHs of its own, named by the bulk
//! ops' second byte; two at most, on endpoints the first does not hold.

use crate::model::{addressed, bulk_ready, call, high, rig, Cx};
use crate::wire::ops::*;
use crate::wire::{E_BUSY, E_INVAL};

/// The model's second pair: IN endpoint 4, OUT endpoint 5.
fn second(cx: &mut Cx, slot: u8) -> crate::model::Answer {
    call(cx, OP_CONFIGURE_BULK2, &[slot, 0x84, 0x05, 0, 0, 2, 0, 2])
}

#[test]
fn each_pair_moves_its_own_bytes_with_its_own_toggles() {
    let (mut cx, w) = rig(vec![high(), None]);
    let slot = addressed(&mut cx, 1);
    bulk_ready(&mut cx, slot);
    assert_eq!(second(&mut cx, slot).data, [9, 10, 0, 0], "DCIs as xHCI numbers them");
    let out = |cx: &mut Cx, pair: u8, data: &[u8]| {
        let mut body = vec![slot, pair, data.len() as u8, 0];
        body.extend_from_slice(data);
        call(cx, OP_BULK_OUT, &body).status
    };
    assert_eq!(
        (out(&mut cx, 0, &[1, 2]), out(&mut cx, 1, &[7; 40]), out(&mut cx, 1, &[8])),
        (0, 0, 0)
    );
    w.borrow_mut().devs[0].as_mut().unwrap().bulk_in2.push_back(vec![5; 30]);
    let r = call(&mut cx, OP_BULK_IN, &[slot, 1, 64, 0]);
    assert_eq!((r.status, &r.data[4..]), (0, &[5u8; 30][..]));
    let w = w.borrow();
    let d = w.devs[0].as_ref().unwrap();
    assert_eq!((&d.bulk_out[..], d.bulk_out2.len(), d.toggle_errors), (&[1u8, 2][..], 41, 0));
}

#[test]
fn a_second_pair_is_refused_on_the_firsts_endpoints_twice_or_past_two() {
    let (mut cx, _) = rig(vec![high(), high(), high(), None]);
    let slots: Vec<u8> = (1..=3).map(|p| addressed(&mut cx, p)).collect();
    bulk_ready(&mut cx, slots[0]);
    let same = [slots[0], 0x82, 0x05, 0, 0, 2, 0, 2];
    assert_eq!(call(&mut cx, OP_CONFIGURE_BULK2, &same).status, E_INVAL, "the first's IN");
    assert_eq!(second(&mut cx, slots[0]).status, 0);
    assert_eq!(second(&mut cx, slots[0]).status, E_INVAL, "one a slot");
    assert_eq!(second(&mut cx, slots[1]).status, 0, "beside no first pair, too");
    assert_eq!(second(&mut cx, slots[2]).status, E_BUSY, "two places");
    assert_eq!(call(&mut cx, OP_DISABLE_SLOT, &[slots[1]]).status, 0);
    assert_eq!(second(&mut cx, slots[2]).status, 0, "a place given back");
}

#[test]
fn the_pair_byte_is_0_or_1_and_only_the_first_pair_polls() {
    let (mut cx, _) = rig(vec![high(), None]);
    let slot = addressed(&mut cx, 1);
    bulk_ready(&mut cx, slot);
    assert_eq!(call(&mut cx, OP_BULK_IN, &[slot, 1, 64, 0]).status, E_INVAL, "no second pair yet");
    assert_eq!(second(&mut cx, slot).status, 0);
    assert_eq!(call(&mut cx, OP_BULK_IN, &[slot, 2, 64, 0]).status, E_INVAL);
    assert_eq!(call(&mut cx, OP_BULK_IN_POLL, &[slot, 1, 64, 0]).status, E_INVAL);
    assert_eq!(call(&mut cx, OP_RESET_BULK, &[slot, 3]).data, [0; 4], "second pair, IN");
    assert_eq!(call(&mut cx, OP_RESET_BULK, &[slot, 4]).status, E_INVAL);
}
