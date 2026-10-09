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

//! The words a log line uses for each error.

use super::kind::RtsxError;

impl RtsxError {
    pub const fn name(self) -> &'static [u8] {
        match self {
            Self::Claim => b"device claim refused",
            Self::MapBar => b"register BAR map refused",
            Self::PciCommand => b"PCI command write refused",
            Self::DmaMap => b"DMA buffer refused",
            Self::DmaAbove4G => b"DMA buffer above 4 GiB",
            Self::RegisterTimeout => b"internal register access timed out",
            Self::RegisterMismatch => b"internal register write not taken",
            Self::PhyTimeout => b"PHY register write timed out",
            Self::CmdOverflow => b"command buffer overflow",
            Self::CmdTimeout => b"command buffer timed out",
            Self::CmdFailed => b"command buffer failed",
            Self::Gone => b"reader no longer answers",
            Self::ResponseShort => b"response short",
            Self::ResponseStartBits => b"response start bits wrong",
            Self::ResponseCrc7 => b"response CRC7 error",
            Self::CardStatus => b"card did not take CMD55 (no APP_CMD in its status)",
            Self::NotSd => b"card does not answer ACMD41 (not an SD card)",
            Self::PowerUpTimeout => b"card still busy after 1 s of ACMD41",
            Self::UnknownCsd => b"CSD structure unknown",
            Self::ClockRange => b"card clock outside the SSC range",
            Self::DataTimeout => b"data transfer timed out",
            Self::DataFailed => b"data transfer failed",
        }
    }
}
