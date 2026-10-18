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

//! The firmware tables' entry (F-90): one entry under pid 0, first in the
//! set, carrying the tables' digest with no capabilities and the vendor's
//! byte; recording again replaces it, and the set still folds to the root.

use crate::registry_support::{refold, TABLE_IN_USE};
use crate::security::attest_registry::{
    attested_count, forget_attested, record_firmware, registry_entries, registry_root, FIRMWARE_PID,
};

#[test]
fn the_firmware_entry_is_one_replaced_entry_under_pid_0() {
    let _guard = TABLE_IN_USE.lock().unwrap();
    let (before, count) = (registry_root(), attested_count());
    record_firmware([0x11; 32]);
    record_firmware([0x22; 32]);
    assert_eq!(attested_count(), count + 1, "recorded again, replaced");
    let entries = registry_entries();
    assert_eq!(entries[..4], FIRMWARE_PID.to_be_bytes(), "pid 0 sorts first");
    assert_eq!(entries[4..36], [0x22; 32]);
    assert_eq!(entries[36..44], [0; 8], "no capabilities");
    assert_eq!(entries[44], 0, "the vendor's byte");
    assert_eq!(refold(&entries), registry_root());
    assert_ne!(registry_root(), before, "the tables move the signed root");
    forget_attested(FIRMWARE_PID);
    assert_eq!(registry_root(), before, "the table must be left as it was found");
}
