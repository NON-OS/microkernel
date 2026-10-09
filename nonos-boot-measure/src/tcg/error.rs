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

/// Why a log was refused. Each has a stable code for the boot log.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogError {
    /// A field runs past the end of the bytes.
    Truncated,
    /// The first event is not the crypto-agile Spec ID header.
    NotCryptoAgile,
    /// The header declares no SHA-256 bank of 32 bytes.
    NoSha256Bank,
    /// More banks than `MAX_ALGS`, or an event with more digests than banks.
    TooManyBanks,
    /// A digest under a bank the header did not declare, or declared twice.
    UnknownBank,
    /// A PCR 4 event that carries no SHA-256 digest.
    MissingSha256,
    /// An event body over `MAX_EVENT_BYTES`.
    EventTooLarge,
    /// More than `MAX_EVENTS` events.
    TooManyEvents,
    /// A log over `MAX_LOG_BYTES`.
    TooLarge,
}

impl LogError {
    pub const fn code(self) -> u32 {
        match self {
            LogError::Truncated => 1,
            LogError::NotCryptoAgile => 2,
            LogError::NoSha256Bank => 3,
            LogError::TooManyBanks => 4,
            LogError::UnknownBank => 5,
            LogError::MissingSha256 => 6,
            LogError::EventTooLarge => 7,
            LogError::TooManyEvents => 8,
            LogError::TooLarge => 9,
        }
    }
}
