// NONOS Operating System (AGPL-3.0-or-later)
//! Devices below a high-speed hub, reached through the hub class's walk:
//! a low-speed keyboard's reports and a full-speed stick's bulk data by
//! split transactions through the hub's TT, a high-speed device without.

use nonos_usb_hub::{Place, Tt};

use crate::env::Clock;
use crate::facts::port::PORT_PE;
use crate::hub_fixture::tree;
use crate::hub_seen::child;
use crate::model::hub::HubModel;
use crate::model::{call, Device};
use crate::wire::ops::*;
use crate::wire::E_AGAIN;

#[test]
fn a_low_speed_keyboard_behind_the_hub_reports_through_its_tt() {
    let (mut cx, w, _, seen) = tree(HubModel::new(true, 4).plug(2, Device::low(), 2));
    let tt = Some(Tt { hub_slot: 1, port: 2 });
    let want = Place { root_port: 1, route: 0x2, depth: 1, speed: 2, tt, parent: (1, 2) };
    assert_eq!(seen.bound, vec![(2, want)]);
    assert_eq!(call(&mut cx, OP_ALLOC_TRANSFER_RING, &[2, 0x81, 8, 0, 10, 0]).status, 0);
    child(&w, 2).reports.push_back(vec![0, 0, 4, 0, 0, 0, 0, 0]);
    let mut got = call(&mut cx, OP_INTERRUPT_IN, &[2, 3, 8, 0]);
    for _ in 0..3 {
        if got.status != E_AGAIN {
            break;
        }
        cx.clock.idle();
        got = call(&mut cx, OP_INTERRUPT_IN, &[2, 3, 8, 0]);
    }
    assert_eq!((got.status, &got.data[2..]), (0, &[0u8, 0, 4, 0, 0, 0, 0, 0][..]));
    assert_eq!(child(&w, 2).toggle_errors, 0);
}

#[test]
fn a_full_speed_stick_behind_the_hub_moves_bulk_data_through_its_tt() {
    let (mut cx, w, _, seen) = tree(HubModel::new(true, 4).plug(3, Device::full(64), 1));
    assert_eq!(seen.bound[0].1.tt, Some(Tt { hub_slot: 1, port: 3 }));
    let r = call(&mut cx, OP_CONFIGURE_BULK, &[2, 0x82, 0x03, 0, 64, 0, 64, 0]);
    assert_eq!(r.status, 0);
    let data: Vec<u8> = (0..150).map(|i| i as u8).collect();
    let mut body = vec![2, 0, 150, 0];
    body.extend_from_slice(&data);
    assert_eq!(call(&mut cx, OP_BULK_OUT, &body).data, [150, 0, 0, 0]);
    child(&w, 3).send(&[7; 90]);
    let r = call(&mut cx, OP_BULK_IN, &[2, 0, 0, 2]);
    assert_eq!((r.status, &r.data[4..]), (0, &[7u8; 90][..]));
    let d = child(&w, 3);
    assert_eq!((&d.bulk_out, d.toggle_errors), (&data, 0));
}

#[test]
fn a_high_speed_device_needs_no_tt_and_leaves_the_root_port_up_when_given_back() {
    let (mut cx, w, _, seen) = tree(HubModel::new(true, 4).plug(1, Device::full(64), 3));
    assert_eq!((seen.bound[0].1.speed, seen.bound[0].1.tt), (3, None));
    let r = call(&mut cx, OP_GET_DEVICE_DESCRIPTOR, &[2]);
    assert_eq!(r.status, 0);
    assert_eq!(call(&mut cx, OP_DISABLE_SLOT, &[2]).status, 0);
    assert_ne!(w.borrow().ports[0] & PORT_PE, 0, "the hub's root port stays enabled");
    let port1 = call(&mut cx, OP_CONTROL_TRANSFER, &[1, 0, 0xA3, 0, 0, 0, 1, 0, 4, 0]);
    assert_eq!((port1.status, port1.data[2] & 1), (0, 1), "the hub still answers");
}
