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

use crate::identity::Refusal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AhciError {
    DeviceNotFound,
    /// No ATA disk on a port: its link stayed quiet after COMRESET (nothing
    /// attached), or the device on it is no ATA disk (an optical drive).
    NoDisk,
    BrokerCallFailed(i64),
    CommandFailed,
    Timeout,
    /// The disk's IDENTIFY block breaks the named rule in `identity`.
    IdentityRefused(Refusal),
    /// A transfer named sectors outside the served disk, or more bytes than
    /// the data buffer holds; no command was built.
    OutOfRange,
}

pub type AhciResult<T> = Result<T, AhciError>;
