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

use alloc::vec::Vec;
use nonos_aml::namespace::Id;
use nonos_aml::video::Levels;

pub struct Output {
    pub id: Id,
    /// The output's _ADR, which matches its adapter's _DOD entry: the
    /// panel's identity on the brightness wire. Else 0xFFFFFFFF less its
    /// index, unique all the same.
    pub panel: u32,
    pub levels: Levels,
    /// Percent, from _BQC at start and then what was last set; None when
    /// neither is known.
    pub current: Option<u8>,
}

#[derive(Default)]
pub struct Backlight {
    pub outputs: Vec<Output>,
}
