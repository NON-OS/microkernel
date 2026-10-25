// NONOS Operating System (AGPL-3.0-or-later)
//! The polled bulk IN: armed once, looked at by each poll, armed again by
//! the poll after the one that took its data.

use crate::model::{addressed, bulk_ready, call, rig, Device};
use crate::wire::ops::*;
use crate::wire::{E_AGAIN, E_BUSY, E_PIPE};

fn ready() -> (crate::model::Cx, crate::model::Shared, u8) {
    let (mut cx, w) = rig([Some(Device::full(64)), None]);
    let slot = addressed(&mut cx, 1);
    bulk_ready(&mut cx, slot);
    (cx, w, slot)
}

fn poll(cx: &mut crate::model::Cx, slot: u8) -> crate::model::Answer {
    call(cx, OP_BULK_IN_POLL, &[slot, 0, 0, 6])
}

#[test]
fn frames_come_whenever_the_device_has_them_with_toggles_kept() {
    let (mut cx, w, slot) = ready();
    assert_eq!(poll(&mut cx, slot).status, E_AGAIN);
    for i in 0..3u8 {
        let frame = vec![i; 60 + 64 * i as usize];
        w.borrow_mut().devs[0].as_mut().unwrap().send(&frame);
        crate::env::Clock::idle(&cx.clock);
        let r = poll(&mut cx, slot);
        assert_eq!(r.status, 0, "frame {i}");
        assert_eq!(&r.data[4..], &frame[..]);
        assert_eq!(poll(&mut cx, slot).status, E_AGAIN, "armed again, nothing yet");
    }
    assert_eq!(w.borrow().devs[0].as_ref().unwrap().toggle_errors, 0);
}

#[test]
fn a_bulk_in_while_armed_is_busy_and_a_reset_disarms() {
    let (mut cx, w, slot) = ready();
    assert_eq!(poll(&mut cx, slot).status, E_AGAIN);
    assert_eq!(call(&mut cx, OP_BULK_IN, &[slot, 0, 64, 0]).status, E_BUSY);
    assert_eq!(call(&mut cx, OP_RESET_BULK, &[slot, 1]).status, 0);
    assert!(cx.table.dev(slot).unwrap().poll.is_none());
    w.borrow_mut().devs[0].as_mut().unwrap().send(&[3; 10]);
    let r = call(&mut cx, OP_BULK_IN, &[slot, 0, 64, 0]);
    assert_eq!((r.status, &r.data[4..]), (0, &[3u8; 10][..]));
}

#[test]
fn a_stall_on_the_polled_chain_is_e_pipe_once() {
    let (mut cx, w, slot) = ready();
    assert_eq!(poll(&mut cx, slot).status, E_AGAIN);
    w.borrow_mut().devs[0].as_mut().unwrap().stall_bulk_in = true;
    crate::env::Clock::idle(&cx.clock);
    assert_eq!(poll(&mut cx, slot).status, E_PIPE);
    assert!(cx.table.dev(slot).unwrap().poll.is_none());
}

#[test]
fn bulk_out_runs_while_the_polled_in_waits() {
    let (mut cx, w, slot) = ready();
    assert_eq!(poll(&mut cx, slot).status, E_AGAIN);
    let r = call(&mut cx, OP_BULK_OUT, &[slot, 0, 3, 0, 7, 8, 9]);
    assert_eq!(r.data, [3, 0, 0, 0]);
    assert_eq!(w.borrow().devs[0].as_ref().unwrap().bulk_out, [7, 8, 9]);
}
