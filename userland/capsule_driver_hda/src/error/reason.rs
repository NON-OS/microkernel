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
//! up, so each cause reads apart. A machine with no HD Audio controller
//! never gets this far: discovery decides that before any attempt, and the
//! driver leaves with `EXIT_ABSENT`.

use super::types::HdaError;

pub const fn reason(e: HdaError) -> &'static str {
    match e {
        HdaError::BrokerCallFailed(_) => "hda: broker refused a claim or grant",
        HdaError::ControllerResetTimeout => "hda: controller did not leave reset",
        HdaError::UnsupportedController => "hda: unsupported controller or codec",
        HdaError::VerbTimeout => "hda: codec did not answer a verb",
        HdaError::ControllerNotResponding => "hda: controller reads all ones (powered down or gone)",
        HdaError::DmaOutOfReach => "hda: DMA buffer above 4 GiB on a 32-bit controller",
        HdaError::StreamResetTimeout => "hda: stream descriptor did not finish its reset",
        HdaError::CodecPowerTimeout => "hda: codec did not reach power state D0",
    }
}
