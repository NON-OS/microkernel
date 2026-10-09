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

/// What bring-up needs to know about the controller beyond its registers.
#[derive(Clone, Copy)]
pub struct BusSetup {
    /// DesignWare input clock in Hz, which the SCL counts derive from.
    pub clock_hz: u32,
    /// Physical base of the MMIO window when this is an Intel LPSS function
    /// (it has the LPSS private block); None for a platform controller.
    pub lpss_base: Option<u64>,
    /// Run the bus in standard mode (100 kHz): a device on it declared a
    /// ConnectionSpeed below fast mode.
    pub standard_mode: bool,
}
