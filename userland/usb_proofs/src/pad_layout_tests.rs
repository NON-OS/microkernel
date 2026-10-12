// NONOS Operating System (AGPL-3.0-or-later)
//! A gamepad's layout from its report descriptor. The descriptor here is
//! composed from the HID usage tables as recalled, unverified: report 1,
//! twelve buttons, four padding bits, X Y Z Rz as bytes, a 4-bit hat with
//! a null state, four padding bits.

use crate::descriptors::pad::pad_layout;

#[rustfmt::skip]
pub const PAD: [u8; 70] = [
    0x05, 0x01, 0x09, 0x05, 0xA1, 0x01, 0x85, 0x01,
    0x05, 0x09, 0x19, 0x01, 0x29, 0x0C, 0x15, 0x00, 0x25, 0x01, 0x75, 0x01, 0x95, 0x0C, 0x81, 0x02,
    0x75, 0x01, 0x95, 0x04, 0x81, 0x03,
    0x05, 0x01, 0x09, 0x30, 0x09, 0x31, 0x09, 0x32, 0x09, 0x35,
    0x15, 0x00, 0x26, 0xFF, 0x00, 0x75, 0x08, 0x95, 0x04, 0x81, 0x02,
    0x09, 0x39, 0x15, 0x00, 0x25, 0x07, 0x75, 0x04, 0x95, 0x01, 0x81, 0x42,
    0x75, 0x04, 0x95, 0x01, 0x81, 0x03, 0xC0,
];

#[test]
fn a_game_pad_gives_its_buttons_axes_and_hat_where_they_sit() {
    let l = pad_layout(&PAD).expect("a game pad");
    assert!(l.ids);
    assert_eq!(l.fields.len(), 12 + 4 + 1);
    let b12 = l.fields[11];
    assert_eq!((b12.page, b12.usage, b12.bit_offset, b12.bit_size), (0x09, 12, 11, 1));
    let at = |u: u16| l.fields.iter().find(|f| f.page == 1 && f.usage == u).copied().unwrap();
    assert_eq!((at(0x30).bit_offset, at(0x35).bit_offset, at(0x30).max), (16, 40, 255));
    assert_eq!((at(0x39).bit_offset, at(0x39).bit_size, at(0x39).max), (48, 4, 7));
    assert!(l.fields.iter().all(|f| f.report_id == 1));
}

#[test]
fn a_mouse_a_keyboard_or_a_nested_pad_is_no_gamepad() {
    let mouse = [
        0x05, 0x01, 0x09, 0x02, 0xA1, 0x01, 0x05, 0x09, 0x19, 0x01, 0x29, 0x03, 0x75, 0x01, 0x95,
        0x03, 0x81, 0x02, 0xC0,
    ];
    assert_eq!(pad_layout(&mouse), None);
    let mut keyboard = PAD;
    keyboard[3] = 0x06;
    assert_eq!(pad_layout(&keyboard), None);
    let nested = [&[0x05, 0x0C, 0x09, 0x01, 0xA1, 0x01][..], &PAD[..], &[0xC0]].concat();
    assert_eq!(pad_layout(&nested), None, "only a top-level application counts");
}

#[test]
fn hostile_counts_and_pushes_end_bounded() {
    let mut huge = PAD.to_vec();
    // Report Count of 0x7FFFFFFF for the buttons: capped, no long loop.
    huge.splice(20..22, [0x97, 0xFF, 0xFF, 0xFF, 0x7F]);
    let l = pad_layout(&huge).expect("still a pad");
    assert!(l.fields.len() <= 64);
    let pushes = [&[0x05, 0x01, 0x09, 0x05, 0xA1, 0x01][..], &[0xA4; 9]].concat();
    assert_eq!(pad_layout(&pushes), None, "Push nested past its bound");
    assert_eq!(pad_layout(&[0xB4]), None, "Pop of nothing");
    assert_eq!(pad_layout(&PAD[..22]), None, "cut before any field");
    assert!(pad_layout(&PAD[..24]).is_some(), "cut after the buttons: they are read");
}
