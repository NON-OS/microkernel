// NONOS Operating System (AGPL-3.0-or-later)
//! A shared device's slot served only to its claimants, and each pipe only
//! to the claimant that configured it; a device nobody claimed is served to
//! anyone, as before sharing.

use crate::protocol::*;
use crate::slots::{use_of, Asker, Sharing};

pub(crate) const HID: u32 = 10;
pub(crate) const FIDO: u32 = 20;
pub(crate) const STRANGER: u32 = 99;

pub(crate) fn ask(s: &mut Sharing, pid: u32, op: u16, body: &[u8]) -> Result<(), i32> {
    let (slot, u) = use_of(op, body).expect("a slot op");
    let who = Asker { pid, shared: s.claims.any(slot), holds: s.claims.holds(slot, pid) };
    s.owners.admit(slot, u, who)
}

/// usb_hid configured interrupt IN 0x81 (DCI 3) before it claimed
/// interface 0; fido0 claimed interface 1 and configured 0x02 and 0x82.
pub(crate) fn composite_key() -> Sharing {
    let mut s = Sharing::default();
    ask(&mut s, HID, OP_ALLOC_TRANSFER_RING, &[4, 0x81, 8, 0, 10, 0]).unwrap();
    s.claims.claim(4, 3, 0, HID).unwrap();
    s.claims.claim(4, 3, 1, FIDO).unwrap();
    ask(&mut s, FIDO, OP_ALLOC_TRANSFER_RING, &[4, 0x02, 64, 0, 5, 0]).unwrap();
    ask(&mut s, FIDO, OP_ALLOC_TRANSFER_RING, &[4, 0x82, 64, 0, 5, 0]).unwrap();
    s
}

#[test]
fn a_device_nobody_claimed_is_served_to_anyone() {
    let mut s = Sharing::default();
    assert_eq!(ask(&mut s, HID, OP_CONFIGURE_BULK, &[4, 0x81, 0x02, 0, 0, 2, 0, 2]), Ok(()));
    assert_eq!(ask(&mut s, STRANGER, OP_BULK_OUT, &[4, 0, 1, 0, 0xAA]), Ok(()));
    assert_eq!(
        ask(&mut s, STRANGER, OP_CONTROL_TRANSFER, &[4, 0, 0x80, 6, 0, 1, 0, 0, 18, 0]),
        Ok(())
    );
}

#[test]
fn a_stranger_is_refused_a_shared_device_even_its_control_pipe() {
    let mut s = composite_key();
    assert_eq!(
        ask(&mut s, STRANGER, OP_CONTROL_TRANSFER, &[4, 0, 0x80, 6, 0, 1, 0, 0, 18, 0]),
        Err(-1)
    );
    assert_eq!(ask(&mut s, STRANGER, OP_INTERRUPT_IN, &[4, 3, 8, 0]), Err(-1));
    use crate::slots::use_of;
    assert_eq!(use_of(OP_GET_CONFIG_DESCRIPTOR, &[4, 0, 0, 1]), None, "a read, open to a joiner");
}

#[test]
fn each_claimant_moves_only_its_own_pipes() {
    let mut s = composite_key();
    assert_eq!(ask(&mut s, HID, OP_INTERRUPT_IN, &[4, 3, 8, 0]), Ok(()));
    assert_eq!(ask(&mut s, FIDO, OP_INTERRUPT_IN, &[4, 3, 8, 0]), Err(-1), "the keyboard's");
    assert_eq!(ask(&mut s, FIDO, OP_INTERRUPT_OUT, &[4, 4, 1, 0, 0]), Ok(()));
    assert_eq!(ask(&mut s, HID, OP_INTERRUPT_OUT, &[4, 4, 1, 0, 0]), Err(-1), "the key's");
    assert_eq!(ask(&mut s, FIDO, OP_ALLOC_TRANSFER_RING, &[4, 0x81, 8, 0, 10, 0]), Err(-16));
    assert_eq!(
        ask(&mut s, FIDO, OP_CONTROL_TRANSFER, &[4, 0, 0x81, 6, 0, 0x22, 1, 0, 0, 1]),
        Ok(())
    );
}
