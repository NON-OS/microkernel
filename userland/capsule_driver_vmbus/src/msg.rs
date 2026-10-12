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

//! The channel messages this core sends, and the host's it reads. Every
//! host message is held to its length before a field is read.

use crate::constants::{
    INITIATE_CONTACT, MESSAGE_SINT, RELID_RELEASED, REQUESTOFFERS, UNLOAD, VERSION_V5,
};
use crate::guid::Guid;

fn put32(m: &mut [u8], at: usize, v: u32) {
    m[at..at + 4].copy_from_slice(&v.to_le_bytes());
}

fn put64(m: &mut [u8], at: usize, v: u64) {
    m[at..at + 8].copy_from_slice(&v.to_le_bytes());
}

fn u32_at(p: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(p.get(at..at + 4)?.try_into().ok()?))
}

fn u16_at(p: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(p.get(at..at + 2)?.try_into().ok()?))
}

/// The message's type, its first word.
pub fn kind(p: &[u8]) -> Option<u32> {
    u32_at(p, 0)
}

pub fn header(kind: u32) -> [u8; 8] {
    let mut m = [0u8; 8];
    put32(&mut m, 0, kind);
    m
}

pub fn request_offers() -> [u8; 8] {
    header(REQUESTOFFERS)
}

pub fn unload() -> [u8; 8] {
    header(UNLOAD)
}

pub fn relid_released(relid: u32) -> [u8; 12] {
    let mut m = [0u8; 12];
    put32(&mut m, 0, RELID_RELEASED);
    put32(&mut m, 8, relid);
    m
}

/// The contact, aimed at VP 0: from 5.0 the host sends on SINT 2 and needs
/// no interrupt page; before, it is named.
pub fn initiate_contact(version: u32, interrupt: u64, monitor1: u64, monitor2: u64) -> [u8; 40] {
    let mut m = [0u8; 40];
    put32(&mut m, 0, INITIATE_CONTACT);
    put32(&mut m, 8, version);
    if version >= VERSION_V5 {
        m[16] = MESSAGE_SINT;
    } else {
        put64(&mut m, 16, interrupt);
    }
    put64(&mut m, 24, monitor1);
    put64(&mut m, 32, monitor2);
    m
}

/// VERSION_RESPONSE: whether the version was taken, and the connection the
/// host names for messages from 5.0 (zero when it names none).
pub fn version_response(p: &[u8]) -> Option<(bool, u32)> {
    let supported = *p.get(8)? != 0;
    Some((supported, u32_at(p, 12).unwrap_or(0)))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Offer {
    pub relid: u32,
    pub interface: Guid,
    pub instance: Guid,
    pub sub_channel: u16,
    pub connection: u32,
    pub monitor: Option<u8>,
    pub dedicated_interrupt: bool,
}

/// OFFERCHANNEL, 196 bytes: the offer (its interface and instance, its
/// sub-channel index at 180), then the relid, the monitor, and the
/// connection. Each field is read only where the message reaches it.
pub fn offer(p: &[u8]) -> Option<Offer> {
    Some(Offer {
        relid: u32_at(p, 184)?,
        interface: Guid::from_bytes(p.get(8..24)?)?,
        instance: Guid::from_bytes(p.get(24..40)?)?,
        sub_channel: u16_at(p, 180)?,
        connection: u32_at(p, 192)?,
        monitor: (*p.get(189)? & 1 != 0).then_some(*p.get(188)?),
        dedicated_interrupt: u16_at(p, 190)? & 1 != 0,
    })
}

/// RESCIND_CHANNELOFFER: the relid withdrawn.
pub fn rescind(p: &[u8]) -> Option<u32> {
    u32_at(p, 8)
}
