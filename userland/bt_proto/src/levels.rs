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

//! BATTERY's reply, one entry a level.

use super::list::{entries, LEVELS_MAX, LEVEL_LEN};

/// One level: the bond's place in BONDS's list (`NO_PLACE` for a link that
/// kept no bond), which Battery Service, and the percent.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Level {
    pub place: u8,
    pub instance: u8,
    pub percent: u8,
}

/// A checked BATTERY payload.
#[derive(Clone, Copy, Debug)]
pub struct Levels<'a>(&'a [u8]);

impl<'a> Levels<'a> {
    /// A percent over 100 is no level: Battery Level is 0 to 100 (BAS 1.0,
    /// 3.1), and the capsule keeps none other.
    pub fn read(data: &'a [u8]) -> Option<Self> {
        let rest = entries(data, LEVEL_LEN, LEVELS_MAX)?;
        rest.chunks_exact(LEVEL_LEN).all(|e| e[2] <= 100).then_some(Self(rest))
    }

    pub fn len(&self) -> usize {
        self.0.len() / LEVEL_LEN
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self, i: usize) -> Option<Level> {
        let e = self.0.chunks_exact(LEVEL_LEN).nth(i)?;
        Some(Level { place: e[0], instance: e[1], percent: e[2] })
    }
}
