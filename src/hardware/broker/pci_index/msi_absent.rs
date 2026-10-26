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

//! Where no device may raise MSI (`msi_only!`: riscv64), the boot scan's
//! entries are not kept; nothing binds or tears down an MSI there.

use alloc::vec::Vec;

use crate::drivers::pci::types::MsiInfo;

pub(in crate::hardware::broker) fn install(entries: Vec<(u64, MsiInfo)>) {
    drop(entries);
}
