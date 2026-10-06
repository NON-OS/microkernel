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

//! PHY register reads and writes for the bring-up. The caller holds the
//! software flag.

use crate::constants::phy::PhyReg;
use crate::constants::Family;
use crate::regs::Regs;

use super::paged::once;
use super::retry::with_retry;

pub fn read(regs: &Regs, family: Family, r: PhyReg) -> Result<u16, &'static str> {
    with_retry(family, || once(regs, family, r, None))
}

pub fn write(regs: &Regs, family: Family, r: PhyReg, v: u16) -> Result<(), &'static str> {
    with_retry(family, || once(regs, family, r, Some(v))).map(|_| ())
}

/// One attempt, never retried: Linux turns retries off while the MAC-PHY
/// link is being switched, since every access then fails at first.
pub fn read_once(regs: &Regs, family: Family, r: PhyReg) -> Result<u16, &'static str> {
    once(regs, family, r, None)
}

pub fn write_once(regs: &Regs, family: Family, r: PhyReg, v: u16) -> Result<(), &'static str> {
    once(regs, family, r, Some(v)).map(|_| ())
}
