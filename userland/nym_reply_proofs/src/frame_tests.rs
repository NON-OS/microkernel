// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Proofs that a reply pushed in a packet larger than the 2 KiB regular one
//! is read off the gateway link whole, that a frame too long for a caller's
//! buffer is stepped over without losing the frames behind it, and that the
//! errors the link can report are not mistaken for one another.

use crate::ws_frame::parse::next;
use crate::ws_frame::read::{E_BAD_FRAME, E_NEED_MORE, FRAME_MAX};
use crate::ws_frame::types::FrameKind;

/// A binary frame as a gateway sends it: final, unmasked.
fn binary(payload: &[u8]) -> Vec<u8> {
    let mut out = vec![0x82];
    if payload.len() < 126 {
        out.push(payload.len() as u8);
    } else {
        out.push(126);
        out.extend_from_slice(&(payload.len() as u16).to_be_bytes());
    }
    out.extend_from_slice(payload);
    out
}

/// What a gateway pushes for one reply packet: kind, flag, nonce, then the
/// sealed reply with its tag.
fn pushed(reply_bytes: usize) -> Vec<u8> {
    (0..2 + 12 + reply_bytes + 16).map(|i| (i * 37) as u8).collect()
}

/// A reply in a 32 KiB packet is pushed as a frame of about 32.5 KiB. Read
/// into a buffer sized for the regular packet, it failed with a code equal
/// to the timeout's, so it was dropped as though the link were quiet, along
/// with every byte held behind it.
#[test]
fn a_reply_in_the_largest_packet_is_read_whole() {
    for packet in [2 * 1024, 8 * 1024, 16 * 1024, 32 * 1024] {
        let blob = pushed(packet - 405 + 32);
        let wire = binary(&blob);
        let mut out = vec![0u8; FRAME_MAX];
        let mut ctrl = [0u8; 125];
        let frame = next(&wire, &mut out, &mut ctrl).expect("parses").expect("complete");
        assert!(frame.kind == FrameKind::Binary, "packet {packet}");
        assert_eq!(frame.len, blob.len());
        assert_eq!(frame.consumed, wire.len());
        assert_eq!(&out[..frame.len], &blob[..]);
    }
}

#[test]
fn any_frame_the_length_field_can_name_fits_the_buffer() {
    let wire = binary(&vec![5u8; u16::MAX as usize]);
    let mut out = vec![0u8; FRAME_MAX];
    let mut ctrl = [0u8; 125];
    let frame = next(&wire, &mut out, &mut ctrl).expect("parses").expect("complete");
    assert!(frame.kind == FrameKind::Binary);
    assert_eq!(frame.len, u16::MAX as usize);
}

/// A frame too long for the buffer is named as such with its full length,
/// so the reader steps over it and the next frame is read in its place.
#[test]
fn a_frame_too_long_for_the_buffer_is_stepped_over() {
    let mut wire = binary(&vec![1u8; 3000]);
    let first = wire.len();
    wire.extend(binary(b"the next reply"));
    let mut out = vec![0u8; 2413];
    let mut ctrl = [0u8; 125];
    let frame = next(&wire, &mut out, &mut ctrl).expect("parses").expect("complete");
    assert!(frame.kind == FrameKind::Oversized);
    assert_eq!(frame.consumed, first);
    let frame = next(&wire[first..], &mut out, &mut ctrl).expect("parses").expect("complete");
    assert!(frame.kind == FrameKind::Binary);
    assert_eq!(&out[..frame.len], b"the next reply");
}

#[test]
fn a_frame_still_arriving_asks_for_more() {
    let wire = binary(&vec![2u8; 4000]);
    let mut out = vec![0u8; FRAME_MAX];
    let mut ctrl = [0u8; 125];
    for cut in [0, 1, 3, 4, 100, wire.len() - 1] {
        assert!(next(&wire[..cut], &mut out, &mut ctrl).expect("not an error").is_none());
    }
}

/// The link's errors used to share values with a timeout (8) and a close
/// (9), and with a net.tcp call that never completed (8).
#[test]
fn the_link_errors_are_told_apart() {
    for code in [E_BAD_FRAME, E_NEED_MORE] {
        assert!(code != 8 && code != 9, "{code}");
    }
    assert_ne!(E_BAD_FRAME, E_NEED_MORE);
}

#[test]
fn arbitrary_bytes_never_panic() {
    let mut state = 0xdead_beefu64;
    let mut out = vec![0u8; 512];
    let mut ctrl = [0u8; 125];
    for _ in 0..20_000 {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let len = (state >> 57) as usize;
        let bytes: Vec<u8> = (0..len).map(|i| (state.rotate_left(i as u32) >> 3) as u8).collect();
        if let Ok(Some(frame)) = next(&bytes, &mut out, &mut ctrl) {
            assert!(frame.consumed <= bytes.len());
        }
    }
}
