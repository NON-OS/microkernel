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

//! K1 (plan/50 P4): firmware's DMA protection is kept until the kernel's
//! tables are live, and on every failure path. Bring-up's failures never
//! reach a release: only AfterTables lets go, and only after its loop.

use super::dmar_flags::{dmar_platform_opt_in, record};
use super::handover::{amd_stop, vtd, Moment};

#[test]
fn vtd_lets_go_only_after_the_kernels_tables_or_under_the_dev_opt_in() {
    for dev in [false, true] {
        for platform in [false, true] {
            assert_eq!(vtd(true, dev, platform), Moment::AfterTables, "built in");
        }
    }
    assert_eq!(vtd(false, false, false), Moment::Never, "not built in: kept");
    assert_eq!(vtd(false, false, true), Moment::Never);
    assert_eq!(vtd(false, true, false), Moment::AtProbe, "the dev opt-in alone");
    assert_eq!(vtd(false, true, true), Moment::Never, "the platform flag keeps it");
}

#[test]
fn amd_units_firmware_left_running_keep_running_without_the_backend() {
    assert!(!amd_stop(false, false), "built without the backend: left as firmware set it");
    assert!(amd_stop(true, false), "stopped to be brought up with the kernel's tables");
    assert!(amd_stop(false, true), "the dev opt-in");
}

#[test]
fn the_dmar_platform_opt_in_is_bit_two_of_the_flags() {
    let _turn = super::flags_tests::FLAGS_HELD.lock().unwrap_or_else(|e| e.into_inner());
    record(0);
    assert!(!dmar_platform_opt_in());
    record(0b0000_0100);
    assert!(dmar_platform_opt_in());
    record(0b1111_1011);
    assert!(!dmar_platform_opt_in(), "interrupt remapping and x2APIC opt-out are not it");
    record(0);
}
