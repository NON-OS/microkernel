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
//! Which igb MAC a device id is, and what differs between them: the reset,
//! the receive-address table and the PHY access. Source: observed in Linux
//! igb, unverified against Intel's datasheets.

/// The port's media as its device id names it: only copper is brought up
/// (bring/port.rs).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Media {
    Copper,
    /// Fiber, SerDes, SGMII or backplane.
    Other,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Part {
    I82575,
    I82576,
    /// The 82580, and the DH89xxCC, which runs as one.
    I82580,
    I350,
    I354,
    I210,
    I211,
}

impl Part {
    /// The PHY power register with Go Link Disconnect exists on these.
    pub fn has_go_link_disconnect(self) -> bool {
        matches!(self, Part::I82580 | Part::I350 | Part::I210 | Part::I211)
    }

    /// The I210/I211 PHY is paged; page 0 holds the IEEE registers.
    pub fn paged_phy(self) -> bool {
        matches!(self, Part::I210 | Part::I211)
    }

    /// Receive-address entries.
    pub fn rar_entries(self) -> usize {
        match self {
            Part::I82575 | Part::I210 | Part::I211 => 16,
            Part::I82576 | Part::I82580 => 24,
            Part::I350 | Part::I354 => 32,
        }
    }
}
