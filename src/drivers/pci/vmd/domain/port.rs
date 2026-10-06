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

//! Config space as the assignment sees it, and what the assignment reports.

/// Config space as the assignment sees it: 32-bit registers of one function.
pub trait ConfigPort {
    fn read32(&mut self, bus: u8, device: u8, function: u8, offset: u16) -> u32;
    fn write32(&mut self, bus: u8, device: u8, function: u8, offset: u16, value: u32);
}

/// What assignment did, for the boot log and the proofs.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Assigned {
    pub bridges: u16,
    pub endpoints: u16,
    pub bars: u16,
    /// BARs left unassigned because the window or the bus range ran out.
    pub starved: u16,
    pub last_bus: u8,
}
