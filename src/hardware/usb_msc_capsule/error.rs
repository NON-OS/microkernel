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

//! Errors the kernel-side USB mass-storage client surfaces, in the shape the
//! other block clients use, with one more: the driver is still looking for
//! its device.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverUsbMscError {
    Dead,
    Stale,
    AccessDenied,
    InvalidArgument,
    OversizedRequest,
    OutOfRange,
    DeviceFailure,
    /// The device's logical blocks are not 512 bytes, so it is not served.
    BlockSize,
    NoCallerPid,
    TransportFailure,
    ProtocolMismatch,
    /// The driver has not yet decided whether a device is there.
    NotReady,
    /// The driver looked, and no mass-storage device is plugged in.
    NoDevice,
}
