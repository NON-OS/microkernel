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

//! The window manager's lifecycle notification (capsule_wm
//! protocol/notify.rs): `NWMV`, version 1, then `event_kind, owner_pid,
//! window_id, x, y`, each a little-endian u32.

const MAGIC: u32 = 0x4E57_4D56;
const VERSION: u16 = 1;
pub const NOTICE_LEN: usize = 28;

const OPENED: u32 = 0;
const CLOSED: u32 = 1;
const FULL_SCREEN: u32 = 2;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WmEvent {
    Opened,
    Closed,
    /// The window covers the dock's band now (true) or no longer (false).
    FullScreen(bool),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct WmNotice {
    pub event: WmEvent,
    pub owner_pid: u32,
    pub window_id: u32,
}

/// What a message is: not a notification at all (`None`, so another
/// decoder may take it), or one, with the event when it is a kind and
/// version this shell knows (`Some(None)` for one it drops).
pub fn decode_wm_notice(buf: &[u8]) -> Option<Option<WmNotice>> {
    let word =
        |at: usize| buf.get(at..at + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]));
    if buf.len() != NOTICE_LEN || word(0) != Some(MAGIC) {
        return None;
    }
    if u16::from_le_bytes([buf[4], buf[5]]) != VERSION {
        return Some(None);
    }
    let (Some(kind), Some(owner_pid), Some(window_id), Some(x)) =
        (word(8), word(12), word(16), word(20))
    else {
        return Some(None);
    };
    let event = match kind {
        OPENED => WmEvent::Opened,
        CLOSED => WmEvent::Closed,
        FULL_SCREEN => WmEvent::FullScreen(x != 0),
        _ => return Some(None),
    };
    Some(Some(WmNotice { event, owner_pid, window_id }))
}
