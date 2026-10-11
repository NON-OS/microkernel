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

//! The High Speed clock DEVICE_TYPE allows.

use super::super::super::sdhci::clock::{HS26_HZ, HS52_HZ};
use super::register::ExtCsd;
use super::values::{TYPE_HS_26, TYPE_HS_52};

impl ExtCsd {
    /// High Speed SDR at 52 MHz, at 26 MHz, or neither (0).
    pub const fn hs_hz(&self) -> u32 {
        let t = self.device_type();
        if t & TYPE_HS_52 != 0 {
            HS52_HZ
        } else if t & TYPE_HS_26 != 0 {
            HS26_HZ
        } else {
            0
        }
    }
}
