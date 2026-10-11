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

//! The one-line reason a failed bring-up reports.

use super::EmmcError;

/// The words a failed bring-up attempt reports to the shared bring-up
/// schedule, which logs the last one when it gives up.
pub const fn reason(e: EmmcError) -> &'static str {
    match e {
        EmmcError::Broker(_) => "emmc: broker refused a claim or grant",
        EmmcError::Window => "emmc: register window too small",
        EmmcError::NotEmbedded => "emmc: host slot is not embedded",
        EmmcError::NoAdma => "emmc: host has no ADMA2",
        EmmcError::HostReset => "emmc: host reset did not finish",
        EmmcError::ClockUnstable => "emmc: host clock never stable",
        EmmcError::NoPower => "emmc: bus power did not stay on",
        EmmcError::NoVoltage => "emmc: no voltage shared by host and card",
        EmmcError::Inhibit(_) => "emmc: command lines stayed busy",
        EmmcError::CmdTimeout(_) => "emmc: card did not answer a command",
        EmmcError::CmdError { .. } => "emmc: command failed on the bus",
        EmmcError::DataError { .. } => "emmc: data transfer failed on the bus",
        EmmcError::NoCompletion(_) => "emmc: host posted no completion in time",
        EmmcError::NoCard => "emmc: no card answered CMD1",
        EmmcError::CardBusy => "emmc: card never finished power up",
        EmmcError::Status { .. } => "emmc: card reported an error status",
        EmmcError::Locked => "emmc: card is password locked",
        EmmcError::SwitchRefused(_) => "emmc: card refused a mode switch",
        EmmcError::BadState(_) => "emmc: card in an unexpected state",
        EmmcError::CardStuck => "emmc: card stayed busy",
        EmmcError::NoCapacity => "emmc: card reports no usable capacity",
        EmmcError::BusWidth => "emmc: no data bus width works",
        EmmcError::DmaAddress => "emmc: DMA region out of host reach",
        EmmcError::OutOfRange => "emmc: sectors outside the disk or buffer",
    }
}
