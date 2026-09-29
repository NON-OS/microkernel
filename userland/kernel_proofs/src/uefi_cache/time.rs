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

use crate::arch::x86_64::uefi::tables::{EfiTime, EfiTimeCapabilities};

const EFI_UNSUPPORTED: u64 = 0x8000_0000_0000_0003;

pub(super) extern "efiapi" fn get_time(_: *mut EfiTime, _: *mut EfiTimeCapabilities) -> u64 {
    EFI_UNSUPPORTED
}
pub(super) extern "efiapi" fn set_time(_: *const EfiTime) -> u64 {
    EFI_UNSUPPORTED
}
pub(super) extern "efiapi" fn get_wakeup_time(_: *mut u8, _: *mut u8, _: *mut EfiTime) -> u64 {
    EFI_UNSUPPORTED
}
pub(super) extern "efiapi" fn set_wakeup_time(_: u8, _: *const EfiTime) -> u64 {
    EFI_UNSUPPORTED
}
