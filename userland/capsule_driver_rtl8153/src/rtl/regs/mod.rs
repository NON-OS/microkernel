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

//! The register addresses the steps touch, under this driver's own names.
//! The addresses are the chip's, as Linux v6.12 r8152.c reaches them; what
//! each does is said where a step uses it.

pub mod pla;
pub mod usb;

/// The PHY's own registers, reached through the PLA window (phy.rs).
pub mod phy {
    /// The MII registers from 0xa400, two bytes apart.
    pub const BMCR: u16 = 0xa400;
    pub const ADVERTISE: u16 = 0xa408;
    pub const CTRL1000: u16 = 0xa412;
    /// The PHY's power state in bits 2:0.
    pub const STATE: u16 = 0xa420;
    pub const POWER_CFG: u16 = 0xa430;
    pub const ADC_CFG: u16 = 0xbc06;
}
