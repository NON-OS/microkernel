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

use super::state::{BOOT_CHAIN_VERIFIED, BOOT_MEASUREMENTS};
use core::sync::atomic::Ordering;

pub fn record_boot_measurements(
    bootloader_hash: [u8; 32],
    kernel_hash: [u8; 32],
    uefi_secure_boot: bool,
) {
    let mut measurements = BOOT_MEASUREMENTS.write();
    measurements.bootloader_hash = bootloader_hash;
    measurements.kernel_hash = kernel_hash;
    measurements.uefi_secure_boot = uefi_secure_boot;
    measurements.boot_timestamp = crate::arch::cpu::read_time_counter();
}

pub fn is_boot_chain_verified() -> bool {
    BOOT_CHAIN_VERIFIED.load(Ordering::SeqCst)
}
