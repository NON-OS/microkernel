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
//! A missing controller is not among them: discovery decides that before
//! any attempt, and the driver leaves with `EXIT_ABSENT`.

use super::types::NvmeError;

pub const fn reason(e: NvmeError) -> &'static str {
    match e {
        NvmeError::ClaimFailed => "nvme: claim refused",
        NvmeError::BrokerCallFailed => "nvme: broker refused a grant",
        NvmeError::UnsupportedController => "nvme: unsupported or fatal controller",
        NvmeError::UnsupportedPageSize => "nvme: controller lacks 4 KiB pages",
        NvmeError::ControllerTimeout => "nvme: controller did not answer in time",
        NvmeError::ControllerFatal => "nvme: controller stayed fatal through a reset",
        NvmeError::ClockFailed => "nvme: monotonic clock unreadable, no wait can be bounded",
        NvmeError::AdminCommandFailed => "nvme: admin command failed",
        NvmeError::InvalidTransfer => "nvme: invalid transfer",
    }
}
