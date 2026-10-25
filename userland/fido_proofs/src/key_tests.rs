// NONOS Operating System (AGPL-3.0-or-later)
//! A HID interface is a candidate key only with both 64-byte interrupt
//! endpoints; its report descriptor decides the rest at bind.

use crate::ctaphid::{find_keys, free_keys};
use crate::keyside::{config, KEY};

#[test]
fn a_key_has_its_in_and_out_endpoints() {
    let keys = find_keys(&config(&KEY));
    assert_eq!(keys.len(), 1);
    let k = keys[0];
    assert_eq!((k.interface, k.input.address, k.output.address), (0, 0x81, 0x01));
    assert_eq!((k.input.max_packet, k.output.max_packet, k.input.interval), (64, 64, 5));
}

#[test]
fn a_keyboard_or_a_half_key_is_no_candidate() {
    assert!(find_keys(&config(&[(0x81, 8)])).is_empty(), "a boot keyboard");
    assert!(find_keys(&config(&[(0x81, 64)])).is_empty(), "IN alone");
    assert!(find_keys(&config(&[(0x81, 64), (0x02, 32)])).is_empty(), "a short OUT");
    let mut c = config(&KEY);
    c[9 + 5] = 0x0B;
    assert!(find_keys(&c).is_empty(), "not HID");
}

#[test]
fn a_shared_device_offers_only_the_interfaces_no_driver_holds() {
    // A composite key: a boot keyboard (interface 0) and the FIDO one (1).
    let mut c = config(&KEY);
    let kbd = [9, 0x04, 0, 0, 1, 0x03, 0x01, 0x01, 0, 7, 0x05, 0x82, 0x03, 8, 0, 10];
    c.extend(kbd);
    c[2] = c.len() as u8;
    c[9 + 2] = 1; // the key's interface is number 1
    assert_eq!(free_keys(&c, 0b01).iter().map(|k| k.interface).collect::<Vec<_>>(), [1]);
    assert!(free_keys(&c, 0b10).is_empty(), "the key's interface is held");
    assert_eq!(find_keys(&c).len(), 1, "the keyboard is no candidate");
}
