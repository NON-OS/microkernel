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
pub use nonos_audio_proto::{
    output_status_reply, E_NODEV, OP_OUTPUT_STATUS, OUTPUT_NOT_ANSWERING, OUTPUT_NO_DEVICE,
    OUTPUT_READY, OUTPUT_REPLY_LEN,
};
pub use nonos_audio_proto::{volume_reply, MasterVolume, OP_SET_VOLUME, VOLUME_REPLY_LEN};

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

/// The request a frame `decode` refused is answered under: the op and request
/// id it names, or zeros when it is too short to name them.
pub fn refused(msg: &[u8]) -> Request {
    let Some(h) = msg.first_chunk::<HDR_LEN>() else {
        return Request { op: 0, request_id: 0, payload_len: 0 };
    };
    Request {
        op: u16::from_le_bytes([h[6], h[7]]),
        request_id: u32::from_le_bytes([h[12], h[13], h[14], h[15]]),
        payload_len: 0,
    }
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

/// The reply to `OP_OUTPUT_STATUS`: what this machine's audio is.
pub fn encode_output_reply(req: &Request, code: u32, flags: u32, out: &mut [u8]) -> usize {
    output_status_reply(out, req.request_id, code, flags)
}

/// The reply to `OP_SET_VOLUME`: `status`, then the volume in force.
pub fn encode_volume_reply(
    req: &Request,
    status: i32,
    volume: MasterVolume,
    out: &mut [u8],
) -> usize {
    volume_reply(out, req.request_id, status, volume)
}
