// NONOS Operating System (AGPL-3.0-or-later)
//! What is let go is counted: runts, broken descriptors, frames too long.

use nonos_usbnet::Nic;

use crate::helpers::up;
use crate::rx_tests::packet;

#[test]
fn runts_and_broken_descriptors_are_counted() {
    let (p, mut r) = up(0x5c00);
    let mut broken = packet(64, 0, 9);
    broken[..4].copy_from_slice(&3000u32.to_le_bytes());
    for t in [vec![1u8; 10], broken] {
        p.bus.0.borrow_mut().bulk_in.push_back(Ok(Some(t)));
        assert_eq!(r.recv(&mut [0u8; 1514]), Ok(None));
    }
    assert_eq!((r.counts.runts, r.counts.broken), (1, 1));
}

#[test]
fn a_frame_over_1514_bytes_is_dropped_whatever_room_the_caller_gives() {
    let (p, mut r) = up(0x5c30);
    p.bus.0.borrow_mut().bulk_in.push_back(Ok(Some(packet(1522, 0, 9))));
    assert_eq!(r.recv(&mut [0u8; 4096]), Ok(None));
    assert_eq!(r.counts.too_long, 1);
}
