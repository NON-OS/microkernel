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

//! The device power states (ACPI 6.5, 2.3 and 7.3): D0 fully on, D1 and
//! D2 between, D3hot off with the power resources _PR3 lists still on,
//! and D3cold with none.

use crate::namespace::Seg;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DState {
    D0,
    D1,
    D2,
    D3Hot,
    D3Cold,
}

impl DState {
    /// _PSC's answer, 0 to 3; _PSC does not tell D3hot from D3cold.
    pub fn from_psc(v: u64) -> Option<DState> {
        [DState::D0, DState::D1, DState::D2, DState::D3Hot].get(usize::try_from(v).ok()?).copied()
    }

    pub fn from_wire(v: u8) -> Option<DState> {
        [DState::D0, DState::D1, DState::D2, DState::D3Hot, DState::D3Cold].get(v as usize).copied()
    }

    /// The method that enters the state: both D3s run _PS3.
    pub fn ps(self) -> Seg {
        [b'_', b'P', b'S', b'0' + (self as u8).min(3)]
    }

    /// The power resources the state needs: _PR0 to _PR3, and for D3cold
    /// none at all.
    pub fn pr(self) -> Option<Seg> {
        (self != DState::D3Cold).then_some([b'_', b'P', b'R', b'0' + self as u8])
    }
}
