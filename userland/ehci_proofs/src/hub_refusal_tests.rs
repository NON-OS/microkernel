// NONOS Operating System (AGPL-3.0-or-later)
//! What the hub ops refuse, and that the model refuses a QH naming the
//! wrong TT, so the routed proofs could fail.

use crate::env::Mem;
use crate::hub_fixture::tree;
use crate::model::hub::HubModel;
use crate::model::{addressed, call, high, rig, Device};
use crate::sched::layout::{ctrl_ring, qh};
use crate::wire::ops::*;
use crate::wire::{E_INVAL, E_IO};

fn routed(slot: u8, port: u8, speed: u8, route: u32, tt: (u8, u8)) -> Vec<u8> {
    let mut b = vec![slot, port, speed];
    b.extend(route.to_le_bytes());
    b.extend([tt.0, tt.1]);
    b
}

#[test]
fn a_routed_address_needs_a_hub_a_route_and_the_right_tt() {
    let (mut cx, _) = rig(vec![high(), None]);
    let _ = addressed(&mut cx, 1);
    let slot = call(&mut cx, OP_ENABLE_SLOT, &[]).data[0];
    let no_hub = routed(slot, 1, 3, 0x1, (0, 0));
    assert_eq!(call(&mut cx, OP_ADDRESS_ROUTED, &no_hub).status, E_INVAL, "no hub on port 1");
    assert_eq!(call(&mut cx, OP_CONFIGURE_HUB, &[1, 4, 1, 0]).status, 0);
    for (speed, route, tt, why) in [
        (3, 0x1, (1, 1), "a TT for a high-speed device"),
        (1, 0x1, (0, 0), "no TT for a full-speed one"),
        (2, 0x1, (slot, 1), "a TT in a slot that is no hub"),
        (1, 0x1, (1, 16), "a TT port past 15"),
        (3, 0, (0, 0), "route 0 is the root port"),
        (3, 0x101, (0, 0), "a gap in the route"),
        (4, 0x1, (0, 0), "SuperSpeed on EHCI"),
    ] {
        let r = call(&mut cx, OP_ADDRESS_ROUTED, &routed(slot, 1, speed, route, tt));
        assert_eq!(r.status, E_INVAL, "{why}");
    }
    assert_eq!(call(&mut cx, OP_CONFIGURE_HUB, &[slot, 4, 1, 0]).status, E_INVAL, "unaddressed");
    assert_eq!(call(&mut cx, OP_CONFIGURE_HUB, &[1, 4, 4, 0]).status, E_INVAL, "think time 4");
}

#[test]
fn a_qh_naming_another_port_of_the_tt_reaches_nothing() {
    let (mut cx, _, _, _) = tree(HubModel::new(true, 4).plug(2, Device::full(64), 1));
    assert_eq!(call(&mut cx, OP_GET_DEVICE_DESCRIPTOR, &[2]).status, 0);
    let caps = qh(ctrl_ring(2)) + 8;
    let v = cx.mem.rd(caps);
    cx.mem.wr(caps, v & !(0x7F << 23) | 3 << 23);
    assert_eq!(call(&mut cx, OP_GET_DEVICE_DESCRIPTOR, &[2]).status, E_IO, "port 3 has no device");
}
