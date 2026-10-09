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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApicError {
    NotSupported,
    AlreadyInitialized,
    NotInitialized,
    X2ApicNotSupported,
    MmioMapFailed,
    InvalidVector,
    IcrBusy,
    /// The id does not fit the destination field of the current mode.
    DestinationUnreachable,
    /// The local APIC logged a send error (ESR) for the command.
    SendRejected,
}

impl ApicError {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotSupported => "APIC not supported",
            Self::AlreadyInitialized => "APIC already initialized",
            Self::NotInitialized => "APIC not initialized",
            Self::X2ApicNotSupported => "x2APIC not supported",
            Self::MmioMapFailed => "APIC MMIO mapping failed",
            Self::InvalidVector => "Invalid interrupt vector",
            Self::IcrBusy => "ICR busy timeout",
            Self::DestinationUnreachable => "APIC id not addressable in this mode",
            Self::SendRejected => "IPI send error latched in ESR",
        }
    }

    pub const fn to_errno(self) -> i32 {
        match self {
            Self::NotSupported | Self::X2ApicNotSupported => -19,
            Self::AlreadyInitialized => -16,
            Self::NotInitialized | Self::MmioMapFailed => -5,
            Self::InvalidVector => -22,
            Self::IcrBusy => -16,
            Self::DestinationUnreachable => -22,
            Self::SendRejected => -5,
        }
    }
}

pub type ApicResult<T> = Result<T, ApicError>;
