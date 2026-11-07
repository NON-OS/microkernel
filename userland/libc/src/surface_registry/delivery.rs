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

//! The frame the input router delivers an event in, as abi/wire.toml
//! [input.delivery_v1] pins it: an 8-byte header, the 32-byte event, then the
//! window the event was routed to. A frame of 40 bytes is the same frame
//! from before the window was carried; the flags say whether it is there.

use super::types::InputEvent;

/// "NINP" read little-endian.
pub const DELIVERY_MAGIC: u32 = 0x4E49_4E50;
pub const DELIVERY_VERSION: u16 = 1;
pub const DELIVERY_HDR_LEN: usize = 8;
/// The frame without the window, as older routers sent it.
pub const DELIVERY_LEN_V1: usize = DELIVERY_HDR_LEN + 32;
/// The frame the router sends: the window id at 40, a reserved word at 44.
pub const DELIVERY_LEN: usize = DELIVERY_LEN_V1 + 8;
pub const DELIVERY_WINDOW_OFFSET: usize = DELIVERY_LEN_V1;
/// Header flag: the window id at DELIVERY_WINDOW_OFFSET names the window the
/// pointer event was routed to, and its x and y are local to that window.
pub const DELIVERY_FLAG_WINDOW: u16 = 1;

/// An event as a client receives it.
#[derive(Clone, Copy, Default)]
pub struct Delivered {
    pub event: InputEvent,
    /// The window of the receiver's the event is for; None for a key, an
    /// event in screen pixels (the desktop shell's), or a frame without it.
    pub window_id: Option<u32>,
}

fn u16_at(b: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(at..at + 2)?.try_into().ok()?))
}

fn u32_at(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(at..at + 4)?.try_into().ok()?))
}

fn i32_at(b: &[u8], at: usize) -> Option<i32> {
    Some(i32::from_le_bytes(b.get(at..at + 4)?.try_into().ok()?))
}

/// The window a delivery frame names, when its flags say it carries one.
pub fn delivery_window(frame: &[u8]) -> Option<u32> {
    let flags = u16_at(frame, 6)?;
    if flags & DELIVERY_FLAG_WINDOW == 0 {
        return None;
    }
    u32_at(frame, DELIVERY_WINDOW_OFFSET)
}

/// Read a delivery frame: None for a frame that is not one (short, another
/// magic, another version).
pub fn decode_delivery(frame: &[u8]) -> Option<Delivered> {
    if frame.len() < DELIVERY_LEN_V1 || u32_at(frame, 0)? != DELIVERY_MAGIC {
        return None;
    }
    if u16_at(frame, 4)? != DELIVERY_VERSION {
        return None;
    }
    let e = &frame[DELIVERY_HDR_LEN..];
    let event = InputEvent {
        kind: u16_at(e, 0)?,
        flags: u16_at(e, 2)?,
        code: u32_at(e, 4)?,
        x: i32_at(e, 8)?,
        y: i32_at(e, 12)?,
        delta_x: i32_at(e, 16)?,
        delta_y: i32_at(e, 20)?,
        timestamp_ns: u64::from_le_bytes(e.get(24..32)?.try_into().ok()?),
    };
    Some(Delivered { event, window_id: delivery_window(frame) })
}
