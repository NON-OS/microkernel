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

//! Each item in turn. A main item (HID 1.11, 6.2.2.4) lays out its bits
//! in its report whether or not it carries data, so padding moves the
//! fields after it; a constant item is padding and is not kept.

use super::error::MapError;
use super::field::Field;
use super::item::{Item, Type};
use super::parser::{Parser, FIELDS_MAX};
use super::span::Dir;

impl Parser {
    pub(super) fn on(&mut self, it: &Item) -> Result<(), MapError> {
        match (it.kind, it.tag) {
            (Type::Global, 10) => self.push(),
            (Type::Global, 11) => self.pop(),
            (Type::Global, _) => self.g.set(it),
            (Type::Local, _) => self.l.set(it, self.g.page),
            (Type::Main, _) => self.main(it),
        }
    }

    fn main(&mut self, it: &Item) -> Result<(), MapError> {
        let spans = self.l.take()?;
        let dir = match it.tag {
            8 => Dir::Input,
            9 => Dir::Output,
            11 => Dir::Feature,
            10 | 12 => return self.nest(it.tag == 10),
            _ => return Err(MapError::Unknown),
        };
        let g = self.g;
        // A map numbers all its reports or none (HID 1.11, 6.2.2.7).
        if *self.numbered.get_or_insert(g.id != 0) != (g.id != 0) {
            return Err(MapError::Shape);
        }
        let bits = u64::from(g.size) * u64::from(g.count);
        let bit = self.layout.grow(dir, g.id, bits)?;
        if it.data & 1 != 0 || bits == 0 || spans.is_empty() {
            return Ok(());
        }
        if g.max() < i64::from(g.lmin) {
            return Err(MapError::Value);
        }
        if self.layout.fields.len() == FIELDS_MAX {
            return Err(MapError::TooMany);
        }
        self.layout.fields.push(Field::new(dir, &g, bit, it.data, spans));
        Ok(())
    }
}
