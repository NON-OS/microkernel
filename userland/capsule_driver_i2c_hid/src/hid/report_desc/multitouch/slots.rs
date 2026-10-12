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
//! Every finger a multitouch report carries. A precision touchpad or a
//! touchscreen in parallel mode repeats one logical collection per finger
//! (Tip Switch, Confidence, Contact Identifier, X, Y); the n-th time a
//! usage appears it is the n-th finger's (HID Usage Tables, Digitizers page
//! 0Dh, as recalled, unverified). A pad in hybrid mode has one such
//! collection and sends its fingers over several reports (assemble.rs).

use super::super::layout::Field;

/// Fingers kept per report: a ten-finger screen's worth.
pub const MAX_SLOTS: usize = 10;

const PAGE_DESKTOP: u16 = 0x01;
const PAGE_DIGITIZER: u16 = 0x0D;

#[derive(Clone, Copy, Default)]
pub struct Slot {
    pub tip: Field,
    pub confidence: Field,
    pub id: Field,
    pub x: Field,
    pub y: Field,
}

#[derive(Clone, Copy, Default)]
pub struct Slots {
    pub list: [Slot; MAX_SLOTS],
    /// Slots that carry at least one field.
    pub n: usize,
}

impl Slots {
    /// An input field of the touch report, given to the first slot that
    /// lacks its usage; a usage this does not keep is ignored.
    pub fn record(&mut self, page: u16, usage: u16, field: Field) {
        for (k, slot) in self.list.iter_mut().enumerate() {
            let Some(f) = field_of(slot, page, usage) else { return };
            if !f.present() {
                *f = field;
                self.n = self.n.max(k + 1);
                return;
            }
        }
    }
}

fn field_of(s: &mut Slot, page: u16, usage: u16) -> Option<&mut Field> {
    match (page, usage) {
        (PAGE_DIGITIZER, 0x42) => Some(&mut s.tip),
        (PAGE_DIGITIZER, 0x47) => Some(&mut s.confidence),
        (PAGE_DIGITIZER, 0x51) => Some(&mut s.id),
        (PAGE_DESKTOP, 0x30) => Some(&mut s.x),
        (PAGE_DESKTOP, 0x31) => Some(&mut s.y),
        _ => None,
    }
}
