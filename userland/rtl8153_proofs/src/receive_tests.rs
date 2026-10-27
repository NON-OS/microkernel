// NONOS Operating System (AGPL-3.0-or-later)
//! Frames in through the bound driver: one a call from a gathered
//! transfer, an unended transfer let go with its rest, frames too long for
//! the stack dropped and counted, and a stalled pipe brought back.

use nonos_usbnet::Nic;

use crate::helpers::up;
use crate::rtl::rx::RX_BUFFER;
use crate::rx_tests::packet;

#[test]
fn a_gathered_transfer_comes_up_a_frame_a_call() {
    let (p, mut r) = up(0x5c20);
    let mut t = packet(64, 0, 1);
    t.resize(88, 0);
    t.extend(packet(1518, 0, 2));
    p.bus.0.borrow_mut().bulk_in.push_back(Ok(Some(t)));
    let mut out = [0u8; 1514];
    assert_eq!(r.recv(&mut out), Ok(Some(60)));
    assert_eq!(out[..60], [1; 60]);
    assert_eq!(r.recv(&mut out), Ok(Some(1514)));
    assert_eq!(out[..1514], [2; 1514]);
    assert_eq!(r.recv(&mut out), Ok(None));
}

#[test]
fn a_transfer_that_fills_the_buffer_is_let_go_with_its_rest() {
    let (p, mut r) = up(0x5c30);
    let full = packet(1518, 0, 3).into_iter().cycle().take(RX_BUFFER).collect::<Vec<_>>();
    let rest = packet(64, 0, 4);
    let after = packet(64, 0, 5);
    for t in [full, rest, after] {
        p.bus.0.borrow_mut().bulk_in.push_back(Ok(Some(t)));
    }
    let mut out = [0u8; 1514];
    assert_eq!(r.recv(&mut out), Ok(None), "the unended transfer");
    assert_eq!(r.recv(&mut out), Ok(None), "its rest");
    assert_eq!(r.recv(&mut out), Ok(Some(60)));
    assert_eq!(out[0], 5);
    assert_eq!(r.counts.overruns, 1);
}

#[test]
fn a_frame_longer_than_the_stack_takes_is_dropped_and_the_next_comes_up() {
    let (p, mut r) = up(0x6000);
    let mut t = packet(1522, 0, 6); // a tagged frame: 1518 bytes
    t.resize(1552, 0);
    t.extend(packet(100, 0, 7));
    p.bus.0.borrow_mut().bulk_in.push_back(Ok(Some(t.clone())));
    let mut out = [0u8; 1514];
    assert_eq!(r.recv(&mut out), Ok(Some(96)));
    assert_eq!(r.counts.too_long, 1);
    p.bus.0.borrow_mut().bulk_in.push_back(Ok(Some(packet(200, 0, 8))));
    let mut small = [0u8; 100];
    assert_eq!(r.recv(&mut small), Ok(None), "no room in the caller's buffer");
    assert_eq!(r.counts.too_long, 2);
}
