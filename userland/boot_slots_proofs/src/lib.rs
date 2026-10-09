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

//! `MkBootSlots`, held from three sides: the kernel's footer reader against
//! the bootloader's own parser, the slots against the enroll tool's tree, and
//! the kernel's record against libc's reader.

/// The bootloader's footer parser, unchanged: the reference the kernel's
/// reader is held to, byte for byte. Public, as it is in the loader.
#[path = "../../../nonos-bootloader/src/image_format/mod.rs"]
pub mod image_format;

#[cfg(test)]
#[path = "../../libc/src/boot_slots/record.rs"]
mod libc_slots;

#[cfg(test)]
mod fixture;

#[cfg(test)]
mod slots;
