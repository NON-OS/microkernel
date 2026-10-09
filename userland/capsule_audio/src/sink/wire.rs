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

pub const HDR_LEN: usize = 20;
pub const STATUS_LEN: usize = 4;
pub const MAX_PCM_CHUNK: usize = 4096;

const MAGIC: u32 = 0x4e48_4441;
const VERSION: u16 = 1;
const OP_WRITE_PCM: u16 = 7;
const OP_STREAM_START: u16 = 8;
const OP_STREAM_STOP: u16 = 9;
/// driver.hda0's `OP_OUTPUT_STATUS`: its verdict on this machine's audio.
const OP_OUTPUT_STATUS: u16 = 10;
/// The driver's status reply: header, status, then verdict u32, codec
/// vendor u16, codec device u16, outputs u8, plugged u8, reserved u16.
pub const OUTPUT_REPLY_LEN: usize = HDR_LEN + STATUS_LEN + 12;
const E_OK: i32 = 0;

pub fn request(request_id: u32, pcm: &[u8], out: &mut [u8]) -> usize {
    let payload_len = pcm.len();
    if payload_len == 0 || payload_len > MAX_PCM_CHUNK || out.len() < HDR_LEN + payload_len {
        return 0;
    }
    out[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    out[4..6].copy_from_slice(&VERSION.to_le_bytes());
    out[6..8].copy_from_slice(&OP_WRITE_PCM.to_le_bytes());
    out[8..10].copy_from_slice(&0u16.to_le_bytes());
    out[10..12].copy_from_slice(&0u16.to_le_bytes());
    out[12..16].copy_from_slice(&request_id.to_le_bytes());
    out[16..20].copy_from_slice(&(payload_len as u32).to_le_bytes());
    out[HDR_LEN..HDR_LEN + payload_len].copy_from_slice(pcm);
    HDR_LEN + payload_len
}

fn ctl_request(op: u16, request_id: u32, out: &mut [u8]) -> usize {
    if out.len() < HDR_LEN {
        return 0;
    }
    out[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    out[4..6].copy_from_slice(&VERSION.to_le_bytes());
    out[6..8].copy_from_slice(&op.to_le_bytes());
    out[8..10].copy_from_slice(&0u16.to_le_bytes());
    out[10..12].copy_from_slice(&0u16.to_le_bytes());
    out[12..16].copy_from_slice(&request_id.to_le_bytes());
    out[16..20].copy_from_slice(&0u32.to_le_bytes());
    HDR_LEN
}

pub fn start_request(request_id: u32, out: &mut [u8]) -> usize {
    ctl_request(OP_STREAM_START, request_id, out)
}

pub fn stop_request(request_id: u32, out: &mut [u8]) -> usize {
    ctl_request(OP_STREAM_STOP, request_id, out)
}

pub fn output_status_request(request_id: u32, out: &mut [u8]) -> usize {
    ctl_request(OP_OUTPUT_STATUS, request_id, out)
}

/// The driver's verdict and the output flags `nonos_audio_proto::output`
/// gives applications, from its status reply. The verdict numbers are the
/// same on both wires. The outputs byte keeps its bits (speaker, headphone,
/// line out) and plugged becomes bit 8. None for a short or refused reply,
/// or a verdict outside the ones the driver gives.
pub fn read_output_status(rx: &[u8]) -> Option<(u32, u32)> {
    if rx.len() < OUTPUT_REPLY_LEN || reply_status(rx) != E_OK {
        return None;
    }
    let b = &rx[HDR_LEN + STATUS_LEN..];
    let verdict = u32::from_le_bytes([b[0], b[1], b[2], b[3]]);
    if verdict == 1 || verdict > 6 {
        return None;
    }
    let flags = (b[8] & 0x07) as u32 | if b[9] != 0 { 1 << 8 } else { 0 };
    Some((verdict, flags))
}

pub fn reply_ok(rx: &[u8]) -> bool {
    if rx.len() < HDR_LEN + STATUS_LEN {
        return false;
    }
    let mut status = [0u8; STATUS_LEN];
    status.copy_from_slice(&rx[HDR_LEN..HDR_LEN + STATUS_LEN]);
    i32::from_le_bytes(status) == E_OK
}

pub fn reply_status(rx: &[u8]) -> i32 {
    if rx.len() < HDR_LEN + STATUS_LEN {
        return i32::MIN;
    }
    let mut status = [0u8; STATUS_LEN];
    status.copy_from_slice(&rx[HDR_LEN..HDR_LEN + STATUS_LEN]);
    i32::from_le_bytes(status)
}
