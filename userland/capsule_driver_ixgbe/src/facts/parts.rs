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

//! Which ixgbe MAC a function is. The 82598 (another register layout) and
//! the X550EM parts (firmware- and external-PHY-managed links) are not
//! taken. Source: observed in Linux ixgbe, unverified.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Part {
    I82599,
    X540,
    X550,
}

impl Part {
    /// Only the 82599 needs AUTOC put back after its reset, and only it
    /// needs the security block's receive path held around RXEN.
    pub fn is_82599(self) -> bool {
        self == Part::I82599
    }

    /// The X550's NBASE-T rates (2.5 and 5 Gb/s) in LINKS.
    pub fn has_nbase_t(self) -> bool {
        self == Part::X550
    }
}
