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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WireError {
    /// The capsule is not running.
    Dead,
    /// The capsule restarted while the request was out.
    Stale,
    AccessDenied,
    InvalidArgument,
    OversizedRequest,
    OutOfRange,
    DeviceFailure,
    TimedOut,
    NoCallerPid,
    TransportFailure,
    ProtocolMismatch,
    /// The driver is still bringing its device up (EAGAIN).
    NotReady,
    /// The driver runs and serves no device (ENODEV).
    NoDevice,
    /// The slot is empty (ENOMEDIUM, or GEOMETRY without media present).
    NoMedium,
    /// The medium refuses writes (EROFS, or GEOMETRY's read-only flag).
    ReadOnly,
    /// Its blocks cannot be addressed as 512-byte sectors.
    BlockSize,
}
