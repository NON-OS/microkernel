// NONOS Operating System (AGPL-3.0-or-later)
//! EP0 when the device refuses or never answers, and requests refused
//! before they reach the bus.

use crate::model::{addressed, call, rig, Device};
use crate::wire::ops::*;
use crate::wire::{E_INVAL, E_IO, E_PIPE};

#[test]
fn a_stall_is_e_pipe_and_ep0_works_after_it() {
    let mut dev = Device::full(64);
    dev.stall_request = Some(0x33);
    let (mut cx, _) = rig(vec![Some(dev), None]);
    let slot = addressed(&mut cx, 1);
    let r = call(&mut cx, OP_CONTROL_TRANSFER, &[slot, 0, 0xC0, 0x33, 0, 0, 0, 0, 4, 0]);
    assert_eq!(r.status, E_PIPE);
    assert_eq!(call(&mut cx, OP_GET_DEVICE_DESCRIPTOR, &[slot]).status, 0);
}

#[test]
fn a_device_that_only_naks_times_out_at_the_bound() {
    let (mut cx, w) = rig(vec![Some(Device::full(64)), None]);
    let slot = addressed(&mut cx, 1);
    w.borrow_mut().devs[0].as_mut().unwrap().mute = true;
    let t0 = w.borrow().ms;
    assert_eq!(call(&mut cx, OP_GET_DEVICE_DESCRIPTOR, &[slot]).status, E_IO);
    let spent = w.borrow().ms - t0;
    assert!((5000..5100).contains(&spent), "{spent} ms");
}

#[test]
fn malformed_and_oversized_requests_are_refused_before_the_bus() {
    let (mut cx, w) = rig(vec![Some(Device::full(64)), None]);
    let slot = addressed(&mut cx, 1);
    let asked = w.borrow().devs[0].as_ref().unwrap().requests.len();
    let big = [slot, 0, 0xC0, 1, 0, 0, 0, 0, 0x01, 0x02];
    assert_eq!(call(&mut cx, OP_CONTROL_TRANSFER, &big).status, E_INVAL, "513 bytes");
    let short_out = [slot, 0, 0x40, 1, 0, 0, 0, 0, 4, 0, 1];
    assert_eq!(call(&mut cx, OP_CONTROL_TRANSFER, &short_out).status, E_INVAL);
    assert_eq!(call(&mut cx, OP_GET_DEVICE_DESCRIPTOR, &[7]).status, E_INVAL);
    assert_eq!(w.borrow().devs[0].as_ref().unwrap().requests.len(), asked);
}
