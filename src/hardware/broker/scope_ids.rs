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

//! Device id ranges for scope_table.rs and scope_hosts.rs that are not a
//! network card's (scope_ids_nic.rs), each restating one capsule's discover
//! table; check_driver_scope.py fails on drift.

/// Intel eMMC hosts (capsule_driver_sdhci emmc/pci/ids.rs INTEL_EMMC).
pub(crate) const EMMC: &[u16] = &[
    0x0F14, 0x0F14, 0x0F50, 0x0F50, 0x2294, 0x2294, 0x0ACC, 0x0ACC, 0x1AA8, 0x1AA8, 0x5ACC, 0x5ACC,
    0x31CC, 0x31CC, 0x9D2B, 0x9D2B, 0x9DC4, 0x9DC4, 0x34C4, 0x34C4, 0x18DB, 0x18DB, 0x4B47, 0x4B47,
    0x4DC4, 0x4DC4,
];
