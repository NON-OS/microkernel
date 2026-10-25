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

//! Where the firmware found this loader: the removable-media path the UEFI
//! specification gives each architecture (3.5.1.1), which an image installs
//! the loader at. The kernel's check hashes the file found here.

use uefi::CStr16;

#[cfg(target_arch = "x86_64")]
pub const LOADER_PATH: &CStr16 = uefi::cstr16!("\\EFI\\BOOT\\BOOTX64.EFI");
#[cfg(target_arch = "aarch64")]
pub const LOADER_PATH: &CStr16 = uefi::cstr16!("\\EFI\\BOOT\\BOOTAA64.EFI");
