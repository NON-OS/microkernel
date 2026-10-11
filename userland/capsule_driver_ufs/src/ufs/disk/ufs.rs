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

//! The device once up: the context requests need, and the units served,
//! wire unit 0 first in LUN order.

use super::unit::Unit;
use crate::ufs::ctx::Ctx;
use crate::ufs::scsi::luns::MAX_UNITS;

pub struct Ufs {
    pub ctx: Ctx,
    pub units: [Option<Unit>; MAX_UNITS],
}

impl Ufs {
    pub fn unit(&self, i: u16) -> Option<&Unit> {
        self.units.get(i as usize)?.as_ref()
    }

    /// The most blocks of `u` one request moves: what the data region holds.
    pub fn max_blocks(&self, u: &Unit) -> u32 {
        (self.ctx.lists.data.len / u.block_len as usize) as u32
    }

    /// The most bytes one request moves, whatever the unit.
    pub fn max_bytes(&self) -> usize {
        self.ctx.lists.data.len
    }
}
