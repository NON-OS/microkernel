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
//! The reply that tells the audio server why there is no sound.

use crate::protocol::{
    write_output_status, OutputStatus, E_NODEV, OP_OUTPUT_STATUS, OUTPUT_STATUS_PAYLOAD_LEN,
    OUT_HEADPHONE, OUT_SPEAKER,
};

#[test]
fn the_status_body_lays_out_at_the_offsets_the_audio_server_reads() {
    let s = OutputStatus {
        verdict: 0,
        codec_vendor: 0x10ec,
        codec_device: 0x0236,
        outputs: OUT_SPEAKER | OUT_HEADPHONE,
        plugged: true,
    };
    let mut b = [0xaau8; OUTPUT_STATUS_PAYLOAD_LEN];
    write_output_status(&mut b, &s);
    assert_eq!(b, [0, 0, 0, 0, 0xec, 0x10, 0x36, 0x02, 0b11, 1, 0, 0]);
}

#[test]
fn an_unsupported_machine_carries_only_its_verdict() {
    let mut b = [0xaau8; OUTPUT_STATUS_PAYLOAD_LEN];
    write_output_status(&mut b, &OutputStatus::silent(2));
    assert_eq!(b, [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn the_op_and_errno_numbers_are_the_ones_the_audio_server_uses() {
    assert_eq!(OP_OUTPUT_STATUS, 10);
    assert_eq!(E_NODEV, -19);
}
