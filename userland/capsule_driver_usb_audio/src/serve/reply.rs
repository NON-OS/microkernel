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

//! The two replies this driver gives: a status word, or the output status.

use super::play::Device;
use crate::wire::{encode_response_header, write_output_status, write_status, OutputStatus};
use crate::wire::{Request, OUTPUT_NO_DEVICE, OUTPUT_READY, OUTPUT_STATUS_PAYLOAD_LEN};
use crate::wire::{E_OK, OUT_HEADPHONE, OUT_LINE, OUT_SPEAKER, RESP_HDR_LEN, STATUS_LEN};

pub const REPLY_MAX: usize = RESP_HDR_LEN + STATUS_LEN + OUTPUT_STATUS_PAYLOAD_LEN;

pub fn status(tx: &mut [u8], req: &Request, code: i32) -> usize {
    encode_response_header(tx, req, STATUS_LEN as u32);
    write_status(&mut tx[RESP_HDR_LEN..], code);
    RESP_HDR_LEN + STATUS_LEN
}

/// Ready with the device's ids where the hda driver puts its codec's, or
/// no device. Worn outputs count as plugged: a USB headset is in its jack
/// for as long as it is bound.
pub fn output_status(tx: &mut [u8], req: &Request, device: Option<&Device>) -> usize {
    let s = match device {
        None => OutputStatus::silent(OUTPUT_NO_DEVICE),
        Some(d) => OutputStatus {
            verdict: OUTPUT_READY,
            codec_vendor: d.ids.0,
            codec_device: d.ids.1,
            outputs: if d.outputs.speaker { OUT_SPEAKER } else { 0 }
                | if d.outputs.headphone { OUT_HEADPHONE } else { 0 }
                | if d.outputs.line { OUT_LINE } else { 0 },
            plugged: d.outputs.headphone,
        },
    };
    encode_response_header(tx, req, (STATUS_LEN + OUTPUT_STATUS_PAYLOAD_LEN) as u32);
    write_status(&mut tx[RESP_HDR_LEN..], E_OK);
    write_output_status(&mut tx[RESP_HDR_LEN + STATUS_LEN..], &s);
    REPLY_MAX
}
