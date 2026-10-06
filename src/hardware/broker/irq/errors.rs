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

//! Why a bind, ack or unbind was refused, and what a poll reads back.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrqBindError {
    NotClaimed,
    StaleEpoch,
    UnknownDevice,
    NotDeviceIrq,
    AlreadyBound,
    /// The line is one the kernel routes to itself.
    ReservedGsi,
    NoVector,
    UnsupportedFlags,
    NotIntx,
    NoMsixCap,
    /// MSI asked of a function with no MSI capability.
    NoMsiCap,
    BadMsixBar,
    BadVectorCount,
    MsixProgramFailed,
    MsiProgramFailed,
    NoDeviceHandle,
    PlatformError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrqError {
    UnknownGrant,
    NotHolder,
    PlatformError,
}

#[derive(Debug, Clone, Copy)]
pub struct IrqPollResult {
    pub seq: u64,
    pub overflow: u64,
}
