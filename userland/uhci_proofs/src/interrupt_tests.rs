// NONOS Operating System (AGPL-3.0-or-later)
//! Interrupt IN: an armed TD per endpoint, polled; NAKs cost nothing.

use crate::model::{addressed, call, rig, Device};
use crate::wire::ops::*;
use crate::wire::{E_AGAIN, E_BUSY, E_INVAL};

fn armed(dev: Device) -> (crate::model::Cx, crate::model::Shared, u8) {
    let (mut cx, w) = rig([Some(dev), None]);
    let slot = addressed(&mut cx, 1);
    let r = call(&mut cx, OP_ALLOC_TRANSFER_RING, &[slot, 0x81, 8, 0, 10, 0]);
    assert_eq!((r.status, r.data.as_slice()), (0, &[3u8, 0, 0, 0][..]));
    (cx, w, slot)
}

#[test]
fn nothing_yet_is_e_again_and_each_report_comes_once_in_order() {
    let (mut cx, w, slot) = armed(Device::low());
    assert_eq!(call(&mut cx, OP_INTERRUPT_IN, &[slot, 3, 8, 0]).status, E_AGAIN);
    for r in [[1u8, 2, 3], [4, 5, 6], [7, 8, 9]] {
        w.borrow_mut().devs[0].as_mut().unwrap().reports.push_back(r.to_vec());
    }
    for want in [[1u8, 2, 3], [4, 5, 6], [7, 8, 9]] {
        let mut got = call(&mut cx, OP_INTERRUPT_IN, &[slot, 3, 8, 0]);
        for _ in 0..3 {
            if got.status != E_AGAIN {
                break;
            }
            crate::env::Clock::idle(&cx.clock);
            got = call(&mut cx, OP_INTERRUPT_IN, &[slot, 3, 8, 0]);
        }
        assert_eq!((got.status, &got.data[..2], &got.data[2..]), (0, &[3u8, 0][..], &want[..]));
    }
    assert_eq!(w.borrow().devs[0].as_ref().unwrap().toggle_errors, 0, "toggles alternate");
}

#[test]
fn an_unknown_endpoint_an_out_endpoint_and_a_ninth_are_refused() {
    let (mut cx, _, slot) = armed(Device::full(64));
    assert_eq!(call(&mut cx, OP_INTERRUPT_IN, &[slot, 5, 8, 0]).status, E_INVAL);
    let out = call(&mut cx, OP_ALLOC_TRANSFER_RING, &[slot, 0x01, 8, 0, 10, 0]);
    assert_eq!(out.status, E_INVAL);
    for _ in 0..7 {
        assert_eq!(call(&mut cx, OP_ALLOC_TRANSFER_RING, &[slot, 0x81, 8, 0, 10, 0]).status, 0);
    }
    let ninth = call(&mut cx, OP_ALLOC_TRANSFER_RING, &[slot, 0x81, 8, 0, 10, 0]);
    assert_eq!(ninth.status, E_BUSY);
}

#[test]
fn a_disabled_slot_frees_its_queues_and_port() {
    let (mut cx, w, slot) = armed(Device::full(64));
    assert_eq!(call(&mut cx, OP_DISABLE_SLOT, &[slot]).status, 0);
    assert_eq!(w.borrow().portsc[0] & crate::facts::io::PORT_PE, 0, "port disabled");
    assert!(cx.table.intr.iter().all(|e| e.is_none()));
    let r = call(&mut cx, OP_PORT_STATUS, &[]);
    assert_eq!(r.data[5], 0, "port 1 free again");
    let again = addressed(&mut cx, 1);
    assert_eq!(again, slot, "the lowest free slot");
}
