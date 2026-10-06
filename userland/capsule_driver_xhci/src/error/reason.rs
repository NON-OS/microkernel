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
//! up, which replaced the negated-errno exit codes, so each cause reads
//! apart. A machine with no xHCI controller never gets this far: discovery
//! decides that before any attempt, and the driver leaves with `EXIT_ABSENT`.

use super::xhci_error::XhciError;

pub const fn reason(e: XhciError) -> &'static str {
    match e {
        XhciError::BrokerCallFailed(_) => "xhci: broker refused a claim or grant",
        XhciError::ControllerUnsupported => "xhci: unsupported controller",
        XhciError::ResetTimeout => "xhci: controller reset did not finish",
        XhciError::ControllerNotReadyTimeout => "xhci: controller never left not-ready",
        XhciError::StartTimeout => "xhci: controller did not start running",
        XhciError::HaltTimeout => "xhci: controller did not halt",
        XhciError::CommandRingFull => "xhci: command ring full",
        XhciError::TransferRingFull => "xhci: transfer ring full",
        XhciError::NoDeviceOnPort => "xhci: no device on port",
        XhciError::PortResetTimeout => "xhci: port reset did not finish",
        XhciError::TransferCompletionTimeout => "xhci: transfer did not complete",
        XhciError::CommandCompletionTimeout => "xhci: command did not complete",
        XhciError::CommandCompletionFailed(_) => "xhci: command completed with an error",
        XhciError::UnexpectedCompletionSlot => "xhci: completion named another slot",
        XhciError::TransferCompletionFailed(_) => "xhci: transfer completed with an error",
    }
}
