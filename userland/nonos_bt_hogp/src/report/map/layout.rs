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

//! A parsed Report Map: the fields of every report, and each report's
//! length in bits, by direction and Report ID (0 when the map numbers
//! none).

use super::error::MapError;
use super::field::Field;
use super::span::Dir;
use alloc::vec::Vec;

/// Reports, counting each direction of one ID apart.
pub const IDS_MAX: usize = 16;
/// The longest report this host lays out, in bits: 64 octets.
pub const REPORT_BITS_MAX: u64 = 512;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Layout {
    pub fields: Vec<Field>,
    reports: Vec<(Dir, u8, u32)>,
}

impl Layout {
    /// The report's length in octets, if the map has it.
    pub fn octets(&self, dir: Dir, id: u8) -> Option<usize> {
        let r = self.reports.iter().find(|r| (r.0, r.1) == (dir, id))?;
        Some(r.2.div_ceil(8) as usize)
    }

    pub fn fields(&self, dir: Dir, id: u8) -> impl Iterator<Item = &Field> {
        self.fields.iter().filter(move |f| (f.dir, f.id) == (dir, id))
    }

    pub fn ids(&self, dir: Dir) -> impl Iterator<Item = u8> + '_ {
        self.reports.iter().filter(move |r| r.0 == dir).map(|r| r.1)
    }

    /// Room for `bits` more in a report; where they start.
    pub(super) fn grow(&mut self, dir: Dir, id: u8, bits: u64) -> Result<u32, MapError> {
        let i = match self.reports.iter().position(|r| (r.0, r.1) == (dir, id)) {
            Some(i) => i,
            None if self.reports.len() == IDS_MAX => return Err(MapError::TooMany),
            None => {
                self.reports.push((dir, id, 0));
                self.reports.len() - 1
            }
        };
        let start = self.reports[i].2;
        let end = u64::from(start) + bits;
        if end > REPORT_BITS_MAX {
            return Err(MapError::TooMany);
        }
        self.reports[i].2 = end as u32;
        Ok(start)
    }
}
