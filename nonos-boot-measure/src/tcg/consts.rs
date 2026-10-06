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

/// Logged and never extended. The log's first event, the Spec ID header, is one.
pub const EV_NO_ACTION: u32 = 0x0000_0003;
/// A UEFI application the firmware started, measured by its Authenticode digest.
pub const EV_EFI_BOOT_SERVICES_APPLICATION: u32 = 0x8000_0003;
pub const TPM_ALG_SHA256: u16 = 0x000B;
/// The boot manager's PCR: every application the firmware started.
pub const PCR_BOOT_MANAGER: u32 = 4;

/// Banks a log may declare. A PC has at most a handful.
pub const MAX_ALGS: usize = 8;
/// The longest digest of any bank, SHA-512.
pub const MAX_DIGEST: usize = 64;
/// Events one log may hold. A boot logs about a hundred.
pub const MAX_EVENTS: usize = 4096;
/// One event's body. Variable events carry a few KB, the GPT event a table.
pub const MAX_EVENT_BYTES: usize = 64 * 1024;
/// The whole log, as the loader carries it and the kernel reads it.
pub const MAX_LOG_BYTES: usize = 1024 * 1024;

pub(crate) const SPEC_ID_SIGNATURE: &[u8; 16] = b"Spec ID Event03\0";
