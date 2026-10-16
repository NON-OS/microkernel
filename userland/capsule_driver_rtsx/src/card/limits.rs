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

//! How long each step of a request may take. The kernel waits 35 s for a
//! driver's reply (src/services/lifecycle/reply_wait.rs); the longest
//! request here, the served card identified again in the look before it
//! (about 10.5 s) and then a multi-block write whose every step runs out
//! its time (CMD25, the data, CMD12, then the programming wait, about
//! 23.3 s, failed or not), stays inside it, so the driver and not the
//! kernel says why a request failed. The proofs sum these limits.

/// A command and its response.
pub const COMMAND_MS: u64 = 100;
/// A busy response (R1b): the card may hold the line while it works.
pub const BUSY_MS: u64 = 3_000;
/// A data phase, as rtsx_pci_sdmmc gives rtsx_pci_dma_transfer.
pub const DATA_MS: u64 = 10_000;
/// A card programming what it was sent, asked with CMD13 until it is done.
pub const PROGRAM_MS: u64 = 10_000;
/// A command buffer of register writes alone: power, pulls, timing, drive.
pub const REGS_MS: u64 = 100;
/// The SSC clock reprogrammed, and locked.
pub const SSC_MS: u64 = 2_000;
/// A card reporting power-up done to ACMD41, asked every 10 ms.
pub const POWER_UP_MS: u64 = 1_000;
