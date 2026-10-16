// NONOS Operating System (AGPL-3.0-or-later)
//! A slot no interface of is claimed is its enabler's: every request that
//! names it, transfers, rings, pipes, streams, resets, hubs, addresses,
//! descriptors, the first claim and the give-back, is refused to any other
//! driver. A holder that is gone is replaced by the next asker; the kernel
//! is not held to it. Once shared: joined_tests.rs.

use super::police_tests::{ask, HID, STRANGER};
use crate::protocol::*;
use crate::slots::slot_ops::names_slot;
use crate::slots::{gate, Sharing};

/// Every request that names slot 4, with a body its handler would take.
const SLOT_OPS: [(u16, &[u8]); 21] = [
    (OP_DISABLE_SLOT, &[4]),
    (OP_ADDRESS_DEVICE, &[4, 1]),
    (OP_GET_DEVICE_DESCRIPTOR, &[4]),
    (OP_GET_CONFIG_DESCRIPTOR, &[4, 0, 0, 1]),
    (OP_ALLOC_TRANSFER_RING, &[4, 0x81, 8, 0, 10, 0]),
    (OP_CONTROL_TRANSFER, &[4, 0, 0x80, 6, 0, 1, 0, 0, 18, 0]),
    (OP_INTERRUPT_IN, &[4, 3, 8, 0]),
    (OP_INTERRUPT_OUT, &[4, 4, 1, 0, 0]),
    (OP_CONFIGURE_BULK, &[4, 0x81, 0x02, 0, 0, 2, 0, 2]),
    (OP_CONFIGURE_BULK2, &[4, 0x83, 0x04, 0, 0, 2, 0, 2]),
    (OP_CONFIGURE_STREAMS, &[4, 1, 3, 4]),
    (OP_BULK_OUT, &[4, 0, 1, 0, 0xAA]),
    (OP_BULK_IN, &[4, 1, 64, 0]),
    (OP_BULK_IN_POLL, &[4, 1, 64, 0]),
    (OP_RESET_BULK, &[4, 0b10]),
    (OP_ADDRESS_ROUTED, &[4, 1, 0, 0, 0, 0, 0, 0]),
    (OP_CONFIGURE_HUB, &[4, 4, 0, 0]),
    (OP_CONFIGURE_ISOCH, &[4, 0x83, 1, 0, 64, 0, 0, 0]),
    (OP_ISOCH_OUT, &[4, 1]),
    (OP_ISOCH_IN, &[4, 1]),
    (OP_CLAIM_INTERFACE, &[4, 3, 0]),
];

pub(crate) fn held_by_hid() -> Sharing {
    let mut s = Sharing::default();
    s.holders.set(4, HID);
    s
}

#[test]
fn every_slot_request_is_refused_to_another_driver() {
    for (op, body) in SLOT_OPS {
        assert!(names_slot(op), "{op:#x} names a slot");
        assert_eq!(ask(&mut held_by_hid(), STRANGER, op, body), Err(-1), "{op:#x}");
        assert_eq!(ask(&mut held_by_hid(), HID, op, body), Ok(()), "{op:#x}");
        assert_eq!(gate(&mut held_by_hid(), op, body, 0, |_| true), Ok(()), "the kernel");
    }
    for op in [OP_HEALTHCHECK, OP_CONTROLLER_STATUS, OP_PORT_STATUS, OP_ENABLE_SLOT] {
        assert!(!names_slot(op), "{op:#x} names no slot");
    }
    assert!(!names_slot(OP_SLOT_OF_PORT), "it names a port");
}

#[test]
fn a_gone_holders_slot_passes_to_the_next_asker() {
    let mut s = held_by_hid();
    let get = [4, 0, 0, 1];
    assert_eq!(gate(&mut s, OP_GET_CONFIG_DESCRIPTOR, &get, STRANGER, |p| p != HID), Ok(()));
    assert_eq!(s.holders.holder(4), Some(STRANGER));
    assert_eq!(ask(&mut s, HID, OP_GET_CONFIG_DESCRIPTOR, &get), Err(-1), "the slot moved on");
}
