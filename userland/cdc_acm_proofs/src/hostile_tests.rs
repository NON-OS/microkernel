// NONOS Operating System (AGPL-3.0-or-later)
//! A modem's descriptors made hostile: endpoint sizes and intervals no
//! endpoint may have, functional descriptors cut short, and a line-state
//! notice about another interface.

use crate::acm::{find_acm, serial_state};
use crate::modem::*;

/// The modem with its bulk IN endpoint's wMaxPacketSize set to `mps`.
fn bulk_in(mps: u16) -> Vec<u8> {
    let mut parts: Vec<Vec<u8>> = vec![iface(0, 0, 1, COMM), UNION.to_vec()];
    parts.extend([iface(1, 0, 2, DATA), ep(0x81, 0x02, mps, 0), ep(0x02, 0x02, 64, 0)]);
    config(&parts.iter().map(|p| p.as_slice()).collect::<Vec<_>>())
}

#[test]
fn a_bulk_endpoint_of_no_size_usb_allows_is_no_pipe() {
    assert!(find_acm(&bulk_in(512)).is_some());
    for mps in [0, 1025, 1 << 11 | 512, 1 << 13 | 512] {
        assert!(find_acm(&bulk_in(mps)).is_none(), "{mps:#x}");
    }
}

#[test]
fn a_notification_endpoint_usb_does_not_allow_is_not_polled() {
    let notify = |mps: u16, interval: u8| {
        let f = find_acm(&modem(&[&HEADER, &UNION], &ep(0x83, 0x03, mps, interval)));
        f.expect("the line still binds").notify
    };
    assert!(notify(16, 10).is_some());
    assert_eq!(notify(0, 10), None);
    assert_eq!(notify(16, 0), None);
    assert_eq!(notify(1025, 10), None);
    assert_eq!(notify(3 << 11 | 16, 10), None, "3 added transactions is reserved");
    assert_eq!(notify(1 << 13 | 16, 10), None);
}

#[test]
fn functional_descriptors_cut_short_are_not_read() {
    let short_union = [4, 0x24, 0x06, 0];
    let f = find_acm(&modem(&[&HEADER, &short_union], &[]));
    assert!(f.is_none(), "a Union with no subordinate names no data interface");
    let short_acm = [3, 0x24, 0x02];
    let f = find_acm(&modem(&[&HEADER, &short_acm, &UNION], &[])).expect("bound");
    assert_eq!(f.capabilities, 0, "no capabilities read from past its end");
}

#[test]
fn a_line_state_about_another_interface_is_not_this_lines() {
    let mut n = vec![0xA1, 0x20, 0, 0, 1, 0, 2, 0, 0x03, 0];
    assert_eq!(serial_state(&n, 0), None);
    assert_eq!(serial_state(&n, 1), Some(0x03));
    n[5] = 1;
    assert_eq!(serial_state(&n, 1), None, "wIndex 0x0101");
}
