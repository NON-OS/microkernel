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

//! A window table for the popup proofs: windows and popups opened the way
//! the window manager's handlers open them.

use crate::geometry::positioner::{Placed, Positioner};
use crate::geometry::Rect;
use crate::window::popup::{open, PopupAsk};
use crate::window::{Kind, Visibility, Window, WindowTable};
use crate::z_order::ZStack;

pub(super) const DISPLAY: (u32, u32) = (1280, 800);

pub(super) struct Wm {
    pub(super) table: WindowTable,
    pub(super) z: ZStack,
}

impl Wm {
    pub(super) fn new() -> Wm {
        Wm { table: WindowTable::new(), z: ZStack::new() }
    }

    pub(super) fn window(&mut self, pid: u32, id: u32, r: (u32, u32, u32, u32)) {
        let window = Window {
            owner_pid: pid,
            window_id: id,
            rect: Rect { x: r.0, y: r.1, width: r.2, height: r.3 },
            kind: Kind::Normal,
            visibility: Visibility::Visible,
            z: self.z.allocate(),
            in_use: true,
            full_screen: false,
            parent: 0,
            grab: false,
        };
        self.table.insert(window).expect("room for the window");
    }

    /// A popup as OP_POPUP_OPEN opens it: placed against its parent.
    pub(super) fn popup(
        &mut self,
        pid: u32,
        id: u32,
        parent: u32,
        p: Positioner,
        grab: bool,
    ) -> Placed {
        let over = *self.table.find(pid, parent).expect("the parent is open");
        let ask = PopupAsk { window_id: id, parent_id: parent, positioner: p, grab };
        // Bounded from the display's top row, as the shell's own popups are;
        // wm_proofs popup_tests hold an app's popup below the menubar.
        let (window, placed) = open(&ask, &over, self.z.allocate(), DISPLAY, 0);
        self.table.insert(window).expect("room for the popup");
        placed
    }

    pub(super) fn z_of(&self, pid: u32, id: u32) -> u32 {
        self.table.find(pid, id).expect("open").z
    }
}

/// A menu's positioner: `w` by `h`, hanging from the bottom-left corner of
/// an anchor at (ax, ay, aw, ah) on the parent, with every adjustment.
pub(super) fn menu(a: (i32, i32, u32, u32), w: u32, h: u32) -> Positioner {
    Positioner {
        anchor_x: a.0,
        anchor_y: a.1,
        anchor_w: a.2,
        anchor_h: a.3,
        width: w,
        height: h,
        anchor: 6,
        gravity: 8,
        adjust: 63,
        offset_x: 0,
        offset_y: 0,
    }
}

/// A small deterministic generator, so a failing case replays exactly.
pub(super) struct Rng(u64);

impl Rng {
    pub(super) fn new(seed: u64) -> Rng {
        Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }

    pub(super) fn below(&mut self, n: u32) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 % u64::from(n.max(1))) as u32
    }
}
