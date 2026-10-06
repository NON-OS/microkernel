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

//! `OP_SET_VOLUME` on the wire, both ways: what a client builds is what the
//! service reads, what the service answers is what the client reads, and a
//! payload that names no volume the service sets is refused, never clamped.

use nonos_audio_proto::{
    read_volume_reply, read_volume_request, volume_query, volume_reply, volume_request,
    MasterVolume, E_INVAL, E_NODEV, E_OK, HDR_LEN, OP_OUTPUT_STATUS, OP_SET_VOLUME, VOLUME_MAX,
    VOLUME_MSG_LEN, VOLUME_PAYLOAD_LEN, VOLUME_REPLY_LEN,
};

use crate::server::proto::{decode, encode_volume_reply};

fn v(level: u8, muted: bool) -> MasterVolume {
    MasterVolume { level, muted }
}

fn every_volume() -> impl Iterator<Item = MasterVolume> {
    (0..=VOLUME_MAX).flat_map(|level| [v(level, false), v(level, true)])
}

#[test]
fn the_op_takes_the_next_number_after_the_output_status() {
    assert_eq!(OP_OUTPUT_STATUS, 9);
    assert_eq!(OP_SET_VOLUME, 10);
    assert_eq!((VOLUME_PAYLOAD_LEN, VOLUME_MSG_LEN, VOLUME_REPLY_LEN), (4, 24, 28));
}

#[test]
fn the_payload_lands_where_the_service_reads_it() {
    let mut out = [0xFFu8; VOLUME_MSG_LEN];
    assert_eq!(volume_request(&mut out, 7, v(45, true)), VOLUME_MSG_LEN);
    let req = decode(&out).expect("a header the service takes");
    assert_eq!((req.op, req.request_id, req.payload_len), (OP_SET_VOLUME, 7, 4));
    assert_eq!(&out[HDR_LEN..], [45, 1, 0, 0], "level, mute, two zeroed reserved bytes");
}

#[test]
fn every_volume_round_trips_through_the_request() {
    for want in every_volume() {
        let mut out = [0u8; VOLUME_MSG_LEN];
        assert_eq!(volume_request(&mut out, 1, want), VOLUME_MSG_LEN);
        assert_eq!(read_volume_request(&out[HDR_LEN..]), Some(want));
    }
}

#[test]
fn every_volume_and_status_round_trips_through_the_reply() {
    for want in every_volume() {
        let mut out = [0u8; VOLUME_REPLY_LEN];
        let req = decode(&request(want)).unwrap();
        assert_eq!(encode_volume_reply(&req, E_OK, want, &mut out), VOLUME_REPLY_LEN);
        let declared = u32::from_le_bytes([out[16], out[17], out[18], out[19]]) as usize;
        assert_eq!(declared, VOLUME_REPLY_LEN - HDR_LEN, "the body is everything after the header");
        assert_eq!(u16::from_le_bytes([out[6], out[7]]), OP_SET_VOLUME);
        assert_eq!(read_volume_reply(&out), Ok(want));
    }
    for status in [E_INVAL, E_NODEV] {
        let mut out = [0u8; VOLUME_REPLY_LEN];
        assert_eq!(volume_reply(&mut out, 3, status, MasterVolume::FULL), VOLUME_REPLY_LEN);
        assert_eq!(read_volume_reply(&out), Err(status));
    }
}

fn request(want: MasterVolume) -> [u8; VOLUME_MSG_LEN] {
    let mut out = [0u8; VOLUME_MSG_LEN];
    volume_request(&mut out, 1, want);
    out
}

#[test]
fn a_level_over_full_is_refused_not_clamped() {
    for level in VOLUME_MAX + 1..=u8::MAX {
        for muted in [0, 1] {
            assert_eq!(read_volume_request(&[level, muted, 0, 0]), None, "level {level}");
        }
        let mut out = [0u8; VOLUME_MSG_LEN];
        assert_eq!(volume_request(&mut out, 1, v(level, false)), 0, "a client builds none");
        assert!(out.iter().all(|&b| b == 0), "level {level} was partly written");
    }
}

#[test]
fn a_mute_byte_other_than_zero_or_one_is_refused() {
    for muted in 2..=u8::MAX {
        assert_eq!(read_volume_request(&[50, muted, 0, 0]), None, "mute byte {muted}");
    }
}

#[test]
fn a_payload_too_short_to_hold_a_volume_is_refused() {
    for len in 0..VOLUME_PAYLOAD_LEN {
        assert_eq!(read_volume_request(&[50, 0, 0, 0][..len]), None, "{len} bytes");
    }
}

#[test]
fn a_reply_naming_no_volume_a_service_sets_is_refused() {
    let mut out = [0u8; VOLUME_REPLY_LEN];
    volume_reply(&mut out, 1, E_OK, v(50, false));
    out[24] = 101;
    assert_eq!(read_volume_reply(&out), Err(E_INVAL));
    out[24] = 50;
    out[25] = 2;
    assert_eq!(read_volume_reply(&out), Err(E_INVAL));
    for len in 0..VOLUME_REPLY_LEN {
        assert_eq!(read_volume_reply(&out[..len]), Err(E_INVAL), "{len} bytes");
    }
    // What a service from before the op answers it with: the plain status
    // reply, E_INVAL, which a client reads as refused.
    let mut old = [0u8; HDR_LEN + 4];
    old[HDR_LEN..].copy_from_slice(&E_INVAL.to_le_bytes());
    assert_eq!(read_volume_reply(&old), Err(E_INVAL));
}

#[test]
fn the_query_is_the_header_alone_and_names_the_op() {
    let mut out = [0xFFu8; HDR_LEN];
    assert_eq!(volume_query(&mut out, 4), HDR_LEN);
    let req = decode(&out).unwrap();
    assert_eq!((req.op, req.request_id, req.payload_len), (OP_SET_VOLUME, 4, 0));
    assert_eq!(volume_query(&mut [0u8; HDR_LEN - 1], 4), 0);
}

#[test]
fn a_short_buffer_builds_nothing() {
    for len in 0..VOLUME_MSG_LEN {
        let mut out = vec![0u8; len];
        assert_eq!(volume_request(&mut out, 1, v(50, false)), 0, "{len} bytes");
        assert!(out.iter().all(|&b| b == 0), "{len} bytes was partly written");
    }
    for len in 0..VOLUME_REPLY_LEN {
        let mut out = vec![0u8; len];
        assert_eq!(volume_reply(&mut out, 1, E_OK, v(50, false)), 0, "{len} bytes");
        assert!(out.iter().all(|&b| b == 0), "{len} bytes was partly written");
    }
}

#[test]
fn the_service_starts_full_and_not_muted() {
    assert_eq!(MasterVolume::FULL, v(100, false));
}
