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

use crate::arch::x86_64::uefi::types::Guid;

const EFI_UNSUPPORTED: u64 = 0x8000_0000_0000_0003;

pub(super) extern "efiapi" fn set_virtual_address_map(_: u64, _: u64, _: u32, _: *const u8) -> u64 {
    EFI_UNSUPPORTED
}
pub(super) extern "efiapi" fn convert_pointer(_: u64, _: *mut *const u8) -> u64 {
    EFI_UNSUPPORTED
}
pub(super) extern "efiapi" fn get_next_variable_name(
    _: *mut u64,
    _: *mut u16,
    _: *mut Guid,
) -> u64 {
    EFI_UNSUPPORTED
}
pub(super) extern "efiapi" fn set_variable(
    _: *const u16,
    _: *const Guid,
    _: u32,
    _: u64,
    _: *const u8,
) -> u64 {
    EFI_UNSUPPORTED
}
pub(super) extern "efiapi" fn get_next_high_mono_count(_: *mut u32) -> u64 {
    EFI_UNSUPPORTED
}
pub(super) extern "efiapi" fn reset_system(_: u32, _: u64, _: u64, _: *const u8) -> ! {
    panic!("reset_system is not part of this test")
}
pub(super) extern "efiapi" fn update_capsule(_: *const *const u8, _: u64, _: u64) -> u64 {
    EFI_UNSUPPORTED
}
pub(super) extern "efiapi" fn query_capsule_capabilities(
    _: *const *const u8,
    _: u64,
    _: *mut u64,
    _: *mut u32,
) -> u64 {
    EFI_UNSUPPORTED
}
pub(super) extern "efiapi" fn query_variable_info(
    _: u32,
    _: *mut u64,
    _: *mut u64,
    _: *mut u64,
) -> u64 {
    EFI_UNSUPPORTED
}
