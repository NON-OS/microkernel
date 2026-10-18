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

//! Parsing a Report Map (HID 1.11, 6.2.2) into a `Layout`, within bounds
//! a peripheral cannot widen: the map's length, the collections and
//! pushes, the fields, the usages and the reports' lengths.

use super::error::MapError;
use super::item::item;
use super::layout::Layout;
use super::parser::Parser;

/// The longest Report Map HIDS 1.0 allows.
pub const MAP_MAX: usize = 512;

pub fn parse(map: &[u8]) -> Result<Layout, MapError> {
    if map.len() > MAP_MAX {
        return Err(MapError::TooMany);
    }
    let mut p = Parser::default();
    let mut at = 0;
    while let Some((it, next)) = item(map, at)? {
        p.on(&it)?;
        at = next;
    }
    if p.depth != 0 {
        return Err(MapError::Shape);
    }
    Ok(p.layout)
}
