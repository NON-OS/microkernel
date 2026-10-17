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

//! The kinds `MkInstallSource` serves.

/// Module kinds the bootloader records, shared with its `Module::kind`.
pub const KIND_LOADER_IMAGE: u32 = 1;
pub const KIND_KERNEL_IMAGE: u32 = 2;
/* Not an image but a small record: the partition the loader was read
 * from, so the installer can leave the boot stick off its list. */
pub const KIND_BOOT_MEDIA: u32 = crate::boot::handoff::types::MODULE_KIND_BOOT_MEDIA;
/* The loader's own trailer and its signed boot-root record,
 * `bootloader.trailer` and `boot_root.approval` on the ESP. They describe the
 * loader the installer copies, not this machine, and a disk without them
 * halts at "The bootloader could not be checked". */
pub const KIND_BOOT_TRAILER: u32 = crate::boot::handoff::types::MODULE_KIND_BOOT_TRAILER;
pub const KIND_BOOT_ROOT_RECORD: u32 = crate::boot::handoff::types::MODULE_KIND_BOOT_ROOT_RECORD;
/* The release's approval of this kernel, `kernel.approval` on the ESP. The
 * loader hands it over in the attest policy rather than as a module, so it
 * is read from there, under a number no module kind takes. */
pub const KIND_KERNEL_APPROVAL: u32 = 0x100;
/* The disk this boot's block layer chose, by backend and instance
 * (install_source_in_use.rs), also under a number no module kind takes. */
pub const KIND_IN_USE_DISK: u32 = 0x101;
