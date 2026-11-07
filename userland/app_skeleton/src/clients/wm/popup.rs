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

//! OP_POPUP_OPEN: a popup over one of the caller's windows, placed by the
//! window manager against an anchor on that window and kept on the output
//! (xdg_positioner's rules); and the notice that the window manager closed
//! one.

use crate::wire::{call_payload, read_u32, HDR_LEN, NWMP_MAGIC};

const OP: u16 = 0x0010;
pub const POPUP_BODY_LEN: usize = 56;
const BODY_LEN: usize = POPUP_BODY_LEN;
const RESP_LEN: usize = 4 + 24;
const NOTIFY_MAGIC: u32 = 0x4E57_4D56;
const NOTIFY_LEN: usize = 28;
const POPUP_DONE: u32 = 5;
/// Close the popup on a press outside it and its own popups.
pub const POPUP_GRAB: u32 = 1;

/// What the popup is placed against, as xdg_positioner holds it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PopupSpec {
    /// The popup's own window id, new.
    pub window_id: u32,
    /// The caller's window it opens over; a popup may open over a popup.
    pub parent_id: u32,
    /// The anchor rectangle, relative to the parent window's origin.
    pub anchor_x: i32,
    pub anchor_y: i32,
    pub anchor_w: u32,
    pub anchor_h: u32,
    pub width: u32,
    pub height: u32,
    /// xdg_positioner.anchor and .gravity (0 none .. 8 bottom right).
    pub anchor: u32,
    pub gravity: u32,
    /// xdg_positioner.constraint_adjustment bits.
    pub adjust: u32,
    pub offset_x: i32,
    pub offset_y: i32,
    /// POPUP_GRAB or 0.
    pub flags: u32,
}

/// Where the popup was placed: on the output, and relative to its parent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PopupPlacement {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub rel_x: i32,
    pub rel_y: i32,
}

/// The request body, as the window manager reads it (window/popup.rs).
pub fn popup_body(spec: &PopupSpec) -> [u8; BODY_LEN] {
    let words = [
        spec.window_id,
        spec.parent_id,
        spec.anchor_x as u32,
        spec.anchor_y as u32,
        spec.anchor_w,
        spec.anchor_h,
        spec.width,
        spec.height,
        spec.anchor,
        spec.gravity,
        spec.adjust,
        spec.offset_x as u32,
        spec.offset_y as u32,
        spec.flags,
    ];
    let mut body = [0u8; BODY_LEN];
    for (i, w) in words.iter().enumerate() {
        body[i * 4..i * 4 + 4].copy_from_slice(&w.to_le_bytes());
    }
    body
}

pub fn popup_open(
    port: u32,
    request_id: u32,
    spec: &PopupSpec,
) -> Result<PopupPlacement, &'static str> {
    let body = popup_body(spec);
    let mut rx = [0u8; HDR_LEN + RESP_LEN];
    let n = call_payload(port, NWMP_MAGIC, OP, request_id, &body, &mut rx)?;
    if read_u32(&rx, HDR_LEN)? != 0 || n < HDR_LEN + RESP_LEN {
        return Err("wm rejected popup_open");
    }
    let at = |i: usize| read_u32(&rx, HDR_LEN + 4 + i * 4);
    Ok(PopupPlacement {
        x: at(0)?,
        y: at(1)?,
        width: at(2)?,
        height: at(3)?,
        rel_x: at(4)? as i32,
        rel_y: at(5)? as i32,
    })
}

/// The popup a window manager notice says was closed (its window went,
/// moved or was minimised, or a press fell outside it), or None when `msg`
/// is not that notice. Who sent it is checked by the caller: only the window
/// manager's own pid is believed.
pub fn popup_done(msg: &[u8]) -> Option<u32> {
    if msg.len() != NOTIFY_LEN {
        return None;
    }
    let word = |at: usize| u32::from_le_bytes([msg[at], msg[at + 1], msg[at + 2], msg[at + 3]]);
    let version = u16::from_le_bytes([msg[4], msg[5]]);
    (word(0) == NOTIFY_MAGIC && version == 1 && word(8) == POPUP_DONE).then(|| word(16))
}
