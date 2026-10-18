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

//! Bits from Linux's include/linux/rtsx_pci.h the model needs and the
//! driver did not name before this unit, kept here so the model speaks the
//! hardware's values and not only the driver's.

/// BIPR: a card-detect change on the SD slot, latched until written back.
pub const SD_INT: u32 = 1 << 25;
/// BIPR: the SD slot's mechanical write-protect switch is set.
pub const SD_WRITE_PROTECT: u32 = 1 << 19;
/// SD_TRANSFER: data from the ring buffer to the card, no command first.
pub const SD_TM_AUTO_WRITE_3: u8 = 0x01;
/// DMACTL: reset the DMA engine. RBCTL: flush the ring buffer.
pub const DMA_RST: u8 = 0x80;
pub const RB_FLUSH: u8 = 0x80;
/// HDBCTLR bit 29: from the device to memory.
pub const HDBCTLR_TO_HOST: u32 = 1 << 29;
/// R1 card status (SD Physical Layer 4.10.1).
pub const READY_FOR_DATA: u32 = 1 << 8;
pub const APP_CMD: u32 = 1 << 5;
pub const WP_VIOLATION: u32 = 1 << 26;
/// The card's states, CURRENT_STATE in R1 bits 12:9.
pub const IDLE: u32 = 0;
pub const READY: u32 = 1;
pub const IDENT: u32 = 2;
pub const STBY: u32 = 3;
pub const TRAN: u32 = 4;
pub const RCV: u32 = 6;
pub const PRG: u32 = 7;
