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

//! A step's register, MAC side or PHY, read and written alike.

use nonos_usbnet::Bus;

use super::ocp::{read, write};
use super::op::Target;
use super::phy::{phy_read, phy_write};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Stop {
    /// A register transfer failed with this errno.
    Transfer(i32),
    /// A wait that must end did not.
    TimedOut(&'static str),
}

/// ETIMEDOUT, for a wait that never ended.
pub const E_TIMEDOUT: i32 = -110;

impl Stop {
    pub fn errno(self) -> i32 {
        match self {
            Stop::Transfer(e) => e,
            Stop::TimedOut(_) => E_TIMEDOUT,
        }
    }
}

pub fn get<B: Bus>(bus: &mut B, page: &mut Option<u16>, t: Target) -> Result<u32, Stop> {
    match t {
        Target::Mac(r) => read(bus, r),
        Target::Phy(a) => phy_read(bus, page, a),
    }
    .map_err(Stop::Transfer)
}

pub fn put<B: Bus>(bus: &mut B, page: &mut Option<u16>, t: Target, v: u32) -> Result<(), Stop> {
    match t {
        Target::Mac(r) => write(bus, r, v),
        Target::Phy(a) => phy_write(bus, page, a, v),
    }
    .map_err(Stop::Transfer)
}
