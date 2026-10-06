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
//! Telling an application, in words, why this machine plays no sound.
//!
//! The HD Audio driver decides what the machine's audio is (it plays, it
//! needs Intel's SOF DSP firmware, it runs through AMD's ACP, it has only
//! HDMI, ...). The audio server passes the driver's number on unchanged and
//! the player and Settings turn it into a sentence. These pin the three
//! hops: the driver's reply as the server reads it, the server's reply as a
//! client reads it, and the sentence each number names.

use nonos_audio_proto::{
    is_output_message, output_message, output_short, output_status_reply, output_status_request,
    read_output_status, E_NODEV, FLAG_HEADPHONE, FLAG_PLUGGED, FLAG_SPEAKER, HDR_LEN,
    OP_OUTPUT_STATUS, OUTPUT_AMD_ACP, OUTPUT_HDMI_ONLY, OUTPUT_NEEDS_SOF, OUTPUT_NOT_ANSWERING,
    OUTPUT_NO_CODEC, OUTPUT_NO_DEVICE, OUTPUT_NO_PATH, OUTPUT_READY, OUTPUT_REPLY_LEN,
};

use crate::server::proto::{decode, encode_output_reply};
use crate::server::wire;

/// The driver's reply: header, status, verdict, vendor, device, outputs,
/// plugged, as `capsule_driver_hda/src/protocol/output.rs` lays it out.
fn driver_reply(status: i32, verdict: u32, outputs: u8, plugged: u8) -> Vec<u8> {
    let mut b = vec![0u8; wire::OUTPUT_REPLY_LEN];
    b[20..24].copy_from_slice(&status.to_le_bytes());
    b[24..28].copy_from_slice(&verdict.to_le_bytes());
    b[28..30].copy_from_slice(&0x10ecu16.to_le_bytes());
    b[30..32].copy_from_slice(&0x0236u16.to_le_bytes());
    b[32] = outputs;
    b[33] = plugged;
    b
}

#[test]
fn the_sof_sentence_is_the_one_the_user_reads() {
    assert_eq!(
        output_message(OUTPUT_NEEDS_SOF),
        "This laptop's audio needs Intel's DSP firmware (SOF), which NONOS does not support"
    );
    assert!(output_message(OUTPUT_AMD_ACP).contains("AMD's audio coprocessor (ACP)"));
}

#[test]
fn every_code_has_its_own_sentence_and_its_own_short_form() {
    let codes = [
        OUTPUT_READY,
        OUTPUT_NO_DEVICE,
        OUTPUT_NEEDS_SOF,
        OUTPUT_NO_CODEC,
        OUTPUT_HDMI_ONLY,
        OUTPUT_NO_PATH,
        OUTPUT_AMD_ACP,
        OUTPUT_NOT_ANSWERING,
    ];
    for (i, &c) in codes.iter().enumerate() {
        assert_eq!(c as usize, i);
        let m = output_message(c);
        assert!(is_output_message(m));
        assert!(!m.contains('\u{2014}'), "{m:?}");
        assert!(output_short(c).len() <= 40, "{:?} is too long for the value column", output_short(c));
        for &d in &codes[i + 1..] {
            assert_ne!(m, output_message(d));
            assert_ne!(output_short(c), output_short(d));
        }
    }
    assert!(!is_output_message("audio.server open rejected"));
    assert_eq!(output_message(99), "No audio output");
}

#[test]
fn the_server_reads_the_drivers_verdict_and_outputs() {
    assert_eq!(wire::read_output_status(&driver_reply(0, 0, 0b011, 1)), Some((0, FLAG_SPEAKER | FLAG_HEADPHONE | FLAG_PLUGGED)));
    assert_eq!(wire::read_output_status(&driver_reply(0, 2, 0, 0)), Some((OUTPUT_NEEDS_SOF, 0)));
    assert_eq!(wire::read_output_status(&driver_reply(0, 6, 0, 0)), Some((OUTPUT_AMD_ACP, 0)));
    assert_eq!(wire::read_output_status(&driver_reply(-19, 0, 0, 0)), None, "a refused reply");
    assert_eq!(wire::read_output_status(&driver_reply(0, 7, 0, 0)), None, "a code only the server gives");
    assert_eq!(wire::read_output_status(&driver_reply(0, 0, 0, 0)[..30]), None, "a short reply");
}

#[test]
fn the_server_asks_the_driver_with_its_op_number() {
    let mut b = [0u8; wire::HDR_LEN];
    assert_eq!(wire::output_status_request(5, &mut b), wire::HDR_LEN);
    assert_eq!(u16::from_le_bytes([b[6], b[7]]), 10, "driver.hda0's OP_OUTPUT_STATUS is 10");
    assert_eq!(&b[0..4], &0x4e48_4441u32.to_le_bytes());
}

#[test]
fn a_client_reads_back_what_the_server_sends() {
    let mut req = [0u8; HDR_LEN];
    assert_eq!(output_status_request(&mut req, 9), HDR_LEN);
    let r = decode(&req).expect("the request decodes");
    assert_eq!(r.op, OP_OUTPUT_STATUS);
    let mut tx = [0u8; OUTPUT_REPLY_LEN];
    assert_eq!(encode_output_reply(&r, OUTPUT_HDMI_ONLY, 0, &mut tx), OUTPUT_REPLY_LEN);
    assert_eq!(read_output_status(&tx), Some((OUTPUT_HDMI_ONLY, 0)));
    assert_eq!(u32::from_le_bytes([tx[12], tx[13], tx[14], tx[15]]), 9, "the request id");
    assert_eq!(output_status_reply(&mut [0u8; 8], 1, 0, 0), 0, "a short buffer is not written");
}

#[test]
fn the_refusal_a_client_asks_after_is_its_own_number() {
    assert_eq!(E_NODEV, -19);
    assert_ne!(E_NODEV, nonos_audio_proto::E_INVAL);
    assert_ne!(OP_OUTPUT_STATUS, nonos_audio_proto::OP_RESUME);
}
