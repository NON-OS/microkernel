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

use super::amd_control::{stopped, IOMMU_EN};
use super::behaviour::has_protected_regions;
use super::global::{gcmd_without_te, GCMD_ONE_SHOT, GCMD_TE};

#[test]
fn a_running_amd_unit_is_stopped_and_nothing_else_changes() {
    assert_eq!(stopped(0), None);
    assert_eq!(stopped(1 << 5), None, "not enabled: left alone");
    let running = IOMMU_EN | (1 << 2) | (1 << 12) | (1 << 17) | (1 << 50) | (1 << 5) | (1 << 8);
    let off = stopped(running).unwrap();
    assert_eq!(off & IOMMU_EN, 0);
    assert_eq!(off & ((1 << 2) | (1 << 12) | (1 << 17) | (1 << 50)), 0);
    assert_eq!(off, (1 << 5) | (1 << 8), "coherency and timeout bits are kept");
}

#[test]
fn turning_vtd_translation_off_repeats_no_one_shot() {
    let status = GCMD_TE | GCMD_ONE_SHOT | (1 << 26);
    let cmd = gcmd_without_te(status);
    assert_eq!(cmd & GCMD_TE, 0);
    assert_eq!(cmd & GCMD_ONE_SHOT, 0);
    assert_eq!(cmd & (1 << 26), 1 << 26, "queued invalidation stays as it was");
}

#[test]
fn pmen_is_touched_only_on_units_that_have_it() {
    assert!(!has_protected_regions(0));
    assert!(has_protected_regions(1 << 5));
    assert!(has_protected_regions(1 << 6));
    assert!(!has_protected_regions(1 << 4));
}
