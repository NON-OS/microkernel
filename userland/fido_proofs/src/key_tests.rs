// NONOS Operating System (AGPL-3.0-or-later)
//! A HID interface is a candidate key only with both 64-byte interrupt
//! endpoints; its report descriptor decides the rest at bind.

use crate::ctaphid::find_keys;
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
