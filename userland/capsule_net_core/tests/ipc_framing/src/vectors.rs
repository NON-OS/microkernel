//! The framing faults that let Sphinx packets vanish between two capsules.

use crate::protocol::errno::E_BAD_LEN;
use crate::server::parse_req::{parse, HDR_LEN, IPC_BUF_MAX, RECV_PAYLOAD_MAX};

/// Build a request frame carrying `payload`.
fn frame(op: u16, request_id: u32, payload: &[u8]) -> Vec<u8> {
    let mut buf = vec![0u8; HDR_LEN + payload.len()];
    buf[0..4].copy_from_slice(&0x4e544350u32.to_le_bytes());
    buf[4..6].copy_from_slice(&1u16.to_le_bytes());
    buf[6..8].copy_from_slice(&op.to_le_bytes());
    buf[12..16].copy_from_slice(&request_id.to_le_bytes());
    buf[16..20].copy_from_slice(&(payload.len() as u32).to_le_bytes());
    buf[HDR_LEN..].copy_from_slice(payload);
    buf
}

/// The regression itself. A request of the largest body carries the socket
/// handle as well, and the inbox has to hold both or it never parses.
#[test]
fn a_full_size_request_fits_the_inbox() {
    let payload = vec![0xa5u8; 4 + RECV_PAYLOAD_MAX];
    let req = frame(5, 7, &payload);
    assert!(req.len() <= HDR_LEN + IPC_BUF_MAX, "inbox too small for a full send");
    let (parsed, body) = parse(&req).expect("a full size request must parse");
    assert_eq!(parsed.request_id, 7);
    assert_eq!(body.len(), payload.len());
}

/// What a truncated read looks like from the far side: the header still
/// claims the full length, so the shortfall is visible rather than silent.
#[test]
fn a_truncated_request_is_refused_not_read_short() {
    let payload = vec![0u8; 4 + RECV_PAYLOAD_MAX];
    let req = frame(5, 9, &payload);
    let cut = &req[..1044];
    assert_eq!(parse(cut).err(), Some(E_BAD_LEN));
}

/// A refusal has to be addressable, which means reading the header back out
/// of bytes that failed to parse as a whole request.
#[test]
fn a_refusable_request_still_carries_its_reply_fields() {
    let req = frame(5, 0xdeadbeef, &vec![0u8; 64]);
    let cut = &req[..HDR_LEN];
    assert!(parse(cut).is_err());
    assert_eq!(u16::from_le_bytes([cut[6], cut[7]]), 5);
    assert_eq!(u32::from_le_bytes([cut[12], cut[13], cut[14], cut[15]]), 0xdeadbeef);
}
