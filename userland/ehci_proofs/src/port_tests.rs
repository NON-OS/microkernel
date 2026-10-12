// NONOS Operating System (AGPL-3.0-or-later)
//! Root ports decided: a high-speed device stays and shows at speed 3; a
//! full-speed device is reset, found not enabled and handed on; a
//! low-speed one is handed on at once, unreset. A handed port shows
//! nothing here.

use crate::model::{addressed, call, high, rig, Device, Speed};
use crate::wire::ops::*;
use crate::wire::{E_BUSY, E_INVAL, E_IO};

fn portsc(data: &[u8], port: usize) -> u32 {
    let at = 4 + 8 * (port - 1);
    u32::from_le_bytes(data[at + 4..at + 8].try_into().unwrap())
}

fn mixed() -> Vec<Option<(Device, Speed)>> {
    vec![high(), Some((Device::full(64), Speed::Full)), Some((Device::low(), Speed::Low))]
}

#[test]
fn only_the_high_speed_device_stays_and_the_others_go_to_the_companion() {
    let (mut cx, w) = rig(mixed());
    let r = call(&mut cx, OP_PORT_STATUS, &[]);
    assert_eq!((r.status, r.data[0], r.data.len()), (0, 3, 28));
    assert_eq!(portsc(&r.data, 1), 1 | 2 | 3 << 10 | 1 << 17, "high speed, enabled");
    assert_eq!((portsc(&r.data, 2), portsc(&r.data, 3)), (0, 0), "not this host's");
    let w = w.borrow();
    assert_eq!(w.handed, [0, 1, 1]);
    assert_eq!(w.resets, [1, 1, 0], "a low-speed device is handed on unreset");
    assert_eq!(w.configflag, 1, "ports were routed here first");
}

#[test]
fn a_port_is_decided_once_and_its_change_cleared_once_reported() {
    let (mut cx, w) = rig(mixed());
    let first = call(&mut cx, OP_PORT_STATUS, &[]);
    let again = call(&mut cx, OP_PORT_STATUS, &[]);
    assert_eq!(portsc(&first.data, 1) & 1 << 17, 1 << 17);
    assert_eq!(portsc(&again.data, 1), 1 | 2 | 3 << 10, "the change reported once");
    assert_eq!(w.borrow().resets, [1, 1, 0], "no port reset twice");
}

#[test]
fn addressing_names_slot_port_high_speed_and_ep0_size() {
    let (mut cx, w) = rig(vec![high(), high(), None]);
    let (s1, s2) = (addressed(&mut cx, 1), addressed(&mut cx, 2));
    assert_eq!(w.borrow().devs[1].as_ref().unwrap().addr, s2);
    let t = call(&mut cx, OP_ENABLE_SLOT, &[]).data[0];
    assert_eq!(call(&mut cx, OP_ADDRESS_DEVICE, &[t, 1]).status, E_BUSY, "slot {s1} holds it");
    call(&mut cx, OP_DISABLE_SLOT, &[s1]);
    let r = call(&mut cx, OP_ADDRESS_DEVICE, &[t, 1]);
    assert_eq!(r.data, [t, 1, 3, 0, 64, 0, 0, 0]);
}

#[test]
fn a_handed_port_an_empty_port_and_a_bad_port_are_refused() {
    let (mut cx, _) = rig(mixed());
    call(&mut cx, OP_PORT_STATUS, &[]);
    let s = call(&mut cx, OP_ENABLE_SLOT, &[]).data[0];
    assert_eq!(call(&mut cx, OP_ADDRESS_DEVICE, &[s, 2]).status, E_IO, "the companion's");
    assert_eq!(call(&mut cx, OP_ADDRESS_DEVICE, &[s, 3]).status, E_IO);
    assert_eq!(call(&mut cx, OP_ADDRESS_DEVICE, &[s, 4]).status, E_INVAL);
    assert_eq!(call(&mut cx, OP_ADDRESS_DEVICE, &[9, 1]).status, E_INVAL, "slot not enabled");
}
