// NONOS Operating System (AGPL-3.0-or-later)
//! The OP_IDENTIFY reply body, at the offsets its layout names, so a client
//! written from the layout reads what the driver writes.

use crate::protocol::{encode_identify, IDENTIFY_PAYLOAD_LEN, OP_IDENTIFY};

#[test]
fn the_body_sits_where_the_layout_says() {
    assert_eq!(OP_IDENTIFY, 8);
    assert_eq!(IDENTIFY_PAYLOAD_LEN, 76);
    let mut out = [0xaau8; 100];
    encode_identify(&mut out, 1_953_525_168, 512, b"ST1000LM035-1RK172", b"WL1A2B3C", 0);
    assert_eq!(u64::from_le_bytes(out[0..8].try_into().unwrap()), 1_953_525_168);
    assert_eq!(u32::from_le_bytes(out[8..12].try_into().unwrap()), 512);
    assert_eq!((out[12], out[13], out[14], out[15]), (18, 8, 0, 0));
    assert_eq!(&out[16..34], b"ST1000LM035-1RK172");
    assert!(out[34..56].iter().all(|&b| b == 0), "model padded with zeros");
    assert_eq!(&out[56..64], b"WL1A2B3C");
    assert!(out[64..76].iter().all(|&b| b == 0), "serial padded with zeros");
    assert!(out[76..].iter().all(|&b| b == 0xaa), "nothing past the body written");
}

#[test]
fn an_overlong_name_is_cut_to_its_field() {
    let mut out = [0u8; IDENTIFY_PAYLOAD_LEN];
    encode_identify(&mut out, 1, 512, &[b'm'; 64], &[b's'; 64], 0);
    assert_eq!((out[12], out[13]), (40, 20));
    assert!(out[16..56].iter().all(|&b| b == b'm'));
    assert!(out[56..76].iter().all(|&b| b == b's'));
}

#[test]
fn the_medium_byte_says_what_kind_of_part_is_served() {
    let mut out = [0u8; IDENTIFY_PAYLOAD_LEN];
    encode_identify(&mut out, 1, 512, b"m", b"s", crate::protocol::MEDIUM_SATA);
    assert_eq!(out[14], 0);
    encode_identify(&mut out, 1, 512, b"m", b"s", 1);
    assert_eq!(out[14], 1, "an eMMC part served through this capsule");
    assert_eq!(out[15], 0);
}
