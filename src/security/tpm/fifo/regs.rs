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

//! FIFO register file of locality 0 (TCG PC Client Platform TPM Profile).
//!
//! Offsets are from the start of the window; locality 0 is its first page,
//! so they are also the architectural offsets.

/// Locality access: request, grant and relinquish.
pub(crate) const TPM_ACCESS: u32 = 0x00;
/// Status, burst count and the command doorbell.
pub(crate) const TPM_STS: u32 = 0x18;
/// Command bytes in, response bytes out, one byte per access.
pub(crate) const TPM_DATA_FIFO: u32 = 0x24;

pub(crate) const ACCESS_REQUEST_USE: u8 = 0x02;
pub(crate) const ACCESS_ACTIVE_LOCALITY: u8 = 0x20;
/// Set when the other bits of the access register mean something.
pub(crate) const ACCESS_VALID: u8 = 0x80;

pub(crate) const STS_VALID: u8 = 0x80;
pub(crate) const STS_COMMAND_READY: u8 = 0x40;
pub(crate) const STS_GO: u8 = 0x20;
pub(crate) const STS_DATA_AVAIL: u8 = 0x10;
pub(crate) const STS_EXPECT: u8 = 0x08;

/// Bits 8..23 of the status register: how many bytes the FIFO takes or
/// gives before the part must be asked again.
pub(crate) const STS_BURST_SHIFT: u32 = 8;
pub(crate) const STS_BURST_MASK: u32 = 0xFFFF;

/// TCG TIMEOUT_A: locality grant, and a nonzero burst count.
pub(crate) const TIMEOUT_A_MS: u64 = 750;
/// TCG TIMEOUT_B: commandReady after asking for it.
pub(crate) const TIMEOUT_B_MS: u64 = 2_000;
/// TCG TIMEOUT_C: stsValid after a FIFO access, and each next response byte.
pub(crate) const TIMEOUT_C_MS: u64 = 200;
/// Execution: from tpmGo to the first response byte. The commands this
/// kernel sends are an HMAC key, an ECC P-256 key and a quote, each well
/// inside this on a discrete part; the bound exists so a hung part costs
/// thirty seconds and not the machine.
pub(crate) const TIMEOUT_D_MS: u64 = 30_000;

/// Response header: tag, size, response code.
pub(crate) const HEADER_LEN: usize = 10;
