// NONOS Operating System (AGPL-3.0-or-later)
use crate::constants::dma::{RX_BUF_BYTES, RX_BUF_DATA_BYTES};
use crate::constants::MAX_ETHERNET_FRAME;
use crate::protocol::{decode_request, encode_response_header, Request, HDR_LEN};
use crate::ring::{copy, u16_at, u8_at};

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

// The device fills a 32 KiB byte ring with per-packet records: a status
// word, a raw length, then the frame. RCR.WRAP is set, so a record that
// crosses the end of the ring is written on past it into the slack the
// allocation carries (RX_BUF_BYTES), not back at the start, and the driver
// reads linearly. The primitives below are the only way the walk touches
// memory, so their property is the isolation property: no read leaves the
// allocation for any offset, the u16 assembly is little-endian across the
// seam, and the copy fills exactly the caller's buffer and nothing beyond it.

fn pattern(i: usize) -> u8 {
    (i % 251) as u8
}

fn ring_buf() -> Box<[u8; RX_BUF_BYTES]> {
    let mut b = Box::new([0u8; RX_BUF_BYTES]);
    for (i, byte) in b.iter_mut().enumerate() {
        *byte = pattern(i);
    }
    b
}

fn expected(off: usize) -> u8 {
    if off < RX_BUF_BYTES {
        pattern(off)
    } else {
        0
    }
}

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

#[test]
fn no_read_leaves_the_allocation() {
    let buf = ring_buf();
    let base = buf.as_ptr() as u64;
    let edges = [
        0usize,
        1,
        RX_BUF_DATA_BYTES - 1,
        RX_BUF_DATA_BYTES,
        RX_BUF_DATA_BYTES + 1,
        RX_BUF_BYTES - 1,
        RX_BUF_BYTES,
        RX_BUF_BYTES + 1,
        7 * RX_BUF_DATA_BYTES + 13,
        usize::MAX / 2,
        usize::MAX - 1,
    ];
    for &off in &edges {
        assert_eq!(u8_at(base, off), expected(off), "offset {off}");
    }
    let mut s = 1u64;
    for _ in 0..200_000 {
        let off = xorshift(&mut s) as usize % (2 * RX_BUF_BYTES);
        assert_eq!(u8_at(base, off), expected(off), "offset {off}");
    }
}

#[test]
fn u16_reads_assemble_little_endian_and_run_on_past_the_seam() {
    let buf = ring_buf();
    let base = buf.as_ptr() as u64;
    assert_eq!(u16_at(base, 0), u16::from_le_bytes([pattern(0), pattern(1)]));
    // With WRAP set the byte after the ring's last one is in the slack, where
    // the device wrote it, not at the start of the ring.
    let seam = RX_BUF_DATA_BYTES - 1;
    assert_eq!(u16_at(base, seam), u16::from_le_bytes([pattern(seam), pattern(seam + 1)]));
    // The last byte of the allocation pairs with nothing past it.
    let last = RX_BUF_BYTES - 1;
    assert_eq!(u16_at(base, last), u16::from_le_bytes([pattern(last), 0]));
}

#[test]
fn a_frame_crossing_the_seam_is_read_from_the_slack() {
    let buf = ring_buf();
    let base = buf.as_ptr() as u64;
    // A frame that starts near the end of the ring and runs past it.
    let start = RX_BUF_DATA_BYTES - 5;
    let mut out = [0u8; 64];
    let n = out.len();
    copy(base, start, &mut out, n);
    for (i, b) in out.iter().enumerate() {
        assert_eq!(*b, pattern(start + i), "byte {i}");
    }
}

#[test]
fn an_oversized_length_never_writes_past_the_caller_buffer() {
    let buf = ring_buf();
    let base = buf.as_ptr() as u64;
    // A hostile raw length far beyond the output buffer: the copy must fill
    // exactly the buffer and stop, with the guard bytes untouched.
    let mut guarded = [0xEEu8; 96];
    let out_len = 32;
    copy(base, 7, &mut guarded[..out_len], usize::MAX / 2);
    for (i, b) in guarded.iter().enumerate().take(out_len) {
        assert_eq!(*b, pattern(7 + i), "byte {i}");
    }
    for b in guarded.iter().skip(out_len) {
        assert_eq!(*b, 0xEE, "the copy must never write past the caller's slice");
    }
}

#[test]
#[allow(clippy::assertions_on_constants)] // guarding constant relations is the point
fn the_frame_gate_constants_fit_the_ring_and_the_reply() {
    // read_frame rejects raw_len <= 4 and frame lengths above the Ethernet
    // maximum or the caller's buffer, so a copy is at most a full frame and
    // always shorter than the ring.
    assert!(MAX_ETHERNET_FRAME + 4 < RX_BUF_DATA_BYTES);
}

// The wire header.

#[test]
fn decode_never_panics_and_reads_fields_from_their_offsets() {
    const MAGIC: u32 = 0x4E4E_4554; // the NNET wire tag from protocol/header.rs
    for seed in 1..100_000u64 {
        let mut s = seed;
        let blen = (xorshift(&mut s) % 40) as usize;
        let mut buf: Vec<u8> = (0..blen).map(|_| (xorshift(&mut s) & 0xff) as u8).collect();
        if buf.len() >= 6 && seed % 2 == 0 {
            buf[0..4].copy_from_slice(&MAGIC.to_le_bytes());
            buf[4..6].copy_from_slice(&1u16.to_le_bytes());
        }
        if let Some(req) = decode_request(&buf) {
            assert!(buf.len() >= HDR_LEN);
            assert_eq!(req.op, u16::from_le_bytes([buf[6], buf[7]]));
            assert_eq!(req.flags, u16::from_le_bytes([buf[8], buf[9]]));
            assert_eq!(req.request_id, u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]));
            assert_eq!(req.payload_len, u32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]));
        }
    }
}

#[test]
fn encoded_response_headers_decode_back_to_the_request_fields() {
    for seed in 1..20_000u64 {
        let mut s = seed;
        let request = Request {
            op: (xorshift(&mut s) & 0xffff) as u16,
            flags: (xorshift(&mut s) & 0xffff) as u16,
            request_id: (xorshift(&mut s) & 0xffff_ffff) as u32,
            payload_len: 0,
        };
        let payload_len = (xorshift(&mut s) & 0xffff_ffff) as u32;
        let mut out = [0u8; HDR_LEN];
        encode_response_header(&mut out, &request, payload_len);
        let back = decode_request(&out).expect("a response header carries the wire tag");
        assert_eq!(back.op, request.op);
        assert_eq!(back.flags, request.flags);
        assert_eq!(back.request_id, request.request_id);
        assert_eq!(back.payload_len, payload_len);
    }
}
