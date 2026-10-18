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

//! The firmware tables' entry (F-90): the digest over every ACPI table the
//! kernel handed the ACPI capsule (broker::acpi::table_measure), kept under
//! pid 0, which no process has. It sorts first and folds into the root like
//! a capsule's measurement, with no capabilities and the vendor's byte, so
//! the root an attestation signs changes when the machine's AML does.

use super::complete::mark_incomplete;
use super::entry::AttestedCapsule;
use super::table::TABLE;
use crate::security::dev_roots::Authority;

pub const FIRMWARE_PID: u32 = 0;

/// Replaces the entry with the digest over the list as it now stands.
pub fn record_firmware(measurement: [u8; 32]) {
    let mut table = TABLE.lock();
    let _ = table.remove(FIRMWARE_PID);
    let entry =
        AttestedCapsule { pid: FIRMWARE_PID, measurement, caps: 0, authority: Authority::Vendor };
    if !table.insert(entry) {
        drop(table);
        mark_incomplete();
        crate::sys::serial::println(b"[ATTEST] registry full; attestation disabled until reboot");
    }
}

/// More tables than the list keeps: the machine can no longer describe
/// what it runs, and says so the way a full registry does.
pub fn mark_firmware_cut() {
    mark_incomplete();
    crate::sys::serial::println(
        b"[ATTEST] firmware table list full; attestation disabled until reboot",
    );
}
