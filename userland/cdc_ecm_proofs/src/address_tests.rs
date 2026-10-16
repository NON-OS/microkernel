// NONOS Operating System (AGPL-3.0-or-later)
//! The station address and segment size: index 0, a string of the wrong
//! length or not hexadecimal, a group or zero address, and a segment size
//! under a frame are refused and no address is made up; a large segment
//! size is taken.

use nonos_usbnet::qemu_usb_net::MAC_STRING;
use nonos_usbnet::Nic;

use crate::device::bound;

fn mac(digits: &str) -> Vec<u8> {
    let mut s = vec![(2 + 2 * digits.len()) as u8, 0x03];
    digits.bytes().for_each(|b| s.extend([b, 0]));
    s
}

fn with_mac(digits: &str) -> String {
    bound(|_| {}, &mac(digits)).err().expect("bound")
}

#[test]
fn the_address_is_exactly_twelve_hex_digits_naming_a_station() {
    assert_eq!(bound(|c| c[31] = 0, &MAC_STRING).err().unwrap(), "no MAC address string -5");
    assert_eq!(with_mac("40540012345"), "MAC string not twelve digits -5");
    assert_eq!(with_mac("4054001234567"), "MAC string not twelve digits -5");
    assert_eq!(with_mac("40540012345g"), "MAC address refused -5");
    assert_eq!(with_mac("415400123456"), "MAC address refused -5", "group");
    assert_eq!(with_mac("000000000000"), "MAC address refused -5");
    let mut long_header = MAC_STRING.to_vec();
    long_header[0] = 0x1c;
    assert_eq!(bound(|_| {}, &long_header).err().unwrap(), "MAC string not twelve digits -5");
    let (_, nic) = bound(|_| {}, &mac("02aBcDeF0102")).unwrap();
    assert_eq!(nic.mac(), [0x02, 0xab, 0xcd, 0xef, 0x01, 0x02]);
}

#[test]
fn a_segment_size_under_one_frame_is_refused_and_a_large_one_taken() {
    let size = |n: u16| move |c: &mut Vec<u8>| c[36..38].copy_from_slice(&n.to_le_bytes());
    for n in [0, 1500, 1513] {
        let e = bound(size(n), &MAC_STRING).err();
        assert_eq!(e.as_deref(), Some("segment size under one frame -5"), "{n}");
    }
    assert!(bound(size(1514), &MAC_STRING).is_ok());
    assert!(bound(size(9014), &MAC_STRING).is_ok());
}
