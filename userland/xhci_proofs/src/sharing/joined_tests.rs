// NONOS Operating System (AGPL-3.0-or-later)
//! Once a slot is shared, a joiner may read its descriptors and claim a
//! free interface, which it must do before it can join; readdressing it or
//! configuring it as a hub stays its claimants' or its holder's.

use super::police_tests::{ask, HID, STRANGER};
use super::unclaimed_tests::held_by_hid;
use crate::protocol::*;

#[test]
fn once_shared_a_joiner_reads_and_claims_but_does_not_readdress() {
    let mut s = held_by_hid();
    s.claims.claim(4, 3, 0, HID).unwrap();
    assert_eq!(ask(&mut s, STRANGER, OP_GET_CONFIG_DESCRIPTOR, &[4, 0, 0, 1]), Ok(()));
    assert_eq!(ask(&mut s, STRANGER, OP_CLAIM_INTERFACE, &[4, 3, 1]), Ok(()));
    assert_eq!(ask(&mut s, STRANGER, OP_CONFIGURE_HUB, &[4, 4, 0, 0]), Err(-1));
    assert_eq!(ask(&mut s, STRANGER, OP_ADDRESS_DEVICE, &[4, 1]), Err(-1));
    s.claims.claim(4, 3, 1, STRANGER).unwrap();
    assert_eq!(ask(&mut s, STRANGER, OP_ADDRESS_DEVICE, &[4, 1]), Ok(()), "a claimant");
}
