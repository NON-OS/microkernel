// NONOS Operating System (AGPL-3.0-or-later)
//! The capture gate: nothing opens or starts without MicrophoneCapture,
//! asked every time; one peer's stream, read and stopped by it alone; and
//! the NMIC framing.

use crate::frame::{decode, reply};
use crate::gate::{Session, MICROPHONE_CAPTURE};
use crate::wire::{E_BUSY, E_PERM, HDR_LEN, MAGIC, OP_READ};

#[test]
fn without_microphone_capture_nothing_opens_or_starts() {
    let mut s = Session::default();
    assert_eq!((s.open(false), s.start(7, false)), (Err(E_PERM), Err(E_PERM)));
    assert_eq!(s.streamer, None);
    assert_eq!(MICROPHONE_CAPTURE, 274_877_906_944, "bit 38");
}

#[test]
fn one_peers_stream_read_and_stopped_by_it_alone() {
    let mut s = Session::default();
    assert_eq!(s.start(7, true), Ok(()));
    assert_eq!(s.start(8, true), Err(E_BUSY));
    assert_eq!((s.own(7), s.own(8)), (Ok(()), Err(E_PERM)));
    assert_eq!(s.start(7, false), Err(E_PERM), "asked again at every start");
    s.stop();
    assert_eq!(s.own(7), Err(E_PERM));
}

#[test]
fn a_request_reads_and_a_reply_is_framed() {
    let mut msg = vec![0u8; HDR_LEN + 2];
    msg[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    msg[4..6].copy_from_slice(&1u16.to_le_bytes());
    msg[6..8].copy_from_slice(&OP_READ.to_le_bytes());
    msg[16..20].copy_from_slice(&2u32.to_le_bytes());
    let (req, body) = decode(&msg).expect("a request");
    assert_eq!((req.op, body.len()), (OP_READ, 2));
    let mut tx = [0u8; 32];
    assert_eq!(reply(&mut tx, &req, 0, &[9; 8]), HDR_LEN + 4 + 8);
    assert_eq!(&tx[0..4], b"CIMN", "NMIC little-endian");
}
