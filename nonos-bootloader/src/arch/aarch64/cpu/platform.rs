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

//! What differs by architecture in how the loader treats the platform and
//! the kernel image.

/// Arm makes FEAT_RNG optional and the loader's other inputs (the counter,
/// the RTC) are guessable, so a boot here must draw from the firmware's
/// EFI_RNG_PROTOCOL or from RNDR, or refuse.
pub const REQUIRE_STRONG_ENTROPY: bool = true;

/// The kernel relocates itself (start.S), as an arm64 Image does: it may lie
/// in any 2 MiB block at its link offset inside one.
pub const IMAGE_ALIGN: Option<u64> = Some(0x20_0000);

/// No fixed TPM window: an Arm platform names its TPM in ACPI or the device
/// tree, and nothing answers at the PC address, where a read takes a data
/// abort. The firmware's TCG2 protocol is the TPM here.
pub const PC_TPM_WINDOW: bool = false;

/// The kernel image's ELF machine: EM_AARCH64.
pub const ELF_MACHINE: u16 = crate::loader::types::elf_machine::EM_AARCH64;
