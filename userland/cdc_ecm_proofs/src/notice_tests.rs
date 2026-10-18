// NONOS Operating System (AGPL-3.0-or-later)
//! Notifications held to their interface and their own length: the link
//! taken from NETWORK_CONNECTION, a speed change of eight bytes let be,
//! and anything short, oversized, misdirected or malformed counted.

use nonos_usbnet::Nic;

use crate::device::qemu;
use crate::ecm::notice::{notice, Notice};
use crate::ecm::watch::armed;

fn n(kind: u8, value: u16, index: u16, data: &[u8]) -> Vec<u8> {
    let mut v = vec![0xA1, kind];
    for f in [value, index, data.len() as u16] {
        v.extend(f.to_le_bytes());
    }
    v.extend(data);
    v
}

#[test]
fn each_notification_is_held_to_its_kind() {
    assert_eq!(notice(&n(0, 1, 0, &[]), 0), Some(Notice::Link(true)));
    assert_eq!(notice(&n(0, 0, 0, &[]), 0), Some(Notice::Link(false)));
    assert_eq!(notice(&n(0x2A, 0, 0, &[0; 8]), 0), Some(Notice::Speed));
    assert_eq!(notice(&n(0x01, 0, 0, &[]), 0), Some(Notice::Other));
    assert_eq!(notice(&n(0, 1, 2, &[]), 0), None, "another interface");
    assert_eq!(notice(&n(0, 2, 0, &[]), 0), None, "connection 2");
    assert_eq!(notice(&n(0, 1, 0, &[0; 4]), 0), None, "connection with data");
    assert_eq!(notice(&n(0x2A, 0, 0, &[0; 4]), 0), None, "speed of four bytes");
    assert_eq!(notice(&n(0x2A, 0, 0, &[0; 8])[..12], 0), None, "short of its wLength");
    assert_eq!(notice(&[n(0, 1, 0, &[]), vec![0; 2]].concat(), 0), None, "oversized");
    assert_eq!(notice(&[0xA1, 0, 1, 0], 0), None, "short of a header");
    let mut wrong = n(0, 1, 0, &[]);
    wrong[0] = 0x21;
    assert_eq!(notice(&wrong, 0), None);
}

#[test]
fn the_link_follows_its_notifications_and_bad_ones_are_counted() {
    let (bus, mut nic) = qemu();
    let looks = |nic: &mut crate::ecm::link::Ecm<_>| (0..8).for_each(|_| nic.tick().unwrap());
    bus.0.borrow_mut().notices.push_back(n(0, 0, 0, &[]));
    looks(&mut nic);
    assert!(!nic.link_up());
    bus.0.borrow_mut().notices.push_back(n(0, 1, 1, &[]));
    looks(&mut nic);
    assert!(!nic.link_up(), "for the data interface");
    assert_eq!(nic.counts.notices, 1);
    bus.0.borrow_mut().notices.push_back(n(0, 1, 0, &[]));
    looks(&mut nic);
    assert!(nic.link_up());
    assert_eq!((armed(8), armed(16), armed(64), armed(0)), (8, 16, 16, 1));
}
