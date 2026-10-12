// NONOS Operating System (AGPL-3.0-or-later)
//! Root ports on the wire, slots, and addressing.

use crate::model::{addressed, call, rig, Device};
use crate::wire::ops::*;
use crate::wire::{E_BUSY, E_INVAL, E_IO};

fn portsc(data: &[u8], port: usize) -> u32 {
    let at = 4 + 8 * (port - 1);
    u32::from_le_bytes(data[at + 4..at + 8].try_into().unwrap())
}

#[test]
fn ports_read_as_xhci_portsc_with_speed_and_owner() {
    let (mut cx, _) = rig([Some(Device::full(64)), Some(Device::low())]);
    let r = call(&mut cx, OP_PORT_STATUS, &[]);
    assert_eq!((r.status, r.data[0], r.data.len()), (0, 2, 20));
    assert_eq!(portsc(&r.data, 1), 1 | 1 << 10 | 1 << 17, "full speed, connect change");
    assert_eq!(portsc(&r.data, 2), 1 | 2 << 10 | 1 << 17, "low speed");
    let again = call(&mut cx, OP_PORT_STATUS, &[]);
    assert_eq!(portsc(&again.data, 1) & 1 << 17, 0, "the change is cleared once reported");
    let slot = addressed(&mut cx, 1);
    let r = call(&mut cx, OP_PORT_STATUS, &[]);
    assert_eq!((r.data[5], r.data[13]), (1, 0), "port 1 addressed, port 2 free");
    assert_eq!(portsc(&r.data, 1) & 2, 2, "enabled");
    assert_eq!(slot, 1);
}

#[test]
fn addressing_gives_the_slot_number_as_the_usb_address() {
    let (mut cx, w) = rig([Some(Device::full(64)), Some(Device::low())]);
    let s1 = addressed(&mut cx, 1);
    let s2 = addressed(&mut cx, 2);
    let w = w.borrow();
    assert_eq!(w.devs[0].as_ref().unwrap().addr, s1);
    assert_eq!(w.devs[1].as_ref().unwrap().addr, s2);
    assert_eq!(w.devs[1].as_ref().unwrap().toggle_errors, 0);
}

#[test]
fn the_reply_names_slot_port_speed_and_ep0_size() {
    let (mut cx, _) = rig([None, Some(Device::low())]);
    let slot = call(&mut cx, OP_ENABLE_SLOT, &[]).data[0];
    let r = call(&mut cx, OP_ADDRESS_DEVICE, &[slot, 2]);
    assert_eq!(r.data, [slot, 2, 2, 0, 8, 0, 0, 0]);
}

#[test]
fn an_empty_port_a_held_port_and_a_bad_port_are_refused() {
    let (mut cx, _) = rig([Some(Device::full(8)), None]);
    let s = call(&mut cx, OP_ENABLE_SLOT, &[]).data[0];
    assert_eq!(call(&mut cx, OP_ADDRESS_DEVICE, &[s, 2]).status, E_IO);
    assert_eq!(call(&mut cx, OP_ADDRESS_DEVICE, &[s, 3]).status, E_INVAL);
    addressed(&mut cx, 1);
    let t = call(&mut cx, OP_ENABLE_SLOT, &[]).data[0];
    assert_eq!(call(&mut cx, OP_ADDRESS_DEVICE, &[t, 1]).status, E_BUSY);
    assert_eq!(call(&mut cx, OP_ADDRESS_DEVICE, &[9, 1]).status, E_INVAL, "slot not enabled");
}
