// NONOS Operating System (AGPL-3.0-or-later)
//! The touchpad's line asked of driver.gpio0 (EVENTS, op 1, on F-57's
//! wire): the request names this driver's device, a reply's events are
//! read whole or not at all, and only the touchpad's own pin rings.

use crate::gpio0_wire::{events, rang, request, EVENTS_MAX, REPLY_MAX};

fn reply(status: i32, list: &[(u16, u16)]) -> Vec<u8> {
    let mut r = status.to_le_bytes().to_vec();
    r.push(list.len() as u8);
    for &(uid, pin) in list {
        r.extend(uid.to_le_bytes());
        r.extend(pin.to_le_bytes());
    }
    r
}

#[test]
fn the_request_is_the_op_then_the_device_key() {
    assert_eq!(request(0x00A3), [1, 0xA3, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(REPLY_MAX, 4 + 1 + EVENTS_MAX * 4);
}

#[test]
fn only_the_touchpads_own_pin_rings() {
    let r = reply(0, &[(0, 9), (0, 132), (1, 132)]);
    let list = events(&r).unwrap().unwrap();
    assert_eq!(list, [(0, 9), (0, 132), (1, 132)]);
    assert!(rang(&list, 0, 132));
    assert!(!rang(&list, 0, 133) && !rang(&list, 2, 132), "another pin or controller");
    assert!(!rang(&events(&reply(0, &[])).unwrap().unwrap(), 0, 132), "nothing came");
}

#[test]
fn a_refusal_or_a_short_reply_is_no_ring() {
    assert_eq!(events(&reply(-1, &[])), Some(Err(-1)), "not this device's holder");
    let mut cut = reply(0, &[(0, 132), (0, 133)]);
    cut.pop();
    assert_eq!(events(&cut), None, "a count past the bytes");
    assert_eq!(events(&[0, 0]), None);
}
