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

//! The master volume: one level and one mute switch over everything the
//! service plays, streams and tones alike.
//!
//! `OP_SET_VOLUME` carries the level, 0 to 100, in one byte, the mute switch,
//! 0 or 1, in the next, and two reserved bytes, zero on the wire. A level over
//! 100 or a mute byte that is neither is refused, not clamped: a client that
//! sends one has a bug, and a clamp would play it louder than it meant. The
//! reply is the header, the status word, then the level and mute switch in
//! force after the request, laid out as the request lays them, so the caller
//! shows what is playing rather than what it asked for.
//!
//! The same op with an empty payload changes nothing and replies with the
//! volume in force. A client that steps the volume (the desktop shell's volume
//! keys) reads it first, so one started after the level was turned down steps
//! from that level and not from full.

use super::header::{write_header, HDR_LEN, STATUS_LEN};
use super::ops::{E_INVAL, E_OK};

pub const OP_SET_VOLUME: u16 = 10;
/// The loudest level, which plays every sample as it was mixed.
pub const VOLUME_MAX: u8 = 100;
/// Level, mute switch, two reserved bytes.
pub const VOLUME_PAYLOAD_LEN: usize = 4;
pub const VOLUME_MSG_LEN: usize = HDR_LEN + VOLUME_PAYLOAD_LEN;
pub const VOLUME_REPLY_LEN: usize = HDR_LEN + STATUS_LEN + VOLUME_PAYLOAD_LEN;

/// What the master volume is set to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MasterVolume {
    /// 0 to `VOLUME_MAX`.
    pub level: u8,
    pub muted: bool,
}

impl MasterVolume {
    /// Where the service starts: full and not muted, so a client that never
    /// sets it hears what it heard before there was a master volume.
    pub const FULL: Self = Self { level: VOLUME_MAX, muted: false };
}

/// A whole request, header included. Zero when `out` is too short or `volume`
/// names a level over `VOLUME_MAX`, which the service would refuse.
pub fn volume_request(out: &mut [u8], request_id: u32, volume: MasterVolume) -> usize {
    if out.len() < VOLUME_MSG_LEN || volume.level > VOLUME_MAX {
        return 0;
    }
    write_header(out, OP_SET_VOLUME, request_id, VOLUME_PAYLOAD_LEN as u32);
    write_body(&mut out[HDR_LEN..VOLUME_MSG_LEN], volume);
    VOLUME_MSG_LEN
}

/// A request that changes nothing and asks for the volume in force: the
/// header alone. Zero when `out` is too short.
pub fn volume_query(out: &mut [u8], request_id: u32) -> usize {
    if out.len() < HDR_LEN {
        return 0;
    }
    write_header(out, OP_SET_VOLUME, request_id, 0);
    HDR_LEN
}

/// The volume a request's payload asks for, or None for a payload too short
/// to hold one, a level over `VOLUME_MAX`, or a mute byte other than 0 or 1.
pub fn read_volume_request(payload: &[u8]) -> Option<MasterVolume> {
    let body: &[u8; VOLUME_PAYLOAD_LEN] = payload.first_chunk()?;
    read_body(body)
}

/// The reply, as the service sends it: `status`, then the volume in force.
/// Zero when `out` is too short.
pub fn volume_reply(out: &mut [u8], request_id: u32, status: i32, volume: MasterVolume) -> usize {
    if out.len() < VOLUME_REPLY_LEN {
        return 0;
    }
    write_header(out, OP_SET_VOLUME, request_id, (STATUS_LEN + VOLUME_PAYLOAD_LEN) as u32);
    out[HDR_LEN..HDR_LEN + STATUS_LEN].copy_from_slice(&status.to_le_bytes());
    write_body(&mut out[HDR_LEN + STATUS_LEN..VOLUME_REPLY_LEN], volume);
    VOLUME_REPLY_LEN
}

/// The volume in force from a reply, or the status it was refused with.
/// A reply too short to read, or one naming a volume no service sets, is
/// `E_INVAL`.
pub fn read_volume_reply(resp: &[u8]) -> Result<MasterVolume, i32> {
    let Some(reply) = resp.first_chunk::<VOLUME_REPLY_LEN>() else { return Err(E_INVAL) };
    let status = i32::from_le_bytes([reply[20], reply[21], reply[22], reply[23]]);
    if status != E_OK {
        return Err(status);
    }
    let body = [reply[24], reply[25], reply[26], reply[27]];
    read_body(&body).ok_or(E_INVAL)
}

fn write_body(out: &mut [u8], volume: MasterVolume) {
    out[0] = volume.level;
    out[1] = u8::from(volume.muted);
    out[2..4].copy_from_slice(&0u16.to_le_bytes());
}

fn read_body(body: &[u8; VOLUME_PAYLOAD_LEN]) -> Option<MasterVolume> {
    let muted = match body[1] {
        0 => false,
        1 => true,
        _ => return None,
    };
    (body[0] <= VOLUME_MAX).then_some(MasterVolume { level: body[0], muted })
}
