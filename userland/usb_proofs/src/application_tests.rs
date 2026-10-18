// NONOS Operating System (AGPL-3.0-or-later)
//! The usage of a report descriptor's first application collection, which
//! driver.fido0 takes a security key by (usb_hid reads its interfaces
//! through report/ instead). The descriptors here are written from the HID
//! usage tables as recalled, unverified.

use crate::descriptors::application::application;

const MOUSE: [u8; 8] = [0x05, 0x01, 0x09, 0x02, 0xA1, 0x01, 0x09, 0x01];
const FIDO: [u8; 7] = [0x06, 0xD0, 0xF1, 0x09, 0x01, 0xA1, 0x01];
const GAMEPAD: [u8; 6] = [0x05, 0x01, 0x09, 0x05, 0xA1, 0x01];
const KEYBOARD: [u8; 6] = [0x05, 0x01, 0x09, 0x06, 0xA1, 0x01];

#[test]
fn a_mouse_a_key_a_gamepad_and_a_keyboard_are_told_apart() {
    assert_eq!(application(&MOUSE), Some((0x01, 0x02)));
    assert_eq!(application(&FIDO), Some((0xF1D0, 0x01)));
    assert_eq!(application(&GAMEPAD), Some((0x01, 0x05)));
    assert_eq!(application(&KEYBOARD), Some((0x01, 0x06)));
}

#[test]
fn a_four_byte_usage_carries_its_own_page() {
    let d = [0x05, 0x0C, 0x0B, 0x01, 0x00, 0x01, 0x00, 0xA1, 0x01];
    assert_eq!(application(&d), Some((0x01, 0x01)));
}

#[test]
fn a_physical_collection_and_a_long_item_are_passed_over() {
    let d =
        [0xFE, 0x02, 0x10, 0xAA, 0xBB, 0x05, 0x01, 0x09, 0x01, 0xA1, 0x00, 0x09, 0x02, 0xA1, 0x01];
    assert_eq!(application(&d), Some((0x01, 0x02)));
}

#[test]
fn a_cut_or_empty_descriptor_names_no_application() {
    assert_eq!(application(&MOUSE[..5]), None);
    assert_eq!(application(&[]), None);
    assert_eq!(application(&[0xA1, 0x01]), None, "a collection with no usage");
    assert_eq!(application(&[0xFE, 0xFF]), None);
}
