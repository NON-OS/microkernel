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

//! What the served part is called, and the console line that says so.

use super::super::env::{Clock, Log, Mmio};
use super::super::info::{names, sd_names, Names};
use super::super::text::Line;
use super::EmmcDisk;

impl<M: Mmio, C: Clock, L: Log> EmmcDisk<M, C, L> {
    /// The part's model and serial, from its CID.
    pub fn names(&self) -> Names {
        if self.card.sd {
            sd_names(&self.card.cid)
        } else {
            names(&self.card.cid)
        }
    }

    /// One line naming the part being served and the OCR it came up with, so
    /// a boot log says which chip a report is about.
    pub fn say_part(&self) {
        let n = self.names();
        self.host.say(
            Line::new()
                .s(b"serving ")
                .ascii(n.model())
                .s(b", serial ")
                .ascii(&n.serial)
                .s(b", OCR ")
                .hex(self.card.ocr as u64),
        );
    }
}
