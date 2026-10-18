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
//! The multitouch path for one device: its slots, whether it is a screen
//! or a pad, and the frame being put together. It serves a touchscreen,
//! and a pad that reports fingers in parallel; a pad with one finger
//! collection keeps the touch layout's path, which reads hybrid reports by
//! their Contact Identifier.

use super::super::parse::{parse_fingers, Surface};
use super::super::Field;
use super::assemble::Assembler;
use super::frame::{decode_frame, Frame};
use super::screen::Screen;
use super::slots::Slots;

#[derive(Clone, Copy, Default)]
pub struct Multitouch {
    pub slots: Slots,
    pub surface: Surface,
    rid: u8,
    count: Field,
    button: Field,
    assembler: Assembler,
    pub screen: Screen,
}

impl Multitouch {
    pub fn new(desc: &[u8]) -> Multitouch {
        let g = parse_fingers(desc);
        let (slots, surface, rid, count, button) = (g.slots, g.surface, g.rid, g.count, g.button);
        Multitouch { slots, surface, rid, count, button, ..Multitouch::default() }
    }

    pub fn serves(&self) -> bool {
        self.slots.n > 0 && (self.surface == Surface::Screen || self.slots.n >= 2)
    }

    /// Whether `report` (a report after the i2c-hid length) is the touch
    /// report this path reads: every report, where the device numbers none.
    pub fn takes(&self, report: &[u8]) -> bool {
        self.rid == 0 || report.first() == Some(&self.rid)
    }

    /// The frame `report` (a report after the i2c-hid length) completes;
    /// `None` for another report id, a torn read, or a frame not yet whole.
    pub fn frame(&mut self, report: &[u8]) -> Option<Frame> {
        let body = match self.rid {
            0 => report,
            rid => report.split_first().filter(|(&r, _)| r == rid)?.1,
        };
        let f = decode_frame(body, &self.slots, self.count, self.button)?;
        self.assembler.push(f)
    }

    /// The first slot's position ranges, which every slot shares: logical
    /// maximum less minimum, as frame.rs counts positions from the minimum.
    pub fn max(&self) -> (i32, i32) {
        let range = |f: Field| {
            let r = i64::from(f.logical_max) - i64::from(f.logical_min);
            r.clamp(0, i64::from(i32::MAX)) as i32
        };
        (range(self.slots.list[0].x), range(self.slots.list[0].y))
    }
}
