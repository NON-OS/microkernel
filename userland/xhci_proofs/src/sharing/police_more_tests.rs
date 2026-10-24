// NONOS Operating System (AGPL-3.0-or-later)
//! The rest of the policing: isochronous pipes by direction, a disabled
//! slot's pipes forgotten, and which pipe each slot op names.

use super::police_tests::{ask, composite_key, FIDO, HID, STRANGER};
use crate::protocol::*;

#[test]
fn an_isochronous_pipe_is_its_configurers_by_direction() {
    let mut s = composite_key();
    assert_eq!(ask(&mut s, FIDO, OP_CONFIGURE_ISOCH, &[4, 0x83, 1, 0, 64, 0, 0, 0]), Ok(()));
    assert_eq!(ask(&mut s, HID, OP_ISOCH_IN, &[4, 1]), Err(-1));
    assert_eq!(ask(&mut s, FIDO, OP_ISOCH_IN, &[4, 1]), Ok(()));
    assert_eq!(ask(&mut s, HID, OP_ISOCH_OUT, &[4, 1]), Ok(()), "OUT is nobody's yet");
}

#[test]
fn a_disabled_slot_forgets_its_pipes() {
    let mut s = composite_key();
    s.owners.forget(4);
    s.holders.forget(4);
    s.claims.release(4, HID, |_| true);
    s.claims.release(4, FIDO, |_| true);
    s.holders.set(4, STRANGER);
    assert_eq!(ask(&mut s, STRANGER, OP_INTERRUPT_IN, &[4, 3, 8, 0]), Ok(()), "a new device");
}

#[test]
fn every_slot_op_names_its_pipe_and_others_none() {
    use crate::slots::uses::use_of as u;
    assert_eq!(u(OP_BULK_OUT, &[4, 0x21]), u(OP_BULK_IN, &[4, 1]), "a stream is its pair's");
    assert_ne!(u(OP_BULK_IN, &[4, 0]), u(OP_BULK_IN, &[4, 1]));
    assert_eq!(u(OP_RESET_BULK, &[4, 0b10]), u(OP_BULK_IN, &[4, 1]));
    assert!([OP_PORT_STATUS, OP_ENABLE_SLOT, OP_ADDRESS_DEVICE, OP_CONFIGURE_HUB]
        .iter()
        .all(|&op| u(op, &[4, 3]).is_none()));
    assert_eq!(u(OP_BULK_OUT, &[4, 7]), None, "no such pair: the handler refuses it");
}
