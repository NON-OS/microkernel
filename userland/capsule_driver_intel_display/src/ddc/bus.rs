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

//! The DDC lines of one output, as DDC/CI uses them. The address is the
//! carrier's to fix at DDC_CI_ADDR, not the caller's to pass, so nothing
//! here can be turned into a way to reach another device on the bus.

use super::error::DdcError;

pub trait DdcBus {
    /// One write to the display: `bytes`, then a stop.
    fn send(&mut self, bytes: &[u8]) -> Result<(), DdcError>;
    /// One read from the display, `out.len()` bytes, then a stop.
    fn receive(&mut self, out: &mut [u8]) -> Result<(), DdcError>;
}
