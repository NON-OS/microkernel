// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::error::WireError as E;
use crate::hardware::block_device::BlockDeviceError;

/// An empty slot or a driver with no device answers like a stopped driver:
/// no disk there, so the block layer moves on. One still starting is asked
/// again. A read-only medium refuses the write, not the caller.
pub(in crate::hardware::block_device) fn map_wire_error(e: E) -> BlockDeviceError {
    match e {
        E::Dead | E::NoDevice | E::NoMedium => BlockDeviceError::Dead,
        E::Stale => BlockDeviceError::Stale,
        E::AccessDenied => BlockDeviceError::AccessDenied,
        E::InvalidArgument => BlockDeviceError::InvalidArgument,
        E::OversizedRequest => BlockDeviceError::OversizedRequest,
        E::OutOfRange => BlockDeviceError::OutOfRange,
        E::DeviceFailure | E::TimedOut => BlockDeviceError::DeviceFailure,
        E::ReadOnly => BlockDeviceError::Unsupported,
        E::NoCallerPid => BlockDeviceError::NoCallerPid,
        E::TransportFailure => BlockDeviceError::TransportFailure,
        E::ProtocolMismatch => BlockDeviceError::ProtocolMismatch,
        E::NotReady => BlockDeviceError::NotReady,
        E::BlockSize => BlockDeviceError::BlockSize,
    }
}
