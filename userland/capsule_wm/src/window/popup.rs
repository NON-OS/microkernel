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

//! A popup: a menu, a completion list, a tooltip, a combo box's list, opened
//! over one of its owner's windows and placed against it (OP_POPUP_OPEN).

use super::{Kind, Visibility, Window};
use crate::geometry::positioner::{place, Placed, Positioner, ADJUST_ALL, EDGE_MAX};
use crate::geometry::Rect;

/// window id, parent id, anchor x, y (i32), w, h, width, height, anchor,
/// gravity, constraint adjustment, offset x, y (i32), flags.
pub const POPUP_OPEN_REQ_LEN: usize = 56;
/// x, y, width, height on the output, x and y relative to the parent (i32).
pub const POPUP_OPEN_RESP_LEN: usize = 24;
/// Request flag: the popup closes on a press outside it and its own popups.
pub const POPUP_GRAB: u32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PopupAsk {
    pub window_id: u32,
    pub parent_id: u32,
    pub positioner: Positioner,
    pub grab: bool,
}

fn word(body: &[u8], i: usize) -> Option<u32> {
    Some(u32::from_le_bytes(body.get(i * 4..i * 4 + 4)?.try_into().ok()?))
}

/// The request, or None for a body of another length, an id of 0, an empty
/// size, an edge or an adjustment xdg_positioner does not have, or a flag
/// this version does not know.
pub fn decode(body: &[u8]) -> Option<PopupAsk> {
    if body.len() != POPUP_OPEN_REQ_LEN {
        return None;
    }
    let w = |i| word(body, i);
    let (window_id, parent_id) = (w(0)?, w(1)?);
    let positioner = Positioner {
        anchor_x: w(2)? as i32,
        anchor_y: w(3)? as i32,
        anchor_w: w(4)?,
        anchor_h: w(5)?,
        width: w(6)?,
        height: w(7)?,
        anchor: w(8)?,
        gravity: w(9)?,
        adjust: w(10)?,
        offset_x: w(11)? as i32,
        offset_y: w(12)? as i32,
    };
    let flags = w(13)?;
    let p = &positioner;
    let sound = window_id != 0
        && parent_id != 0
        && window_id != parent_id
        && p.width != 0
        && p.height != 0
        && p.anchor <= EDGE_MAX
        && p.gravity <= EDGE_MAX
        && p.adjust & !ADJUST_ALL == 0
        && flags & !POPUP_GRAB == 0;
    sound.then_some(PopupAsk { window_id, parent_id, positioner, grab: flags & POPUP_GRAB != 0 })
}

/// The popup `ask` over `parent`, on an output `display_w` by `display_h`
/// whose rows from `top` down are the windows', at stack place `z`, and
/// where it was placed. Kept below `top`, the menubar's foot: the shell's
/// chrome is drawn over every window, so a menu flipped or slid up over
/// the menubar was drawn under it while its presses still went to the menu.
pub fn open(
    ask: &PopupAsk,
    parent: &Window,
    z: u32,
    display: (u32, u32),
    top: u32,
) -> (Window, Placed) {
    let top = top.min(display.1);
    let bounds = Rect { x: 0, y: top, width: display.0, height: display.1 - top };
    let placed = place(&ask.positioner, parent.rect, bounds);
    let window = Window {
        owner_pid: parent.owner_pid,
        window_id: ask.window_id,
        rect: placed.rect,
        kind: Kind::Popup,
        visibility: Visibility::Visible,
        z,
        in_use: true,
        full_screen: false,
        parent: parent.window_id,
        grab: ask.grab,
    };
    (window, placed)
}

/// The reply body: the placed rectangle, then its place on the parent.
pub fn encode(placed: &Placed) -> [u8; POPUP_OPEN_RESP_LEN] {
    let mut out = [0u8; POPUP_OPEN_RESP_LEN];
    let r = placed.rect;
    for (i, v) in
        [r.x, r.y, r.width, r.height, placed.rel_x as u32, placed.rel_y as u32].iter().enumerate()
    {
        out[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes());
    }
    out
}
