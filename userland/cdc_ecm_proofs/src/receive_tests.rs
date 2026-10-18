// NONOS Operating System (AGPL-3.0-or-later)
//! Received transfers: a runt dropped, one longer than a frame and its
//! padding byte dropped and counted, a transfer that fills the buffer let go to its end (a short or
//! zero-length transfer), and a frame too long for the stack's room
//! counted; frames out of 14 to 1514 bytes are refused.

use nonos_usbnet::Nic;

use crate::device::qemu;
use crate::ecm::link::RX_LEN;

#[test]
fn a_transfer_that_fills_the_buffer_is_let_go_to_its_end() {
    let (bus, mut nic) = qemu();
    let full = || Ok(Some(vec![5u8; RX_LEN]));
    let q = [full(), full(), Ok(Some(vec![6; 100])), Ok(Some(vec![7; 60]))];
    bus.0.borrow_mut().bulk_in.extend(q);
    let mut out = [0u8; 1514];
    for _ in 0..3 {
        assert_eq!(nic.recv(&mut out), Ok(None));
    }
    assert_eq!(nic.recv(&mut out), Ok(Some(60)));
    assert_eq!(out[..60], [7; 60]);
    assert_eq!(nic.counts.overruns, 1);
    bus.0.borrow_mut().bulk_in.extend([full(), Ok(Some(vec![])), Ok(Some(vec![8; 64]))]);
    assert_eq!((nic.recv(&mut out), nic.recv(&mut out)), (Ok(None), Ok(None)));
    assert_eq!(nic.recv(&mut out), Ok(Some(64)), "a zero-length transfer ended it");
}

#[test]
fn runts_long_frames_and_frames_past_the_room_are_counted() {
    let (bus, mut nic) = qemu();
    let q = [vec![1; 13], vec![2; 1600], vec![3; 200]];
    bus.0.borrow_mut().bulk_in.extend(q.map(|v| Ok(Some(v))));
    let mut out = [0u8; 1514];
    assert_eq!(nic.recv(&mut out), Ok(None));
    assert_eq!(nic.recv(&mut out), Ok(None), "dropped, not cut");
    assert_eq!(nic.recv(&mut out[..100]), Ok(None));
    let c = nic.counts;
    assert_eq!((c.runts, c.cut, c.too_long, c.overruns), (1, 1, 1, 0));
}

#[test]
fn a_frame_outside_ethernet_sizes_is_not_sent() {
    let (_, mut nic) = qemu();
    assert_eq!(nic.send(&[0; 13]), Err(-22));
    assert_eq!(nic.send(&[0; 1515]), Err(-22));
    assert_eq!(nic.send(&[0; 1514]), Ok(()));
}
