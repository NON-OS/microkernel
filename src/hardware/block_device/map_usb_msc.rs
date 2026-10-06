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

use super::BlockDeviceError;
use crate::hardware::usb_msc_capsule::DriverUsbMscError as E;

/// A driver that looked and found no device answers like a stopped driver:
/// no disk there. One still looking is asked again.
pub(super) fn map_usb_msc_error(e: E) -> BlockDeviceError {
    match e {
        E::Dead | E::NoDevice => BlockDeviceError::Dead,
        E::Stale => BlockDeviceError::Stale,
        E::AccessDenied => BlockDeviceError::AccessDenied,
        E::InvalidArgument => BlockDeviceError::InvalidArgument,
        E::OversizedRequest => BlockDeviceError::OversizedRequest,
        E::OutOfRange => BlockDeviceError::OutOfRange,
        E::DeviceFailure => BlockDeviceError::DeviceFailure,
        E::BlockSize => BlockDeviceError::BlockSize,
        E::NoCallerPid => BlockDeviceError::NoCallerPid,
        E::TransportFailure => BlockDeviceError::TransportFailure,
        E::ProtocolMismatch => BlockDeviceError::ProtocolMismatch,
        E::NotReady => BlockDeviceError::NotReady,
    }
}
