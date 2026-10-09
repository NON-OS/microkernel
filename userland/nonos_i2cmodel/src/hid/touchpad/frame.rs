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

//! One input report of the touchpad, laid out as its descriptor promises.

use super::report_desc::TOUCH_REPORT_ID;

/// A finger frame. `confidence` false is how a pad reports a palm.
#[derive(Clone, Copy, Debug)]
pub struct Touch {
    pub x: u16,
    pub y: u16,
    pub tip: bool,
    pub confidence: bool,
    pub contacts: u8,
    pub button: bool,
    /// Which finger the report describes (Contact Identifier, six bits). A
    /// pad in hybrid mode sends one finger per report and the contact count
    /// only in the first report of a frame.
    pub id: u8,
}

impl Touch {
    /// One confident finger down at (x, y).
    pub fn finger(x: u16, y: u16) -> Self {
        Self { x, y, tip: true, confidence: true, contacts: 1, button: false, id: 0 }
    }

    /// Nothing on the pad.
    pub fn lifted() -> Self {
        Self { x: 0, y: 0, tip: false, confidence: true, contacts: 0, button: false, id: 0 }
    }
}

/// The report as the device hands it to the input register, id first. The
/// device adds the two-byte length when it frames it.
pub fn touch_report(t: &Touch) -> Vec<u8> {
    let x = t.x.to_le_bytes();
    let y = t.y.to_le_bytes();
    vec![
        TOUCH_REPORT_ID,
        (t.confidence as u8) | ((t.tip as u8) << 1) | ((t.id & 0x3f) << 2),
        x[0],
        x[1],
        y[0],
        y[1],
        t.contacts,
        t.button as u8,
    ]
}
