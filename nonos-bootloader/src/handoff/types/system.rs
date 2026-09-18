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

/// ACPI RSDP pointer for kernel ACPI table parsing.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AcpiInfo {
    pub rsdp: u64,
}

/// SMBIOS entry point for hardware inventory.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SmbiosInfo {
    pub entry: u64,
}

/// One region the loader leaves in memory for the kernel: base, size and
/// what it is. `kind` 1 is the loader's own image, 2 is the kernel image
/// file as loaded and verified. Layout matches the kernel's `Module`.
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Module {
    pub base: u64,
    pub size: u64,
    pub kind: u32,
    pub reserved: u32,
}

pub const MODULE_KIND_LOADER_IMAGE: u32 = 1;
pub const MODULE_KIND_KERNEL_IMAGE: u32 = 2;

/// Boot modules the loader leaves for the kernel: today the two images the
/// installer writes to a disk.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Modules {
    pub ptr: u64,
    pub count: u32,
    pub reserved: u32,
}

/// Timing info for kernel clock initialization.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Timing {
    pub tsc_hz: u64,
    pub unix_epoch_ms: u64,
}
