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

//! The main items (HID 1.11, 6.2.2.4 to 6.2.2.6): collections open and
//! close, and Input and Feature items lay their bits into their report.
//! Output items are not read.

use super::field_at::FieldAt;
use super::item::Item;
use super::limits::{Refused, MAX_DEPTH};
use super::walk::Walk;

const INPUT: u8 = 0x8;
const COLLECTION: u8 = 0xA;
const FEATURE: u8 = 0xB;
const END_COLLECTION: u8 = 0xC;

/// Where the walk hands each data field.
pub(super) type Hand<'a> = &'a mut dyn FnMut(&FieldAt);

impl Walk {
    pub(super) fn main(&mut self, item: Item, f: Hand<'_>) -> Result<(), Refused> {
        let g = self.g.now;
        match item.tag {
            COLLECTION if self.depth == MAX_DEPTH => Err(Refused),
            COLLECTION => {
                if self.depth == 0 {
                    self.app = self.l.usage(0);
                }
                self.depth += 1;
                Ok(())
            }
            END_COLLECTION => self.depth.checked_sub(1).map(|d| self.depth = d).ok_or(Refused),
            INPUT => {
                let bits = g.bits()?;
                let at = self.inputs.claim(g.id, bits)?;
                self.long.note(g.id, at.saturating_add(bits));
                self.hand(at, item.data, false, f);
                Ok(())
            }
            FEATURE => {
                let Some(c) = self.features.as_mut() else { return Ok(()) };
                match g.bits().and_then(|bits| c.claim(g.id, bits)) {
                    Ok(at) => self.hand(at, item.data, true, f),
                    Err(Refused) => self.features = None,
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
