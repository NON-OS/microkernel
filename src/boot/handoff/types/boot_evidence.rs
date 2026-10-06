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

//! The module kinds the loader uses for what the kernel's check of the
//! bootloader reads, beside the install regions. Shared with the loader's
//! `Module::kind`.

/// The bootloader file as the firmware read it.
pub const MODULE_KIND_LOADER_IMAGE: u32 = 1;
/// The firmware's TCG event log, crypto-agile format, as the loader copied it.
pub const MODULE_KIND_TCG_LOG: u32 = 4;
/// The loader's own v4 trailer, `EFI/nonos/bootloader.trailer` on the ESP.
pub const MODULE_KIND_BOOT_TRAILER: u32 = 5;
/// The signed boot-root record, `EFI/nonos/boot_root.approval` on the ESP.
pub const MODULE_KIND_BOOT_ROOT_RECORD: u32 = 6;
/// The package store from its header on, read by the loader through the
/// firmware's disk driver.
pub const MODULE_KIND_STORE: u32 = 7;
/// Ranges of the boot disk the loader copied: the live plan's sector and the
/// files it names for import (`hardware::block_device::mirror`).
pub const MODULE_KIND_DISK_MIRROR: u32 = 8;
