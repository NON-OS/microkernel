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
//! What the audio service said when the Sound page asked whether this
//! The Nym cover profile as net.nym reports it, and the most it lets a
//! client get through: what the Network page shows next to the default
//! route, so the person sees the trade the cover traffic makes.

/// Bytes of a net.nym header, and of its OP_TIMING_STATUS answer.
pub const HDR_LEN: usize = 20;
pub const STATUS_LEN: usize = 20;
const MAGIC: u32 = 0x4E59_4D31;
pub const OP_TIMING_STATUS: u16 = 16;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NymCover {
    /// Not asked yet: the Network page has not been opened.
    Unasked,
    /// net.nym is not running, or did not answer as it should.
    NoService,
    /// Profile 0 is the low rate, 1 Nym's own; the ceilings in KiB a second.
    Rates { profile: u8, out_kib: u16, in_kib: u16 },
}

/// The request: a header with no body.
pub fn request() -> [u8; HDR_LEN] {
    let mut r = [0u8; HDR_LEN];
    r[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    r[4..6].copy_from_slice(&1u16.to_le_bytes());
    r[6..8].copy_from_slice(&OP_TIMING_STATUS.to_le_bytes());
    r[12..16].copy_from_slice(&1u32.to_le_bytes());
    r
}

/// Read the answer; anything short, foreign or refused is NoService.
pub fn read(reply: &[u8]) -> NymCover {
    let u16_at = |i: usize| reply.get(i..i + 2).map(|b| u16::from_le_bytes([b[0], b[1]]));
    let u32_at = |i: usize| reply.get(i..i + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]));
    let ok = u32_at(0) == Some(MAGIC)
        && u16_at(6) == Some(OP_TIMING_STATUS)
        && u16_at(8) == Some(0)
        && u32_at(16).is_some_and(|n| n as usize >= STATUS_LEN)
        && reply.len() >= HDR_LEN + STATUS_LEN;
    if !ok {
        return NymCover::NoService;
    }
    let p = &reply[HDR_LEN..];
    NymCover::Rates {
        profile: p[4],
        out_kib: u16::from_le_bytes([p[16], p[17]]),
        in_kib: u16::from_le_bytes([p[18], p[19]]),
    }
}

/// The words before the number, and the line under the row's label.
pub fn said(c: NymCover) -> (&'static str, &'static str) {
    match c {
        NymCover::Unasked => ("--", NOTE),
        NymCover::NoService => ("Nym not running", NOTE),
        NymCover::Rates { profile: 1, .. } => ("Nym rate, about ", NOTE),
        NymCover::Rates { .. } => ("Low rate, about ", NOTE),
    }
}

const NOTE: &str = "Cover traffic hides when Nym is in use; the rate caps what it carries";
