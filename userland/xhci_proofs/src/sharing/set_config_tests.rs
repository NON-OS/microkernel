// NONOS Operating System (AGPL-3.0-or-later)
//! A device judged for every configuration it is set to, not only its
//! first: one that shows a harmless configuration and is then set to a
//! keyboard one is held at that SET_CONFIGURATION, and its pipes with it.
//! Under Ask any HID interface is held, boot protocol or not.

use super::authorize_tests::config;
use crate::protocol::OP_CONTROL_TRANSFER;
use crate::slots::authorize::{Authority, Policy};
use crate::slots::binds::set_configuration;
use crate::slots::Standing;

const STORAGE: (u8, u8, u8) = (8, 6, 0x50);

#[test]
fn a_switch_to_another_configuration_is_judged_again() {
    let mut a = Authority::new(Policy::Ask);
    assert_eq!(a.judge(4, 1, Some(&config(&[STORAGE]))), Standing::Authorized);
    assert_eq!(a.admit_set(4, 1), Some(Ok(())));
    assert_eq!(a.admit_set(4, 2), None, "configuration 2 is not judged yet");
    assert!(matches!(a.judge(4, 2, Some(&config(&[(3, 0, 0)]))), Standing::Held { .. }));
    assert_eq!(a.admit_set(4, 2), Some(Err(-13)), "the keyboard configuration is held");
    assert_eq!(a.admit(4), Some(Err(-13)), "and the pipes under it");
    assert_eq!(a.admit_set(4, 1), Some(Ok(())), "back to the storage one");
    assert_eq!(a.admit(4), Some(Ok(())));
    assert_eq!(a.admit_set(4, 0), Some(Ok(())), "unconfiguring binds nothing");
}

#[test]
fn any_hid_interface_is_held_under_ask() {
    let mut a = Authority::new(Policy::Ask);
    for (slot, hid) in [(1, (3, 1, 2)), (2, (3, 0, 0)), (3, (3, 1, 1))] {
        assert!(matches!(a.judge(slot, 1, Some(&config(&[hid]))), Standing::Held { .. }));
    }
}

#[test]
fn set_configuration_names_the_value_it_sets() {
    let set = [4, 0, 0x00, 0x09, 2, 0, 0, 0, 0, 0];
    assert_eq!(set_configuration(OP_CONTROL_TRANSFER, &set), Some(2));
    let get = [4, 0, 0x80, 0x06, 0, 2, 0, 0, 9, 0];
    assert_eq!(set_configuration(OP_CONTROL_TRANSFER, &get), None);
}
