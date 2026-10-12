// NONOS Operating System (AGPL-3.0-or-later)
//! Several UHCI controllers served as one host: their ports numbered on,
//! slots the host's, each slot's requests reaching the controller it was
//! addressed on, and its device only.

use crate::model::{call_host, rig, Cx, Device};
use crate::server::many::front::Bind;
use crate::server::many::Front;
use crate::wire::ops::*;
use crate::wire::{E_INVAL, E_IO};

/// Three controllers: a device on port 1 of the first, port 2 of the
/// second, none on the third; the worlds kept for the tests to read.
fn host() -> (Front<Cx>, Vec<crate::model::Shared>) {
    let mut f = Front::new();
    let mut worlds = vec![];
    for devs in [[Some(Device::full(64)), None], [None, Some(Device::low())], [None, None]] {
        let (cx, w) = rig(devs);
        assert!(f.add(cx));
        worlds.push(w);
    }
    (f, worlds)
}

#[test]
fn ports_are_numbered_on_across_controllers() {
    let (mut f, _) = host();
    let r = call_host(&mut f, OP_PORT_STATUS, &[]);
    assert_eq!(r.data[0], 6, "two ports each");
    let ids: Vec<u8> = r.data[4..].chunks(8).map(|e| e[0]).collect();
    assert_eq!(ids, [1, 2, 3, 4, 5, 6]);
    let connected: Vec<u8> = r.data[4..].chunks(8).map(|e| e[4] & 1).collect();
    assert_eq!(connected, [1, 0, 0, 1, 0, 0], "first's port 1, second's port 2");
    assert_eq!((f.local(4), f.local(6), f.local(7)), (Some((1, 2)), Some((2, 2)), None));
}

#[test]
fn each_slot_reaches_the_controller_it_was_addressed_on() {
    let (mut f, w) = host();
    let a = call_host(&mut f, OP_ENABLE_SLOT, &[]).data[0];
    let b = call_host(&mut f, OP_ENABLE_SLOT, &[]).data[0];
    let ra = call_host(&mut f, OP_ADDRESS_DEVICE, &[a, 1]);
    let rb = call_host(&mut f, OP_ADDRESS_DEVICE, &[b, 4]);
    assert_eq!((ra.status, ra.data[1], rb.status, rb.data[1]), (0, 1, 0, 4), "host ports");
    assert_eq!((rb.data[2], f.bound[b as usize - 1]), (2, Bind::On(1)), "low speed, second");
    assert_eq!(w[1].borrow().devs[1].as_ref().unwrap().addr, b, "addressed on its own bus");
    let da = call_host(&mut f, OP_GET_DEVICE_DESCRIPTOR, &[a]);
    let db = call_host(&mut f, OP_GET_DEVICE_DESCRIPTOR, &[b]);
    assert_eq!((da.status, da.data[7], db.status, db.data[7]), (0, 64, 0, 8));
    assert_eq!(call_host(&mut f, OP_DISABLE_SLOT, &[b]).status, 0);
    assert_eq!(f.bound[b as usize - 1], Bind::Free);
    assert_eq!(call_host(&mut f, OP_GET_DEVICE_DESCRIPTOR, &[b]).status, E_INVAL);
}

#[test]
fn a_failed_address_or_an_unknown_port_binds_nothing() {
    let (mut f, _) = host();
    let s = call_host(&mut f, OP_ENABLE_SLOT, &[]).data[0];
    assert_eq!(call_host(&mut f, OP_ADDRESS_DEVICE, &[s, 5]).status, E_IO, "port 5 is empty");
    assert_eq!(f.bound[s as usize - 1], Bind::Enabled);
    assert!(f.hcs[2].as_mut().unwrap().table.dev(s).is_none(), "the third holds nothing");
    assert_eq!(call_host(&mut f, OP_ADDRESS_DEVICE, &[s, 9]).status, E_INVAL, "no port 9");
    assert_eq!(call_host(&mut f, OP_ADDRESS_DEVICE, &[s, 1]).status, 0, "then port 1");
}
