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

/// Hand the broker the devices firmware describes rather than a bus
/// enumerates, once its table exists. Returns how many it took.
///
/// aarch64 reads them from the device tree. x86_64's are ACPI's, which the
/// broker registers itself; riscv64 publishes none yet.
pub(crate) fn publish_described_devices() -> usize {
    #[cfg(target_arch = "aarch64")]
    return crate::arch::aarch64::platform_devices::publish();

    #[cfg(not(target_arch = "aarch64"))]
    return 0;
}
