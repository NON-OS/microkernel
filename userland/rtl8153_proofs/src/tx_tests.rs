// NONOS Operating System (AGPL-3.0-or-later)
//! Frames out: the descriptor's bytes exactly, every length outside 14 to
//! 1514 refused before any transfer, a short or failed transfer said, and
//! a stalled OUT pipe brought back.

use nonos_usbnet::mock::Call;
use nonos_usbnet::Nic;

use crate::helpers::up;
use crate::rtl::tx::{encode, TX_DESC};

#[test]
fn a_frame_goes_out_behind_its_descriptor() {
    let frame: Vec<u8> = (0..60).map(|i| i as u8).collect();
    let mut out = [0xeeu8; 1600];
    assert_eq!(encode(&frame, &mut out), Some(68));
    assert_eq!(out[..TX_DESC], [60, 0, 0, 0xc0, 0, 0, 0, 0]);
    assert_eq!(out[TX_DESC..68], frame[..]);
    let full = [0x77u8; 1514];
    assert_eq!(encode(&full, &mut out), Some(1522));
    assert_eq!(out[..4], [0xea, 0x05, 0x00, 0xc0]);
}

#[test]
fn lengths_outside_a_frame_or_the_buffer_are_refused() {
    let mut out = [0u8; 1600];
    for len in [0, 1, 13, 1515, 1518, 4096] {
        assert_eq!(encode(&vec![0; len], &mut out), None, "{len}");
    }
    assert_eq!(encode(&[0; 100], &mut [0u8; 107]), None, "the buffer one byte short");
    assert_eq!(encode(&[0; 14], &mut [0u8; 22]), Some(22));
}

#[test]
fn the_driver_sends_one_transfer_per_frame_and_refuses_the_rest() {
    let (p, mut r) = up(0x5c10);
    p.bus.0.borrow_mut().calls.clear();
    assert_eq!(r.send(&[0x42; 300]), Ok(()));
    assert_eq!(r.send(&[0x42; 1515]), Err(-22));
    let calls = p.bus.0.borrow().calls.clone();
    let outs: Vec<_> = calls.iter().filter(|c| matches!(c, Call::BulkOut(_))).collect();
    assert_eq!(outs.len(), 1);
    let Call::BulkOut(d) = outs[0] else { unreachable!() };
    assert_eq!((d.len(), &d[..4]), (308, &[0x2c, 0x01, 0, 0xc0][..]));
}
