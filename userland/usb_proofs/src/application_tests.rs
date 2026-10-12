// NONOS Operating System (AGPL-3.0-or-later)
//! A non-boot interface is fed to the tablet decoder only when its report
//! descriptor's first application collection is a Generic Desktop pointer
//! or mouse. The descriptors here are written from the HID usage tables as
//! recalled, unverified.

use crate::descriptors::application::application;
use crate::descriptors::pointer::is_pointer;

const MOUSE: [u8; 8] = [0x05, 0x01, 0x09, 0x02, 0xA1, 0x01, 0x09, 0x01];
const FIDO: [u8; 7] = [0x06, 0xD0, 0xF1, 0x09, 0x01, 0xA1, 0x01];
const GAMEPAD: [u8; 6] = [0x05, 0x01, 0x09, 0x05, 0xA1, 0x01];
const KEYBOARD: [u8; 6] = [0x05, 0x01, 0x09, 0x06, 0xA1, 0x01];

#[test]
fn a_mouse_is_a_pointer_and_a_key_a_gamepad_or_a_keyboard_is_not() {
    assert_eq!(application(&MOUSE), Some((0x01, 0x02)));
    assert!(is_pointer(&MOUSE));
    assert_eq!(application(&FIDO), Some((0xF1D0, 0x01)));
    assert!(!is_pointer(&FIDO), "a security key's packets are no pointer's");
    assert!(!is_pointer(&GAMEPAD) && !is_pointer(&KEYBOARD));
}

#[test]
fn a_four_byte_usage_carries_its_own_page() {
    let d = [0x05, 0x0C, 0x0B, 0x01, 0x00, 0x01, 0x00, 0xA1, 0x01];
    assert_eq!(application(&d), Some((0x01, 0x01)));
    assert!(is_pointer(&d));
}

#[test]
fn a_physical_collection_and_a_long_item_are_passed_over() {
    let d =
        [0xFE, 0x02, 0x10, 0xAA, 0xBB, 0x05, 0x01, 0x09, 0x01, 0xA1, 0x00, 0x09, 0x02, 0xA1, 0x01];
    assert_eq!(application(&d), Some((0x01, 0x02)));
}

#[test]
fn a_cut_or_empty_descriptor_is_no_pointer() {
    assert_eq!(application(&MOUSE[..5]), None);
    assert_eq!(application(&[]), None);
    assert_eq!(application(&[0xA1, 0x01]), None, "a collection with no usage");
    assert_eq!(application(&[0xFE, 0xFF]), None);
}
