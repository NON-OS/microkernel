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

/// Whether the boot must refuse when no hardware or firmware source gave
/// entropy. x86_64 keeps the loader's existing rule (its sources are mixed,
/// none demanded); aarch64 demands one (arch::aarch64::cpu).
pub const REQUIRE_STRONG_ENTROPY: bool = false;

/// The kernel image lies where its ELF says (or anywhere, mapped high): it
/// does not relocate itself.
pub const IMAGE_ALIGN: Option<u64> = None;

/// The kernel's own claim of its footprint: none here, the segments say it.
pub fn image_size(_head: &[u8]) -> Option<u64> {
    None
}

/// A TPM may answer at the PC Client platform's fixed MMIO window
/// (0xFED4_0000), so the loader may probe it there.
pub const PC_TPM_WINDOW: bool = true;

/// The kernel image's ELF machine: EM_X86_64.
pub const ELF_MACHINE: u16 = crate::loader::types::elf_machine::EM_X86_64;
