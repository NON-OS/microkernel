// NONOS Operating System (AGPL-3.0-or-later)
//! Frames out: the 8-byte header (length, then the padding mark in bits 31
//! and 15), a padding byte exactly when the transfer would fill its last
//! packet, and any length outside a frame refused before the bus.

use nonos_usbnet::mock::Call;
use nonos_usbnet::Nic;

use crate::ax::tx_header::{framed, header};
use crate::harness::bound;

#[test]
fn the_header_is_the_length_then_the_padding_mark() {
    assert_eq!(header(60, false), [60, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(header(1514, true), [0xea, 0x05, 0, 0, 0, 0x80, 0, 0x80]);
}

#[test]
fn a_transfer_never_ends_on_a_packet_boundary() {
    for mps in [64u16, 512, 1024] {
        for len in 14..=1514 {
            let (n, padded) = framed(len, mps);
            assert_eq!(padded, (len + 8) % 64 == 0, "{len} at {mps}");
            assert_eq!(n, len + 8 + padded as usize);
            assert!(n % 64 != 0 && n % mps as usize != 0, "{len} at {mps} ends on a packet");
        }
    }
    assert_eq!(framed(992, 1000), (1001, true), "an odd packet size is padded too");
}

#[test]
fn a_frame_goes_out_behind_its_header() {
    let (bus, mut ax, _) = bound();
    for len in [14usize, 60, 1008, 1016, 1514] {
        let f: Vec<u8> = (0..len).map(|i| i as u8 ^ 0x5A).collect();
        assert_eq!(ax.send(&f), Ok(()));
        let Some(Call::BulkOut(sent)) = bus.0.borrow().calls.last().cloned() else { panic!() };
        let (n, padded) = framed(len, 1024);
        assert_eq!(sent.len(), n);
        assert_eq!((sent[..8] == header(len, padded), &sent[8..8 + len]), (true, &f[..]));
        assert!(sent[8 + len..].iter().all(|&b| b == 0));
    }
}

#[test]
fn a_length_outside_a_frame_is_refused_before_the_bus() {
    let (bus, mut ax, _) = bound();
    for len in [0usize, 13, 1515, 4096] {
        assert_eq!(ax.send(&vec![0; len]), Err(-22), "{len}");
    }
    assert!(bus.0.borrow().calls.is_empty());
}

#[test]
fn a_stalled_send_is_brought_back() {
    let (bus, mut ax, _) = bound();
    bus.0.borrow_mut().bulk_out_fails = Some(-32);
    assert_eq!(ax.send(&[0; 60]), Err(-32));
    let calls = bus.0.borrow().calls.clone();
    assert!(calls.contains(&Call::ResetBulk(false)), "the host side reset");
    let clear = |c: &Call| matches!(c, Call::Out(s, _) if s.request == 1 && s.index == 0x03);
    assert!(calls.iter().any(clear), "the device's halt cleared");
}
