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

//! The detail word a failure reports.

use super::super::bringup::BootError;
use super::super::layout::LayoutError;
use super::super::select::Refusal;
use super::super::up::UpError;
use super::kinds::Failure;
use super::master::MasterError;
use super::subcodes::{start_code, wait_code};

impl Failure {
    /// The detail word: what was refused, which status or command, and how.
    pub fn detail(&self) -> u32 {
        match *self {
            Failure::Window
            | Failure::DeadMmio
            | Failure::Image
            | Failure::NoDma
            | Failure::Lost => 0,
            Failure::Start(e) => start_code(e),
            Failure::Master(MasterError::NotStopped) => 1,
            Failure::Master(MasterError::Refused) => 2,
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
}
