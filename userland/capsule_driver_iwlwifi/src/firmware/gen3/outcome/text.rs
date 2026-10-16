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

//! The phrase a failure prints on the boot console.

use super::super::{bringup::BootError, select::Refusal, start::StartError, up::UpError};
use super::kinds::Failure;

impl Failure {
    /// One phrase for the boot console.
    pub fn text(&self) -> &'static str {
        match *self {
            Failure::Window => "the register window is too small",
            Failure::DeadMmio => "the registers read back all ones",
            Failure::Start(StartError::NotReady)
            | Failure::Boot(BootError::Start(StartError::NotReady)) => {
                "the NIC never reported ready (the management engine may hold it)"
            }
            Failure::Start(StartError::ClockNotReady)
            | Failure::Boot(BootError::Start(StartError::ClockNotReady)) => {
                "the MAC clock never came up"
            }
            Failure::Start(StartError::NoNicAccess)
            | Failure::Boot(BootError::Start(StartError::NoNicAccess))
            | Failure::Boot(BootError::NoNicAccess) => {
                "the MAC would not wake for a register write"
            }
            Failure::Refused(Refusal::NotSoDevice(_)) => {
                "not an SO platform this driver has firmware for"
            }
            Failure::Refused(Refusal::MacNotBundled(_)) => "no bundled firmware for this MAC type",
            Failure::Refused(Refusal::RfNotBundled(_)) => "no bundled firmware for this RF module",
            Failure::Refused(Refusal::CdbNotBundled) => {
                "a dual-radio module, whose firmware is not bundled"
            }
            Failure::Refused(Refusal::MacStepNotBundled(_)) => {
                "a Meteor Lake MAC step whose firmware is not bundled"
            }
            Failure::Refused(Refusal::ImageNotBundled(_)) => {
                "its firmware is known but not in this build"
            }
            Failure::Image => "the bundled firmware image did not parse",
            Failure::NoDma => "the broker refused a DMA region",
            Failure::Boot(BootError::Layout(_)) => "the firmware did not fit the DMA regions",
            Failure::Boot(BootError::NoAliveCause) => "the firmware never raised ALIVE",
            Failure::Boot(BootError::NoAliveNotif(_)) => {
                "ALIVE was raised but no ALIVE notification came"
            }
            Failure::Boot(BootError::NotAlive(_)) => "the ALIVE status was not 0xCAFE",
            Failure::Boot(BootError::Pnvm(_)) => "the platform NVM step did not complete",
            Failure::Up(UpError::Command(..)) => "a firmware command went unanswered",
            Failure::Up(UpError::NoInitComplete(_)) => "INIT_COMPLETE never came",
            Failure::Up(UpError::BadNvm) => "the NVM reply was malformed",
            Failure::Up(UpError::BadMcc) => "the regulatory reply was malformed",
            Failure::Up(UpError::Unsupported(what)) => what,
            Failure::Lost => "the firmware raised its error cause while running",
            Failure::Master(e) => e.text(),
            Failure::Unanswered => "a firmware command went unanswered",
        }
    }
}
