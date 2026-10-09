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

//! The words a failed bring-up attempt reports. The shared bring-up schedule
//! carries the last one into the single line logged when the driver gives
//! up, which replaced the per-error exit codes, so each cause reads apart.
//! A machine with no AHCI controller never gets this far: discovery decides
//! that before any attempt, and the driver leaves with `EXIT_ABSENT`.

use super::types::AhciError;

pub const fn reason(e: AhciError) -> &'static str {
    match e {
        AhciError::DeviceNotFound => "ahci: no controller could be opened",
        AhciError::NoDisk => "ahci: controller up, but no port holds an ATA disk",
        AhciError::BrokerCallFailed(_) => "ahci: broker refused a claim or grant",
        AhciError::CommandFailed => "ahci: device reported a command error",
        AhciError::Timeout => "ahci: controller or link did not answer in time",
        AhciError::IdentityRefused(_) => "ahci: disk's IDENTIFY answer cannot be served",
        AhciError::OutOfRange => "ahci: a transfer fell outside the disk or the data buffer",
    }
}
