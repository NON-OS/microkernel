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

/// Why an MSI controller would not hand out, route or take back vectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MsiError {
    /// No MSI controller is in service.
    NotReady,
    /// The device id is past what the controller's tables hold.
    DeviceOutOfRange,
    /// No LPI, event, SPI or device slot left to hand out.
    Exhausted,
    /// Memory for a translation table could not be had.
    NoMemory,
    /// The controller stalled on a command or did not finish one in time.
    Hardware,
}
