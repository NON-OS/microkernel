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

//! The failures a gen3 bring-up names, and the stage and step codes.

use super::super::bringup::BootError;
use super::super::select::Refusal;
use super::super::start::StartError;
use super::super::up::UpError;
use super::master::MasterError;

/// The client's stage codes this path reports.
pub const STAGE_READY: u8 = 0;
pub const STAGE_POWER_FAILED: u8 = 2;
pub const STAGE_DEAD_MMIO: u8 = 3;
pub const STAGE_FIRMWARE_FAILED: u8 = 4;
pub const STAGE_NO_DMA: u8 = 5;
pub const STAGE_NO_AIR_PATH: u8 = 8;

/// Why the radio is not up.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Failure {
    /// The register window is smaller than the registers this path uses.
    Window,
    /// The hardware revision register reads all ones: nothing answers.
    DeadMmio,
    /// The NIC or its clock could not be taken before the firmware was chosen.
    Start(StartError),
    /// The adapter has no bundled firmware this path runs.
    Refused(Refusal),
    /// The bundled image did not parse or plan.
    Image,
    /// The broker refused a DMA region.
    NoDma,
    /// The firmware did not boot to ALIVE.
    Boot(BootError),
    /// The firmware booted but did not come up for scanning.
    Up(UpError),
    /// The firmware came up, then raised its error cause while running.
    Lost,
    /// Bus Master did not go on before the boot.
    Master(MasterError),
    /// A command went unanswered within `COMMAND_MS`: the firmware stopped
    /// taking commands, though it raised no error cause.
    Unanswered,
    /// A transmitted frame went unanswered `TX_STUCK_MS`.
    TxStuck,
}

/// The step numbers the status reply carries, in bring-up order.
pub const STEP_WINDOW: u8 = 1;
pub const STEP_DEAD_MMIO: u8 = 2;
pub const STEP_START: u8 = 3;
pub const STEP_REFUSED: u8 = 4;
pub const STEP_IMAGE: u8 = 5;
pub const STEP_NO_DMA: u8 = 6;
pub const STEP_BOOT: u8 = 7;
pub const STEP_UP: u8 = 8;
pub const STEP_LOST: u8 = 9;
pub const STEP_MASTER: u8 = 10;
pub const STEP_UNANSWERED: u8 = 11;
pub const STEP_TX_STUCK: u8 = 12;
