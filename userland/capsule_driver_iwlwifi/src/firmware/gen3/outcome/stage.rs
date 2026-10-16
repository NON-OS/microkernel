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

//! The stage code and step a failure reports.

use super::super::bringup::BootError;
use super::kinds::*;

impl Failure {
    /// The stage code `nonos_wifi_client` reads.
    pub fn stage(&self) -> u8 {
        match self {
            Failure::Window | Failure::DeadMmio => STAGE_DEAD_MMIO,
            Failure::Start(_)
            | Failure::Boot(BootError::Start(_))
            | Failure::Boot(BootError::NoNicAccess) => STAGE_POWER_FAILED,
            Failure::Refused(_) => STAGE_NO_AIR_PATH,
            Failure::NoDma => STAGE_NO_DMA,
            Failure::Image | Failure::Boot(_) | Failure::Up(_) | Failure::Lost => {
                STAGE_FIRMWARE_FAILED
            }
        }
    }

    /// The step it stopped at.
    pub fn step(&self) -> u8 {
        match self {
            Failure::Window => STEP_WINDOW,
            Failure::DeadMmio => STEP_DEAD_MMIO,
            Failure::Start(_) => STEP_START,
            Failure::Refused(_) => STEP_REFUSED,
            Failure::Image => STEP_IMAGE,
            Failure::NoDma => STEP_NO_DMA,
            Failure::Boot(_) => STEP_BOOT,
            Failure::Up(_) => STEP_UP,
            Failure::Lost => STEP_LOST,
        }
    }
}
