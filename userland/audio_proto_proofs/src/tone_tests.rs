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

//! The tone request, against the offsets the service reads it at.

use nonos_audio_proto::{tone_request, OP_PLAY_TONE, TONE_MSG_LEN, TONE_PAYLOAD_LEN};

#[test]
fn the_payload_lands_where_the_service_reads_it() {
    let mut out = [0u8; TONE_MSG_LEN];
    assert_eq!(tone_request(&mut out, 9, 880, 90, 0x2000), TONE_MSG_LEN);
    assert_eq!(u16::from_le_bytes([out[6], out[7]]), OP_PLAY_TONE);
    assert_eq!(u32::from_le_bytes([out[16], out[17], out[18], out[19]]), TONE_PAYLOAD_LEN as u32);
    assert_eq!(u32::from_le_bytes([out[20], out[21], out[22], out[23]]), 880);
    assert_eq!(u32::from_le_bytes([out[24], out[25], out[26], out[27]]), 90);
    assert_eq!(u16::from_le_bytes([out[28], out[29]]), 0x2000);
}

#[test]
fn the_declared_length_matches_the_body() {
    let mut out = [0u8; TONE_MSG_LEN];
    let n = tone_request(&mut out, 1, 440, 20, 0x1000);
    let declared = u32::from_le_bytes([out[16], out[17], out[18], out[19]]) as usize;
    assert_eq!(n, TONE_MSG_LEN);
    assert_eq!(declared, n - 20, "the body is everything after the header");
    assert_eq!(declared, TONE_PAYLOAD_LEN);
}

#[test]
fn the_padding_is_zeroed() {
    let mut out = [0xFFu8; TONE_MSG_LEN];
    tone_request(&mut out, 1, 440, 20, 0x1000);
    assert_eq!(u16::from_le_bytes([out[30], out[31]]), 0);
}

#[test]
fn a_short_buffer_builds_nothing() {
    for len in 0..TONE_MSG_LEN {
        let mut out = vec![0u8; len];
        assert_eq!(tone_request(&mut out, 1, 440, 20, 0x1000), 0, "{len} bytes");
        assert!(out.iter().all(|&b| b == 0), "{len} bytes was partly written");
    }
}
