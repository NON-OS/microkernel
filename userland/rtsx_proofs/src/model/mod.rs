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

//! A Realtek RTS5227 card reader with an SD card in its slot, as the
//! driver's registers reach it (src/hw/mmio.rs). Written from what Linux's
//! rtsx_pcr.c and rtsx_pci_sdmmc.c expect of the part: the command buffer
//! run from DMA memory with its results written back over it, one internal
//! register at a time through HAIMR, the SD engine's command and data
//! transfers, ADMA through one scatter-gather entry, and BIPR's completion,
//! card-detect and write-protect bits. One per test thread, answering each
//! access at once; a test turns knobs to make it stall, lose its card, or
//! leave the bus.

mod adma;
mod card;
mod card_cmd;
mod card_data;
mod card_id;
mod card_poll;
mod cmdbuf;
mod host;
mod internal;
mod knobs;
mod late;
mod linux;
mod move_data;
mod predecessor;
mod reader;
mod run;
mod sd_command;
mod sd_engine;
mod slot;

pub use card::Card;
pub use knobs::{Knobs, Phase};
pub use late::complete_late_at_teardown;
pub use linux::*;
pub use predecessor::left_armed;
pub use reader::{with, Reader};
