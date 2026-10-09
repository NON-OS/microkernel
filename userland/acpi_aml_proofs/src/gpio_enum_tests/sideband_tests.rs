// NONOS Operating System (AGPL-3.0-or-later)
//! The sideband port table, the `_HID` match, the P2SB BAR decode and `_UID`.

use super::super::hid_match::hid_is_gpio_controller;
use super::super::sideband::{community_pids, community_window, sbreg_from_bar0};
use super::super::uid::parse_uid;

#[test]
fn sideband_ports_follow_the_linux_community_order() {
    assert_eq!(community_pids(b"INT344B\0"), &[0xAF, 0xAE, 0xAC]);
    assert_eq!(community_pids(b"INT34BB\0"), &[0x6E, 0x6D, 0x6A]);
    assert_eq!(community_pids(b"INTC1055"), &[0x6E, 0x6D, 0x6A, 0x69]);
    assert_eq!(community_pids(b"INTC1056"), &[0x6E, 0x6D, 0x6B, 0x6A, 0x69]);
    assert!(community_pids(b"INT344BX").is_empty());
    assert!(community_pids(b"INT3452\0").is_empty(), "Broxton windows are static");
    // Vendor firmware writes SBRG + 0x006E0000 for community 0.
    assert_eq!(community_window(0xFD00_0000, 0x6E), (0xFD6E_0000, 0x1_0000));
}

#[test]
fn controllers_matched_by_hid() {
    for hid in [b"AMDI0030", b"AMD0030\0", b"INT3452\0", b"INT34C5\0", b"INTC1057"] {
        assert!(hid_is_gpio_controller(hid), "{:?}", hid);
    }
    assert!(!hid_is_gpio_controller(b"AMDI0010"), "the AMD I2C controller");
    assert!(!hid_is_gpio_controller(b"PNP0C0A\0"));
}

#[test]
fn p2sb_bar0_decode() {
    // Memory BAR, 64-bit type (bits 2:1 = 10b), as P2SB reports it.
    assert_eq!(sbreg_from_bar0(0xFD00_0004, 0), Some(0xFD00_0000));
    assert_eq!(sbreg_from_bar0(0xE000_000C, 0x3F), Some(0x3F_E000_0000));
    assert_eq!(sbreg_from_bar0(0xFD00_0000, 0x3F), Some(0xFD00_0000), "32-bit BAR");
    assert_eq!(sbreg_from_bar0(u32::MAX, u32::MAX), None, "still hidden");
    assert_eq!(sbreg_from_bar0(0x4, 0), None, "never placed");
    assert_eq!(sbreg_from_bar0(0xF001, 0), None, "an I/O BAR");
}

#[test]
fn uid_encodings() {
    // NameOp "_UID" then a ByteConst, One, and the string "3" some firmware uses.
    assert_eq!(parse_uid(&[0x08, b'_', b'U', b'I', b'D', 0x0A, 0x02]), Some(2));
    assert_eq!(parse_uid(&[0x08, b'_', b'U', b'I', b'D', 0x01]), Some(1));
    assert_eq!(parse_uid(&[0x08, b'_', b'U', b'I', b'D', 0x0D, b'3', 0]), Some(3));
    assert_eq!(parse_uid(&[0x08, b'_', b'H', b'I', b'D', 0x01]), None);
}
