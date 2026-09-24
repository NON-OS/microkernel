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

/*
 * The numbers and the header layout come from the shared crate. They were written
 * out here and again in the player's client, and the two copies had already
 * diverged in what they covered.
 */
pub use nonos_audio_proto::{write_header, HDR_LEN, MAGIC, STATUS_LEN, VERSION};
pub use nonos_audio_proto::{E_AGAIN, E_INVAL, E_OK};
pub use nonos_audio_proto::{OP_CLOSE, OP_FEED_PCM, OP_PAUSE, OP_PLAY_PCM, OP_PLAY_TONE};
pub use nonos_audio_proto::{OP_RESUME, OP_STOP, OP_STREAM_OPEN};

pub struct Request {
    pub op: u16,
    pub request_id: u32,
    pub payload_len: u32,
}

pub fn decode(msg: &[u8]) -> Option<Request> {
    if msg.len() < HDR_LEN {
        return None;
    }
    let magic = u32::from_le_bytes([msg[0], msg[1], msg[2], msg[3]]);
    let version = u16::from_le_bytes([msg[4], msg[5]]);
    if magic != MAGIC || version != VERSION {
        return None;
    }
    Some(Request {
        op: u16::from_le_bytes([msg[6], msg[7]]),
        request_id: u32::from_le_bytes([msg[12], msg[13], msg[14], msg[15]]),
        payload_len: u32::from_le_bytes([msg[16], msg[17], msg[18], msg[19]]),
    })
}

pub fn encode_reply(req: &Request, status: i32, out: &mut [u8]) -> usize {
    if out.len() < HDR_LEN + STATUS_LEN {
        return 0;
    }
    write_header(out, req.op, req.request_id, STATUS_LEN as u32);
    out[HDR_LEN..HDR_LEN + STATUS_LEN].copy_from_slice(&status.to_le_bytes());
    HDR_LEN + STATUS_LEN
}

pub fn encode_open_reply(req: &Request, status: i32, stream_id: u32, out: &mut [u8]) -> usize {
    if out.len() < 28 {
        return 0;
    }
    write_header(
        out,
        req.op,
        req.request_id,
        STATUS_LEN as u32 + core::mem::size_of::<u32>() as u32,
    );
    out[HDR_LEN..HDR_LEN + STATUS_LEN].copy_from_slice(&status.to_le_bytes());
    out[24..28].copy_from_slice(&stream_id.to_le_bytes());
    28
}
