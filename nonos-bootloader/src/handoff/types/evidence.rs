// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The module kinds for what the kernel's check of this loader reads. They
//! follow the install regions' kinds and match the kernel's
//! `boot_evidence.rs`.

/// The firmware's TCG event log, crypto-agile format.
pub const MODULE_KIND_TCG_LOG: u32 = 4;
/// This loader's own v4 trailer, `EFI/nonos/bootloader.trailer`.
pub const MODULE_KIND_BOOT_TRAILER: u32 = 5;
/// The signed boot-root record, `EFI/nonos/boot_root.approval`.
pub const MODULE_KIND_BOOT_ROOT_RECORD: u32 = 6;
