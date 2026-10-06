// NONOS Operating System (AGPL-3.0-or-later)
//! Frames out as ax88179_tx_fixup frames them, the padding byte and its
//! header flag, and a stalled pipe brought back.

use nonos_usbnet::mock::Call;
use nonos_usbnet::{Nic, Setup};

use crate::ax::tx::{tx_transfer, TX_MAX};
use crate::bound::bound;
use crate::chip::Chip;
use crate::layout::frame;

fn out(len: usize, mps: u16) -> Vec<u8> {
    let mut buf = vec![0xee; TX_MAX];
    let n = tx_transfer(&frame(len, 0x5a), mps, &mut buf);
    buf.truncate(n);
    buf
}

#[test]
fn the_header_is_the_length_then_a_zero_mss_word() {
    let x = out(60, 1024);
    assert_eq!(x.len(), 68);
    assert_eq!(x[..8], [60, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(x[8..], frame(60, 0x5a)[..]);
    assert_eq!(out(1514, 1024)[..4], [0xea, 0x05, 0, 0]);
}

#[test]
fn a_transfer_filling_its_last_packet_gets_one_byte_and_the_padding_flags() {
    // Linux: (len + 8) % maxpacket == 0 sets 0x80008000 and usbnet adds
    // one byte.
    for (len, mps) in [(1016, 1024), (504, 512), (1016, 512), (56, 64)] {
        let x = out(len, mps);
        assert_eq!(x.len(), len + 9, "{len} on {mps}");
        assert_eq!(x[4..8], [0x00, 0x80, 0x00, 0x80]);
        assert_eq!(x[len + 8], 0);
    }
    // A multiple of 512 on a 1024-byte pipe is padded as on a 512 one.
    assert_eq!(out(504, 1024).len(), 513);
    for (len, mps) in [(568, 1024), (60, 512), (1514, 1024), (1000, 1024)] {
        let x = out(len, mps);
        assert_eq!((x.len(), &x[4..8]), (len + 8, &[0u8; 4][..]), "{len} on {mps}");
    }
}

#[test]
fn a_frame_goes_out_in_one_bulk_transfer_and_a_stall_is_cleared() {
    let (bus, mut nic) = bound(&Chip::default());
    nic.send(&frame(100, 3)).unwrap();
    let last = bus.0.borrow().calls.last().cloned();
    assert_eq!(last, Some(Call::BulkOut(out_with(100, 3))));
    bus.0.borrow_mut().bulk_out_fails = Some(-32);
    assert_eq!(nic.send(&frame(100, 3)), Err(-32));
    let calls = bus.0.borrow().calls.clone();
    let n = calls.len();
    assert_eq!(calls[n - 2], Call::ResetBulk(false));
    assert_eq!(calls[n - 1], Call::Out(Setup::new(0x02, 0x01, 0, 0x03), vec![]));
    assert_eq!(nic.send(&frame(1515, 0)), Err(-22), "longer than the stack sends");
}

fn out_with(len: usize, fill: u8) -> Vec<u8> {
    let mut v = (len as u32).to_le_bytes().to_vec();
    v.extend([0u8; 4]);
    v.extend(frame(len, fill));
    v
}
