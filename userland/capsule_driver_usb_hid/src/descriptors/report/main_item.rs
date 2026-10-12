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
//! close, Input items lay out the reports this driver reads, and Output and
//! Feature items are held to the same bounds but not read.

use super::apps::hand;
use super::input::Input;
use super::limits::{Refused, MAX_DEPTH};
use super::walk::Walk;

const INPUT: u8 = 0x8;
const OUTPUT: u8 = 0x9;
const COLLECTION: u8 = 0xA;
const FEATURE: u8 = 0xB;
const END_COLLECTION: u8 = 0xC;
// Input item flags: Constant, Variable (not Array), Relative.
const CONSTANT: u32 = 1 << 0;
const VARIABLE: u32 = 1 << 1;
const RELATIVE: u32 = 1 << 2;

impl Walk {
    pub(super) fn main(&mut self, item: super::item::Item) -> Result<(), Refused> {
        match item.tag {
            COLLECTION if self.depth == MAX_DEPTH => Err(Refused),
            COLLECTION => {
                if self.depth == 0 {
                    self.app = self.l.usage(0);
                    self.apps = self.apps.saturating_add(1);
                }
                self.depth += 1;
                Ok(())
            }
            END_COLLECTION => self.depth.checked_sub(1).map(|d| self.depth = d).ok_or(Refused),
            INPUT => self.input(item.data),
            OUTPUT | FEATURE => self.g.now.bits().map(drop),
            _ => Ok(()),
        }
    }

    /// Lay the item's bits into its report, then hand its fields to the
    /// collection kind they are in. Constant fields are padding.
    fn input(&mut self, flags: u32) -> Result<(), Refused> {
        let g = self.g.now;
        let at = self.cursors.claim(g.id, g.bits()?)?;
        if flags & CONSTANT != 0 || self.depth == 0 {
            return Ok(());
        }
        let (variable, relative) = (flags & VARIABLE != 0, flags & RELATIVE != 0);
        let inp = Input { id: g.id, app: self.apps, at, g, variable, relative, l: &self.l };
        hand(self.app, &mut self.layout, &inp);
        Ok(())
    }
}
