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

//! DME_GET and DME_SET on a local attribute, selector 0, normal set type.

use super::uic;
use super::uic_codes::{mib, DME_GET, DME_SET};
use crate::ufs::env::Hw;
use crate::ufs::error::UfsError;

pub fn get<H: Hw>(hw: &H, attr: u16) -> Result<u32, UfsError> {
    uic::run(hw, DME_GET, [mib(attr), 0, 0])
}

pub fn set<H: Hw>(hw: &H, attr: u16, value: u32) -> Result<(), UfsError> {
    uic::run(hw, DME_SET, [mib(attr), 0, value]).map(|_| ())
}
