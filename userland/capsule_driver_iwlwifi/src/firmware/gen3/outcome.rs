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

//! How far a gen3 bring-up got, in the terms the Wi-Fi control family and the
//! boot console use. A failure names its step, carries a detail word (the
//! refused id, the ALIVE status, the command that went unanswered) and maps
//! to the stage code `nonos_wifi_client` reads, so a panel on a machine with
//! no serial port still says whether the card was refused, never powered,
//! got no DMA memory or ran a firmware that did not come up.

use super::bringup::BootError;
use super::dev::WaitError;
use super::layout::LayoutError;
use super::select::Refusal;
use super::start::StartError;
use super::up::UpError;

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

fn wait_code(w: WaitError) -> u32 {
    match w {
        WaitError::TimedOut => 1,
        WaitError::FirmwareError => 2,
        WaitError::TooLarge => 3,
    }
}

fn start_code(e: StartError) -> u32 {
    match e {
        StartError::NotReady => 1,
        StartError::ClockNotReady => 2,
        StartError::NoNicAccess => 3,
    }
}

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

    /// The detail word: what was refused, which status or command, and how.
    pub fn detail(&self) -> u32 {
        match *self {
            Failure::Window
            | Failure::DeadMmio
            | Failure::Image
            | Failure::NoDma
            | Failure::Lost => 0,
            Failure::Start(e) => start_code(e),
            Failure::Refused(r) => match r {
                Refusal::NotSoDevice(id) => 0x1_0000 | u32::from(id),
                Refusal::MacNotBundled(mac) => 0x2_0000 | u32::from(mac),
                Refusal::RfNotBundled(rf) => 0x3_0000 | u32::from(rf),
                Refusal::CdbNotBundled => 0x4_0000,
                Refusal::MacStepNotBundled(step) => 0x5_0000 | u32::from(step),
                Refusal::ImageNotBundled(image) => 0x6_0000 | image as u32,
            },
            Failure::Boot(b) => match b {
                BootError::Start(e) => 0x10 | start_code(e),
                BootError::Layout(LayoutError::Plan) => 0x21,
                BootError::Layout(LayoutError::TooManySections) => 0x22,
                BootError::Layout(LayoutError::Write) => 0x23,
                BootError::NoAliveCause => 0x30,
                BootError::NoAliveNotif(w) => 0x40 | wait_code(w),
                BootError::NotAlive(status) => 0x5_0000 | u32::from(status),
                BootError::Pnvm(w) => 0x60 | wait_code(w),
                BootError::NoNicAccess => 0x70,
            },
            Failure::Up(u) => match u {
                UpError::Command(group, cmd, w) => {
                    0x0100_0000 | u32::from(group) << 16 | u32::from(cmd) << 8 | wait_code(w)
                }
                UpError::NoInitComplete(w) => 0x0200_0000 | wait_code(w),
                UpError::BadNvm => 0x0300_0000,
                UpError::BadMcc => 0x0400_0000,
                UpError::Unsupported(_) => 0x0500_0000,
            },
        }
    }

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
        }
    }
}
