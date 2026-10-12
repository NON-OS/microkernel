// NONOS Operating System (AGPL-3.0-or-later)
//! Bulk: packets of the endpoint's size, toggles carried across transfers,
//! short IN packets ending a transfer, and a STALL reset.

use crate::model::{addressed, bulk_ready, call, high, rig};
use crate::wire::ops::*;
use crate::wire::{E_INVAL, E_PIPE};

fn ready() -> (crate::model::Cx, crate::model::Shared, u8) {
    let (mut cx, w) = rig(vec![high(), None]);
    let slot = addressed(&mut cx, 1);
    bulk_ready(&mut cx, slot);
    (cx, w, slot)
}

fn out(cx: &mut crate::model::Cx, slot: u8, data: &[u8]) -> crate::model::Answer {
    let mut body = vec![slot, 0];
    body.extend((data.len() as u16).to_le_bytes());
    body.extend_from_slice(data);
    call(cx, OP_BULK_OUT, &body)
}

#[test]
fn out_transfers_arrive_whole_with_toggles_carried_between_them() {
    let (mut cx, w, slot) = ready();
    let a: Vec<u8> = (0..200).map(|i| i as u8).collect();
    let b: Vec<u8> = (0..31).map(|i| 0xA0 ^ i as u8).collect();
    assert_eq!(out(&mut cx, slot, &a).data, [200, 0, 0, 0]);
    assert_eq!(out(&mut cx, slot, &b).data, [31, 0, 0, 0]);
    let (w, all) = (w.borrow(), [a, b].concat());
    let d = w.devs[0].as_ref().unwrap();
    assert_eq!((&d.bulk_out, d.toggle_errors), (&all, 0));
}

#[test]
fn a_short_in_packet_ends_the_transfer_and_the_next_continues_its_toggle() {
    let (mut cx, w, slot) = ready();
    let msg: Vec<u8> = (0..100).collect();
    w.borrow_mut().devs[0].as_mut().unwrap().send(&msg);
    w.borrow_mut().devs[0].as_mut().unwrap().send(&[9; 70]);
    let r = call(&mut cx, OP_BULK_IN, &[slot, 0, 0, 2]);
    assert_eq!((r.status, &r.data[..4], &r.data[4..]), (0, &[100u8, 0, 0, 0][..], &msg[..]));
    let r = call(&mut cx, OP_BULK_IN, &[slot, 0, 0, 2]);
    assert_eq!((&r.data[..2], &r.data[4..]), (&[70u8, 0][..], &[9u8; 70][..]));
    assert_eq!(w.borrow().devs[0].as_ref().unwrap().toggle_errors, 0);
}

#[test]
fn a_stall_is_e_pipe_and_a_reset_with_the_halt_cleared_resumes() {
    let (mut cx, w, slot) = ready();
    w.borrow_mut().devs[0].as_mut().unwrap().send(&[1; 64 + 5]);
    assert_eq!(call(&mut cx, OP_BULK_IN, &[slot, 0, 64, 0]).status, 0);
    w.borrow_mut().devs[0].as_mut().unwrap().stall_bulk_in = true;
    assert_eq!(call(&mut cx, OP_BULK_IN, &[slot, 0, 64, 0]).status, E_PIPE);
    assert_eq!(call(&mut cx, OP_RESET_BULK, &[slot, 1]).data, [0, 0, 0, 0]);
    w.borrow_mut().devs[0].as_mut().unwrap().clear_halt(2, true);
    let r = call(&mut cx, OP_BULK_IN, &[slot, 0, 64, 0]);
    assert_eq!((r.status, r.data[0]), (0, 5));
    assert_eq!(w.borrow().devs[0].as_ref().unwrap().toggle_errors, 0);
}

#[test]
fn bulk_is_refused_unconfigured_twice_configured_or_out_of_shape() {
    let (mut cx, _) = rig(vec![high(), None]);
    let slot = addressed(&mut cx, 1);
    assert_eq!(call(&mut cx, OP_BULK_IN, &[slot, 0, 64, 0]).status, E_INVAL);
    let swapped = [slot, 0x03, 0x82, 0, 64, 0, 64, 0];
    assert_eq!(call(&mut cx, OP_CONFIGURE_BULK, &swapped).status, E_INVAL);
    bulk_ready(&mut cx, slot);
    let twice = [slot, 0x82, 0x03, 0, 64, 0, 64, 0];
    assert_eq!(call(&mut cx, OP_CONFIGURE_BULK, &twice).status, E_INVAL);
    assert_eq!(call(&mut cx, OP_BULK_IN, &[slot, 0, 0x01, 0x10]).status, E_INVAL, "4097");
    assert_eq!(out(&mut cx, slot, &[]).status, E_INVAL, "no data");
    assert_eq!(call(&mut cx, OP_RESET_BULK, &[slot, 2]).status, E_INVAL);
}
