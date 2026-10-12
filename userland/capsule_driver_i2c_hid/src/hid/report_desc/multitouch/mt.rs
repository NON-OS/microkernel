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

use super::super::layout::{Field, TouchLayout};
use super::super::parse::{parse_slots, surface, Surface};
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
    pub fn new(desc: &[u8], layout: &TouchLayout) -> Multitouch {
        let (rid, count, button) = (layout.report_id, layout.contact_count, layout.button);
        let slots = parse_slots(desc, rid);
        Multitouch { slots, surface: surface(desc), rid, count, button, ..Multitouch::default() }
    }

    pub fn serves(&self) -> bool {
        self.slots.n > 0 && (self.surface == Surface::Screen || self.slots.n >= 2)
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

    /// The first slot's logical maxima, which every slot shares.
    pub fn max(&self) -> (i32, i32) {
        (self.slots.list[0].x.logical_max, self.slots.list[0].y.logical_max)
    }
}
