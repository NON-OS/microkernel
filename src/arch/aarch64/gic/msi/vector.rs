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

/// One interrupt a device raises by message: the INTID the GIC delivers,
/// and the data word the device writes to the doorbell to raise it. An ITS
/// takes an event id there, a GICv2m frame the SPI's INTID.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Vector {
    pub intid: u32,
    pub data: u32,
}
