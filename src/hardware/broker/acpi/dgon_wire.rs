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

//! DGON: the one message the kernel sends a driver whose device has left,
//! into the inbox it reads requests on, from sender 0. 16 bytes,
//! little-endian: the tag "DGON" (u32), the version u16 (1), why u16 (1:
//! the device left), the broker device id u64. By the time it is read the
//! device is out of the table and every grant on it is gone; it only says
//! to stop and clean up. Pure, and the one definition: the kernel builds
//! with it, libc parses with it (mounted by path), the proofs hold both.

pub const DGON_TAG: u32 = u32::from_le_bytes(*b"DGON");
pub const DGON_VERSION: u16 = 1;
pub const DGON_LEN: usize = 16;
/// The device left: surprise removal, or its dock went.
pub const WHY_GONE: u16 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Dgon {
    pub why: u16,
    pub device_id: u64,
}

pub fn dgon_encode(d: Dgon) -> [u8; DGON_LEN] {
    let mut b = [0u8; DGON_LEN];
    b[..4].copy_from_slice(&DGON_TAG.to_le_bytes());
    b[4..6].copy_from_slice(&DGON_VERSION.to_le_bytes());
    b[6..8].copy_from_slice(&d.why.to_le_bytes());
    b[8..].copy_from_slice(&d.device_id.to_le_bytes());
    b
}

/// A DGON only from sender 0, of exactly its length, tag and version, with
/// a reason this version defines; anything else is not one.
pub fn dgon_parse(msg: &[u8], sender: u32) -> Option<Dgon> {
    let b: &[u8; DGON_LEN] = msg.try_into().ok()?;
    let tag = u32::from_le_bytes([b[0], b[1], b[2], b[3]]);
    let version = u16::from_le_bytes([b[4], b[5]]);
    let why = u16::from_le_bytes([b[6], b[7]]);
    let ok = sender == 0 && tag == DGON_TAG && version == DGON_VERSION && why == WHY_GONE;
    let mut id = [0u8; 8];
    id.copy_from_slice(&b[8..]);
    ok.then_some(Dgon { why, device_id: u64::from_le_bytes(id) })
}
