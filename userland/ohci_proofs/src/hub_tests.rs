// NONOS Operating System (AGPL-3.0-or-later)
//! Devices below a full-speed hub, reached through the hub class's walk:
//! a low-speed keyboard (its EDs marked low speed, so the controller
//! sends the preamble) and a full-speed stick; what the hub ops refuse.

use nonos_usb_hub::Place;

use crate::env::Clock;
use crate::facts::port::PORT_PES;
use crate::hub_fixture::tree;
use crate::hub_seen::child;
use crate::model::hub::HubModel;
use crate::model::{addressed, call, rig, Device};
use crate::wire::ops::*;
use crate::wire::{E_AGAIN, E_INVAL, E_IO};

#[test]
fn a_low_speed_keyboard_behind_the_hub_reports_and_needs_its_preamble() {
    let (mut cx, w, _, seen) = tree(HubModel::new(false, 4).plug(2, Device::low(), 2));
    let want = Place { root_port: 1, route: 0x2, depth: 1, speed: 2, tt: None, parent: (1, 2) };
    assert_eq!(seen.bound, vec![(2, want)]);
    assert_eq!(call(&mut cx, OP_ALLOC_TRANSFER_RING, &[2, 0x81, 8, 0, 10, 0]).status, 0);
    child(&w, 2).reports.push_back(vec![0, 0, 5, 0, 0, 0, 0, 0]);
    let mut got = call(&mut cx, OP_INTERRUPT_IN, &[2, 3, 8, 0]);
    for _ in 0..12 {
        if got.status != E_AGAIN {
            break;
        }
        cx.clock.idle();
        got = call(&mut cx, OP_INTERRUPT_IN, &[2, 3, 8, 0]);
    }
    assert_eq!((got.status, &got.data[2..]), (0, &[0u8, 0, 5, 0, 0, 0, 0, 0][..]));
    assert_eq!(child(&w, 2).toggle_errors, 0);
    // Its EDs without the low-speed mark reach nothing.
    cx.table.dev(2).unwrap().low_speed = false;
    assert_eq!(call(&mut cx, OP_GET_DEVICE_DESCRIPTOR, &[2]).status, E_IO);
}

#[test]
fn a_full_speed_stick_behind_the_hub_moves_bulk_data_and_leaves_the_hub_up() {
    let (mut cx, w, _, seen) = tree(HubModel::new(false, 4).plug(4, Device::full(64), 1));
    assert_eq!((seen.bound[0].1.route, seen.bound[0].1.speed), (0x4, 1));
    assert_eq!(call(&mut cx, OP_CONFIGURE_BULK, &[2, 0x82, 0x03, 0, 64, 0, 64, 0]).status, 0);
    let mut body = vec![2, 0, 100, 0];
    body.extend(0..100u8);
    assert_eq!(call(&mut cx, OP_BULK_OUT, &body).data, [100, 0, 0, 0]);
    child(&w, 4).send(&[3; 40]);
    let r = call(&mut cx, OP_BULK_IN, &[2, 0, 64, 0]);
    assert_eq!((r.status, &r.data[4..]), (0, &[3u8; 40][..]));
    assert_eq!(child(&w, 4).bulk_out, (0..100).collect::<Vec<u8>>());
    assert_eq!(call(&mut cx, OP_DISABLE_SLOT, &[2]).status, 0);
    assert_ne!(w.borrow().ports[0] & PORT_PES, 0, "the hub's root port stays enabled");
}

#[test]
fn a_routed_address_needs_a_hub_a_route_and_no_tt() {
    let (mut cx, _) = rig(vec![Some(Device::full(64)), None]);
    let _ = addressed(&mut cx, 1);
    let slot = call(&mut cx, OP_ENABLE_SLOT, &[]).data[0];
    let routed = |speed: u8, route: u32, tt: [u8; 2]| {
        let mut b = vec![slot, 1, speed];
        b.extend(route.to_le_bytes());
        b.extend(tt);
        b
    };
    assert_eq!(call(&mut cx, OP_ADDRESS_ROUTED, &routed(1, 1, [0, 0])).status, E_INVAL, "no hub");
    assert_eq!(call(&mut cx, OP_CONFIGURE_HUB, &[1, 4, 0, 0]).status, 0);
    for (speed, route, tt) in [(1, 1, [1, 1]), (3, 1, [0, 0]), (1, 0, [0, 0]), (2, 0x10, [0, 0])] {
        let r = call(&mut cx, OP_ADDRESS_ROUTED, &routed(speed, route, tt));
        assert_eq!(r.status, E_INVAL, "speed {speed} route {route:#x} tt {tt:?}");
    }
}
